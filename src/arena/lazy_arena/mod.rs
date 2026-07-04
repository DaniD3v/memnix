//! This module extends the Arena
//!
//! It allows storing `Deferred` values or `References`
//! to other expressions directly in the arena.

use std::ops::{Deref, DerefMut, Index};

use crate::{
    Arena, ArenaId,
    arena::{DebugState, debug::DebugArena},
};

pub type LazyDebugState<'id, 'a, T> = DebugState<'id, 'a, LazyArena<'id, T>>;

pub struct LazyArena<'id, T>(Arena<'id, MaybeOrRef<'id, T>>);

pub enum MaybeOrRef<'id, S> {
    Some(S),

    /// Invariant:
    ///
    /// The ref is not allowed to point to another ref variant.
    Ref(ArenaId<'id>),

    /// A deferred item should be filled as quickly as possible.
    /// Certain arena operations may panic when performed on a deferred item.
    Deferred,
}

// TODO: check for correctness errors because of partialeq based on idx
impl<'id, T> LazyArena<'id, T> {
    pub fn new() -> Self {
        Self(Arena::new())
    }

    pub fn alloc(&mut self, t: T) -> ArenaId<'id> {
        self.deref_mut().alloc(MaybeOrRef::Some(t))
    }

    pub fn alloc_deferred(&mut self) -> ArenaId<'id> {
        self.deref_mut().alloc(MaybeOrRef::Deferred)
    }

    pub fn replace_none_with_ref(&mut self, idx: ArenaId<'id>, reference: ArenaId<'id>) {
        let reference = self.flatten_ref(reference);

        let ret = std::mem::replace(&mut self.deref_mut()[idx], MaybeOrRef::Ref(reference));
        assert!(matches!(ret, MaybeOrRef::Deferred));
    }

    /// Flattens a lazy arena into a normal arena, removing `Ref` indirections.
    ///
    /// Panics if there are any `Deferred` values.
    ///
    /// `transform_idx`: Should transform all of T's internal `LazyArenaId` references
    /// to `ArenaId` references using the provided mapping closure.
    pub fn flatten<O>(
        self,
        root: ArenaId<'id>,
        // TODO: fix this lifetime. the lifetime of the returned object should differ
        transform_idx: impl Fn(T, &dyn Fn(ArenaId<'id>) -> ArenaId<'id>) -> O,
    ) -> (Arena<'id, O>, ArenaId<'id>) {
        let mut new_arena = Arena::new(); // TODO: with_capacity
        let mut idx_mapping = vec![None; self.size()];

        // Assign sequential new indices to non-`Ref` entries.
        self.0
            .iter()
            .enumerate()
            .filter(|(_, val)| match val {
                MaybeOrRef::Some(_) => true,
                MaybeOrRef::Ref(_) => false,

                MaybeOrRef::Deferred => {
                    panic!("LazyMap: attempted to flatten with unresolved deferred values")
                }
            })
            .for_each(|(idx, _)| idx_mapping[idx] = Some(new_arena.alloc(None)));

        // Update the `idx_mapping` for `Ref` entries
        //
        // We need to do this so that internal references inside T can get resolved properly
        // This needs to be a separate step so that resolve doesn't need to capture `&self`
        self.0
            .iter()
            .enumerate()
            .filter_map(|(idx, val)| match val {
                MaybeOrRef::Ref(val) => Some((idx, val)),
                _ => None,
            })
            .for_each(|(idx, val)| {
                idx_mapping[idx] = Some(
                    idx_mapping[val.idx()]
                        .expect("idx_mapping[reference] should already be `Some`"),
                )
            });

        let resolve = |id: ArenaId<'id>| idx_mapping[id.idx()].unwrap();
        self.0
            .into_iter()
            .enumerate()
            .filter_map(|(idx, val)| match val {
                MaybeOrRef::Some(val) => Some((idx, val)),
                _ => None,
            })
            .for_each(|(idx, val)| {
                new_arena[idx_mapping[idx].unwrap()] = Some(transform_idx(val, &resolve))
            });

        let new_root = idx_mapping[root.idx()].unwrap();
        (new_arena.map(|val| val.unwrap()), new_root)
    }

    fn flatten_ref(&self, mut idx: ArenaId<'id>) -> ArenaId<'id> {
        // TODO:
        // this doesn't have to be a loop as the reference
        // itself can also only be 1 reference deep
        while let MaybeOrRef::Ref(value) = self.0[idx] {
            idx = value;
        }
        idx
    }
}

impl<'id, T> Index<ArenaId<'id>> for LazyArena<'id, T> {
    type Output = T;

    fn index(&self, index: ArenaId<'id>) -> &Self::Output {
        match &self.0[index] {
            MaybeOrRef::Some(val) => val,
            MaybeOrRef::Ref(idx) => &self[*idx],
            MaybeOrRef::Deferred => {
                unreachable!("deferred expressions should already be resolved on first access")
            }
        }
    }
}

impl<'id, T> Deref for LazyArena<'id, T> {
    type Target = Arena<'id, MaybeOrRef<'id, T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'id, T> DerefMut for LazyArena<'id, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'id, T> DebugArena<'id> for LazyArena<'id, T> {
    type Item = T;

    fn canonical_idx(&self, id: ArenaId<'id>) -> usize {
        match &self.0[id] {
            MaybeOrRef::Ref(idx) => idx.idx(),
            _ => id.idx(),
        }
    }

    fn get(&self, id: ArenaId<'id>) -> &T {
        &self[id]
    }

    fn size(&self) -> usize {
        self.0.size()
    }
}
