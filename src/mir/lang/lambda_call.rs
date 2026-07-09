use rnix::ast;

use crate::{
    arena::LazyArenaId,
    generic_lang::GenericLambdaCall,
    mir::{
        error::MirResolveError,
        ident_resolver::{Resolve, Resolver},
        lang::{LazyExprArena, LazyMirExpr},
    },
};

pub type LazyMirLambdaCall<'bump> = GenericLambdaCall<LazyArenaId<'bump>>;

impl<'b> LazyMirLambdaCall<'b> {
    /// In nix lambas only take one input parameter.
    /// In order to take multiple you simply return a second function
    /// that takes another parameter from the first function.
    pub fn new_curried(
        lambda: LazyArenaId<'b>,
        args: &[LazyArenaId<'b>],
        arena: &mut LazyExprArena<'b>,
    ) -> Self {
        assert!(!args.is_empty());

        if args.len() == 1 {
            LazyMirLambdaCall::new(lambda, args[0])
        } else {
            let (&argument, curried_args) = args.split_last().expect("args cannot be empty");

            // The lambda that is to the left of this lambda.
            // e.g. `builtins.add 1` in `(builtins.add 1) 2`
            let inner = LazyMirExpr::LambdaCall(LazyMirLambdaCall::new_curried(
                lambda,
                curried_args,
                arena,
            ));
            let inner = arena.alloc(inner);

            LazyMirLambdaCall::new(inner, argument)
        }
    }
}

impl Resolve for ast::Apply {
    type Target<'bump> = LazyMirLambdaCall<'bump>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &mut LazyExprArena<'bump>,
    ) -> Result<LazyMirLambdaCall<'bump>, MirResolveError> {
        let lambda = self.lambda().unwrap().resolve(resolver, bump)?;
        let argument = self.argument().unwrap().resolve(resolver, bump)?;

        Ok(LazyMirLambdaCall::new(lambda, argument))
    }
}
