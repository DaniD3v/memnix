use petgraph::visit::{GraphBase, IntoNeighbors, IntoNodeIdentifiers, NodeIndexable};

use crate::{ArenaId, mir::LazyExprArena};

struct ArenaBackedGraph<'b> {
    arena: LazyExprArena<'b>,
}

// TODO: PartialEq for ArenaId is a bit goofy here, as this is a lazyArena
// Switch it to use the proper arena that's owned
impl<'b> GraphBase for ArenaBackedGraph<'b> {
    type NodeId = ArenaId<'b>;
    type EdgeId = ();
}

impl<'id> IntoNodeIdentifiers for &ArenaBackedGraph<'id> {
    type NodeIdentifiers = <Vec<ArenaId<'id>> as IntoIterator>::IntoIter;

    // this iterates over all indices, not all values!
    fn node_identifiers(self) -> Self::NodeIdentifiers {
        (0..self.arena.size())
            .map(|i| self.from_index(i))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<'id> IntoNeighbors for &ArenaBackedGraph<'id> {
    type Neighbors = Box<dyn Iterator<Item = ArenaId<'id>> + 'id>;

    fn neighbors(self, node: Self::NodeId) -> Self::Neighbors {
        self.arena[node].into_iter()
    }
}

impl<'b> NodeIndexable for ArenaBackedGraph<'b> {
    fn node_bound(&self) -> usize {
        self.arena.size()
    }

    fn to_index(&self, expr_id: Self::NodeId) -> usize {
        expr_id.idx()
    }

    fn from_index(&self, numeric_idx: usize) -> Self::NodeId {
        self.arena
            .get_index_from(numeric_idx)
            .expect("NodeIndexable: invalid index i provided")
    }
}
