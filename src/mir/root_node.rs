use std::fmt::{Debug, Formatter};

use getset::Getters;
use rnix::Root;

use crate::{
    ArenaId,
    arena::{DebugState, DebugWith, LazyArena},
    generic_lang::WithExprType,
    mir::{
        Intrinsic, MirExpr, MirResolveError,
        expr::ExprArena,
        ident_resolver::{Resolve, RootResolver},
    },
};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct RootExpr<'id> {
    arena: ExprArena<'id>,
    root_node: ArenaId<'id>,
}

impl<'id> RootExpr<'id> {
    pub fn new<'a>(root: Root) -> Result<RootExpr<'a>, MirResolveError> {
        let mut arena = LazyArena::new();

        let root_resolver = RootResolver::new(&mut arena);
        let root_node = root
            .expr()
            .expect("parsing errors")
            .resolve(&root_resolver, &mut arena)?;

        let (arena, root_node) = arena.flatten(
            root_node,
            MirExpr::Intrinsic(Intrinsic::RefCycleError),
            |expr, map| expr.with_expr(&map),
        );

        Ok(RootExpr { arena, root_node })
    }

    // pub fn eval(&self) -> RuntimeValue {
    //     self.root_node
    //         .get_unwrap(&self.arena)
    //         .eval(&EvalState::new(&self.arena))
    // }

    pub fn into_parts(self) -> (ExprArena<'id>, ArenaId<'id>) {
        (self.arena, self.root_node)
    }
}

impl<'id> Debug for RootExpr<'id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_state = DebugState::new(&self.arena);
        self.root_node.fmt_with(&debug_state, f)
    }
}
