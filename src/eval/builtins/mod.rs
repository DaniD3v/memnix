// TODO eliminate 2 sources of truth on parameters of intrinsics

use crate::{
    eval::{Eval, RuntimeValue, error::EvalError},
    mir::Intrinsic,
};

impl<'b> Eval<'b> for Intrinsic {
    fn eval(&self, callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
        match self {
            Self::IfElse => if_else(callstack),
            Self::LessOrEq => less_or_eq(callstack),
            Self::Add => add(callstack),
            Self::Subtract => subtract(callstack),
            // TODO
            // _ => todo!("Evaluate Intrinsic {:?}", self),
        }
    }
}

pub fn if_else<'b>(callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
    let [condition, then_expr, else_call] = parse_params_lazy(callstack);
    let RuntimeValue::Bool(condition) = condition.eval_thunk() else {
        return RuntimeValue::Error(EvalError::WrongType);
    };

    if condition { then_expr } else { else_call }
}

pub fn less_or_eq<'b>(callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
    let [l, r] = parse_params(callstack);
    let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (l, r) else {
        return RuntimeValue::Error(EvalError::WrongType);
    };

    RuntimeValue::Bool(l <= r)
}

pub fn add<'b>(callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
    let [l, r] = parse_params(callstack);
    let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (l, r) else {
        return RuntimeValue::Error(EvalError::WrongType);
    };

    RuntimeValue::Number(l + r)
}

pub fn subtract<'b>(callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
    let [l, r] = parse_params(callstack);
    let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (l, r) else {
        return RuntimeValue::Error(EvalError::WrongType);
    };

    RuntimeValue::Number(l + (-r))
}

fn parse_params_lazy<'b, const SIZE: usize>(
    callstack: &[RuntimeValue<'b>],
) -> [RuntimeValue<'b>; SIZE] {
    callstack[callstack.len() - SIZE..]
        .as_array()
        .expect("size of `(length-n)..` should be n")
        .clone()
}

fn parse_params<'b, const SIZE: usize>(callstack: &[RuntimeValue<'b>]) -> [RuntimeValue<'b>; SIZE] {
    parse_params_lazy(callstack).map(|expr| expr.eval_thunk())
}
