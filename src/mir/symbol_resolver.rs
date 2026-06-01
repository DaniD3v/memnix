use std::collections::BTreeMap;

use bumpalo::Bump;
use rnix::ast;

use crate::mir::{Expr, lazy_eval::LazyEval};

pub trait Resolver<'bump> {
    fn resolve(&self, name: String, bump: &'bump Bump) -> Option<&'bump Expr<'bump>>;
}

pub struct BTreeMapResolver<'a, 'bump>(pub &'a BTreeMap<String, LazyEval<'bump, ast::Expr>>);
impl<'a, 'bump> Resolver<'bump> for BTreeMapResolver<'a, 'bump> {
    fn resolve(&self, name: String, bump: &'bump Bump) -> Option<&'bump Expr<'bump>> {
        let lazy_eval = self.0.get(&name)?;
        Some(lazy_eval.resolve(self, bump))
    }
}

pub struct SingleNameResolver<'bump> {
    pub name: String,
    pub expr: &'bump Expr<'bump>,
}
impl<'bump> Resolver<'bump> for SingleNameResolver<'bump> {
    fn resolve(&self, name: String, _: &'bump Bump) -> Option<&'bump Expr<'bump>> {
        if self.name == name {
            Some(self.expr)
        } else {
            None
        }
    }
}

pub struct NullResolver;
impl<'bump> Resolver<'bump> for NullResolver {
    fn resolve(&self, _: String, _: &'bump Bump) -> Option<&'bump Expr<'bump>> {
        None
    }
}
