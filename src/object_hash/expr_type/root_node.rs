use std::fmt::{self, Debug, Formatter};

use getset::{CopyGetters, Getters, MutGetters};

use crate::{
    Arena,
    arena::{DebugState, DebugWith},
    generic_lang::WithExprType,
    mir::RootExpr,
    object_hash::{OnceHashExpr, expr_type::OnceHashExprId},
};

#[derive(Getters, MutGetters, CopyGetters)]
pub struct OnceHashRootExpr<'id> {
    #[get = "pub"]
    #[get_mut = "pub"]
    arena: Arena<'id, OnceHashExpr<'id>>,

    #[get_copy = "pub"]
    root_node: OnceHashExprId<'id>,
}

impl<'id> OnceHashRootExpr<'id> {
    pub fn from_mir_root(mir_root: RootExpr<'id>) -> Self {
        let (arena, root_node) = mir_root.into_parts();
        let (hash_arena, root_node) = arena.flatten(root_node, |expr, map| expr.with_expr(&map));

        Self {
            arena: hash_arena,
            root_node,
        }
    }
}

impl Debug for OnceHashRootExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let debug_state = DebugState::new(&self.arena);
        self.root_node.fmt_with(&debug_state, f)
    }
}
