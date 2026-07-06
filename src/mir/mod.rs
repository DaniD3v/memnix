#![allow(dead_code)]
//! This module wraps the primitive rnix-ast into a more high-level format

mod error;
mod ident_resolver;
mod intrinsic;
mod lang;
mod root_node;

pub use error::MirResolveError;
pub use intrinsic::{Intrinsic, WrappedIntrinsics};
pub use lang::{Ident, Literal, MirExpr, MirLambda, MirLambdaCall, Param};
pub use root_node::RootExpr;

use ident_resolver::{LambdaParamResolver, LazyMapResolver, Resolve, Resolver};

use crate::arena::{LazyArena, LazyDebugState as GenericLazyDebugState};

pub type LazyExprArena<'id> = LazyArena<'id, MirExpr<'id>>;
pub type LazyDebugState<'id, 'a> = GenericLazyDebugState<'id, 'a, MirExpr<'id>>;
