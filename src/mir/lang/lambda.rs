use std::fmt::Formatter;
use std::iter;

use rnix::ast;

use crate::{
    arena::DebugWith,
    generic_lang::GenericLambda,
    mir::{
        Expr, ExprArena, ExprId, Ident, Intrinsic, LambdaParamResolver, Param, Resolve, Resolver,
        error::MirResolveError, mir_expr_arena::DebugState,
    },
};

pub type Lambda<'bump> = GenericLambda<ExprId<'bump>>;

impl<'b> Lambda<'b> {
    /// Creates a Lambda wrapping an Intrinsic with the parameter names in `params`.
    pub fn with_params(
        intrinsic: Intrinsic,
        params: &[&str],
        bump: &mut ExprArena<'b>,
    ) -> ExprId<'b> {
        Self::at_depth(intrinsic, params, 0, bump)
    }

    fn at_depth(
        intrinsic: Intrinsic,
        params: &[&str],
        depth: usize,
        bump: &mut ExprArena<'b>,
    ) -> ExprId<'b> {
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
        bump: &mut ExprArena<'b>,
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
    type Item = ExprId<'id>;
    type IntoIter = iter::Once<ExprId<'id>>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(*self.body())
    }
}

impl<'b> PartialEq for Lambda<'b> {
    fn eq(&self, other: &Self) -> bool {
        self.param() == other.param() && self.body() == other.body()
    }
}

impl<'id> DebugWith<DebugState<'id, '_>> for Lambda<'id> {
    fn fmt_with(&self, with: &mut DebugState<'id, '_>, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lambda")
            .field("param", self.param())
            .field("body", &self.body().as_wrapper(with))
            .finish()
    }
}
