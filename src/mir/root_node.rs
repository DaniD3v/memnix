use std::fmt::{Debug, Formatter};

use getset::Getters;
use rnix::Root;

use crate::{
    arena::{ArenaId, DebugState, DebugWith, LazyArena},
    mir::{
        Literal, MirExpr, MirResolveError,
        expr::MirExprArena,
        ident_resolver::{Resolve, RootResolver},
    },
};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct RootExpr<'id> {
    arena: MirExprArena<'id>,
    root_node: ArenaId<'id>,
}

impl<'id> RootExpr<'id> {
    pub fn new(root: Root, guard: generativity::Guard<'id>) -> Result<Self, MirResolveError> {
        let mut arena = LazyArena::new(guard);

        let root_node = root
            .expr()
            .expect("parsing errors")
            .resolve(&RootResolver, &mut arena)?;

        let (arena, root_node) = arena.flatten_map(
            root_node,
            MirExpr::Literal(Literal::RefCycleError),
            |expr, map| expr.convert_inner(map),
        );

        Ok(RootExpr { arena, root_node })
    }

    pub fn into_parts(self) -> (MirExprArena<'id>, ArenaId<'id>) {
        (self.arena, self.root_node)
    }
}

impl<'id> Debug for RootExpr<'id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_state = DebugState::new(&self.arena);
        self.root_node.fmt_with(&debug_state, f)
    }
}
