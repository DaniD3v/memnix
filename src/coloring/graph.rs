use core::fmt;

use petgraph::{
    Directed,
    dot::{Config, Dot},
    visit::{
        Data, EdgeRef, GraphBase, GraphProp, IntoEdgeReferences, IntoNeighbors,
        IntoNodeIdentifiers, IntoNodeReferences, NodeIndexable,
    },
};

use crate::{arena::ArenaId, coloring::ColorableRootExpr, mir::MirExpr};

impl<'b> GraphBase for ColorableRootExpr<'b, '_> {
    type NodeId = ArenaId<'b>;

    // The `u32` slot disambiguates parallel edges.
    // e.g. `LambdaCall(f, f)` produces two edges with the same source/target.
    //
    // The field name would suffice semantically, but its `&str`
    // lifetime is tied to the graph borrow and can't appear in `EdgeId`.
    type EdgeId = (ArenaId<'b>, ArenaId<'b>, u32);
}

impl<'id> Data for ColorableRootExpr<'id, '_> {
    type NodeWeight = ();
    type EdgeWeight = ();
}

impl<'id> IntoNodeIdentifiers for &ColorableRootExpr<'id, '_> {
    type NodeIdentifiers = <Vec<ArenaId<'id>> as IntoIterator>::IntoIter;

    fn node_identifiers(self) -> Self::NodeIdentifiers {
        self.arena().iter_indices().collect::<Vec<_>>().into_iter()
    }
}

impl<'id> IntoNeighbors for &ColorableRootExpr<'id, '_> {
    type Neighbors = <Vec<ArenaId<'id>> as IntoIterator>::IntoIter;

    fn neighbors(self, node: Self::NodeId) -> Self::Neighbors {
        self.arena()[node]
            .expr()
            .edges()
            .copied()
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl NodeIndexable for ColorableRootExpr<'_, '_> {
    fn node_bound(&self) -> usize {
        self.arena().size()
    }

    fn to_index(&self, expr_id: Self::NodeId) -> usize {
        expr_id.idx()
    }

    fn from_index(&self, numeric_idx: usize) -> Self::NodeId {
        self.arena()
            .get_index_from(numeric_idx)
            .expect("NodeIndexable: invalid index i provided")
    }
}

impl<'id> IntoNodeReferences for &ColorableRootExpr<'id, '_> {
    type NodeRef = (Self::NodeId, ());
    type NodeReferences = Box<dyn Iterator<Item = Self::NodeRef> + 'id>;

    fn node_references(self) -> Self::NodeReferences {
        Box::new(self.node_identifiers().map(|id| (id, ())))
    }
}

impl<'id, 'a> IntoEdgeReferences for &'a ColorableRootExpr<'id, '_> {
    type EdgeRef = FieldEdgeRef<'a, 'id>;
    type EdgeReferences = Box<dyn Iterator<Item = Self::EdgeRef> + 'a>;

    fn edge_references(self) -> Self::EdgeReferences {
        Box::new(self.arena().iter_indices().flat_map(move |source| {
            self.arena()[source].expr().edges_labeled().enumerate().map(
                move |(slot, (target, field))| FieldEdgeRef {
                    source,
                    target: *target,
                    slot: slot as u32,
                    field,
                },
            )
        }))
    }
}

impl GraphProp for ColorableRootExpr<'_, '_> {
    type EdgeType = Directed;
}

#[allow(dead_code)] // TODO
pub struct AsDot<'a, 'b, 'id>(pub &'a ColorableRootExpr<'id, 'b>);

impl fmt::Debug for AsDot<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dot = Dot::with_attr_getters(
            self.0,
            &[Config::EdgeNoLabel, Config::NodeNoLabel],
            &|_, edge_ref| format!("label = {:?}", edge_ref.field),
            &|graph, (idx, _)| {
                let inner_expr = match graph.arena()[idx].expr() {
                    MirExpr::Literal(inner) => &format!("{:?}", inner),
                    MirExpr::Param(inner) => &format!("{:?}", inner),
                    MirExpr::Intrinsic(inner) => &format!("{:?}", inner),
                    MirExpr::Lambda(_) => "Lambda",
                };
                let inner_expr = format!("{}: {}", idx.idx(), inner_expr);

                format!("label=\"{inner_expr}\"",)
            },
        );
        write!(f, "{:?}", dot)
    }
}

#[derive(Copy, Clone)]
pub struct FieldEdgeRef<'a, 'id> {
    pub source: ArenaId<'id>,
    pub target: ArenaId<'id>,
    pub field: &'a str,
    pub slot: u32,
}

impl<'a, 'id> EdgeRef for FieldEdgeRef<'a, 'id> {
    type NodeId = ArenaId<'id>;
    type EdgeId = (ArenaId<'id>, ArenaId<'id>, u32);
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
        (self.source, self.target, self.slot)
    }
}
