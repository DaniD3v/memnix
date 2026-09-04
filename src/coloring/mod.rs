mod algorithm;
mod expr;
mod graph;
mod tests;

use blake3::Hasher;
use serde::{Deserialize, Serialize};

use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter},
};

pub use algorithm::color_graph;
pub use expr::{ColorableRootExpr, ColoredExpr, ColoredExprArena};

use crate::{arena::Arena, mir::MirExpr};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Color(pub blake3::Hash);

impl<'id> MirExpr<'id> {
    pub fn color(self, arena: &Arena<'id, ColoredExpr<'_>>) -> Color {
        Color(
            postcard::to_io(&self.convert_inner(|id| arena[id].color()), Hasher::new())
                .unwrap()
                .finalize(),
        )
    }
}

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
