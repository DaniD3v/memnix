use core::fmt;
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

use crate::{Arena, ArenaId};

pub trait DebugWith<T>: Sized {
    fn fmt_with(&self, with: &mut T, f: &mut Formatter<'_>) -> std::fmt::Result;

    fn as_wrapper<'a>(&'a self, with: &'a mut T) -> DebugWithWrapper<'a, T, Self> {
        DebugWithWrapper {
            inner: self,
            with: RefCell::new(with),
        }
    }
}

pub trait DebugArena<'id> {
    type Item;
    fn canonical_idx(&self, id: ArenaId<'id>) -> usize;
    fn get(&self, id: ArenaId<'id>) -> &Self::Item;
    fn size(&self) -> usize;
}

impl<'id, T> DebugArena<'id> for Arena<'id, T> {
    type Item = T;

    fn canonical_idx(&self, id: ArenaId<'id>) -> usize {
        id.idx()
    }

    fn get(&self, id: ArenaId<'id>) -> &T {
        &self[id]
    }

    fn size(&self) -> usize {
        self.size()
    }
}

pub struct DebugState<'id, 'a, A: DebugArena<'id>> {
    arena: &'a A,
    already_debugged: Vec<bool>,
    _phantom: PhantomData<fn(&'id ()) -> &'id ()>,
}

impl<'id, 'a, A: DebugArena<'id>> DebugState<'id, 'a, A> {
    pub fn new(arena: &'a A) -> Self {
        Self {
            arena,
            already_debugged: vec![false; arena.size()],
            _phantom: PhantomData,
        }
    }
}

// NOTE:
// This often throws confusing errors because the trait bound is circular.
// The bound `E: DebugWith<DebugState<'id, 'a, E>>` has to be implemented directly.
impl<'id, 'a, A> DebugWith<DebugState<'id, 'a, A>> for ArenaId<'id>
where
    A: DebugArena<'id>,
    A::Item: DebugWith<DebugState<'id, 'a, A>>,
{
    fn fmt_with(&self, with: &mut DebugState<'id, 'a, A>, f: &mut Formatter<'_>) -> fmt::Result {
        let idx = with.arena.canonical_idx(*self);
        if with.already_debugged[idx] {
            write!(f, "<<repeated: {}>>", idx)
        } else {
            with.already_debugged[idx] = true;
            with.arena.get(*self).fmt_with(with, f)
        }
    }
}

pub struct DebugWithWrapper<'a, W, T: DebugWith<W>> {
    inner: &'a T,
    with: RefCell<&'a mut W>,
}

impl<'a, W, T: DebugWith<W>> Debug for DebugWithWrapper<'a, W, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { inner, with } = self;
        let with = &mut with.borrow_mut();
        inner.fmt_with(with, f)
    }
}
