#![expect(dead_code)]

mod builtins;
mod call_lambda;
mod error;
mod value;

use crate::{
    eval::value::{RuntimeLambda, RuntimeNumber, RuntimeValue},
    mir::{Expr, MirLambda, Literal},
};

pub trait Eval<'b> {
    fn eval(&self, callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b>;
}

impl<'b> Eval<'b> for Literal {
    fn eval(&self, _: &[RuntimeValue]) -> RuntimeValue<'b> {
        match self {
            Self::Integer(num) => RuntimeValue::Number(RuntimeNumber::Integer(*num)),
            Self::Float(num) => RuntimeValue::Number(RuntimeNumber::Float(*num)),

            _ => todo!(),
        }
    }
}

impl<'b> Eval<'b> for MirLambda<'b> {
    fn eval(&self, callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
        assert!(self.depth() <= callstack.len());

        RuntimeValue::Lambda(RuntimeLambda::new(
            self.body(),
            callstack[..self.depth()].to_vec(), // TODO(perf): I can prob do this with refs?
        ))
    }
}

impl<'b> Eval<'b> for Expr<'b> {
    fn eval(&self, callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
        match self {
            Self::Lambda(lambda) => lambda.eval(callstack),
            Self::LambdaCall(lambda_call) => lambda_call.eval(callstack),
            Self::Literal(literal) => literal.eval(callstack),

            Self::Intrinsic(intrinsic) => intrinsic.eval(callstack),
            Self::Param(param) => callstack[param.nesting_depth()].clone(),

            Self::Deferred(deferred) => deferred
                .get()
                .expect("deferred expressions should be resolved at eval time")
                .eval(callstack),
        }
    }
}
