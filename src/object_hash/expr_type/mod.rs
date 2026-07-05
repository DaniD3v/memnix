//! A wrapper around `Expr` that can additionally store a hash

mod root_node;

pub use root_node::OnceHashRootExpr;

use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter},
};

use crate::{
    Arena, ArenaId,
    arena::{DebugState, DebugWith},
    generic_lang::WithExprType,
    mir::MirExpr,
};

use getset::{Getters, MutGetters};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color(pub blake3::Hash);

impl Color {
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }
}

impl Ord for Color {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}
impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color({})", self.0)
    }
}

#[derive(Getters, MutGetters)]
pub struct OnceHashExpr<'id> {
    #[get = "pub"]
    expr: MirExpr<'id>,
    #[get = "pub"]
    #[get_mut = "pub"]
    color: Option<Color>,
}

type OnceHashExprId<'id> = ArenaId<'id>;
type OnceHashExprArena<'id> = Arena<'id, OnceHashExpr<'id>>;

impl<'p, 'n: 'p> WithExprType<'p, 'n, OnceHashExpr<'n>> for MirExpr<'p> {
    type State<'s>
        = &'s dyn Fn(ArenaId<'p>) -> ArenaId<'n>
    where
        'p: 's;

    fn with_expr<'s>(&self, state: Self::State<'s>) -> OnceHashExpr<'n> {
        OnceHashExpr {
            expr: match self {
                Self::LambdaCall(inner) => MirExpr::LambdaCall(inner.with_expr(state)),
                Self::Lambda(inner) => MirExpr::Lambda(inner.with_expr(state)),

                Self::Literal(inner) => MirExpr::Literal(inner.clone()),
                Self::Param(inner) => MirExpr::Param(inner.clone()),
                Self::Intrinsic(inner) => MirExpr::Intrinsic(*inner),
            },
            color: None,
        }
    }
}

impl<'p, 'n: 'p> WithExprType<'p, 'n, ArenaId<'n>> for ArenaId<'p> {
    type State<'s>
        = &'s dyn Fn(ArenaId<'p>) -> ArenaId<'n>
    where
        'p: 's;

    fn with_expr<'s>(&self, state: Self::State<'s>) -> ArenaId<'n> {
        state(*self)
    }
}

impl<'id> DebugWith<DebugState<'id, '_, Arena<'id, OnceHashExpr<'id>>>> for OnceHashExpr<'id> {
    fn fmt_with(
        &self,
        with: &DebugState<'id, '_, Arena<'id, OnceHashExpr<'id>>>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("OnceHashExpr")
            .field("expr", &self.expr.as_wrapper(with))
            .field("hash", &self.color)
            .finish()
    }
}

impl<'id> DebugWith<DebugState<'id, '_, Arena<'id, OnceHashExpr<'id>>>> for MirExpr<'id> {
    fn fmt_with(
        &self,
        with: &DebugState<'id, '_, Arena<'id, OnceHashExpr<'id>>>,
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
