use std::{fs, path::PathBuf};

use clap::Parser;
use memnix::{EnvSettings, Evaluator};

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

    let eval = Evaluator::default();
    eval.with_env(EnvSettings::default(), |mut env| {
        let result = env.eval_raw(&input_content);
        println!("result: {:?}", result.unwrap().kind())
    })
}
