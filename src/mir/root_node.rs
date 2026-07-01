use std::fmt::{Debug, Formatter};

use getset::Getters;
use rnix::Root;

use crate::{
    ArenaId,
    arena::DebugWith,
    mir::{
        LazyDebugState, LazyExprArena, MirResolveError,
        ident_resolver::{Resolve, RootResolver},
    },
};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct RootExpr<'id> {
    arena: LazyExprArena<'id>,
    root_node: ArenaId<'id>,
}

impl<'id> RootExpr<'id> {
    pub fn new(root: Root) -> Result<RootExpr<'id>, MirResolveError> {
        let mut arena = LazyExprArena::new();

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

    pub fn into_parts(self) -> (LazyExprArena<'id>, ArenaId<'id>) {
        (self.arena, self.root_node)
    }
}

impl<'id> Debug for RootExpr<'id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_state = LazyDebugState::new(&self.arena);
        self.root_node.fmt_with(&mut debug_state, f)
    }
}
