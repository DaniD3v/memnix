#![allow(dead_code)]
//! This module wraps the primitive rnix-ast into a more high-level format

mod error;
mod expr;
mod ident_resolver;
mod intrinsic;
mod lang;
mod root_node;

pub use error::MirResolveError;
pub use expr::{MirExpr, MirLambda, MirLambdaCall};
pub use intrinsic::{Intrinsic, WrappedIntrinsics};
pub use lang::{Ident, Literal, Param};
pub use root_node::RootExpr;
