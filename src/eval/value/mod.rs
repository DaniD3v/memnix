mod number;
mod thunk;

pub use number::RuntimeNumber;
pub use thunk::Thunk;

use getset::{CopyGetters, Getters};

use crate::{
    eval::{builtins::FromRuntimeValue, error::EvalError},
    mir::Expr,
};

#[derive(Clone, Debug)]
pub enum RuntimeValue<'b> {
    Lambda(RuntimeLambda<'b>),
    Number(RuntimeNumber),
    Thunk(Thunk<'b>),
    Bool(bool),

    Error(EvalError),
}

impl<'b> FromRuntimeValue<'b> for RuntimeValue<'b> {
    fn from(value: RuntimeValue<'b>) -> Result<Self, EvalError> {
        Ok(value)
    }
}

impl<'b> FromRuntimeValue<'b> for bool {
    fn from(value: RuntimeValue<'b>) -> Result<Self, EvalError> {
        match value.eval_thunk() {
            RuntimeValue::Bool(ret) => Ok(ret),
            _ => Err(EvalError::WrongType),
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct RuntimeLambda<'b> {
    #[getset(get_copy = "pub")]
    body: &'b Expr<'b>,
    #[getset(get = "pub")]
    captures: Vec<RuntimeValue<'b>>,
}

impl<'b> RuntimeLambda<'b> {
    pub fn new(body: &'b Expr<'b>, captures: Vec<RuntimeValue<'b>>) -> Self {
        Self { body, captures }
    }
}
