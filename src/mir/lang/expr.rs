use std::fmt::{Debug, Formatter};

use rnix::ast;

use crate::{
    arena::{DebugWith, LazyArena, LazyArenaId, LazyDebugState},
    mir::{
        error::MirResolveError,
        expr::GenericMirExpr,
        ident_resolver::{Resolve, Resolver},
    },
};

pub type LazyMirExpr<'id> = GenericMirExpr<LazyArenaId<'id>>;
pub type LazyExprArena<'id> = LazyArena<'id, LazyMirExpr<'id>>;

impl Resolve for ast::Expr {
    type Target<'bump> = LazyArenaId<'bump>;

    fn resolve<'b>(
        self,
        resolver: &impl Resolver<'b>,
        bump: &mut LazyExprArena<'b>,
    ) -> Result<LazyArenaId<'b>, MirResolveError> {
        Ok(match self {
            ast::Expr::Lambda(lambda) => {
                let resolved_lambda = lambda.resolve(resolver, bump)?;
                bump.alloc(LazyMirExpr::Lambda(resolved_lambda))
            }

            ast::Expr::BinOp(bin_op) => {
                let bin_op = bin_op.resolve(resolver, bump)?;
                bump.alloc(LazyMirExpr::Intrinsic(bin_op))
            }
            ast::Expr::Apply(apply) => {
                let lambda_call = apply.resolve(resolver, bump)?;
                bump.alloc(LazyMirExpr::Intrinsic(lambda_call))
            }
            ast::Expr::IfElse(if_else) => {
                let if_else = if_else.resolve(resolver, bump)?;
                bump.alloc(LazyMirExpr::Intrinsic(if_else))
            }

            ast::Expr::Literal(lit) => bump.alloc(LazyMirExpr::Literal(lit.kind().into())),
            ast::Expr::Paren(paren) => paren.expr().unwrap().resolve(resolver, bump)?,
            ast::Expr::Ident(ident) => resolver.resolve_ident(&ident.into(), bump)?,
            ast::Expr::LetIn(let_in) => let_in.resolve(resolver, bump)?,

            _ => todo!("Translate {:?} to Mir", self),
        })
    }
}

impl<'id> DebugWith<LazyDebugState<'id, '_, LazyMirExpr<'id>>> for LazyMirExpr<'id> {
    fn fmt_with(
        &self,
        with: &LazyDebugState<'id, '_, LazyMirExpr<'id>>,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Lambda(inner) => inner.fmt_with(with, f),
            Self::Intrinsic(inner) => inner.fmt_with(with, f),

            Self::Literal(inner) => inner.fmt(f),
            Self::Param(inner) => inner.fmt(f),
        }
    }
}
