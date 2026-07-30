mod number;
mod record;
mod thunk;

pub use number::Number;
pub use record::{CallstackRecord, RecordRepr, ValueRecord};
pub use thunk::{FromThunk, Thunk};

use getset::{CopyGetters, Getters};

use crate::{
    arena::ArenaId,
    eval::{CacheBackend, EvalState, callstack::Callstack, error::EvalError},
};

// This must Clone within (or reasonably close to) O(1)
#[derive(Clone, Debug)]
pub enum Value<'id> {
    Lambda(Lambda<'id>),
    Thunk(Thunk<'id>),

    Number(Number),
    Bool(bool),
}

impl<'id, B: CacheBackend> FromThunk<'id, B> for Value<'id> {
    fn from_thunk(value: Thunk<'id>, _: EvalState<'id, '_, B>) -> Result<Self, EvalError> {
        Ok(Value::Thunk(value))
    }
}

impl<'b, B: CacheBackend> FromThunk<'b, B> for bool {
    fn from_thunk(value: Thunk<'b>, state: EvalState<'b, '_, B>) -> Result<Self, EvalError> {
        match value.force(state)? {
            Value::Bool(ret) => Ok(ret),
            _ => Err(EvalError::WrongType),
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct Lambda<'id> {
    #[getset(get_copy = "pub")]
    body: ArenaId<'id>,
    #[getset(get = "pub")]
    captures: Callstack<'id>,
}

impl<'id> Lambda<'id> {
    pub fn new(body: ArenaId<'id>, captures: Callstack<'id>) -> Self {
        Self { body, captures }
    }
}
