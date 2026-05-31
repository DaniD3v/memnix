use std::hash::{Hash, Hasher};

use crate::ast_wrapper::Expr;

/// Uniquely identifies a nix object.
/// Objects sharing the same hash must be sementically equivalent
pub trait ObjectHash: Hash {
    fn object_hash<H: Hasher>(&self, state: &mut H) {
        // While this impl makes the code very simple
        // It has the problem of including spans.
        //
        // This means whole files might get invalidated
        // where they wouldn't need to.
        self.hash(state);
    }
}

impl ObjectHash for Expr {}
