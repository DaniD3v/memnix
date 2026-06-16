use crate::eval::{error::EvalError, value::RuntimeValue};

pub fn get_params<'b, T: GetParamsTrait<'b>>(
    callstack: &[RuntimeValue<'b>],
) -> Result<T, EvalError> {
    GetParamsTrait::get_params(callstack)
}

pub trait FromRuntimeValue<'b>: Sized {
    fn from(value: RuntimeValue<'b>) -> Result<Self, EvalError>;
}

pub trait GetParamsTrait<'b>: Sized {
    fn get_params(callstack: &[RuntimeValue<'b>]) -> Result<Self, EvalError>;
}

// TODO use a macro for this
impl<'b, T1: FromRuntimeValue<'b>> GetParamsTrait<'b> for (T1,) {
    fn get_params(callstack: &[RuntimeValue<'b>]) -> Result<Self, EvalError> {
        let [t1] = callstack.as_array()
        .expect("the number of expected parameters differs between the intrinsic and the implementation")
        .clone();

        Ok((FromRuntimeValue::from(t1)?,))
    }
}

impl<'b, T1: FromRuntimeValue<'b>, T2: FromRuntimeValue<'b>> GetParamsTrait<'b> for (T1, T2) {
    fn get_params(callstack: &[RuntimeValue<'b>]) -> Result<Self, EvalError> {
        let [t1, t2] = callstack.as_array()
        .expect("the number of expected parameters differs between the intrinsic and the implementation")
        .clone();

        Ok((FromRuntimeValue::from(t1)?, FromRuntimeValue::from(t2)?))
    }
}

impl<'b, T1: FromRuntimeValue<'b>, T2: FromRuntimeValue<'b>, T3: FromRuntimeValue<'b>>
    GetParamsTrait<'b> for (T1, T2, T3)
{
    fn get_params(callstack: &[RuntimeValue<'b>]) -> Result<Self, EvalError> {
        let [t1, t2, t3] = callstack.as_array()
        .expect("the number of expected parameters differs between the intrinsic and the implementation")
        .clone();

        Ok((
            FromRuntimeValue::from(t1)?,
            FromRuntimeValue::from(t2)?,
            FromRuntimeValue::from(t3)?,
        ))
    }
}
