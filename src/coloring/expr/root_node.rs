use std::fmt::{self, Debug, Formatter};

use getset::{CopyGetters, Getters, MutGetters};

use crate::{
    arena::{ArenaId, DebugState, DebugWith},
    coloring::{ColoredExpr, expr::ColoredExprArena},
    mir::RootExpr,
};

#[derive(Getters, MutGetters, CopyGetters)]
pub struct ColorableRootExpr<'id, 'a> {
    #[get = "pub"]
    #[get_mut = "pub"]
    arena: &'a mut ColoredExprArena<'id>,

    #[get_copy = "pub"]
    root_node_id: ArenaId<'id>,
}

impl<'id: 'b, 'a, 'b> ColorableRootExpr<'id, 'a> {
    pub fn from_mir_root(arena: &'a mut ColoredExprArena<'id>, mir_root: RootExpr<'b>) -> Self {
        let (og_arena, root_node) = mir_root.into_parts();
        let root_node_id = arena.extend_map(og_arena, root_node, |expr, map| {
            ColoredExpr::from_mir(expr, map)
        });

        Self {
            arena,
            root_node_id,
        }
    }

    pub fn root_node(&self) -> &ColoredExpr<'id> {
        &self.arena[self.root_node_id]
    }
}

impl Debug for ColorableRootExpr<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let debug_state = DebugState::new(self.arena);
        self.root_node_id.fmt_with(&debug_state, f)
    }
}
