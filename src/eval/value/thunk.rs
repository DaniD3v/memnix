use std::fmt::{self, Debug};

use crate::{
    ArenaId,
    eval::{Eval, EvalState, value::RuntimeValue},
};

#[derive(Clone)]
pub struct Thunk<'id, 'a> {
    expr: ArenaId<'id>,
    state: EvalState<'id, 'a>,
}

impl<'id, 'a> Thunk<'id, 'a> {
    pub fn new(expr: ArenaId<'id>, state: EvalState<'id, 'a>) -> Self {
        Self { expr, state }
    }

    pub fn eval(self) -> RuntimeValue<'id, 'a> {
        match self.expr.eval(self.state) {
            RuntimeValue::Thunk(thunk) => thunk.eval(),
            any => any,
        }
    }
}

impl<'b> RuntimeValue<'b, '_> {
    pub fn eval_thunk(self) -> Self {
        match self {
            Self::Thunk(thunk) => thunk.eval(),
            any => any,
        }
    }
}

impl<'id> Debug for Thunk<'id, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Thunk")
            .field("expr", &self.expr)
            .field("callstack", &self.state.callstack)
            .finish()
    }
}
