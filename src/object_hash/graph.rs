use petgraph::visit::{GraphBase, IntoNeighbors, IntoNodeIdentifiers, NodeIndexable};

use crate::mir::{ExprArena, ExprId};

struct ArenaBackedGraph<'b> {
    arena: ExprArena<'b>,
}

impl<'b> GraphBase for ArenaBackedGraph<'b> {
    type NodeId = ExprId<'b>;
    type EdgeId = ();
}

impl<'b> IntoNodeIdentifiers for &ArenaBackedGraph<'b> {
    type NodeIdentifiers = Box<dyn Iterator<Item = ExprId<'b>>>;

    // this can be implemented with dfs over the "root" node
    fn node_identifiers(self) -> Self::NodeIdentifiers {
        todo!()
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
