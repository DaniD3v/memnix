use std::fmt::{self, Debug, Formatter};

use getset::{CopyGetters, Getters, MutGetters};

use crate::{
    ArenaId,
    arena::{DebugState, DebugWith},
    coloring::{ColoredExpr, expr::ColoredExprArena},
    mir::RootExpr,
};

#[derive(Getters, MutGetters, CopyGetters)]
pub struct ColorableRootExpr<'id> {
    #[get = "pub"]
    #[get_mut = "pub"]
    arena: ColoredExprArena<'id>,

    #[get_copy = "pub"]
    root_node: ArenaId<'id>,
}

impl<'id> ColorableRootExpr<'id> {
    pub fn from_mir_root(mir_root: RootExpr<'id>) -> Self {
        let (arena, root_node) = mir_root.into_parts();
        let arena = arena.map(|expr| ColoredExpr { expr, color: None });

        Self { arena, root_node }
    }
}

impl Debug for ColorableRootExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let debug_state = DebugState::new(&self.arena);
        self.root_node.fmt_with(&debug_state, f)
    }
}
