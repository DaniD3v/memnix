mod get_param;

use crate::{
    eval::{
        CacheBackend, Eval, EvalState, Value, ValueResult, builtins::get_param::get_params,
        error::EvalError, value::Number,
    },
    mir::Intrinsic,
};

impl<'id, B: CacheBackend> Eval<'id, B> for Intrinsic {
    fn eval(self, state: EvalState<'id, '_, B>) -> ValueResult<'id> {
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

pub fn if_else<'id, B: CacheBackend>(state: EvalState<'id, '_, B>) -> ValueResult<'id> {
    let (condition, then_expr, else_call): (bool, Value, Value) = get_params(state)?;

    Ok(if condition { then_expr } else { else_call })
}

pub fn less_or_eq<'id, B: CacheBackend>(state: EvalState<'id, '_, B>) -> ValueResult<'id> {
    let (l, r): (Number, Number) = get_params(state)?;
    Ok(Value::Bool(l <= r))
}

pub fn add<'id, B: CacheBackend>(state: EvalState<'id, '_, B>) -> ValueResult<'id> {
    let (l, r): (Number, Number) = get_params(state)?;
    Ok(Value::Number(l + r))
}

pub fn subtract<'id, B: CacheBackend>(state: EvalState<'id, '_, B>) -> ValueResult<'id> {
    let (l, r): (Number, Number) = get_params(state)?;
    Ok(Value::Number(l + (-r)))
}
