mod number;
mod thunk;

pub use number::RuntimeNumber;
pub use thunk::Thunk;

use getset::{CopyGetters, Getters};

use crate::{eval::error::EvalError, mir::Expr};

#[derive(Clone, Debug)]
pub enum RuntimeValue<'b> {
    Lambda(RuntimeLambda<'b>),
    Number(RuntimeNumber),
    Thunk(Thunk<'b>),
    Bool(bool),

    Error(EvalError),
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
