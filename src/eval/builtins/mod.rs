mod get_param;

use crate::{
    eval::{
        Eval, EvalResult, EvalState, RuntimeValue, builtins::get_param::get_params,
        error::EvalError, value::RuntimeNumber,
    },
    mir::Intrinsic,
};

impl<'id> Eval<'id> for Intrinsic {
    fn eval(self, state: EvalState<'id, '_>) -> EvalResult<'id> {
        match self {
            Self::IfElse => if_else(state),
            Self::LessOrEq => less_or_eq(state),
            Self::Add => add(state),
            Self::Subtract => subtract(state),

            Self::RefCycleError => Err(EvalError::RefCycle),

            #[expect(unreachable_patterns)]
            _ => todo!("Evaluate Intrinsic {:?}", self),
        }
    }
}

pub fn if_else<'id>(state: EvalState<'id, '_>) -> EvalResult<'id> {
    let (condition, then_expr, else_call): (bool, RuntimeValue, RuntimeValue) = get_params(state)?;

    Ok(if condition { then_expr } else { else_call })
}

pub fn less_or_eq<'id>(state: EvalState<'id, '_>) -> EvalResult<'id> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(state)?;
    Ok(RuntimeValue::Bool(l <= r))
}

pub fn add<'id>(state: EvalState<'id, '_>) -> EvalResult<'id> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(state)?;
    Ok(RuntimeValue::Number(l + r))
}

pub fn subtract<'id>(state: EvalState<'id, '_>) -> EvalResult<'id> {
    let (l, r): (RuntimeNumber, RuntimeNumber) = get_params(state)?;
    Ok(RuntimeValue::Number(l + (-r)))
}
