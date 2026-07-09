//! This module extends the Arena
//!
//! It allows storing `Deferred` values or `References`
//! to other expressions directly in the arena.

mod debug;

pub use debug::LazyDebugState;

use std::{
    collections::BTreeSet,
    ops::{Deref, DerefMut, Index},
};

use crate::{Arena, ArenaId};

/// An extended write-only Arena that allows `Deferred` or `Reference` values.
///
/// It simplifies building lazy / cyclic structures and
/// can be normalized to an `Arena` with the `flatten` method.
#[derive(Default)]
pub struct LazyArena<'id, T>(Arena<'id, MaybeOrRef<'id, T>>);

// new-type because `LazyArenaId` must not implement `PartialEq`
#[derive(Copy, Clone)]
pub struct LazyArenaId<'id>(ArenaId<'id>);

pub enum MaybeOrRef<'id, S> {
    Some(S),
    Ref(LazyArenaId<'id>),

    /// A deferred item should be filled as quickly as possible.
    /// Certain arena operations may panic when performed on a deferred item.
    Deferred,
}

impl<'id, T> LazyArena<'id, T> {
    pub fn new() -> Self {
        Self(Arena::new())
    }

    pub fn alloc(&mut self, t: T) -> LazyArenaId<'id> {
        LazyArenaId(self.deref_mut().alloc(MaybeOrRef::Some(t)))
    }

    pub fn alloc_deferred(&mut self) -> LazyArenaId<'id> {
        LazyArenaId(self.deref_mut().alloc(MaybeOrRef::Deferred))
    }

    pub fn fill_deferred(&mut self, idx: LazyArenaId<'id>, reference: LazyArenaId<'id>) {
        let prev = std::mem::replace(&mut self.0[idx.0], MaybeOrRef::Ref(reference));
        assert!(matches!(prev, MaybeOrRef::Deferred));
    }

    /// Flattens a lazy arena into a normal arena, removing `Ref` indirections.
    ///
    /// Panics if there are any `Deferred` values left.
    ///
    /// `root`: The previous root node.
    /// The returned `ArenaId` will be the `O` equivalent of the previous root node.
    ///
    /// `cycle_placeholder`: A value used in-place of `Ref` cycles.
    /// It is only allocated if a cycle actually exists.
    ///
    /// `transform_idx`: Should transform all of T's internal `LazyArenaId` references
    /// to `ArenaId` references using the provided mapping closure.
    pub fn flatten<O>(
        mut self,
        root_node: LazyArenaId<'id>,
        cycle_placeholder: O,
        transform_idx: impl Fn(T, &dyn Fn(LazyArenaId<'id>) -> ArenaId<'id>) -> O,
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
                    panic!("LazyArena: attempted to flatten with unresolved deferred values")
                }
            })
            .for_each(|(idx, _)| idx_mapping[idx] = Some(new_arena.alloc(None)));

        let mut cycle_placeholder = Some(cycle_placeholder);
        let mut cycle_placeholder_id = None;

        // Update the `idx_mapping` for `Ref` entries
        //
        // We need to do this so that internal references inside T can get resolved properly
        // This needs to be a separate step so that resolve doesn't need to capture `&self`
        for idx in self.0.iter_indices() {
            let MaybeOrRef::Ref(ref_idx) = self.0[idx] else {
                continue;
            };

            idx_mapping[idx.idx()] = Some(match self.flatten_ref(ref_idx, &mut BTreeSet::new()) {
                Some(target) => idx_mapping[target.0.idx()]
                    .expect("flattened ref target should be mapped in pass 1"),

                // All cycles share a single lazily allocated placeholder node.
                None => *cycle_placeholder_id.get_or_insert_with(|| {
                    new_arena.alloc(Some(
                        cycle_placeholder
                            .take()
                            .expect("placeholder is only consumed once"),
                    ))
                }),
            });
        }

        let resolve = |id: LazyArenaId<'id>| idx_mapping[id.0.idx()].unwrap();
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

        let new_root = idx_mapping[root_node.0.idx()].unwrap();
        (new_arena.map(|val| val.unwrap()), new_root)
    }

    /// Flattens a reference so that it points to a non-ref value.
    ///
    /// Returns `None` for `Ref` cycles.
    fn flatten_ref(
        &mut self,
        idx: LazyArenaId<'id>,
        visited: &mut BTreeSet<usize>,
    ) -> Option<LazyArenaId<'id>> {
        // ref cycles must be handled gracefully
        if !visited.insert(idx.0.idx()) {
            return None;
        }

        match &self.0[idx.0] {
            MaybeOrRef::Ref(new_idx) => {
                let result = self.flatten_ref(*new_idx, visited)?;
                self.0[idx.0] = MaybeOrRef::Ref(result);

                Some(result)
            }
            _ => Some(idx),
        }
    }
}

impl<'id, T> Index<ArenaId<'id>> for LazyArena<'id, T> {
    type Output = T;

    fn index(&self, index: ArenaId<'id>) -> &Self::Output {
        match &self.0[index] {
            MaybeOrRef::Some(val) => val,
            MaybeOrRef::Ref(idx) => &self[idx.0],
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
