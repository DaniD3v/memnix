use std::fmt::{self, Debug, Formatter};

use crate::{
    Arena,
    arena::{DebugState, DebugWith},
    generic_lang::WithExprType,
    mir::RootExpr,
    object_hash::{OnceHashExpr, expr_type::OnceHashExprId},
};

pub struct OnceHashRootExpr<'id> {
    arena: Arena<'id, OnceHashExpr<'id>>,
    root_node: OnceHashExprId<'id>,
}

impl<'id> OnceHashRootExpr<'id> {
    pub fn from_mir_root(mir_root: RootExpr<'id>) -> Self {
        let (arena, _) = mir_root.into_parts();

        let hash_arena = arena.flatten(|expr, map| expr.with_expr(&map));
        let root_node = hash_arena
            .get_index_from(hash_arena.size() - 1)
            .expect("`hash_arena` should contain at least one node");

        Self {
            arena: hash_arena,
            root_node,
        }
    }
}

impl Debug for OnceHashRootExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_state = DebugState::new(&self.arena);
        self.root_node.fmt_with(&mut debug_state, f)
    }
}
