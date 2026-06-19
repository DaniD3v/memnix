use crate::{
    eval::{
        Eval,
        value::{RuntimeValue, Thunk},
    },
    mir::LambdaCall,
};

impl<'b> Eval<'b> for LambdaCall<'b> {
    fn eval(&self, callstack: &[RuntimeValue<'b>]) -> RuntimeValue<'b> {
        let evaluated = self.lambda().eval(callstack).eval_thunk();
        let RuntimeValue::Lambda(lambda) = evaluated else {
            panic!("self: {:?}; eval: {:?}", self, evaluated);
            // TODO other error for EvalError
            // return Err(EvalError::NotALambda);
        };
        let arg = RuntimeValue::Thunk(Thunk::new(self.argument(), callstack.to_vec()));

        let mut callstack = lambda.captures().to_owned();
        callstack.push(arg);

        lambda.body().eval(&callstack)
    }
}
