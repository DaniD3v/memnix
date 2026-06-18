use std::fmt::Formatter;

use rnix::ast;

use crate::{
    arena::DebugWith,
    generic_lang::GenericLambdaCall,
    mir::{
        Expr, ExprArena, ExprId, Resolve, Resolver, error::MirResolveError,
        mir_expr_arena::DebugState,
    },
};

pub type LambdaCall<'bump> = GenericLambdaCall<ExprId<'bump>>;

impl<'b> LambdaCall<'b> {
    /// In nix lambas only take one input parameter.
    /// In order to take multiple you simply return a second function
    /// that takes another parameter from the first function.
    pub fn new_curried(lambda: ExprId<'b>, args: &[ExprId<'b>], arena: &mut ExprArena<'b>) -> Self {
        assert!(!args.is_empty());

        if args.len() == 1 {
            LambdaCall::new(lambda, args[0])
        } else {
            let (&argument, curried_args) = args.split_last().expect("args cannot be empty");

            // The lambda that is to the left of this lambda.
            // e.g. `builtins.add 1` in `(builtins.add 1) 2`
            let inner = Expr::LambdaCall(LambdaCall::new_curried(lambda, curried_args, arena));
            let inner = arena.alloc(inner);

            LambdaCall::new(inner, argument)
        }
    }
}

impl Resolve for ast::Apply {
    type Target<'bump> = LambdaCall<'bump>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &mut ExprArena<'bump>,
    ) -> Result<LambdaCall<'bump>, MirResolveError> {
        let lambda = self.lambda().unwrap().resolve(resolver, bump)?;
        let argument = self.argument().unwrap().resolve(resolver, bump)?;

        Ok(LambdaCall::new(lambda, argument))
    }
}

impl<'id> DebugWith<DebugState<'id, '_>> for LambdaCall<'id> {
    fn fmt_with(&self, with: &mut DebugState<'id, '_>, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LambdaCall")
            .field("lambda", &self.lambda().as_wrapper(with))
            .field("argument", &self.argument().as_wrapper(with))
            .finish()
    }
}
