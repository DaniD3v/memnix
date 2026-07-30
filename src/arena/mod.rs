mod debug;
mod lazy_arena;

use core::fmt;
use std::{
    fmt::{Debug, Formatter},
    ops::{Index, IndexMut},
    slice, vec,
};

use getset::CopyGetters;

pub use debug::{DebugState, DebugWith};
pub use lazy_arena::{LazyArena, LazyArenaId, LazyDebugState};

use crate::arena::debug::DebugArena;

#[derive(Debug)]
pub struct Arena<'id, T: 'id> {
    inner: Vec<T>,
    id: generativity::Id<'id>,
}

/// An `ArenaId` is an index into the `Arena` with the lifetime `id`.
/// The id cannot be an invalid index.
#[derive(CopyGetters, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ArenaId<'id> {
    #[getset(get_copy = "pub")]
    idx: usize,
    id: generativity::Id<'id>,
}

impl<'id, T: 'id> Arena<'id, T> {
    // TODO add generativity id
    pub fn new(guard: generativity::Guard<'id>) -> Self {
        Self {
            inner: Vec::new(),
            id: guard.into(),
        }
    }

    pub fn alloc(&mut self, val: T) -> ArenaId<'id> {
        let idx = self.inner.len();
        self.inner.push(val);

        ArenaId { idx, id: self.id }
    }

    /// Extends this arena by the contents of another
    ///
    /// `root`: The previous root node.
    /// The returned `ArenaId` will be the `T` equivalent of the previous root node.
    ///
    /// `transform_idx`: Should transform all of T's internal `LazyArenaId` references
    /// to `ArenaId` references using the provided mapping closure.
    pub fn extend_map<'other, I>(
        &mut self,
        other: Arena<'other, I>,
        // TODO: better design
        root_node: ArenaId<'other>,
        transform_idx: impl Fn(I, &dyn Fn(ArenaId<'other>) -> ArenaId<'id>) -> T,
    ) -> ArenaId<'id> {
        let idx_map = {
            let self_size = self.size();
            let self_id = self.id;

            move |id: ArenaId<'other>| ArenaId {
                idx: self_size + id.idx(),
                id: self_id,
            }
        };

        for item in other {
            self.alloc(transform_idx(item, &idx_map));
        }

        idx_map(root_node)
    }

    pub fn map<I>(self, transform: fn(T) -> I) -> Arena<'id, I> {
        let new_vec: Vec<I> = self.inner.into_iter().map(transform).collect();

        Arena {
            inner: new_vec,
            id: self.id,
        }
    }

    pub fn get_index_from(&self, idx: usize) -> Option<ArenaId<'id>> {
        self.inner.get(idx).map(|_| ArenaId { idx, id: self.id })
    }

    pub fn iter_indices(&self) -> impl Iterator<Item = ArenaId<'id>> + use<'id, T> {
        let id = self.id;
        (0..self.inner.len()).map(move |idx| ArenaId { idx, id })
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

impl Debug for ArenaId<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ArenaId").field(&self.idx()).finish()
    }
}
