use crate::{
    eval::{
        Eval, EvalState, RuntimeValue, builtins::get_param::get_params, error::EvalError,
        value::RuntimeNumber,
    },
    mir::Intrinsic,
};

mod get_param;

pub(in crate::eval) use get_param::FromRuntimeValue;

impl<'id> Eval<'id> for Intrinsic {
    fn eval<'a>(self, state: EvalState<'id, 'a>) -> RuntimeValue<'id, 'a> {
        match self {
            Self::IfElse => if_else(state.callstack),
            Self::LessOrEq => less_or_eq(state.callstack),
            Self::Add => add(state.callstack),
            Self::Subtract => subtract(state.callstack),

            Self::RefCycleError => RuntimeValue::Error(EvalError::RefCycle),

            #[expect(unreachable_patterns)]
            _ => todo!("Evaluate Intrinsic {:?}", self),
        }
    }
}

pub fn if_else<'id, 'a>(callstack: &[RuntimeValue<'id, 'a>]) -> RuntimeValue<'id, 'a> {
    let (condition, then_expr, else_call): (bool, RuntimeValue, RuntimeValue) =
        get_params(callstack).unwrap();

    if condition { then_expr } else { else_call }
}

pub fn less_or_eq<'id, 'a>(callstack: &[RuntimeValue<'id, 'a>]) -> RuntimeValue<'id, 'a> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(callstack).unwrap();
    RuntimeValue::Bool(l <= r)
}

pub fn add<'id, 'a>(callstack: &[RuntimeValue<'id, 'a>]) -> RuntimeValue<'id, 'a> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(callstack).unwrap();
    RuntimeValue::Number(l + r)
}

pub fn subtract<'id, 'a>(callstack: &[RuntimeValue<'id, 'a>]) -> RuntimeValue<'id, 'a> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(callstack).unwrap();
    RuntimeValue::Number(l + (-r))
}
