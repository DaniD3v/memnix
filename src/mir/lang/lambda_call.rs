use std::array;

use rnix::ast;

use crate::{
    ArenaId,
    generic_lang::GenericLambdaCall,
    mir::{Expr, LazyExprArena, Resolve, Resolver, error::MirResolveError},
};

pub type LambdaCall<'bump> = GenericLambdaCall<ArenaId<'bump>>;

impl<'b> LambdaCall<'b> {
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
        bump: &mut LazyExprArena<'bump>,
    ) -> Result<LambdaCall<'bump>, MirResolveError> {
        let lambda = self.lambda().unwrap().resolve(resolver, bump)?;
        let argument = self.argument().unwrap().resolve(resolver, bump)?;

        Ok(LambdaCall::new(lambda, argument))
    }
}

impl<'id> IntoIterator for &LambdaCall<'id> {
    type Item = ArenaId<'id>;
    type IntoIter = array::IntoIter<ArenaId<'id>, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [*self.lambda(), *self.argument()].into_iter()
    }
}

// TODO: test this
// 'b is invariant so we can only compare to
// elements backed by the same bump allocator
impl<'b> PartialEq for LambdaCall<'b> {
    fn eq(&self, other: &Self) -> bool {
        self.into_iter().eq(other)
    }
}
