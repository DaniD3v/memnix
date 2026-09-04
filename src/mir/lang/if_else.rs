use rnix::ast;

use crate::{
    arena::LazyArenaId,
    generic_lang::GenericIntrinsic,
    mir::{
        error::MirResolveError,
        ident_resolver::{Resolve, Resolver},
        lang::LazyExprArena,
    },
};

impl Resolve for ast::IfElse {
    type Target<'a> = GenericIntrinsic<LazyArenaId<'a>>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &mut LazyExprArena<'bump>,
    ) -> Result<Self::Target<'bump>, MirResolveError> {
        let condition = self.condition().unwrap().resolve(resolver, bump)?;
        let then_expr = self.body().unwrap().resolve(resolver, bump)?;
        let else_expr = self.else_body().unwrap().resolve(resolver, bump)?;

        Ok(GenericIntrinsic::IfElse([condition, then_expr, else_expr]))
    }
}
