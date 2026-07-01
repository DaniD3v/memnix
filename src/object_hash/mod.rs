mod expr_type;
mod graph;

pub use expr_type::{OnceHashExpr, OnceHashRootExpr};

use std::collections::BTreeMap;

/// Uniquely identifies a nix object.
/// Objects sharing the same hash must be sementically equivalent
///
/// The algorithm behind this uses color refinement.
pub trait ColorHash {
    /// Implementation Detail:
    ///   The hashes of objects of 2 different types are never allowed to be equal.
    ///   This means the hash must include some sort of type id.
    fn object_hash(&self, hasher: &mut blake3::Hasher, colors: BTreeMap<usize, impl ColorHash>);
}
