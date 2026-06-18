use std::fmt::{Debug, Formatter};

use rnix::ast;

use crate::{
    arena::DebugWith,
    mir::{
        ExprArena, ExprId, Intrinsic, Lambda, LambdaCall, Literal, Param, Resolve, Resolver,
        error::MirResolveError, mir_expr_arena::DebugState,
    },
};

pub enum Expr<'b> {
    LambdaCall(LambdaCall<'b>),
    Lambda(Lambda<'b>),
    Literal(Literal),

    Param(Param),
    Intrinsic(Intrinsic),
}

impl Resolve for ast::Expr {
    type Target<'bump> = ExprId<'bump>;

    fn resolve<'b>(
        self,
        resolver: &impl Resolver<'b>,
        bump: &mut ExprArena<'b>,
    ) -> Result<ExprId<'b>, MirResolveError> {
        Ok(match self {
            ast::Expr::Apply(apply) => {
                let lambda_call = apply.resolve(resolver, bump)?;
                bump.alloc(Expr::LambdaCall(lambda_call))
            }
            ast::Expr::Lambda(lambda) => {
                let resolved_lambda = lambda.resolve(resolver, bump)?;
                bump.alloc(Expr::Lambda(resolved_lambda))
            }
            ast::Expr::Literal(lit) => bump.alloc(Expr::Literal(lit.kind().into())),
            ast::Expr::IfElse(if_else) => {
                let lambda_call = if_else.resolve(resolver, bump)?;
                bump.alloc(Expr::LambdaCall(lambda_call))
            }
            ast::Expr::BinOp(bin_op) => {
                let lambda_call = bin_op.resolve(resolver, bump)?;
                bump.alloc(Expr::LambdaCall(lambda_call))
            }
            ast::Expr::Paren(paren) => paren.expr().unwrap().resolve(resolver, bump)?,
            ast::Expr::Ident(ident) => resolver.resolve_ident(&ident.into(), bump)?,
            ast::Expr::LetIn(let_in) => let_in.resolve(resolver, bump)?,

            _ => todo!("Translate {:?} to Mir", self),
        })
    }
}

impl<'id> DebugWith<DebugState<'id, '_>> for Expr<'id> {
    fn fmt_with(&self, with: &mut DebugState<'id, '_>, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Don't wrap the variants twice
        match self {
            Self::LambdaCall(inner) => inner.fmt_with(with, f),
            Self::Lambda(inner) => inner.fmt_with(with, f),

            Self::Literal(inner) => inner.fmt(f),
            Self::Param(inner) => inner.fmt(f),
            Self::Intrinsic(inner) => inner.fmt(f),
        }
    }
}
