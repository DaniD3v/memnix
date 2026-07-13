use crate::eval::{EvalState, error::EvalError, value::FromThunk};

pub fn get_params<'b, T: GetParamsTrait<'b>>(state: EvalState<'b, '_>) -> Result<T, EvalError> {
    GetParamsTrait::get_params(state)
}

pub trait GetParamsTrait<'id>: Sized {
    fn get_params(state: EvalState<'id, '_>) -> Result<Self, EvalError>;
}

// TODO use a macro for this
impl<'b, T1: FromThunk<'b>> GetParamsTrait<'b> for (T1,) {
    fn get_params(state: EvalState<'b, '_>) -> Result<Self, EvalError> {
        let [t1] = state.callstack.as_array()
        .expect("the number of expected parameters differs between the intrinsic and the implementation")
        .clone();

        Ok((FromThunk::from_thunk(t1, state)?,))
    }
}

impl<'b, T1: FromThunk<'b>, T2: FromThunk<'b>> GetParamsTrait<'b> for (T1, T2) {
    fn get_params(state: EvalState<'b, '_>) -> Result<Self, EvalError> {
        let [t1, t2] = state.callstack.as_array()
        .expect("the number of expected parameters differs between the intrinsic and the implementation")
        .clone();

        Ok((
            FromThunk::from_thunk(t1, state.clone())?,
            FromThunk::from_thunk(t2, state)?,
        ))
    }
}

impl<'b, T1: FromThunk<'b>, T2: FromThunk<'b>, T3: FromThunk<'b>> GetParamsTrait<'b>
    for (T1, T2, T3)
{
    fn get_params(state: EvalState<'b, '_>) -> Result<Self, EvalError> {
        let [t1, t2, t3] = state.callstack.as_array()
        .expect("the number of expected parameters differs between the intrinsic and the implementation")
        .clone();

        Ok((
            FromThunk::from_thunk(t1, state.clone())?,
            FromThunk::from_thunk(t2, state.clone())?,
            FromThunk::from_thunk(t3, state)?,
        ))
    }
}
