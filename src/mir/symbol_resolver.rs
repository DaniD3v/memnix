use std::collections::BTreeMap;

use bumpalo::Bump;
use rnix::ast;

use crate::mir::{
    Expr, error::MirResolveError, ident::Ident, intrinsics::Intrinsics, lazy_eval::LazyEval,
};

pub trait Resolver<'bump> {
    fn resolve_ident(
        &self,
        ident: Ident,
        bump: &'bump Bump,
    ) -> Result<&'bump Expr<'bump>, MirResolveError>;

    fn get_intrinsics(&self) -> &'bump Intrinsics<'bump>;
}

pub struct RootResolver<'bump>(&'bump Intrinsics<'bump>);
impl<'bump> RootResolver<'bump> {
    pub fn new(bump: &'bump Bump) -> Self {
        Self(bump.alloc(Intrinsics::new(bump)))
    }
}

impl<'b> Resolver<'b> for RootResolver<'b> {
    fn resolve_ident(&self, ident: Ident, _: &'b Bump) -> Result<&'b Expr<'b>, MirResolveError> {
        Err(MirResolveError::IdentUnresolvable(ident))
    }

    fn get_intrinsics(&self) -> &'b Intrinsics<'b> {
        self.0
    }
}

pub struct LazyMapResolver<'a, 'bump> {
    pub bindings: &'a BTreeMap<String, LazyEval<'bump, ast::Expr>>,
    // Note: dyn is required as infinite resolver chains have to be possible
    pub parent: &'a dyn Resolver<'bump>,
}
impl<'a, 'bump> Resolver<'bump> for LazyMapResolver<'a, 'bump> {
    fn resolve_ident(
        &self,
        ident: Ident,
        bump: &'bump Bump,
    ) -> Result<&'bump Expr<'bump>, MirResolveError> {
        self.bindings
            .get(ident.as_ref())
            .map(|lazy| lazy.resolve(self, bump))
            .ok_or(MirResolveError::IdentUnresolvable(ident))?
    }

    fn get_intrinsics(&self) -> &'bump Intrinsics<'bump> {
        self.parent.get_intrinsics()
    }
}

pub struct SingleIdentResolver<'a, 'bump> {
    pub ident: Ident,
    pub expr: &'bump Expr<'bump>,
    // Note: dyn is required as infinite resolver chains have to be possible
    pub parent: &'a dyn Resolver<'bump>,
}
impl<'a, 'bump> Resolver<'bump> for SingleIdentResolver<'a, 'bump> {
    fn resolve_ident(
        &self,
        ident: Ident,
        _: &'bump Bump,
    ) -> Result<&'bump Expr<'bump>, MirResolveError> {
        if self.ident == ident {
            Ok(self.expr)
        } else {
            Err(MirResolveError::IdentUnresolvable(ident))
        }
    }

    fn get_intrinsics(&self) -> &'bump Intrinsics<'bump> {
        self.parent.get_intrinsics()
    }
}
