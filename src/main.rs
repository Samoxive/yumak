extern crate common;
extern crate engine;
extern crate failure;

use common::SyncMut;
use common::bytecode::Inst;
use engine::{ExecutionContext, ExecutionEngine};
use std::sync::{Arc, Mutex};
use failure::Error;

fn main() -> Result<(), Error> {
    let mut engine: ExecutionEngine = ExecutionEngine::new();
    let main_context: SyncMut<ExecutionContext> =
        Arc::from(Mutex::new(ExecutionContext::from_instructions(vec![
            Inst::Alloc { name: "x".into() },
            Inst::PushInt { name: "x".into(), value: 32 },
            Inst::Call { name: "print".into(), arguments: vec!["x".into()].into(), this: None },
            Inst::Call { name: "exit".into(), arguments: vec!["x".into()].into(), this: None },
            Inst::Return { name: "x".into() }
        ])));
    engine.push_task(main_context);
    ExecutionEngine::run(Arc::new(Mutex::new(engine)))
}
