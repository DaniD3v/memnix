//! A wrapper around `Expr` that can additionally store a hash

mod root_node;

pub use root_node::ColorableRootExpr;

use std::fmt::{Debug, Formatter};

use crate::{
    arena::{Arena, ArenaId, DebugState, DebugWith},
    coloring::Color,
    mir::MirExpr,
};

use getset::{CopyGetters, Getters, MutGetters};

#[derive(Getters, CopyGetters, MutGetters)]
pub struct ColoredExpr<'id> {
    #[get = "pub"]
    expr: MirExpr<'id>,

    #[get_copy = "pub"]
    #[get_mut = "pub"]
    color: Option<Color>,
}

pub type ColoredExprArena<'id> = Arena<'id, ColoredExpr<'id>>;

impl<'id> ColoredExpr<'id> {
    pub fn convert_inner_idx<'n>(
        self,
        map: impl Fn(ArenaId<'id>) -> ArenaId<'n>,
    ) -> ColoredExpr<'n> {
        ColoredExpr {
            expr: match self.expr {
                MirExpr::Lambda(inner) => MirExpr::Lambda(inner.convert_inner(map)),
                MirExpr::Intrinsic(inner) => MirExpr::Intrinsic(inner.convert_inner(map)),

                MirExpr::Literal(inner) => MirExpr::Literal(inner),
                MirExpr::Param(inner) => MirExpr::Param(inner),
            },
            color: self.color,
        }
    }

    pub fn from_mir<'p>(prev: MirExpr<'p>, map: impl Fn(ArenaId<'p>) -> ArenaId<'id>) -> Self {
        ColoredExpr {
            expr: match prev {
                MirExpr::Lambda(inner) => MirExpr::Lambda(inner.convert_inner(map)),
                MirExpr::Intrinsic(inner) => MirExpr::Intrinsic(inner.convert_inner(map)),

                MirExpr::Literal(inner) => MirExpr::Literal(inner),
                MirExpr::Param(inner) => MirExpr::Param(inner),
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
            Self::Lambda(inner) => inner.fmt_with(with, f),
            Self::Intrinsic(inner) => inner.fmt_with(with, f),

            Self::Literal(inner) => inner.fmt(f),
            Self::Param(inner) => inner.fmt(f),
        }
    }
}
