use bumpalo::Bump;
use rnix::ast;

use crate::mir::{
    Expr, Ident, Lambda, LambdaParamResolver, Param, Resolve, Resolver, error::MirResolveError,
    lambda::RawLambda,
};

pub type LangLambda<'bump> = RawLambda<&'bump Expr<'bump>>;

impl Resolve for ast::Lambda {
    type Target<'bump> = Lambda<'bump>;

    fn resolve<'b>(
        self,
        resolver: &impl Resolver<'b>,
        bump: &'b Bump,
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

        Ok(Lambda::Lang(LangLambda {
            param: Param::new(&resolver),
            body,
        }))
    }
}
