use std::{fs, path::PathBuf};

use clap::Parser;

use crate::{
    coloring::{ArenaBackedGraph, AsDot, ColorableRootExpr, color_graph},
    mir::RootExpr,
};

pub mod arena;
// mod eval; // TODO
pub mod coloring;
pub mod generic_lang;
pub mod mir;

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

    let mut colored_graph =
        ArenaBackedGraph::from_root_node(ColorableRootExpr::from_mir_root(mir_expr));
    color_graph(&mut colored_graph);

    let _ = fs::write("out.dot", format!("{:?}", AsDot(&colored_graph)));
    println!("Hashed: {:#?}", colored_graph.root_node());

    // TODO
    // let eval = expr.eval().eval_thunk();
    // println!("Eval: {:#?}", eval)
}
