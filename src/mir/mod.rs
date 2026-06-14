#![allow(dead_code)]
//! This module wraps the primitive rnix-ast into a more high-level format

mod builtins;
mod error;
mod ident;
mod ident_resolver;
mod lambda;
mod lang;

pub use builtins::Intrinsic;
pub use ident::Ident;
pub use lambda::Lambda;
pub use lang::{Expr, LambdaCall, Literal, Param};

use bumpalo::Bump;
use rnix::Root;

use error::MirResolveError;
use ident_resolver::{LambdaParamResolver, LazyMapResolver, Resolve, Resolver, RootResolver};

pub fn from_root_node<'bump>(
    root: Root,
    bump: &'bump Bump,
) -> Result<&'bump Expr<'bump>, MirResolveError> {
    let root_resolver = RootResolver;

    root.expr()
        .expect("parsing errors")
        .resolve(&root_resolver, bump)
}
