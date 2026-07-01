//! A wrapper around `Expr` that can additionally store a hash

mod root_node;

use std::fmt::{Debug, Formatter};

use crate::{
    Arena, ArenaId,
    arena::{DebugState, DebugWith},
    generic_lang::WithExprType,
    mir::{Expr, RootExpr},
};

pub use root_node::OnceHashRootExpr;

type TodoHash = ();

pub struct OnceHashExpr<'id> {
    expr: Expr<'id>,
    hash: Option<TodoHash>,
}

type OnceHashExprId<'id> = ArenaId<'id>;
type OnceHashExprArena<'id> = Arena<'id, OnceHashExpr<'id>>;

impl<'id> OnceHashExpr<'id> {
    pub fn from_mir_root<'p>(mir_expr: RootExpr<'p>) -> OnceHashExprId<'id> {
        // let arena = Arena::new();

        // mir_expr
        //     .root_node()
        //     .with_expr(&mut WithExprState::new(arena));

        todo!()
    }
}

impl<'p, 'n: 'p> WithExprType<'p, 'n, OnceHashExpr<'n>> for Expr<'p> {
    type State<'s>
        = &'s dyn Fn(ArenaId<'p>) -> ArenaId<'n>
    where
        'p: 's;

    fn with_expr<'s>(&self, state: Self::State<'s>) -> OnceHashExpr<'n> {
        OnceHashExpr {
            expr: match self {
                Self::LambdaCall(inner) => Expr::LambdaCall(inner.with_expr(state)),
                Self::Lambda(inner) => Expr::Lambda(inner.with_expr(state)),

                Self::Literal(inner) => Expr::Literal(inner.clone()),
                Self::Param(inner) => Expr::Param(inner.clone()),
                Self::Intrinsic(inner) => Expr::Intrinsic(inner.clone()),
            },
            hash: None,
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
        with: &mut DebugState<'id, '_, Arena<'id, OnceHashExpr<'id>>>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("OnceHashExpr")
            .field("expr", &self.expr.as_wrapper(with))
            .field("hash", &self.hash)
            .finish()
    }
}

impl<'id> DebugWith<DebugState<'id, '_, Arena<'id, OnceHashExpr<'id>>>> for Expr<'id> {
    fn fmt_with(
        &self,
        with: &mut DebugState<'id, '_, Arena<'id, OnceHashExpr<'id>>>,
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
