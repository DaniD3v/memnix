use std::collections::BTreeMap;

use bumpalo::Bump;

use crate::mir::{Expr, Ident, error::MirResolveError, ident_resolver::Resolver};

pub struct RootResolver;
impl<'b> Resolver<'b> for RootResolver {
    fn resolve_ident(&self, ident: &Ident, _: &'b Bump) -> Result<&'b Expr<'b>, MirResolveError> {
        Err(MirResolveError::IdentUnresolvable(ident.clone()))
    }

    fn get_param_nesting_depth(&self) -> usize {
        0
    }
}

pub struct LazyMapResolver<'a, 'bump> {
    pub bindings: &'a BTreeMap<String, &'bump Expr<'bump>>,
    // Note: dyn is required as infinite resolver chains have to be possible
    pub parent: &'a dyn Resolver<'bump>,
}
impl<'a, 'bump> Resolver<'bump> for LazyMapResolver<'a, 'bump> {
    fn resolve_ident(
        &self,
        ident: &Ident,
        bump: &'bump Bump,
    ) -> Result<&'bump Expr<'bump>, MirResolveError> {
        match self.bindings.get(ident.as_ref()) {
            Some(&found) => Ok(match found {
                Expr::Deferred(cell) => cell.get().copied().unwrap_or(found),
                _ => found,
            }),
            None => self.parent.resolve_ident(ident, bump),
        }
    }

    fn get_param_nesting_depth(&self) -> usize {
        self.parent.get_param_nesting_depth()
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
}
