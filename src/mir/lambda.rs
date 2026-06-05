use bumpalo::Bump;
use rnix::ast;

use crate::mir::{
    Expr, Param,
    error::MirResolveError,
    ident::Ident,
    lazy_eval::Resolve,
    symbol_resolver::{Resolver, SingleIdentResolver},
};

#[derive(Debug)]
pub struct Lambda<'bump> {
    // theres goofy `{}` desugars too but lets ignore those for now
    param: Ident,
    body: &'bump Expr<'bump>,
}

impl<'bump> Lambda<'bump> {
    /// Creates a Lambda wrapping an Intrinsic with the parameter names in `params`.
    pub fn intrinsic_with_params(params: &[&str], bump: &'bump Bump) -> Self {
        assert!(!params.is_empty());

        Self {
            param: Ident::new(params[0].to_owned()),
            body: bump.alloc(if params.len() == 1 {
                Expr::Intrinsic
            } else {
                Expr::Lambda(bump.alloc(Self::intrinsic_with_params(&params[1..], bump)))
            }),
        }
    }
}

impl Resolve for ast::Lambda {
    type Target<'bump> = &'bump Lambda<'bump>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<&'bump Lambda<'bump>, MirResolveError> {
        let param: Ident = match self.param().unwrap() {
            ast::Param::IdentParam(ident) => ident.ident().unwrap(),
            ast::Param::Pattern(_) => todo!("oje"),
        }
        .into();

        let resolver = SingleIdentResolver {
            ident: param.clone(),
            expr: bump.alloc(Expr::Param(Param)),
            parent: resolver,
        };
        let body = self.body().unwrap().resolve(&resolver, bump)?;

        Ok(bump.alloc(Lambda { param, body }))
    }
}
