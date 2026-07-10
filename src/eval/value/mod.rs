mod number;
mod thunk;

pub use number::RuntimeNumber;
pub use thunk::Thunk;

use getset::{CopyGetters, Getters};

use crate::{
    ArenaId,
    eval::{builtins::FromRuntimeValue, error::EvalError},
};

#[derive(Clone, Debug)]
pub enum RuntimeValue<'id, 'a> {
    Lambda(RuntimeLambda<'id, 'a>),
    Number(RuntimeNumber),
    Thunk(Thunk<'id, 'a>),
    Bool(bool),

    Error(EvalError),
}

impl<'b, 'a> FromRuntimeValue<'b, 'a> for RuntimeValue<'b, 'a> {
    fn from(value: RuntimeValue<'b, 'a>) -> Result<Self, EvalError> {
        Ok(value)
    }
}

impl<'b> FromRuntimeValue<'b, '_> for bool {
    fn from(value: RuntimeValue<'b, '_>) -> Result<Self, EvalError> {
        match value.eval_thunk() {
            RuntimeValue::Bool(ret) => Ok(ret),
            _ => Err(EvalError::WrongType),
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct RuntimeLambda<'id, 'a> {
    #[getset(get_copy = "pub")]
    body: ArenaId<'id>,
    #[getset(get = "pub")]
    captures: Vec<RuntimeValue<'id, 'a>>,
}

impl<'id, 'a> RuntimeLambda<'id, 'a> {
    pub fn new(body: ArenaId<'id>, captures: Vec<RuntimeValue<'id, 'a>>) -> Self {
        Self { body, captures }
    }
}
