use rnix::ast;

use crate::{
    arena::LazyArenaId,
    generic_lang::GenericIntrinsic,
    mir::{
        MirResolveError,
        ident_resolver::{Resolve, Resolver},
        lang::LazyExprArena,
    },
};

impl Resolve for ast::Apply {
    // TODO: type alias
    type Target<'id> = GenericIntrinsic<LazyArenaId<'id>>;

    fn resolve<'id>(
        self,
        resolver: &impl Resolver<'id>,
        bump: &mut LazyExprArena<'id>,
    ) -> Result<Self::Target<'id>, MirResolveError> {
        let lambda = self.lambda().unwrap().resolve(resolver, bump)?;
        let argument = self.argument().unwrap().resolve(resolver, bump)?;

        Ok(GenericIntrinsic::LambdaCall([lambda, argument]))
    }
}
