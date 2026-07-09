use std::fmt::{self, Formatter};

use super::{LazyArena, LazyArenaId, MaybeOrRef};
use crate::{
    ArenaId,
    arena::{
        DebugWith,
        debug::{DebugArena, GenericDebugState},
    },
};

pub type LazyDebugState<'id, 'a, T> = GenericDebugState<'id, 'a, LazyArena<'id, T>>;

impl<'id, T> DebugArena<'id> for LazyArena<'id, T> {
    type Item = T;

    fn canonical_idx(&self, id: ArenaId<'id>) -> usize {
        match &self.0[id] {
            MaybeOrRef::Ref(idx) => idx.0.idx(),
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

impl<'id, 'a, A: DebugArena<'id>> DebugWith<GenericDebugState<'id, 'a, A>> for LazyArenaId<'id>
where
    A::Item: DebugWith<GenericDebugState<'id, 'a, A>>,
{
    fn fmt_with(&self, with: &GenericDebugState<'id, 'a, A>, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt_with(with, f)
    }
}
