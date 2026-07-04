use std::{fs, path::PathBuf};

use clap::Parser;
use petgraph::dot::{Config, Dot};

use crate::{
    arena::{DebugState, DebugWithWrapper},
    mir::RootExpr,
    object_hash::{ArenaBackedGraph, OnceHashRootExpr},
};

pub mod arena;
// mod eval; // TODO
pub mod generic_lang;
pub mod mir;
pub mod object_hash;

pub use arena::{Arena, ArenaId};

#[derive(Parser, Debug)]
struct Args {
    /// Input file
    #[arg(short, long)]
    pub input_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let input_content = fs::read_to_string(&args.input_file)
        .unwrap_or_else(|_| panic!("failed to read {:#?}", args.input_file));

    let root = rnix::Root::parse(&input_content).tree();
    println!("Ast: {:#?}", root);

    let mir_expr = RootExpr::new(root).unwrap();
    println!("Mir: {:#?}", mir_expr);

    let hashed = OnceHashRootExpr::from_mir_root(mir_expr);
    println!("Hashed: {:#?}", hashed);

    let hashed_graph = ArenaBackedGraph::from_root_node(hashed);

    let _ = fs::write(
        "out.dot",
        format!(
            "{:?}",
            Dot::with_attr_getters(
                &hashed_graph,
                &[Config::EdgeNoLabel, Config::NodeIndexLabel],
                &|_, _| "".to_owned(),
                &|_, (idx, _)| {
                    let debug_state = DebugState::new(hashed_graph.root_node().arena());
                    format!(
                        "tooltip=\"{:?}\"",
                        DebugWithWrapper::new(&idx, &debug_state)
                    )
                }
            )
        ),
    );

    // TODO
    // let eval = expr.eval().eval_thunk();
    // println!("Eval: {:#?}", eval)
}
