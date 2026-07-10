use crate::{
    eval::{
        Eval, EvalState,
        value::{RuntimeValue, Thunk},
    },
    mir::MirLambdaCall,
};

impl<'id> Eval<'id> for &MirLambdaCall<'id> {
    fn eval<'a>(self, state: EvalState<'id, 'a>) -> RuntimeValue<'id, 'a> {
        let evaluated = self.lambda().eval(state).eval_thunk();
        let RuntimeValue::Lambda(lambda) = evaluated else {
            panic!("self: {:?}; eval: {:?}", self, evaluated);
            // TODO other error for EvalError
            // return Err(EvalError::NotALambda);
        };
        let arg = RuntimeValue::Thunk(Thunk::new(*self.argument(), state));

        let mut callstack = lambda.captures().to_owned();
        callstack.push(arg);

        // TODO: leaks the callstack on every call — replace with proper frame ownership (Rc/arena)
        lambda.body().eval(EvalState {
            callstack: callstack.leak(),
            arena: state.arena,
        })
    }
}
