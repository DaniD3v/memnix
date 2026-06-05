use bumpalo::Bump;

use crate::mir::{Expr, Ident, error::MirResolveError, intrinsics::Intrinsics};

pub trait Resolver<'bump> {
    fn resolve_ident(
        &self,
        ident: Ident,
        bump: &'bump Bump,
    ) -> Result<&'bump Expr<'bump>, MirResolveError>;

    fn get_intrinsics(&self) -> &'bump Intrinsics<'bump>;
}

/// Ast type that can be resolved to a Mir type
pub trait Resolve: Sized {
    type Target<'a>;
    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<Self::Target<'bump>, MirResolveError>;
}
