use rnix::ast;

use crate::{
    arena::LazyArenaId,
    generic_lang::GenericLambda,
    mir::{
        Ident, Intrinsic, Param,
        error::MirResolveError,
        ident_resolver::{LambdaParamResolver, Resolve, Resolver},
        lang::{LazyExprArena, LazyMirExpr},
    },
};

pub type LazyMirLambda<'bump> = GenericLambda<LazyArenaId<'bump>>;

impl<'b> LazyMirLambda<'b> {
    /// Creates a Lambda wrapping an Intrinsic with the parameter names in `params`.
    pub fn with_params(
        intrinsic: Intrinsic,
        params: &[&str],
        bump: &mut LazyExprArena<'b>,
    ) -> LazyArenaId<'b> {
        Self::at_depth(intrinsic, params, 0, bump)
    }

    fn at_depth(
        intrinsic: Intrinsic,
        params: &[&str],
        depth: usize,
        bump: &mut LazyExprArena<'b>,
    ) -> LazyArenaId<'b> {
        assert!(!params.is_empty());

        let expr = LazyMirExpr::Lambda(Self::new(
            Param::at_depth(depth),
            if params.len() == 1 {
                bump.alloc(LazyMirExpr::Intrinsic(intrinsic))
            } else {
                Self::at_depth(intrinsic, &params[1..], depth + 1, bump)
            },
        ));

        bump.alloc(expr)
    }
}

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

        let body_resolver = LambdaParamResolver {
            ident: param_name.clone(),
            expr: bump.alloc(LazyMirExpr::Param(Param::new(resolver))),
            parent: resolver,
        };
        let body = self.body().unwrap().resolve(&body_resolver, bump)?;

        Ok(LazyMirLambda::new(Param::new(&resolver), body))
    }
}
