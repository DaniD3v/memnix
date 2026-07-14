mod builtins;
mod call_lambda;
mod callstack;
mod error;
mod hash;
mod value;

use crate::{
    Arena, ArenaId,
    coloring::{ColorableRootExpr, ColoredExpr},
    eval::{
        callstack::Callstack,
        error::EvalError,
        value::{RuntimeLambda, RuntimeNumber, RuntimeValue},
    },
    mir::{Literal, MirExpr, MirLambda},
};

pub type EvalResult<'id> = Result<RuntimeValue<'id>, EvalError>;

pub trait Eval<'id> {
    fn eval(self, state: EvalState<'id, '_>) -> EvalResult<'id>;
}

#[derive(Clone)]
pub struct EvalState<'id, 'a> {
    callstack: Callstack<'id>,
    arena: &'a Arena<'id, ColoredExpr<'id>>,
}

pub fn eval_root_expr<'id>(root: &ColorableRootExpr<'id>) -> EvalResult<'id> {
    let state = EvalState {
        callstack: Callstack::default(),
        arena: root.arena(),
    };

    root.root_node().eval(state.clone())?.eval_thunk(state)
}

impl<'id> Eval<'id> for &ColoredExpr<'id> {
    fn eval(self, state: EvalState<'id, '_>) -> EvalResult<'id> {
        match self.expr() {
            MirExpr::Lambda(lambda) => lambda.eval(state),
            MirExpr::LambdaCall(lambda_call) => lambda_call.eval(state),
            MirExpr::Literal(literal) => literal.eval(state),

            MirExpr::Intrinsic(intrinsic) => intrinsic.eval(state),
            MirExpr::Param(param) => Ok(RuntimeValue::Thunk(
                state.callstack[param.nesting_depth()].clone(),
            )),
        }
    }
}

impl<'id> Eval<'id> for ArenaId<'id> {
    fn eval(self, state: EvalState<'id, '_>) -> EvalResult<'id> {
        state.arena[self].eval(state)
    }
}

impl<'b> Eval<'b> for &Literal {
    fn eval(self, _: EvalState<'b, '_>) -> EvalResult<'b> {
        Ok(match self {
            Literal::Integer(num) => RuntimeValue::Number(RuntimeNumber::Integer(*num)),
            Literal::Float(num) => RuntimeValue::Number(RuntimeNumber::Float(*num)),

            _ => todo!(),
        })
    }
}

impl<'b> Eval<'b> for &MirLambda<'b> {
    fn eval(self, state: EvalState<'b, '_>) -> EvalResult<'b> {
        assert!(self.depth() <= state.callstack.len());

        Ok(RuntimeValue::Lambda(RuntimeLambda::new(
            *self.body(),
            state.callstack.prefix(self.depth()),
        )))
    }
}
