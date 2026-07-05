mod color_impl;
mod expr_type;
mod graph;

use std::collections::BTreeMap;

pub use expr_type::{Color, OnceHashExpr, OnceHashRootExpr};
pub use graph::{ArenaBackedGraph, AsDot};

use petgraph::{
    Direction,
    algo::{is_cyclic_directed, tarjan_scc},
    graph::DiGraph,
    visit::{Dfs, IntoNeighbors, NodeIndexable},
};

use crate::{Arena, ArenaId};

pub fn hash_graph<'id, 'a>(graph: &'a mut ArenaBackedGraph<'id>) {
    let scc_list = tarjan_scc(&*graph);
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

            // no looping edges
            if idx != other_idx {
                dag.update_edge(idx, other_idx, ());
            }
        }
    });

    assert!(!is_cyclic_directed(&dag));

    // TODO: this may or may not require a little more explaining
    // color the whole dfs walk of each root
    for root_scc in dag.externals(Direction::Incoming) {
        let mut dfs = Dfs::new(&dag, root_scc);
        while let Some(scc) = dfs.next(&dag) {
            color_refinement(graph.arena_mut(), dag[scc]);
        }
    }
}

fn color_refinement<'id>(arena: &mut Arena<'id, OnceHashExpr<'id>>, scc: &[ArenaId<'id>]) {
    let mut previous_distinct_colors = 0;

    loop {
        // Colors is a map of `Color` -> `Vec<ArenaId>`
        //
        // This way `colors.len` automatically tracks the number of colors
        // and all nodes can easily be updated with their new color
        let mut colors = BTreeMap::new();

        // compute all colors in this cycle
        for &node in scc {
            let color = arena[node].expr().compute_color(arena);
            colors.entry(color).or_insert(Vec::new()).push(node);
        }

        // update the colors
        let distinct_colors = colors.len();
        for (color, nodes) in colors {
            for node in nodes {
                *arena[node].color_mut() = Some(color);
            }
        }

        // check whether coloring is stable
        if distinct_colors == previous_distinct_colors {
            break;
        } else {
            assert!(
                previous_distinct_colors < distinct_colors,
                "the amount of colors must never decrease"
            );
            previous_distinct_colors = distinct_colors;
        }
    }
}

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
    fn depend_on(self, hasher: &mut blake3::Hasher, arena: &Arena<'id, OnceHashExpr>);

    fn compute_color(self, arena: &Arena<'id, OnceHashExpr>) -> Color {
        let mut hasher = blake3::Hasher::new();
        self.depend_on(&mut hasher, arena);

        Color(hasher.finalize())
    }
}
