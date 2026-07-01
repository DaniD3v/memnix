#![allow(dead_code)]
//! This module wraps the primitive rnix-ast into a more high-level format

mod error;
mod ident_resolver;
mod intrinsic;
mod lang;
mod mir_expr_arena;
mod root_node;

pub use error::MirResolveError;
pub use intrinsic::{Intrinsic, WrappedIntrinsics};
pub use lang::{Expr, Ident, Lambda, LambdaCall, Literal, Param};
pub use root_node::RootExpr;

use ident_resolver::{LambdaParamResolver, LazyMapResolver, Resolve, Resolver};

use crate::arena::{LazyArena, LazyDebugState as GenericLazyDebugState};

pub type LazyExprArena<'id> = LazyArena<'id, Expr<'id>>;
pub type LazyDebugState<'id, 'a> = GenericLazyDebugState<'id, 'a, Expr<'id>>;
