use std::iter;

use rnix::ast;

use crate::{
    ArenaId,
    generic_lang::GenericLambda,
    mir::{
        Expr, Ident, Intrinsic, LambdaParamResolver, LazyExprArena, Param, Resolve, Resolver,
        error::MirResolveError,
    },
};

pub type Lambda<'bump> = GenericLambda<ArenaId<'bump>>;

impl<'b> Lambda<'b> {
    /// Creates a Lambda wrapping an Intrinsic with the parameter names in `params`.
    pub fn with_params(
        intrinsic: Intrinsic,
        params: &[&str],
        bump: &mut LazyExprArena<'b>,
    ) -> ArenaId<'b> {
        Self::at_depth(intrinsic, params, 0, bump)
    }

    fn at_depth(
        intrinsic: Intrinsic,
        params: &[&str],
        depth: usize,
        bump: &mut LazyExprArena<'b>,
    ) -> ArenaId<'b> {
        assert!(!params.is_empty());

        let expr = Expr::Lambda(Self::new(
            Param::at_depth(depth),
            if params.len() == 1 {
                bump.alloc(Expr::Intrinsic(intrinsic))
            } else {
                Self::at_depth(intrinsic, &params[1..], depth + 1, bump)
            },
        ));

        bump.alloc(expr)
    }
}

impl Resolve for ast::Lambda {
    type Target<'bump> = Lambda<'bump>;

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
            expr: bump.alloc(Expr::Param(Param::new(resolver))),
            parent: resolver,
        };
        let body = self.body().unwrap().resolve(&body_resolver, bump)?;

        Ok(Lambda::new(Param::new(&resolver), body))
    }
}

impl<'id> IntoIterator for &Lambda<'id> {
    type Item = ArenaId<'id>;
    type IntoIter = iter::Once<ArenaId<'id>>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(*self.body())
    }
}

impl<'b> PartialEq for Lambda<'b> {
    fn eq(&self, other: &Self) -> bool {
        self.param() == other.param() && self.body() == other.body()
    }
}
