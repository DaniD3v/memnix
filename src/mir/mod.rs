#![allow(dead_code)]
//! This module wraps the primitive rnix-ast into a more high-level format

mod error;
mod ident;
mod lazy_eval;
mod symbol_resolver;

mod bin_expr;
mod expression;
mod if_else;
mod intrinsics;
mod lambda;
mod lambda_call;
mod let_in;
mod literal;

pub use expression::Expr;
pub use ident::Ident;
pub use lambda::Lambda;
pub use lambda_call::LambdaCall;
pub use let_in::LetIn;
pub use literal::Literal;

use bumpalo::Bump;
use rnix::Root;

use crate::mir::{error::MirResolveError, lazy_eval::Resolve, symbol_resolver::RootResolver};

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
