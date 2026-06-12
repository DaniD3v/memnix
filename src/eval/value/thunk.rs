use crate::{
    eval::{Eval, value::RuntimeValue},
    mir::Expr,
};

// #[derive(Clone, Debug)]
// pub enum Thunk<'b> {
//     Unevaluated(&'b Expr<'b>, Vec<RuntimeValue<'b>>),
//     Evaluated(Box<RuntimeValue<'b>>),
// }

#[derive(Clone, Debug)]
pub struct Thunk<'b> {
    expr: &'b Expr<'b>,
    callstack: Vec<RuntimeValue<'b>>,
}

impl<'b> Thunk<'b> {
    pub fn eval(&self) -> RuntimeValue<'b> {
        match self.expr.eval(&self.callstack) {
            RuntimeValue::Thunk(thunk) => thunk.eval(),
            any => any,
        }
    }
}

impl<'b> RuntimeValue<'b> {
    pub fn eval_thunk(self) -> Self {
        match self {
            Self::Thunk(thunk) => thunk.eval(),
            any => any,
        }
    }
}

impl<'b> Thunk<'b> {
    pub fn new(expr: &'b Expr<'b>, callstack: Vec<RuntimeValue<'b>>) -> Self {
        Self { expr, callstack }
    }
}
