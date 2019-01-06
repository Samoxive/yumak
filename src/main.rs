extern crate common;
extern crate engine;
extern crate failure;

use common::bytecode::Inst;
use common::{new_syncmut, SyncMut};
use engine::context::ExecutionContext;
use engine::ExecutionEngine;
use failure::Error;

fn main() -> Result<(), Error> {
    let mut engine: ExecutionEngine = Default::default();
    let main_context: SyncMut<ExecutionContext> = ExecutionContext::from_instructions(vec![
        Inst::Alloc { name: "x".into() },
        Inst::Alloc { name: "y".into() },
        Inst::Alloc { name: "_x".into() },
        Inst::Alloc { name: "z".into() },
        Inst::PushFloat {
            name: "x".into(),
            value: 3.0,
        },
        Inst::PushInt {
            name: "y".into(),
            value: 4,
        },
        Inst::PopObjectValue {
            pop_to_name: "_x".into(),
            object_name: "x".into(),
            key_name: "plus".into(),
        },
        Inst::Call {
            name: "_x".into(),
            arguments: vec!["y".into()].into(),
            this: Some("x".into()),
        },
        Inst::PushCallResult { name: "z".into() },
        Inst::Call {
            name: "print".into(),
            arguments: vec!["z".into()].into(),
            this: None,
        },
        Inst::Call {
            name: "exit".into(),
            arguments: vec!["x".into()].into(),
            this: None,
        },
    ]);
    engine.push_task(main_context);
    ExecutionEngine::run(&new_syncmut(engine))
}

/*
Inst::Alloc { name: "x".into() },
        Inst::Alloc { name: "y".into() },
        Inst::Alloc { name: "z".into() },
        Inst::PushFunction {
            name: "y".into(),
            argument_names: vec!["foo".into()].into(),
            instructions: vec![
                Inst::Call {
                    name: "print".into(),
                    arguments: vec!["foo".into()].into(),
                    this: None,
                },
                Inst::Alloc { name: "z".into() },
                Inst::PushFloat {
                    name: "z".into(),
                    value: 0.1,
                },
                Inst::Return { name: "z".into() },
            ]
            .into(),
        },
        Inst::PushInt {
            name: "x".into(),
            value: 32,
        },
        Inst::Call {
            name: "y".into(),
            arguments: vec!["x".into()].into(),
            this: None,
        },
        Inst::PushCallResult { name: "z".into() },
        Inst::Call {
            name: "print".into(),
            arguments: vec!["y".into()].into(),
            this: None,
        },
        Inst::Call {
            name: "print".into(),
            arguments: vec!["z".into()].into(),
            this: None,
        },
        Inst::PushInt {
            name: "x".into(),
            value: 34,
        },
        Inst::Call {
            name: "print".into(),
            arguments: vec!["x".into()].into(),
            this: None,
        },
        Inst::Call {
            name: "exit".into(),
            arguments: vec!["x".into()].into(),
            this: None,
        },
*/
