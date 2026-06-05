use bumpalo::Bump;
use rnix::ast;

use crate::mir::{
    Lambda, LambdaCall, LetIn, Literal, Param,
    error::MirResolveError, lazy_eval::Resolve, symbol_resolver::Resolver,
};

#[derive(Debug)]
pub enum Expr<'bump> {
    LetIn(LetIn<'bump>),
    LambdaCall(LambdaCall<'bump>),
    Lambda(Lambda<'bump>),
    Literal(Literal),

    Param(Param),

    Intrinsic,
}

impl Resolve for ast::Expr {
    type Target<'bump> = &'bump Expr<'bump>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<&'bump Expr<'bump>, MirResolveError> {
        Ok(match self {
            ast::Expr::LetIn(let_in) => bump.alloc(Expr::LetIn(let_in.resolve(resolver, bump)?)),
            ast::Expr::Apply(apply) => bump.alloc(Expr::LambdaCall(apply.resolve(resolver, bump)?)),
            ast::Expr::Lambda(lambda) => bump.alloc(Expr::Lambda(lambda.resolve(resolver, bump)?)),
            ast::Expr::Literal(lit) => bump.alloc(Expr::Literal(lit.kind().into())),

            ast::Expr::IfElse(if_else) => {
                bump.alloc(Expr::LambdaCall(if_else.resolve(resolver, bump)?))
            }
            ast::Expr::BinOp(bin_op) => {
                bump.alloc(Expr::LambdaCall(bin_op.resolve(resolver, bump)?))
            }
            ast::Expr::Paren(paren) => paren.expr().unwrap().resolve(resolver, bump)?,
            ast::Expr::Ident(ident) => resolver.resolve_ident(ident.into(), bump)?,

            _ => todo!("Translate {:?} to Mir", self),
        })
    }
}
