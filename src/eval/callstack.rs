use std::{ops::Index, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    coloring::ColoredExprArena,
    eval::{
        CacheBackend, EvalState, ValueResult,
        hash::ValueHash,
        value::{RecordRepr, Thunk, Value},
    },
    mir::Param,
};

#[derive(Clone, Default, Debug)]
pub struct Callstack<'id>(Rc<[Thunk<'id>]>);

impl<'id> Callstack<'id> {
    /// pushes a parameter to the callstack at `depth`.
    ///
    /// Everything above or at `depth` will be removed.
    pub fn with_pushed(&self, depth: Param, arg: Thunk<'id>) -> Self {
        Self(
            self.0[..depth.nesting_level()]
                .iter()
                .cloned()
                .chain([arg])
                .collect(),
        )
    }
}

impl<'id> Index<Param> for Callstack<'id> {
    type Output = Thunk<'id>;

    fn index(&self, index: Param) -> &Self::Output {
        &self.0[index.nesting_level()]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CallstackRecord(Vec<ValueHash>);

impl<'id> RecordRepr<'id> for Callstack<'id> {
    type Record = CallstackRecord;

    fn to_record(
        &self,
        _: &ColoredExprArena<'id>,
        child_hash: impl Fn(&ValueResult<'id>) -> Option<ValueHash>,
    ) -> Option<Self::Record> {
        Some(CallstackRecord(
            self.0
                .iter()
                .map(|thunk| child_hash(&Ok(Value::Thunk(thunk.clone()))))
                .collect::<Option<_>>()?,
        ))
    }

    // TODO: this shouldn't lead to re-evaluation of thunks
    fn from_record<B: CacheBackend>(record: Self::Record, state: &EvalState<'id, '_, B>) -> Self {
        let thunks = record
            .0
            .iter()
            .map(|hash| Thunk::new_forced(Ok(state.cache().get_value(*hash, state))))
            .collect();

        Self(thunks)
    }
}
