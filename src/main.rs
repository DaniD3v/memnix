use std::{fs, path::PathBuf};

use clap::Parser;

use crate::ast_wrapper::from_root_node;

mod ast_wrapper;
mod object_hash;

#[derive(Parser, Debug)]
pub struct Args {
    /// Input file
    #[arg(short, long)]
    pub input_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let input_content = fs::read_to_string(&args.input_file)
        .unwrap_or_else(|_| panic!("failed to read {:#?}", &args.input_file));

    let root = rnix::Root::parse(&input_content).tree();
    let expr = from_root_node(root);

    println!("{:?}", expr);
}
