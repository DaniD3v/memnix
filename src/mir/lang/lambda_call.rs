use rnix::ast;

use crate::{
    ArenaId,
    generic_lang::GenericLambdaCall,
    mir::{LazyExprArena, MirExpr, Resolve, Resolver, error::MirResolveError},
};

pub type MirLambdaCall<'bump> = GenericLambdaCall<ArenaId<'bump>>;

impl<'b> MirLambdaCall<'b> {
    /// In nix lambas only take one input parameter.
    /// In order to take multiple you simply return a second function
    /// that takes another parameter from the first function.
    pub fn new_curried(
        lambda: ArenaId<'b>,
        args: &[ArenaId<'b>],
        arena: &mut LazyExprArena<'b>,
    ) -> Self {
        assert!(!args.is_empty());

        if args.len() == 1 {
            MirLambdaCall::new(lambda, args[0])
        } else {
            let (&argument, curried_args) = args.split_last().expect("args cannot be empty");

            // The lambda that is to the left of this lambda.
            // e.g. `builtins.add 1` in `(builtins.add 1) 2`
            let inner =
                MirExpr::LambdaCall(MirLambdaCall::new_curried(lambda, curried_args, arena));
            let inner = arena.alloc(inner);

            MirLambdaCall::new(inner, argument)
        }
    }

    pub fn children(&self) -> [(ArenaId<'b>, &str); 2] {
        [(*self.lambda(), "lambda"), (*self.argument(), "argument")]
    }
}

impl Resolve for ast::Apply {
    type Target<'bump> = MirLambdaCall<'bump>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &mut LazyExprArena<'bump>,
    ) -> Result<MirLambdaCall<'bump>, MirResolveError> {
        let lambda = self.lambda().unwrap().resolve(resolver, bump)?;
        let argument = self.argument().unwrap().resolve(resolver, bump)?;

        Ok(MirLambdaCall::new(lambda, argument))
    }
}

// TODO: test this
// 'b is invariant so we can only compare to
// elements backed by the same bump allocator
impl<'b> PartialEq for MirLambdaCall<'b> {
    fn eq(&self, other: &Self) -> bool {
        self.children() == other.children()
    }
}
