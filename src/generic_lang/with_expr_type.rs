use crate::{ArenaId, arena::LazyArenaId};

/// Swaps the Expression of a generic lang item to the new Expression type `E`
///
/// `'p`: lifetime of the previous expr
/// `'n`: lifetime of the next expr
pub trait WithExprType<'p, 'n, E> {
    type State<'s>: Clone
    where
        'p: 's;

    fn with_expr<'s>(self, state: Self::State<'s>) -> E;
}

impl<'p, 'n: 'p> WithExprType<'p, 'n, ArenaId<'n>> for ArenaId<'p> {
    type State<'s>
        = &'s dyn Fn(ArenaId<'p>) -> ArenaId<'n>
    where
        'p: 's;

    fn with_expr<'s>(self, state: Self::State<'s>) -> ArenaId<'n> {
        state(self)
    }
}

impl<'p, 'n: 'p> WithExprType<'p, 'n, ArenaId<'n>> for LazyArenaId<'p> {
    type State<'s>
        = &'s dyn Fn(LazyArenaId<'p>) -> ArenaId<'n>
    where
        'p: 's;

    fn with_expr<'s>(self, state: Self::State<'s>) -> ArenaId<'n> {
        state(self)
    }
}
