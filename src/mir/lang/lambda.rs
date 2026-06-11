use bumpalo::Bump;
use rnix::ast;

use crate::mir::{
    Expr, Ident, LambdaParamResolver, Param, Resolve, Resolver, error::MirResolveError,
    lang::intrinsics::Intrinsic,
};

#[derive(Debug)]
pub struct Lambda<'bump> {
    // theres goofy `{}` desugars too but lets ignore those for now
    param: Ident,
    body: &'bump Expr<'bump>,
}

impl<'bump> Lambda<'bump> {
    /// Creates a Lambda wrapping an Intrinsic with the parameter names in `params`.
    pub fn builtin_with_params(intrinsic: Intrinsic, params: &[&str], bump: &'bump Bump) -> Self {
        Self::builtin_at_depth(intrinsic, params, 0, bump)
    }

    fn builtin_at_depth(
        intrinsic: Intrinsic,
        params: &[&str],
        depth: usize,
        bump: &'bump Bump,
    ) -> Self {
        assert!(!params.is_empty());

        Self {
            param: Ident::new(params[0].to_owned()),
            body: bump.alloc(if params.len() == 1 {
                Expr::Intrinsic(intrinsic)
            } else {
                Expr::Lambda(Self::builtin_at_depth(
                    intrinsic,
                    &params[1..],
                    depth + 1,
                    bump,
                ))
            }),
        }
    }
}

impl Resolve for ast::Lambda {
    type Target<'bump> = Lambda<'bump>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<Lambda<'bump>, MirResolveError> {
        let param: Ident = match self.param().unwrap() {
            ast::Param::IdentParam(ident) => ident.ident().unwrap(),
            ast::Param::Pattern(_) => todo!("oje"),
        }
        .into();

        let resolver = LambdaParamResolver {
            ident: param.clone(),
            expr: bump.alloc(Expr::Param(Param::new(resolver))),
            parent: resolver,
        };
        let body = self.body().unwrap().resolve(&resolver, bump)?;

        Ok(Lambda { param, body })
    }
}
