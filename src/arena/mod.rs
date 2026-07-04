mod debug;
mod lazy_arena;

use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice, vec,
};

use getset::CopyGetters;

pub use debug::{DebugState, DebugWith, DebugWithWrapper};
pub use lazy_arena::{LazyArena, LazyDebugState};

use crate::arena::debug::DebugArena;

#[derive(Debug)]
pub struct Arena<'id, T> {
    inner: Vec<T>,
    _id_invariant: PhantomData<fn(&'id ()) -> &'id ()>,
}

/// An `ArenaId` is an index into the `Arena` with the lifetime `id`.
/// The id cannot be an invalid index.
#[derive(CopyGetters)]
pub struct ArenaId<'id> {
    #[getset(get_copy = "pub")]
    idx: usize,
    _id_invariant: PhantomData<fn(&'id ()) -> &'id ()>,
}

impl<'id, T> Default for Arena<'id, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'id, T> Arena<'id, T> {
    // TODO add generativity id
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            _id_invariant: PhantomData,
        }
    }

    pub fn alloc(&mut self, val: T) -> ArenaId<'id> {
        let idx = self.inner.len();
        self.inner.push(val);

        ArenaId {
            idx,
            _id_invariant: PhantomData,
        }
    }

    pub fn map<I>(self, transform: fn(T) -> I) -> Arena<'id, I> {
        let new_vec: Vec<I> = self.inner.into_iter().map(transform).collect();

        Arena {
            inner: new_vec,
            _id_invariant: self._id_invariant,
        }
    }

    pub fn get_index_from(&self, idx: usize) -> Option<ArenaId<'id>> {
        self.inner.get(idx).map(|_| ArenaId {
            idx,
            _id_invariant: PhantomData,
        })
    }

    pub fn iter_indices(&self) -> impl Iterator<Item = ArenaId<'id>> + 'id {
        (0..self.inner.len()).map(|idx| ArenaId {
            idx,
            _id_invariant: PhantomData,
        })
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }

    fn iter(&self) -> slice::Iter<'_, T> {
        self.inner.iter()
    }
}

impl<'id, T> Index<ArenaId<'id>> for Arena<'id, T> {
    type Output = T;

    fn index(&self, index: ArenaId<'id>) -> &Self::Output {
        &self.inner[index.idx()]
    }
}

impl<'id, T> IndexMut<ArenaId<'id>> for Arena<'id, T> {
    fn index_mut(&mut self, index: ArenaId<'id>) -> &mut Self::Output {
        &mut self.inner[index.idx()]
    }
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

impl<'id, T> IntoIterator for Arena<'id, T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'id> Copy for ArenaId<'id> {}
impl<'id> Clone for ArenaId<'id> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'id> PartialEq for ArenaId<'id> {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}
