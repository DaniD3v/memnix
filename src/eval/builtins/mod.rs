use crate::{
    eval::{Eval, RuntimeValue, builtins::get_param::get_params, value::RuntimeNumber},
    mir::Intrinsic,
};

mod get_param;

pub(in crate::eval) use get_param::FromRuntimeValue;

impl<'b> Eval<'b> for Intrinsic {
    fn eval(&self, callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
        match self {
            Self::IfElse => if_else(callstack),
            Self::LessOrEq => less_or_eq(callstack),
            Self::Add => add(callstack),
            Self::Subtract => subtract(callstack),
            // _ => todo!("Evaluate Intrinsic {:?}", self),
        }
    }
}

pub fn if_else<'b>(callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
    let (condition, then_expr, else_call): (bool, RuntimeValue, RuntimeValue) =
        get_params(callstack).unwrap();

    if condition { then_expr } else { else_call }
}

pub fn less_or_eq<'b>(callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(callstack).unwrap();
    RuntimeValue::Bool(l <= r)
}

pub fn add<'b>(callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(callstack).unwrap();
    RuntimeValue::Number(l + r)
}

pub fn subtract<'b>(callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(callstack).unwrap();
    RuntimeValue::Number(l + (-r))
}
