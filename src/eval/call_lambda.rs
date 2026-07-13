use crate::{
    eval::{
        Eval, EvalState,
        error::EvalError,
        value::{RuntimeValue, Thunk},
    },
    mir::MirLambdaCall,
};

impl<'id> Eval<'id> for &MirLambdaCall<'id> {
    fn eval(self, state: EvalState<'id, '_>) -> RuntimeValue<'id> {
        let lambda = self.lambda().eval(state.clone()).eval_thunk(state.clone());
        let RuntimeValue::Lambda(runtime_lambda) = lambda else {
            eprintln!("self: {:?}; eval: {:?}", self, lambda);
            return RuntimeValue::Error(EvalError::NotALambda);
        };

        // TODO: the thunk actually only needs the callstack
        // so I can simply split state instead of cloning
        let arg = Thunk::new(*self.argument(), state.callstack);

        runtime_lambda.body().eval(EvalState {
            callstack: runtime_lambda.captures().with_pushed(arg),
            arena: state.arena,
        })
    }
}
