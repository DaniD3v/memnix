use std::{marker::PhantomData, ops::Index};

mod debug_with;

pub use debug_with::DebugWith;
use getset::CopyGetters;

#[derive(Debug)]
pub struct Arena<'id, T> {
    inner: Vec<T>,
    _id_invariant: PhantomData<fn(&'id ()) -> &'id ()>,
}

#[derive(CopyGetters)]
pub struct ArenaId<'id, T> {
    #[getset(get_copy = "pub")]
    idx: usize,
    _id_invariant: PhantomData<fn(&'id ()) -> &'id T>,
}

impl<'id, T> Arena<'id, T> {
    // TODO add generativity id
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            _id_invariant: PhantomData,
        }
    }

    pub fn alloc(&mut self, val: T) -> ArenaId<'id, T> {
        let idx = self.inner.len();
        self.inner.push(val);

        ArenaId {
            idx,
            _id_invariant: PhantomData,
        }
    }

    pub fn replace(&mut self, idx: ArenaId<'id, T>, val: T) -> T {
        std::mem::replace(&mut self.inner[idx.idx], val)
    }

    pub fn get_index_from(&self, idx: usize) -> Option<ArenaId<'id, T>> {
        self.inner.get(idx).map(|_| ArenaId {
            idx,
            _id_invariant: PhantomData,
        })
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }
}

impl<'b, T> Index<ArenaId<'b, T>> for Arena<'b, T> {
    type Output = T;

    fn index(&self, index: ArenaId<'b, T>) -> &Self::Output {
        &self.inner[index.idx]
    }
}

impl<'id, T> Copy for ArenaId<'id, T> {}
impl<'id, T> Clone for ArenaId<'id, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'id, T> PartialEq<ArenaId<'id, T>> for ArenaId<'id, T> {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}
