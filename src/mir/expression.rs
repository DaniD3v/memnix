use bumpalo::Bump;
use rnix::ast;

use crate::mir::{
    Literal, Param,
    lazy_eval::Resolve,
    let_in::LetIn,
    symbol_resolver::{NullResolver, Resolver},
};

#[derive(Debug)]
pub enum Expr<'bump> {
    Literal(Literal),
    LetIn(&'bump LetIn<'bump>),
    Param(&'bump Param),
}

impl Resolve for ast::Expr {
    type Target<'bump> = &'bump Expr<'bump>;

    fn resolve<'bump>(self, _: &impl Resolver<'bump>, bump: &'bump Bump) -> Self::Target<'bump> {
        bump.alloc(match self {
            ast::Expr::Literal(lit) => Expr::Literal(lit.kind().into()),
            ast::Expr::LetIn(let_in) => Expr::LetIn(let_in.resolve(&NullResolver, bump)),
            _ => todo!(),
        })
    }
}
