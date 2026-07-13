mod number;
mod thunk;

pub use number::RuntimeNumber;
pub use thunk::{FromThunk, Thunk};

use getset::{CopyGetters, Getters};

use crate::{
    ArenaId,
    eval::{EvalState, callstack::Callstack, error::EvalError},
};

// This must Clone within (or reasonably close to) O(1)
#[derive(Clone, Debug)]
pub enum RuntimeValue<'id> {
    Lambda(RuntimeLambda<'id>),
    Number(RuntimeNumber),
    Thunk(Thunk<'id>),
    Bool(bool),
}

impl<'id> FromThunk<'id> for RuntimeValue<'id> {
    fn from_thunk(value: Thunk<'id>, _: EvalState<'id, '_>) -> Result<Self, EvalError> {
        Ok(RuntimeValue::Thunk(value))
    }
}

impl<'b> FromThunk<'b> for bool {
    fn from_thunk(value: Thunk<'b>, state: EvalState<'b, '_>) -> Result<Self, EvalError> {
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
