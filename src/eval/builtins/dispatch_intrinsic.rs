use crate::{
    arena::ArenaId,
    eval::{
        CacheBackend, EvalCtx, EvalState, ValueResult,
        value::{FromThunk, Thunk},
    },
};

// TODO: state -> ctx
pub fn dispatch<'id, F: DispatchTo<'id, S, B, Marker>, const S: usize, B: CacheBackend, Marker>(
    f: F,
    params: [ArenaId<'id>; S],
    state: EvalState<'id, '_, B>,
) -> ValueResult<'id> {
    let params = params.map(|id| Thunk::new(id, state.callstack.clone()));
    DispatchTo::dispatch(f, params, state)
}

pub trait DispatchTo<'id, const S: usize, B: CacheBackend, Marker> {
    fn dispatch(self, params: [Thunk<'id>; S], state: EvalState<'id, '_, B>) -> ValueResult<'id>;
}

// TODO use a macro for this
impl<'id, F: Fn(T1) -> ValueResult<'id>, T1: FromThunk<'id, B>, B: CacheBackend>
    DispatchTo<'id, 1, B, (T1,)> for F
{
    fn dispatch(self, [t1]: [Thunk<'id>; 1], state: EvalState<'id, '_, B>) -> ValueResult<'id> {
        self(FromThunk::from_thunk(t1, state)?)
    }
}

impl<
    'id,
    F: Fn(T1, T2) -> ValueResult<'id>,
    T1: FromThunk<'id, B>,
    T2: FromThunk<'id, B>,
    B: CacheBackend,
> DispatchTo<'id, 2, B, (T1, T2)> for F
{
    fn dispatch(self, [t1, t2]: [Thunk<'id>; 2], state: EvalState<'id, '_, B>) -> ValueResult<'id> {
        self(
            FromThunk::from_thunk(t1, state.clone())?,
            FromThunk::from_thunk(t2, state)?,
        )
    }
}

impl<
    'id,
    F: Fn(T1, T2, &EvalCtx<'id, '_, B>) -> ValueResult<'id>,
    T1: FromThunk<'id, B>,
    T2: FromThunk<'id, B>,
    B: CacheBackend,
> DispatchTo<'id, 2, B, (T1, T2, &EvalCtx<'id, '_, B>)> for F
{
    fn dispatch(self, [t1, t2]: [Thunk<'id>; 2], state: EvalState<'id, '_, B>) -> ValueResult<'id> {
        self(
            FromThunk::from_thunk(t1, state.clone())?,
            FromThunk::from_thunk(t2, state.clone())?,
            state.ctx,
        )
    }
}

impl<
    'id,
    F: Fn(T1, T2, T3) -> ValueResult<'id>,
    T1: FromThunk<'id, B>,
    T2: FromThunk<'id, B>,
    T3: FromThunk<'id, B>,
    B: CacheBackend,
> DispatchTo<'id, 3, B, (T1, T2, T3)> for F
{
    fn dispatch(
        self,
        [t1, t2, t3]: [Thunk<'id>; 3],
        state: EvalState<'id, '_, B>,
    ) -> ValueResult<'id> {
        self(
            FromThunk::from_thunk(t1, state.clone())?,
            FromThunk::from_thunk(t2, state.clone())?,
            FromThunk::from_thunk(t3, state)?,
        )
    }
}
