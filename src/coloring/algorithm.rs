use petgraph::algo::tarjan_scc;

use crate::{
    ArenaId,
    coloring::{ArenaBackedGraph, Colorable, expr::ColoredExprArena},
};

pub fn color_graph<'id, 'a>(graph: &'a mut ArenaBackedGraph<'id>) {
    // this list is already in a `leaf -> root` ordering
    let scc_list = tarjan_scc(&*graph);

    for scc in scc_list {
        color_refinement(graph.arena_mut(), &scc);
    }
}

fn color_refinement<'id>(arena: &mut ColoredExprArena<'id>, scc: &[ArenaId<'id>]) {
    // we need to do `scc.len()` passes
    for _ in 0..scc.len() {
        let mut colors = Vec::with_capacity(scc.len());

        // compute all colors in this cycle
        for &node in scc {
            let color = arena[node].expr().compute_color(arena);
            colors.push(color);
        }

        // update the colors
        for (&node, color) in scc.iter().zip(colors) {
            *arena[node].color_mut() = Some(color);
        }
    }
}
