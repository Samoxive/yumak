use std::fs;

extern crate common;
extern crate engine;

use common::bytecode::Inst;
use common::{new_syncmut, SyncMut};
use engine::{ExecutionEngine};
use engine::context::ExecutionContext;

mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not input file was provided");
        std::process::exit(-1);
    }
    let unparsed_file = fs::read_to_string(&args[1]).expect("cannot read file");
    let file = parser::parse(&unparsed_file)
        .expect("unsuccessful parse"); // get and unwrap the `file` rule; never fails
    println!{"{:?}", file};
    let mut engine: ExecutionEngine = Default::default();
    let main_context: SyncMut<ExecutionContext> = ExecutionContext::from_instructions(file);
    engine.push_task(main_context);
    let _exec_run = ExecutionEngine::run(&new_syncmut(engine));
}