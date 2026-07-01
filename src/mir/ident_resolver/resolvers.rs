use std::collections::BTreeMap;

use crate::{
    ArenaId,
    mir::{
        Ident, LazyExprArena, WrappedIntrinsics, error::MirResolveError, ident_resolver::Resolver,
    },
};

pub struct RootResolver<'bump>(WrappedIntrinsics<'bump>);
impl<'b> RootResolver<'b> {
    pub fn new(bump: &mut LazyExprArena<'b>) -> Self {
        Self(WrappedIntrinsics::new(bump))
    }
}

impl<'b> Resolver<'b> for RootResolver<'b> {
    fn resolve_ident(
        &self,
        ident: &Ident,
        _: &LazyExprArena,
    ) -> Result<ArenaId<'b>, MirResolveError> {
        Err(MirResolveError::IdentUnresolvable(ident.clone()))
    }

    fn get_param_nesting_depth(&self) -> usize {
        0
    }
    fn get_builtins(&self) -> &WrappedIntrinsics<'b> {
        &self.0
    }
}

pub struct LazyMapResolver<'a, 'bump> {
    pub bindings: &'a BTreeMap<String, ArenaId<'bump>>,
    // Note: dyn is required as infinite resolver chains have to be possible
    pub parent: &'a dyn Resolver<'bump>,
}
impl<'a, 'b> Resolver<'b> for LazyMapResolver<'a, 'b> {
    fn resolve_ident(
        &self,
        ident: &Ident,
        arena: &LazyExprArena<'b>,
    ) -> Result<ArenaId<'b>, MirResolveError> {
        match self.bindings.get(ident.as_ref()) {
            Some(&found) => Ok(found),
            None => self.parent.resolve_ident(ident, arena),
        }
    }

    fn get_param_nesting_depth(&self) -> usize {
        self.parent.get_param_nesting_depth()
    }
    fn get_builtins(&self) -> &WrappedIntrinsics<'b> {
        self.parent.get_builtins()
    }
}

pub struct LambdaParamResolver<'a, 'bump> {
    pub ident: Ident,
    pub expr: ArenaId<'bump>,
    // Note: dyn is required as infinite resolver chains have to be possible
    pub parent: &'a dyn Resolver<'bump>,
}
impl<'a, 'b> Resolver<'b> for LambdaParamResolver<'a, 'b> {
    fn resolve_ident(
        &self,
        ident: &Ident,
        bump: &LazyExprArena<'b>,
    ) -> Result<ArenaId<'b>, MirResolveError> {
        if self.ident == *ident {
            Ok(self.expr)
        } else {
            self.parent.resolve_ident(ident, bump)
        }
    }

    fn get_param_nesting_depth(&self) -> usize {
        // TODO cache this
        self.parent.get_param_nesting_depth() + 1
    }
    fn get_builtins(&self) -> &WrappedIntrinsics<'b> {
        self.parent.get_builtins()
    }
}
