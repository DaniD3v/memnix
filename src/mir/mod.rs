#![allow(dead_code)]
//! This module wraps the primitive rnix-ast into a more high-level format

mod error;
mod ident;
mod ident_resolver;
mod lang;

pub use ident::Ident;
pub use lang::{Expr, Lambda, LambdaCall, LetIn, Literal};

use bumpalo::Bump;
use rnix::Root;

use error::MirResolveError;
use ident_resolver::{
    LazyEval, LazyMapResolver, Resolve, Resolver, RootResolver, SingleIdentResolver,
};

pub fn from_root_node<'bump>(
    root: Root,
    bump: &'bump Bump,
) -> Result<&'bump Expr<'bump>, MirResolveError> {
    let root_resolver = RootResolver::new(bump);

    root.expr()
        .expect("parsing errors")
        .resolve(&root_resolver, bump)
}

#[derive(Debug)]
pub struct Param;
