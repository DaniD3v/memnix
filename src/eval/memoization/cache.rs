use std::fmt::Debug;

use cached::{ConcurrentCached, DiskCache};

use crate::eval::{
    CacheBackend, EvalState, ValueResult,
    hash::{EvalHash, ValueHash},
    memoization::Disk,
    value::{RecordRepr, Value, ValueRecord},
};

pub struct Cache<B: CacheBackend> {
    // hash(obj) -> obj
    values: B::Store<ValueHash, ValueRecord>,
    // hash(expr + callstack) -> hash(result)
    evals: B::Store<EvalHash, ValueHash>,
}

impl<B: CacheBackend> Cache<B> {
    pub fn get_result<'id>(
        &self,
        key: EvalHash,
        state: &EvalState<'id, '_, B>,
    ) -> Option<Value<'id>> {
        let result = expect_cache_failure(self.evals.cache_get(&key))?;
        Some(self.get_value(result, state))
    }

    /// Memoizes `result` under `key`.
    ///
    /// Unstorable results (deferred thunks or errors) are silently skipped.
    pub fn store_result<'id>(
        &self,
        key: EvalHash,
        result: &ValueResult<'id>,
        state: &EvalState<'id, '_, B>,
    ) {
        let Some(result) = self.store_value(result, state) else {
            return;
        };
        expect_cache_failure(self.evals.cache_set(key, result));
    }

    pub fn get_value<'id>(&self, hash: ValueHash, state: &EvalState<'id, '_, B>) -> Value<'id> {
        let record = expect_cache_failure(self.values.cache_get(&hash))
            .expect("the hash should be in the value store");

        Value::from_record(record, state)
    }

    /// Inserts `value` into the value store and returns its hash,
    /// or `None` if the value can't be serialized.
    pub fn store_value<'id>(
        &self,
        value: &ValueResult<'id>,
        state: &EvalState<'id, '_, B>,
    ) -> Option<ValueHash> {
        let record = value.to_record(state.arena(), |child| self.store_value(child, state))?;
        let hash = ValueHash::from_record(&record);

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
