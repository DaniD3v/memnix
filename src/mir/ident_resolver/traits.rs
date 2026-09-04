use crate::{
    arena::LazyArenaId,
    mir::{Ident, error::MirResolveError, lang::LazyExprArena},
};

/// Ast type that can be resolved to a Mir type
pub trait Resolve: Sized {
    type Target<'a>;
    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &mut LazyExprArena<'bump>,
    ) -> Result<Self::Target<'bump>, MirResolveError>;
}

pub trait Resolver<'bump> {
    fn resolve_ident(
        &self,
        ident: &Ident,
        bump: &LazyExprArena<'bump>,
    ) -> Result<LazyArenaId<'bump>, MirResolveError>;

    /// Returns how deeply nested the current lambda parameter is
    fn get_param_nesting_level(&self) -> usize;
}

impl<'b, T: Resolver<'b>> Resolver<'b> for &T {
    fn resolve_ident(
        &self,
        ident: &Ident,
        bump: &LazyExprArena<'b>,
    ) -> Result<LazyArenaId<'b>, MirResolveError> {
        (*self).resolve_ident(ident, bump)
    }
    fn get_param_nesting_level(&self) -> usize {
        (*self).get_param_nesting_level()
    }
}
