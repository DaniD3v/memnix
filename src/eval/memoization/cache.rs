use std::fmt::Debug;

use cached::{ConcurrentCached, DiskCache};

use crate::eval::{
    CacheBackend, EvalState,
    hash::{EvalHash, EvalHashable},
    memoization::{
        Disk,
        runtime_value_record::{RecordRepr, ValueRecord},
    },
    value::RuntimeValue,
};

pub struct Cache<B: CacheBackend> {
    // hash(obj) -> obj
    values: B::Store<EvalHash, ValueRecord>,
    // hash(expr + callstack) -> hash(result)
    evals: B::Store<EvalHash, EvalHash>,
}

impl<B: CacheBackend> Cache<B> {
    pub fn get_result<'id>(
        &self,
        key: EvalHash,
        state: &EvalState<'id, '_, B>,
    ) -> Option<RuntimeValue<'id>> {
        let result = expect_cache_failure(self.evals.cache_get(&key))?;
        Some(self.get_value(result, state))
    }

    /// Memoizes `result` under `key`.
    ///
    /// Unstorable results (deferred thunks or errors) are silently skipped.
    pub fn store_result<'id>(
        &self,
        key: EvalHash,
        result: &RuntimeValue<'id>,
        state: &EvalState<'id, '_, B>,
    ) {
        let Some(result) = self.store_value(result, state) else {
            return;
        };
        expect_cache_failure(self.evals.cache_set(key, result));
    }

    pub(super) fn get_value<'id>(
        &self,
        hash: EvalHash,
        state: &EvalState<'id, '_, B>,
    ) -> RuntimeValue<'id> {
        let record = expect_cache_failure(self.values.cache_get(&hash))
            .expect("the hash should be in the value store");

        RuntimeValue::from_record(record, state)
    }

    /// Inserts `value` into the value store and returns its hash,
    /// or `None` if the value can't be serialized.
    ///
    /// Expects the records children to already be inserted
    pub(super) fn store_value<'id>(
        &self,
        value: &RuntimeValue<'id>,
        state: &EvalState<'id, '_, B>,
    ) -> Option<EvalHash> {
        let record = value.clone().to_record(state)?;
        let hash = value.compute_hash(state);

        expect_cache_failure(self.values.cache_set(hash, record));
        Some(hash)
    }
}

impl Cache<Disk> {
    pub fn new() -> Self {
        Self {
            values: DiskCache::new("value_store")
                .build()
                .expect("DiskCache should be valid"),
            evals: DiskCache::new("eval_cache")
                .build()
                .expect("DiskCache should be valid"),
        }
    }
}

fn expect_cache_failure<V, E: Debug>(result: Result<Option<V>, E>) -> Option<V> {
    result.unwrap_or_else(|error| panic!("the cache should be usable: {error:?}",))
}
