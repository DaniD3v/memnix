use getset::Getters;
use petgraph::{
    Directed,
    visit::{
        Data, EdgeRef, GraphBase, GraphProp, IntoEdgeReferences, IntoNeighbors,
        IntoNodeIdentifiers, IntoNodeReferences, NodeIndexable,
    },
};

use crate::{ArenaId, object_hash::OnceHashRootExpr};

#[derive(Copy, Clone)]
pub struct FieldEdgeRef<'a, 'id> {
    pub source: ArenaId<'id>,
    pub target: ArenaId<'id>,
    pub field: &'a str,
}

impl<'a, 'id> EdgeRef for FieldEdgeRef<'a, 'id> {
    type NodeId = ArenaId<'id>;
    type EdgeId = (ArenaId<'id>, ArenaId<'id>);
    type Weight = ();

    fn source(&self) -> Self::NodeId {
        self.source
    }
    fn target(&self) -> Self::NodeId {
        self.target
    }
    fn weight(&self) -> &() {
        &()
    }
    fn id(&self) -> Self::EdgeId {
        (self.source, self.target)
    }
}

// TODO make this generic or sth
#[derive(Getters)]
pub struct ArenaBackedGraph<'b> {
    #[get = "pub"]
    root_node: OnceHashRootExpr<'b>,
}

impl<'id> ArenaBackedGraph<'id> {
    pub fn from_root_node(root_node: OnceHashRootExpr<'id>) -> Self {
        Self { root_node }
    }
}

// TODO: PartialEq for ArenaId is a bit goofy here, as this is a lazyArena
// Switch it to use the proper arena that's owned
impl<'b> GraphBase for ArenaBackedGraph<'b> {
    type NodeId = ArenaId<'b>;
    type EdgeId = (ArenaId<'b>, ArenaId<'b>);
}

impl<'id> IntoNodeIdentifiers for &ArenaBackedGraph<'id> {
    type NodeIdentifiers = <Vec<ArenaId<'id>> as IntoIterator>::IntoIter;

    fn node_identifiers(self) -> Self::NodeIdentifiers {
        self.root_node
            .arena()
            .iter_indices()
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<'id> IntoNeighbors for &ArenaBackedGraph<'id> {
    type Neighbors = <Vec<ArenaId<'id>> as IntoIterator>::IntoIter;

    fn neighbors(self, node: Self::NodeId) -> Self::Neighbors {
        self.root_node.arena()[node]
            .expr()
            .children()
            .map(|(id, _)| id)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl NodeIndexable for ArenaBackedGraph<'_> {
    fn node_bound(&self) -> usize {
        self.root_node.arena().size()
    }

    fn to_index(&self, expr_id: Self::NodeId) -> usize {
        expr_id.idx()
    }

    fn from_index(&self, numeric_idx: usize) -> Self::NodeId {
        self.root_node
            .arena()
            .get_index_from(numeric_idx)
            .expect("NodeIndexable: invalid index i provided")
    }
}

impl<'id> Data for ArenaBackedGraph<'id> {
    type NodeWeight = ();
    type EdgeWeight = ();
}

impl<'id> IntoNodeReferences for &ArenaBackedGraph<'id> {
    type NodeRef = (Self::NodeId, ());
    type NodeReferences = Box<dyn Iterator<Item = Self::NodeRef> + 'id>;

    fn node_references(self) -> Self::NodeReferences {
        Box::new(self.node_identifiers().map(|id| (id, ())))
    }
}

impl<'id, 'a> IntoEdgeReferences for &'a ArenaBackedGraph<'id> {
    type EdgeRef = FieldEdgeRef<'a, 'id>;
    type EdgeReferences = Box<dyn Iterator<Item = Self::EdgeRef> + 'a>;

    fn edge_references(self) -> Self::EdgeReferences {
        Box::new(
            self.root_node
                .arena()
                .iter_indices()
                .flat_map(move |source| {
                    self.root_node.arena()[source]
                        .expr()
                        .children()
                        .map(move |(target, field)| FieldEdgeRef {
                            source,
                            target,
                            field,
                        })
                }),
        )
    }
}

impl GraphProp for ArenaBackedGraph<'_> {
    type EdgeType = Directed;
}
