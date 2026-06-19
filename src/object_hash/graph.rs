use petgraph::visit::{GraphBase, IntoNeighbors, IntoNodeIdentifiers, NodeIndexable};

use crate::mir::{ExprArena, ExprId};

struct ArenaBackedGraph<'b> {
    arena: ExprArena<'b>,
}

impl<'b> GraphBase for ArenaBackedGraph<'b> {
    type NodeId = ExprId<'b>;
    type EdgeId = ();
}

impl<'id> IntoNodeIdentifiers for &ArenaBackedGraph<'id> {
    type NodeIdentifiers = <Vec<ExprId<'id>> as IntoIterator>::IntoIter;

    // this can be implemented with dfs over the "root" node
    fn node_identifiers(self) -> Self::NodeIdentifiers {
        (0..self.arena.size())
            .map(|i| self.from_index(i))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<'id> IntoNeighbors for &ArenaBackedGraph<'id> {
    type Neighbors = Box<dyn Iterator<Item = ExprId<'id>> + 'id>;

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
