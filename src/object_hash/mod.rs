mod expr_type;
mod graph;

pub use expr_type::{OnceHashExpr, OnceHashRootExpr};
pub use graph::ArenaBackedGraph;

use petgraph::visit::{IntoNeighbors, NodeIndexable};
use petgraph::{algo::tarjan_scc, graph::DiGraph};
use std::collections::BTreeMap;

fn hash_graph(graph: ArenaBackedGraph) -> ArenaBackedGraph {
    let scc_list = tarjan_scc(&graph);
    let mut node_to_scc = vec![usize::MAX; graph.node_bound()];

    // fill the node_to_scc lookup
    scc_list.iter().enumerate().for_each(|(scc_idx, scc)| {
        scc.iter()
            .for_each(|node| node_to_scc[graph.to_index(*node)] = scc_idx);
    });

    let mut dag = DiGraph::new();

    for scc in &scc_list {
        dag.add_node(scc);
    }

    scc_list.iter().enumerate().for_each(|(idx, scc)| {
        let idx = dag.from_index(idx);

        for neighbor_node in scc.iter().flat_map(|node| graph.neighbors(*node)) {
            let other_idx = dag.from_index(node_to_scc[graph.to_index(neighbor_node)]);
            dag.update_edge(idx, other_idx, ());
        }
    });

    graph
}

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
