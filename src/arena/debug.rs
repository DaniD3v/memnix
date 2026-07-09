use std::{
    cell::RefCell,
    fmt,
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

use crate::{Arena, ArenaId};

pub trait DebugWith<T>: Sized {
    fn fmt_with(&self, with: &T, f: &mut Formatter<'_>) -> std::fmt::Result;

    fn as_wrapper<'a>(&'a self, with: &'a T) -> DebugWithWrapper<'a, T, Self> {
        DebugWithWrapper { inner: self, with }
    }
}

pub trait DebugArena<'id> {
    type Item;

    // TODO doc
    // index that is the same for 2 equal elements
    // TODO: this is actually broken cause cycles
    fn canonical_idx(&self, id: ArenaId<'id>) -> usize;
    fn get(&self, id: ArenaId<'id>) -> &Self::Item;
    fn size(&self) -> usize;
}

pub struct GenericDebugState<'id, 'a, A: DebugArena<'id>> {
    arena: &'a A,
    already_debugged: RefCell<Vec<bool>>,
    _phantom: PhantomData<fn(&'id ()) -> &'id ()>,
}

pub type DebugState<'id, 'a, T> = GenericDebugState<'id, 'a, Arena<'id, T>>;

impl<'id, 'a, A: DebugArena<'id>> GenericDebugState<'id, 'a, A> {
    pub fn new(arena: &'a A) -> Self {
        Self {
            arena,
            already_debugged: RefCell::new(vec![false; arena.size()]),
            _phantom: PhantomData,
        }
    }
}

// NOTE:
// This often throws confusing errors because the trait bound is circular.
// The bound `E: DebugWith<DebugState<'id, 'a, E>>` has to be implemented directly.
impl<'id, 'a, A: DebugArena<'id>> DebugWith<GenericDebugState<'id, 'a, A>> for ArenaId<'id>
where
    A::Item: DebugWith<GenericDebugState<'id, 'a, A>>,
{
    fn fmt_with(&self, with: &GenericDebugState<'id, 'a, A>, f: &mut Formatter<'_>) -> fmt::Result {
        let idx = with.arena.canonical_idx(*self);

        if with.already_debugged.borrow()[idx] {
            write!(f, "<<repeated: {}>>", idx)
        } else {
            with.already_debugged.borrow_mut()[idx] = true;
            with.arena.get(*self).fmt_with(with, f)
        }
    }
}

/// A wrapper around elements implementing `DebugWith`
/// that requires the `with` and implements `Debug`
pub struct DebugWithWrapper<'a, W, T: DebugWith<W>> {
    inner: &'a T,
    with: &'a W,
}

impl<'a, W, T: DebugWith<W>> DebugWithWrapper<'a, W, T> {
    pub fn new(inner: &'a T, with: &'a W) -> Self {
        Self { inner, with }
    }
}

impl<'a, W, T: DebugWith<W>> Debug for DebugWithWrapper<'a, W, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt_with(self.with, f)
    }
}
