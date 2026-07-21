use crate::{
    eval::{
        CacheBackend, Eval, EvalResult, EvalState,
        error::EvalError,
        value::{RuntimeValue, Thunk},
    },
    mir::MirLambdaCall,
};

impl<'id, B: CacheBackend> Eval<'id, B> for &MirLambdaCall<'id> {
    fn eval(self, state: EvalState<'id, '_, B>) -> EvalResult<'id> {
        let lambda = self
            .lambda()
            .eval(state.clone())?
            .eval_thunk(state.clone())?;
        let RuntimeValue::Lambda(runtime_lambda) = lambda else {
            eprintln!("self: {:?}; eval: {:?}", self, lambda);
            return Err(EvalError::NotALambda);
        };

        // TODO: the thunk actually only needs the callstack
        // so I can simply split state instead of cloning
        let arg = Thunk::new(*self.argument(), state.callstack);

        runtime_lambda.body().eval(EvalState {
            callstack: runtime_lambda.captures().with_pushed(arg),
            ctx: state.ctx,
        })
    }
}
