use std::{fs, path::PathBuf};

use clap::Parser;

use crate::{
    mir::RootExpr,
    object_hash::{ArenaBackedGraph, AsDot, OnceHashRootExpr, hash_graph},
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

    let mut hashed_graph =
        ArenaBackedGraph::from_root_node(OnceHashRootExpr::from_mir_root(mir_expr));
    hash_graph(&mut hashed_graph);

    let _ = fs::write("out.dot", format!("{:?}", AsDot(&hashed_graph)));
    println!("Hashed: {:#?}", hashed_graph.root_node());

    // TODO
    // let eval = expr.eval().eval_thunk();
    // println!("Eval: {:#?}", eval)
}
