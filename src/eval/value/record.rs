use serde::{Deserialize, Serialize};

use crate::{
    coloring::{Color, ColoredExprArena},
    eval::{
        CacheBackend, EvalState, ValueResult,
        callstack::Callstack,
        hash::ValueHash,
        value::{Lambda, Number, Thunk, Value, thunk::ThunkState},
    },
};

pub trait RecordRepr<'id>: Sized {
    type AsRecord;

    fn to_record(
        &self,
        arena: &ColoredExprArena<'id>,
        // this is a lambda so the caller can inject a side effect
        hash_child: impl Fn(&ValueResult<'id>) -> Option<ValueHash>,
    ) -> Option<Self::AsRecord>;
    fn from_record<B: CacheBackend>(record: Self::AsRecord, state: &EvalState<'id, '_, B>) -> Self;
}

/// A `RuntimeValue` serialized as an owned record.
///
/// Expressions are referenced by their `Color`,
/// captures by their `EvalHash`.
#[derive(Serialize, Deserialize, Clone)]
pub enum ValueRecord {
    Lambda(LambdaRecord),

    // TODO: cache deferred thunks once lambdas only capture used params
    // TODO: cache `EvalError`s too
    Number(Number),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LambdaRecord {
    body: Color,
    captures: CallstackRecord,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CallstackRecord(Vec<ValueHash>);

impl<'id> RecordRepr<'id> for ValueResult<'id> {
    type AsRecord = ValueRecord;

    fn to_record(
        &self,
        arena: &ColoredExprArena<'id>,
        hash_children: impl Fn(&ValueResult<'id>) -> Option<ValueHash>,
    ) -> Option<Self::AsRecord> {
        match self {
            Self::Ok(value) => value.to_record(arena, hash_children),
            Self::Err(_) => None,
        }
    }

    fn from_record<B: CacheBackend>(record: ValueRecord, state: &EvalState<'id, '_, B>) -> Self {
        Ok(Value::from_record(record, state))
    }
}

impl<'id> RecordRepr<'id> for Value<'id> {
    type AsRecord = ValueRecord;

    fn to_record(
        &self,
        arena: &ColoredExprArena<'id>,
        hash_children: impl Fn(&ValueResult<'id>) -> Option<ValueHash>,
    ) -> Option<Self::AsRecord> {
        Some(match self {
            Self::Lambda(lambda) => ValueRecord::Lambda(lambda.to_record(arena, hash_children)?),
            Self::Thunk(thunk) => return thunk.to_record(arena, hash_children),

            Self::Number(number) => ValueRecord::Number(number.clone()),
            Self::Bool(value) => ValueRecord::Bool(*value),
        })
    }

    fn from_record<B: CacheBackend>(record: ValueRecord, state: &EvalState<'id, '_, B>) -> Self {
        match record {
            ValueRecord::Lambda(record) => Self::Lambda(Lambda::from_record(record, state)),
            ValueRecord::Number(number) => Self::Number(number),
            ValueRecord::Bool(value) => Self::Bool(value),
        }
    }
}

impl<'id> RecordRepr<'id> for Lambda<'id> {
    type AsRecord = LambdaRecord;

    fn to_record(
        &self,
        arena: &ColoredExprArena<'id>,
        hash_child: impl Fn(&ValueResult<'id>) -> Option<ValueHash>,
    ) -> Option<Self::AsRecord> {
        Some(LambdaRecord {
            body: arena[self.body()]
                .color()
                .expect("stored expressions must be colored"),
            captures: self.captures().to_record(arena, hash_child)?,
        })
    }

    fn from_record<B: CacheBackend>(record: LambdaRecord, state: &EvalState<'id, '_, B>) -> Self {
        Lambda::new(
            *state
                .colors()
                .get(&record.body)
                .expect("expression colors must be in the reverse lookup"),
            Callstack::from_record(record.captures, state),
        )
    }
}

// thunks are transparent: an evaluated thunk records as its result
impl<'id> RecordRepr<'id> for Thunk<'id> {
    type AsRecord = ValueRecord;

    fn to_record(
        &self,
        arena: &ColoredExprArena<'id>,
        hash_children: impl Fn(&ValueResult<'id>) -> Option<ValueHash>,
    ) -> Option<Self::AsRecord> {
        match &*self.state().borrow() {
            ThunkState::Forced(result) => result.to_record(arena, hash_children),

            ThunkState::Evaluating => unreachable!(),
            // TODO: deferred thunks aren't serializable yet
            ThunkState::Deferred { .. } => None,
        }
    }

    fn from_record<B: CacheBackend>(record: ValueRecord, state: &EvalState<'id, '_, B>) -> Self {
        Thunk::new_forced(Ok(Value::from_record(record, state)))
    }
}

impl<'id> RecordRepr<'id> for Callstack<'id> {
    type AsRecord = CallstackRecord;

    fn to_record(
        &self,
        _: &ColoredExprArena<'id>,
        hash_child: impl Fn(&ValueResult<'id>) -> Option<ValueHash>,
    ) -> Option<Self::AsRecord> {
        Some(CallstackRecord(
            self.iter()
                .map(|thunk| hash_child(&Ok(Value::Thunk(thunk.clone()))))
                .collect::<Option<_>>()?,
        ))
    }

    // TODO: this shouldn't lead to re-evaluation of thunks
    fn from_record<B: CacheBackend>(record: Self::AsRecord, state: &EvalState<'id, '_, B>) -> Self {
        let thunks = record
            .0
            .iter()
            .map(|hash| Thunk::new_forced(Ok(state.cache().get_value(*hash, state))))
            .collect();

        // TODO take an iterator here to avoid allocating
        Callstack::from_thunks(thunks)
    }
}
