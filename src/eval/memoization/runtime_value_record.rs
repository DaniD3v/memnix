use serde::{Deserialize, Serialize};

use crate::{
    coloring::Color,
    eval::{
        CacheBackend, EvalState,
        callstack::Callstack,
        hash::EvalHash,
        value::{RuntimeLambda, RuntimeNumber, RuntimeValue, Thunk, thunk::ThunkState},
    },
};

/// A `RuntimeValue` serialized as an owned record.
///
/// Expressions are referenced by their `Color`,
/// captures by their `EvalHash`.
#[derive(Serialize, Deserialize, Clone)]
pub(super) enum ValueRecord {
    Lambda {
        body: Color,
        captures: Vec<EvalHash>,
    },

    // TODO: cache deferred thunks once lambdas only capture used params
    // TODO: cache `EvalError`s too
    Number(RuntimeNumber),
    Bool(bool),
}

pub(super) trait RecordRepr<'id>: Sized {
    fn to_record<B: CacheBackend>(self, state: &EvalState<'id, '_, B>) -> Option<ValueRecord>;
    fn from_record<B: CacheBackend>(record: ValueRecord, state: &EvalState<'id, '_, B>) -> Self;
}

impl<'id> RecordRepr<'id> for RuntimeValue<'id> {
    fn to_record<B: CacheBackend>(self, state: &EvalState<'id, '_, B>) -> Option<ValueRecord> {
        Some(match self {
            Self::Lambda(lambda) => ValueRecord::Lambda {
                body: state.arena()[lambda.body()]
                    .color()
                    .expect("stored expressions must be colored"),
                captures: lambda
                    .captures()
                    .iter()
                    .map(|thunk| thunk.store(state))
                    .collect::<Option<_>>()?,
            },
            // thunks are transparent: an evaluated thunk stores as its result
            Self::Thunk(thunk) => match &*thunk.state().borrow() {
                ThunkState::Forced(Ok(value)) => value.clone().to_record(state)?,
                // errors and not-yet-forced thunks aren't serializable yet
                ThunkState::Forced(Err(_)) | ThunkState::Deferred { .. } => return None,
                ThunkState::Evaluating => unreachable!(),
            },

            Self::Number(number) => ValueRecord::Number(number),
            Self::Bool(value) => ValueRecord::Bool(value),
        })
    }

    fn from_record<B: CacheBackend>(record: ValueRecord, state: &EvalState<'id, '_, B>) -> Self {
        match record {
            ValueRecord::Lambda { body, captures } => Self::Lambda(RuntimeLambda::new(
                *state
                    .colors()
                    .get(&body)
                    .expect("expression colors must be in the reverse lookup"),
                Callstack::from_thunks(
                    captures
                        .into_iter()
                        .map(|hash| Thunk::new_forced(Ok(state.cache().get_value(hash, state))))
                        .collect(),
                ),
            )),
            ValueRecord::Number(number) => Self::Number(number),
            ValueRecord::Bool(value) => Self::Bool(value),
        }
    }
}

impl<'id> Thunk<'id> {
    /// Stores the thunk's result and returns its hash, or `None` if unstorable.
    fn store<B: CacheBackend>(&self, state: &EvalState<'id, '_, B>) -> Option<EvalHash> {
        state
            .cache()
            .store_value(&RuntimeValue::Thunk(self.clone()), state)
    }
}
