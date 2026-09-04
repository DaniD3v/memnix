use std::collections::BTreeMap;

use crate::{
    arena::LazyArenaId,
    mir::{Ident, error::MirResolveError, ident_resolver::Resolver, lang::LazyExprArena},
};

pub struct RootResolver;
impl<'id> Resolver<'id> for RootResolver {
    fn resolve_ident(
        &self,
        ident: &Ident,
        _: &LazyExprArena,
    ) -> Result<LazyArenaId<'id>, MirResolveError> {
        Err(MirResolveError::IdentUnresolvable(ident.clone()))
    }

    fn get_param_nesting_level(&self) -> usize {
        0
    }
}

pub struct LazyMapResolver<'a, 'bump> {
    pub bindings: &'a BTreeMap<String, LazyArenaId<'bump>>,
    // Note: dyn is required as infinite resolver chains have to be possible
    pub parent: &'a dyn Resolver<'bump>,
}
impl<'a, 'b> Resolver<'b> for LazyMapResolver<'a, 'b> {
    fn resolve_ident(
        &self,
        ident: &Ident,
        arena: &LazyExprArena<'b>,
    ) -> Result<LazyArenaId<'b>, MirResolveError> {
        match self.bindings.get(ident.as_ref()) {
            Some(&found) => Ok(found),
            None => self.parent.resolve_ident(ident, arena),
        }
    }

    fn get_param_nesting_level(&self) -> usize {
        self.parent.get_param_nesting_level()
    }
}

pub struct LambdaParamResolver<'id, 'a> {
    ident: Ident,
    expr: LazyArenaId<'id>,

    // Note: dyn is required as infinite resolver chains have to be possible
    parent: &'a dyn Resolver<'id>,
    nesting_level: usize,
}
impl<'id, 'a> LambdaParamResolver<'id, 'a> {
    pub fn new(ident: Ident, expr: LazyArenaId<'id>, resolver: &'a dyn Resolver<'id>) -> Self {
        Self {
            ident,
            expr,

            parent: resolver,
            nesting_level: resolver.get_param_nesting_level() + 1,
        }
    }
}

impl<'id, 'a> Resolver<'id> for LambdaParamResolver<'id, 'a> {
    fn resolve_ident(
        &self,
        ident: &Ident,
        bump: &LazyExprArena<'id>,
    ) -> Result<LazyArenaId<'id>, MirResolveError> {
        if self.ident == *ident {
            Ok(self.expr)
        } else {
            self.parent.resolve_ident(ident, bump)
        }
    }

    fn get_param_nesting_level(&self) -> usize {
        self.nesting_level
    }
}
