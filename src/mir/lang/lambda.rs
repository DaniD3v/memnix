use rnix::ast;

use crate::{
    arena::LazyArenaId,
    generic_lang::GenericLambda,
    mir::{
        Ident, Param,
        error::MirResolveError,
        ident_resolver::{LambdaParamResolver, Resolve, Resolver},
        lang::{LazyExprArena, LazyMirExpr},
    },
};

pub type LazyMirLambda<'bump> = GenericLambda<LazyArenaId<'bump>>;

impl Resolve for ast::Lambda {
    type Target<'bump> = LazyMirLambda<'bump>;

    fn resolve<'b>(
        self,
        resolver: &impl Resolver<'b>,
        bump: &mut LazyExprArena<'b>,
    ) -> Result<Self::Target<'b>, MirResolveError> {
        let param_name: Ident = match self.param().unwrap() {
            ast::Param::IdentParam(ident) => ident.ident().unwrap(),
            ast::Param::Pattern(_) => todo!("oje"),
        }
        .into();

        let body_resolver = LambdaParamResolver::new(
            param_name,
            bump.alloc(LazyMirExpr::Param(Param::new(resolver))),
            resolver,
        );
        let body = self.body().unwrap().resolve(&body_resolver, bump)?;

        Ok(LazyMirLambda::new(Param::new(&resolver), body))
    }
}
