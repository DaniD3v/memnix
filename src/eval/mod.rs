#![expect(dead_code)]

mod builtins;
mod call_lambda;
mod error;
mod value;

use crate::{
    Arena, ArenaId,
    coloring::{ColorableRootExpr, ColoredExpr},
    eval::value::{RuntimeLambda, RuntimeNumber, RuntimeValue},
    mir::{Literal, MirExpr, MirLambda},
};

pub trait Eval<'id> {
    fn eval<'a>(self, state: EvalState<'id, 'a>) -> RuntimeValue<'id, 'a>;
}

#[derive(Copy, Clone)]
pub struct EvalState<'id, 'a> {
    callstack: &'a [RuntimeValue<'id, 'a>],
    arena: &'a Arena<'id, ColoredExpr<'id>>,
}

pub fn eval_root_expr<'id, 'a>(root: &'a ColorableRootExpr<'id>) -> RuntimeValue<'id, 'a> {
    let state = EvalState {
        callstack: &[],
        arena: root.arena(),
    };

    root.root_node().eval(state).eval_thunk()
}

impl<'id> Eval<'id> for &ColoredExpr<'id> {
    fn eval<'a>(self, state: EvalState<'id, 'a>) -> RuntimeValue<'id, 'a> {
        match self.expr() {
            MirExpr::Lambda(lambda) => lambda.eval(state),
            MirExpr::LambdaCall(lambda_call) => lambda_call.eval(state),
            MirExpr::Literal(literal) => literal.eval(state),

            MirExpr::Intrinsic(intrinsic) => intrinsic.eval(state),
            MirExpr::Param(param) => state.callstack[param.nesting_depth()].clone(),
        }
    }
}

impl<'id> Eval<'id> for ArenaId<'id> {
    fn eval<'a>(self, state: EvalState<'id, 'a>) -> RuntimeValue<'id, 'a> {
        state.arena[self].eval(state)
    }
}

impl<'b> Eval<'b> for &Literal {
    fn eval<'a>(self, _: EvalState<'b, 'a>) -> RuntimeValue<'b, 'a> {
        match self {
            Literal::Integer(num) => RuntimeValue::Number(RuntimeNumber::Integer(*num)),
            Literal::Float(num) => RuntimeValue::Number(RuntimeNumber::Float(*num)),

            _ => todo!(),
        }
    }
}

impl<'b> Eval<'b> for &MirLambda<'b> {
    fn eval<'a>(self, state: EvalState<'b, 'a>) -> RuntimeValue<'b, 'a> {
        assert!(self.depth() <= state.callstack.len());

        RuntimeValue::Lambda(RuntimeLambda::new(
            *self.body(),
            state.callstack[..self.depth()].to_vec(), // TODO(perf): I can prob do this with refs?
        ))
    }
}
