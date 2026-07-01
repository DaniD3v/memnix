use std::{
    fmt::{Debug, Formatter},
    iter,
};

use rnix::ast;

use crate::{
    ArenaId,
    arena::{DebugWith, LazyDebugState},
    mir::{
        Intrinsic, LazyExprArena, Literal, MirLambda, MirLambdaCall, Param, Resolve, Resolver,
        error::MirResolveError,
    },
};

pub enum MirExpr<'b> {
    LambdaCall(MirLambdaCall<'b>),
    Lambda(MirLambda<'b>),
    Literal(Literal),

    Param(Param),
    Intrinsic(Intrinsic),
}

impl Resolve for ast::Expr {
    type Target<'bump> = ArenaId<'bump>;

    fn resolve<'b>(
        self,
        resolver: &impl Resolver<'b>,
        bump: &mut LazyExprArena<'b>,
    ) -> Result<ArenaId<'b>, MirResolveError> {
        Ok(match self {
            ast::Expr::Apply(apply) => {
                let lambda_call = apply.resolve(resolver, bump)?;
                bump.alloc(MirExpr::LambdaCall(lambda_call))
            }
            ast::Expr::Lambda(lambda) => {
                let resolved_lambda = lambda.resolve(resolver, bump)?;
                bump.alloc(MirExpr::Lambda(resolved_lambda))
            }
            ast::Expr::Literal(lit) => bump.alloc(MirExpr::Literal(lit.kind().into())),
            ast::Expr::IfElse(if_else) => {
                let lambda_call = if_else.resolve(resolver, bump)?;
                bump.alloc(MirExpr::LambdaCall(lambda_call))
            }
            ast::Expr::BinOp(bin_op) => {
                let lambda_call = bin_op.resolve(resolver, bump)?;
                bump.alloc(MirExpr::LambdaCall(lambda_call))
            }
            ast::Expr::Paren(paren) => paren.expr().unwrap().resolve(resolver, bump)?,
            ast::Expr::Ident(ident) => resolver.resolve_ident(&ident.into(), bump)?,
            ast::Expr::LetIn(let_in) => let_in.resolve(resolver, bump)?,

            _ => todo!("Translate {:?} to Mir", self),
        })
    }
}

impl<'id> IntoIterator for &MirExpr<'id> {
    type Item = ArenaId<'id>;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'id>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            MirExpr::LambdaCall(lambda_call) => Box::new(lambda_call.into_iter()),
            MirExpr::Lambda(lambda) => Box::new(lambda.into_iter()),

            _ => Box::new(iter::empty()),
        }
    }
}

impl<'b> PartialEq for MirExpr<'b> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LambdaCall(l), Self::LambdaCall(r)) => l == r,
            (Self::Lambda(l), Self::Lambda(r)) => l == r,
            (Self::Literal(l), Self::Literal(r)) => l == r,
            (Self::Param(l), Self::Param(r)) => l == r,
            (Self::Intrinsic(l), Self::Intrinsic(r)) => l == r,

            _ => false,
        }
    }
}

impl<'id> DebugWith<LazyDebugState<'id, '_, MirExpr<'id>>> for MirExpr<'id> {
    fn fmt_with(
        &self,
        with: &mut LazyDebugState<'id, '_, MirExpr<'id>>,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::LambdaCall(inner) => inner.fmt_with(with, f),
            Self::Lambda(inner) => inner.fmt_with(with, f),

            Self::Literal(inner) => inner.fmt(f),
            Self::Param(inner) => inner.fmt(f),
            Self::Intrinsic(inner) => inner.fmt(f),
        }
    }
}
