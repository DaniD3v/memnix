mod get_param;

use crate::{
    eval::{
        Eval, EvalState, RuntimeValue, builtins::get_param::get_params, error::EvalError,
        value::RuntimeNumber,
    },
    mir::Intrinsic,
};

impl<'id> Eval<'id> for Intrinsic {
    fn eval(self, state: EvalState<'id, '_>) -> RuntimeValue<'id> {
        match self {
            Self::IfElse => if_else(state),
            Self::LessOrEq => less_or_eq(state),
            Self::Add => add(state),
            Self::Subtract => subtract(state),

            Self::RefCycleError => RuntimeValue::Error(EvalError::RefCycle),

            #[expect(unreachable_patterns)]
            _ => todo!("Evaluate Intrinsic {:?}", self),
        }
    }
}

pub fn if_else<'id>(state: EvalState<'id, '_>) -> RuntimeValue<'id> {
    let (condition, then_expr, else_call): (bool, RuntimeValue, RuntimeValue) =
        get_params(state).unwrap();

    if condition { then_expr } else { else_call }
}

pub fn less_or_eq<'id>(state: EvalState<'id, '_>) -> RuntimeValue<'id> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(state).unwrap();
    RuntimeValue::Bool(l <= r)
}

pub fn add<'id>(state: EvalState<'id, '_>) -> RuntimeValue<'id> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(state).unwrap();
    RuntimeValue::Number(l + r)
}

pub fn subtract<'id>(state: EvalState<'id, '_>) -> RuntimeValue<'id> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(state).unwrap();
    RuntimeValue::Number(l + (-r))
}
