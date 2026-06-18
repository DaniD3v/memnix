use std::fmt::{Debug, Formatter};

use rnix::Root;

use crate::{
    arena::DebugWith,
    mir::{
        ExprArena, ExprId, MirResolveError,
        ident_resolver::{Resolve, RootResolver},
        mir_expr_arena::DebugState,
    },
};

pub struct RootExpr<'id> {
    arena: ExprArena<'id>,
    root_node: ExprId<'id>,
}

impl<'id> RootExpr<'id> {
    pub fn new(root: Root) -> Result<RootExpr<'id>, MirResolveError> {
        let mut arena = ExprArena::new();

        let root_resolver = RootResolver::new(&mut arena);
        let root_node = root
            .expr()
            .expect("parsing errors")
            .resolve(&root_resolver, &mut arena)?;

        Ok(Self { arena, root_node })
    }

    // pub fn eval(&self) -> RuntimeValue {
    //     self.root_node
    //         .get_unwrap(&self.arena)
    //         .eval(&EvalState::new(&self.arena))
    // }
}

impl<'id> Debug for RootExpr<'id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_state = DebugState::new(&self.arena);
        self.root_node.fmt_with(&mut debug_state, f)
    }
}
