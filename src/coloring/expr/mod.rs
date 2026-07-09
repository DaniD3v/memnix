//! A wrapper around `Expr` that can additionally store a hash

mod root_node;

pub use root_node::ColorableRootExpr;

use std::fmt::{Debug, Formatter};

use crate::{
    Arena, ArenaId,
    arena::{DebugState, DebugWith},
    coloring::Color,
    generic_lang::WithExprType,
    mir::MirExpr,
};

use getset::{Getters, MutGetters};

#[derive(Getters, MutGetters)]
pub struct ColoredExpr<'id> {
    #[get = "pub"]
    expr: MirExpr<'id>,
    #[get = "pub"]
    #[get_mut = "pub"]
    color: Option<Color>,
}

pub type ColoredExprArena<'id> = Arena<'id, ColoredExpr<'id>>;

impl<'p, 'n: 'p> WithExprType<'p, 'n, ColoredExpr<'n>> for MirExpr<'p> {
    type State<'s>
        = &'s dyn Fn(ArenaId<'p>) -> ArenaId<'n>
    where
        'p: 's;

    fn with_expr<'s>(self, state: Self::State<'s>) -> ColoredExpr<'n> {
        ColoredExpr {
            expr: match self {
                Self::LambdaCall(inner) => MirExpr::LambdaCall(inner.with_expr(state)),
                Self::Lambda(inner) => MirExpr::Lambda(inner.with_expr(state)),

                Self::Literal(inner) => MirExpr::Literal(inner),
                Self::Param(inner) => MirExpr::Param(inner),
                Self::Intrinsic(inner) => MirExpr::Intrinsic(inner),
            },
            color: None,
        }
    }
}

impl<'id> DebugWith<DebugState<'id, '_, ColoredExpr<'id>>> for ColoredExpr<'id> {
    fn fmt_with(
        &self,
        with: &DebugState<'id, '_, ColoredExpr<'id>>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("ColoredExpr")
            .field("expr", &self.expr.as_wrapper(with))
            .field("color", &self.color)
            .finish()
    }
}

impl<'id> DebugWith<DebugState<'id, '_, ColoredExpr<'id>>> for MirExpr<'id> {
    fn fmt_with(
        &self,
        with: &DebugState<'id, '_, ColoredExpr<'id>>,
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
