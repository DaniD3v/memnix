use crate::eval::{error::EvalError, value::RuntimeValue};

pub fn get_params<'b, 'a, T: GetParamsTrait<'b, 'a>>(
    callstack: &[RuntimeValue<'b, 'a>],
) -> Result<T, EvalError> {
    GetParamsTrait::get_params(callstack)
}

pub trait FromRuntimeValue<'b, 'a>: Sized {
    fn from(value: RuntimeValue<'b, 'a>) -> Result<Self, EvalError>;
}

pub trait GetParamsTrait<'b, 'a>: Sized {
    fn get_params(callstack: &[RuntimeValue<'b, 'a>]) -> Result<Self, EvalError>;
}

// TODO use a macro for this
impl<'b: 'a, 'a, T1: FromRuntimeValue<'b, 'a>> GetParamsTrait<'b, 'a> for (T1,) {
    fn get_params(callstack: &[RuntimeValue<'b, 'a>]) -> Result<Self, EvalError> {
        let [t1] = callstack.as_array()
        .expect("the number of expected parameters differs between the intrinsic and the implementation")
        .clone();

        Ok((FromRuntimeValue::from(t1)?,))
    }
}

impl<'b: 'a, 'a, T1: FromRuntimeValue<'b, 'a>, T2: FromRuntimeValue<'b, 'a>> GetParamsTrait<'b, 'a>
    for (T1, T2)
{
    fn get_params(callstack: &[RuntimeValue<'b, 'a>]) -> Result<Self, EvalError> {
        let [t1, t2] = callstack.as_array()
        .expect("the number of expected parameters differs between the intrinsic and the implementation")
        .clone();

        Ok((FromRuntimeValue::from(t1)?, FromRuntimeValue::from(t2)?))
    }
}

impl<
    'b,
    'a,
    T1: FromRuntimeValue<'b, 'a>,
    T2: FromRuntimeValue<'b, 'a>,
    T3: FromRuntimeValue<'b, 'a>,
> GetParamsTrait<'b, 'a> for (T1, T2, T3)
{
    fn get_params(callstack: &[RuntimeValue<'b, 'a>]) -> Result<Self, EvalError> {
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
