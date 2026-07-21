mod number;
pub(in crate::eval) mod thunk;

pub use number::RuntimeNumber;
pub use thunk::{FromThunk, Thunk};

use getset::{CopyGetters, Getters};

use crate::{
    ArenaId,
    eval::{CacheBackend, EvalState, callstack::Callstack, error::EvalError},
};

// This must Clone within (or reasonably close to) O(1)
#[derive(Clone, Debug)]
pub enum RuntimeValue<'id> {
    Lambda(RuntimeLambda<'id>),
    Thunk(Thunk<'id>),

    Number(RuntimeNumber),
    Bool(bool),
}

impl<'id, B: CacheBackend> FromThunk<'id, B> for RuntimeValue<'id> {
    fn from_thunk(value: Thunk<'id>, _: EvalState<'id, '_, B>) -> Result<Self, EvalError> {
        Ok(RuntimeValue::Thunk(value))
    }
}

impl<'b, B: CacheBackend> FromThunk<'b, B> for bool {
    fn from_thunk(value: Thunk<'b>, state: EvalState<'b, '_, B>) -> Result<Self, EvalError> {
        match value.force(state)? {
            RuntimeValue::Bool(ret) => Ok(ret),
            _ => Err(EvalError::WrongType),
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct RuntimeLambda<'id> {
    #[getset(get_copy = "pub")]
    body: ArenaId<'id>,
    #[getset(get = "pub")]
    captures: Callstack<'id>,
}

impl<'id> RuntimeLambda<'id> {
    pub fn new(body: ArenaId<'id>, captures: Callstack<'id>) -> Self {
        Self { body, captures }
    }
}
