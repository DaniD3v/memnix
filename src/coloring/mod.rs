mod algorithm;
mod colorable_impl;
mod expr;
mod graph;

use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter},
};

pub use algorithm::color_graph;
pub use expr::{ColorableRootExpr, ColoredExpr};
pub use graph::{ArenaBackedGraph, AsDot};

use crate::Arena;

/// Uniquely identifies a nix object.
///
/// Objects sharing the same color must be semantically equivalent.
/// They must thus include the hashes of all their dependencies.
pub trait Colorable<'id>: Sized {
    /// Depend on this object's color.
    ///
    /// Implementation Detail:
    ///   The colors of objects of 2 different types must never be equal.
    ///   This means the color must include some sort of type id.
    fn depend_on(self, hasher: &mut blake3::Hasher, arena: &Arena<'id, ColoredExpr>);

    fn compute_color(self, arena: &Arena<'id, ColoredExpr>) -> Color {
        let mut hasher = blake3::Hasher::new();
        self.depend_on(&mut hasher, arena);

        Color(hasher.finalize())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color(pub blake3::Hash);

impl Color {
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }
}

impl Ord for Color {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}
impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color({})", self.0)
    }
}
