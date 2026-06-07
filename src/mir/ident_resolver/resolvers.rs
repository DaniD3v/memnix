use std::collections::BTreeMap;

use bumpalo::Bump;
use rnix::ast;

use crate::mir::{Expr, Ident, error::MirResolveError, lang::Intrinsics};

use super::{LazyEval, Resolver};

pub struct RootResolver<'bump>(&'bump Intrinsics<'bump>);
impl<'bump> RootResolver<'bump> {
    pub fn new(bump: &'bump Bump) -> Self {
        Self(bump.alloc(Intrinsics::new(bump)))
    }
}

impl<'b> Resolver<'b> for RootResolver<'b> {
    fn resolve_ident(&self, ident: &Ident, _: &'b Bump) -> Result<&'b Expr<'b>, MirResolveError> {
        Err(MirResolveError::IdentUnresolvable(ident.clone()))
    }

    fn get_param_nesting_depth(&self) -> usize {
        0
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
        ident: &Ident,
        bump: &'bump Bump,
    ) -> Result<&'bump Expr<'bump>, MirResolveError> {
        match self
            .bindings
            .get(ident.as_ref())
            .map(|lazy| lazy.resolve(self, bump))
            .transpose()?
        {
            Some(found) => Ok(found),
            None => self.parent.resolve_ident(ident, bump),
        }
    }

    fn get_param_nesting_depth(&self) -> usize {
        self.parent.get_param_nesting_depth()
    }
    fn get_intrinsics(&self) -> &'bump Intrinsics<'bump> {
        self.parent.get_intrinsics()
    }
}

pub struct LambdaParamResolver<'a, 'bump> {
    pub ident: Ident,
    pub expr: &'bump Expr<'bump>,
    // Note: dyn is required as infinite resolver chains have to be possible
    pub parent: &'a dyn Resolver<'bump>,
}
impl<'a, 'bump> Resolver<'bump> for LambdaParamResolver<'a, 'bump> {
    fn resolve_ident(
        &self,
        ident: &Ident,
        bump: &'bump Bump,
    ) -> Result<&'bump Expr<'bump>, MirResolveError> {
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
    fn get_intrinsics(&self) -> &'bump Intrinsics<'bump> {
        self.parent.get_intrinsics()
    }
}
