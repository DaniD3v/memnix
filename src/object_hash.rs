use std::hash::Hasher;

/// Uniquely identifies a nix object.
/// Objects sharing the same hash must be sementically equivalent
pub trait ObjectHash {
    /// Implementation Detail:
    ///   The hashes of objects of 2 different types are never allowed to be equal.
    ///   This means the hash must include some sort of type id.
    fn object_hash<H: Hasher>(&self, state: &mut H);
}
