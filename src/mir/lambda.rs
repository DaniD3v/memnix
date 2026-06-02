use bumpalo::Bump;
use rnix::ast;

use crate::mir::{
    Expr, Param,
    lazy_eval::Resolve,
    symbol_resolver::{Resolver, SingleNameResolver},
};

#[derive(Debug)]
pub struct Lambda<'bump> {
    // theres goofy `{}` desugars too but lets ignore those for now
    param: String,
    body: &'bump Expr<'bump>,
}

impl Resolve for ast::Lambda {
    type Target<'bump> = &'bump Lambda<'bump>;

    fn resolve<'bump>(self, _: &impl Resolver<'bump>, bump: &'bump Bump) -> Self::Target<'bump> {
        let param = match self.param().unwrap() {
            ast::Param::IdentParam(ident) => ident.to_string(),
            ast::Param::Pattern(_) => todo!("oje"),
        };

        let resolver = SingleNameResolver {
            name: param.clone(),
            expr: bump.alloc(Expr::Param(&Param)),
        };
        let body = self.body().unwrap().resolve(&resolver, bump);

        bump.alloc(Lambda { param, body })
    }
}
