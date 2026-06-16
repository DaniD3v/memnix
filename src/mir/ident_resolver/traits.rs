use bumpalo::Bump;

use crate::mir::{Expr, Ident, WrappedIntrinsics, error::MirResolveError};

/// Ast type that can be resolved to a Mir type
pub trait Resolve: Sized {
    type Target<'a>;
    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<Self::Target<'bump>, MirResolveError>;
}

pub trait Resolver<'bump> {
    fn resolve_ident(
        &self,
        ident: &Ident,
        bump: &'bump Bump,
    ) -> Result<&'bump Expr<'bump>, MirResolveError>;

    /// Returns how deeply nested the current lambda parameter is
    fn get_param_nesting_depth(&self) -> usize;
    fn get_builtins(&self) -> &WrappedIntrinsics<'bump>;
}

impl<'b, T: Resolver<'b>> Resolver<'b> for &T {
    fn resolve_ident(
        &self,
        ident: &Ident,
        bump: &'b Bump,
    ) -> Result<&'b Expr<'b>, MirResolveError> {
        (*self).resolve_ident(ident, bump)
    }
    fn get_param_nesting_depth(&self) -> usize {
        (*self).get_param_nesting_depth()
    }
    fn get_builtins(&self) -> &WrappedIntrinsics<'b> {
        (*self).get_builtins()
    }
}
