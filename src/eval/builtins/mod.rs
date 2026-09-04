mod dispatch_intrinsic;

use crate::{
    eval::{
        CacheBackend, Eval, EvalCtx, EvalState, Value, ValueResult,
        builtins::dispatch_intrinsic::dispatch,
        value::{Lambda, Number, Thunk},
    },
    mir::MirIntrinsic,
};

macro_rules! dispatch_intrinsic {
    ( $($name:ident => $fn:ident),* $(,)? ) => {
        impl<'id, B: CacheBackend> Eval<'id, B> for &MirIntrinsic<'id> {
            fn eval(self, state: EvalState<'id, '_, B>) -> ValueResult<'id> {
                match self {$(
                    MirIntrinsic::$name(params) => dispatch($fn, *params, state),
                )*}
            }
        }
    };
}

dispatch_intrinsic! {
    LambdaCall => lambda_call,

    LessOrEq => less_or_eq,
    Add => add,
    Subtract => subtract,

    IfElse => if_else,
}

pub fn if_else<'id>(
    condition: bool,
    then_expr: Value<'id>,
    else_call: Value<'id>,
) -> ValueResult<'id> {
    Ok(if condition { then_expr } else { else_call })
}

pub fn lambda_call<'id, B: CacheBackend>(
    lambda: Lambda<'id>,
    value: Thunk<'id>,
    ctx: &EvalCtx<'id, '_, B>,
) -> ValueResult<'id> {
    lambda.body().eval(EvalState {
        callstack: (lambda.captures().with_pushed(value)),
        ctx,
    })
}

pub fn less_or_eq<'id>(l: Number, r: Number) -> ValueResult<'id> {
    Ok(Value::Bool(l <= r))
}

pub fn add<'id>(l: Number, r: Number) -> ValueResult<'id> {
    Ok(Value::Number(l + r))
}

pub fn subtract<'id>(l: Number, r: Number) -> ValueResult<'id> {
    Ok(Value::Number(l + (-r)))
}
