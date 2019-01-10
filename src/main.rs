extern crate common;
extern crate engine;
extern crate failure;

use common::bytecode::Inst;
use common::{new_syncmut, SyncMut};
use engine::context::ExecutionContext;
use engine::ExecutionEngine;
use failure::Error;

fn fib(n: i64) -> Result<(), Error> {
    let mut engine: ExecutionEngine = Default::default();
    let main_context: SyncMut<ExecutionContext> = ExecutionContext::from_instructions(vec![
        Inst::Alloc {
            name: "fibonacci".into(),
        },
        Inst::PushFunction {
            name: "fibonacci".into(),
            argument_names: vec!["n".into()].into(),
            instructions: vec![
                Inst::Alloc {
                    name: "_lit_0".into(),
                },
                Inst::PushInt {
                    name: "_lit_0".into(),
                    value: 0,
                },
                Inst::Alloc {
                    name: "_lit_1".into(),
                },
                Inst::PushInt {
                    name: "_lit_1".into(),
                    value: 1,
                },
                Inst::Alloc {
                    name: "_n#eq".into(),
                },
                Inst::PopObjectValue {
                    pop_to_name: "_n#eq".into(),
                    object_name: "n".into(),
                    key_name: "eq".into(),
                },
                Inst::Call {
                    name: "_n#eq".into(),
                    arguments: vec!["_lit_0".into()].into(),
                    this: Some("n".into()),
                },
                Inst::Alloc {
                    name: "_eq_n__lit_0".into(),
                },
                Inst::PushCallResult {
                    name: "_eq_n__lit_0".into(),
                },
                Inst::Call {
                    name: "_n#eq".into(),
                    arguments: vec!["_lit_1".into()].into(),
                    this: Some("n".into()),
                },
                Inst::Alloc {
                    name: "_eq_n__lit_1".into(),
                },
                Inst::PushCallResult {
                    name: "_eq_n__lit_1".into(),
                },
                Inst::Alloc {
                    name: "_eq_n__lit_0#or".into(),
                },
                Inst::PopObjectValue {
                    pop_to_name: "_eq_n__lit_0#or".into(),
                    object_name: "_eq_n__lit_0".into(),
                    key_name: "or".into(),
                },
                Inst::Call {
                    name: "_eq_n__lit_0#or".into(),
                    arguments: vec!["_eq_n__lit_1".into()].into(),
                    this: Some("_eq_n__lit_0".into()),
                },
                Inst::Alloc {
                    name: "_if_0_expr".into(),
                },
                Inst::PushCallResult {
                    name: "_if_0_expr".into(),
                },
                Inst::Branch {
                    name: "_if_0_expr".into(),
                    true_label: Some("_if_0_true".into()),
                    false_label: Some("_if_0_false".into()),
                },
                Inst::Label {
                    name: "_if_0_true".into(),
                },
                Inst::Return { name: "n".into() },
                Inst::Label {
                    name: "_if_0_false".into(),
                },
                Inst::Alloc {
                    name: "_lit_2".into(),
                },
                Inst::PushInt {
                    name: "_lit_2".into(),
                    value: 1,
                },
                Inst::Alloc {
                    name: "_lit_3".into(),
                },
                Inst::PushInt {
                    name: "_lit_3".into(),
                    value: 2,
                },
                Inst::Alloc {
                    name: "_n#minus".into(),
                },
                Inst::PushCallResult {
                    name: "_n#minus".into(),
                },
                Inst::PopObjectValue {
                    pop_to_name: "_n#minus".into(),
                    object_name: "n".into(),
                    key_name: "minus".into(),
                },
                Inst::Call {
                    name: "_n#minus".into(),
                    arguments: vec!["_lit_2".into()].into(),
                    this: Some("n".into()),
                },
                Inst::Alloc {
                    name: "_minus_n__lit_2".into(),
                },
                Inst::PushCallResult {
                    name: "_minus_n__lit_2".into(),
                },
                Inst::Call {
                    name: "_n#minus".into(),
                    arguments: vec!["_lit_3".into()].into(),
                    this: Some("n".into()),
                },
                Inst::Alloc {
                    name: "_minus_n__lit_3".into(),
                },
                Inst::PushCallResult {
                    name: "_minus_n__lit_3".into(),
                },
                Inst::Call {
                    name: "fibonacci".into(),
                    arguments: vec!["_minus_n__lit_2".into()].into(),
                    this: None,
                },
                Inst::Alloc {
                    name: "_fibonacci__minus_n__lit_2".into(),
                },
                Inst::PushCallResult {
                    name: "_fibonacci__minus_n__lit_2".into(),
                },
                Inst::Call {
                    name: "fibonacci".into(),
                    arguments: vec!["_minus_n__lit_3".into()].into(),
                    this: None,
                },
                Inst::Alloc {
                    name: "_fibonacci__minus_n__lit_3".into(),
                },
                Inst::PushCallResult {
                    name: "_fibonacci__minus_n__lit_3".into(),
                },
                Inst::Alloc {
                    name: "_fibonacci__minus_n__lit_2#plus".into(),
                },
                Inst::PopObjectValue {
                    pop_to_name: "_fibonacci__minus_n__lit_2#plus".into(),
                    object_name: "_fibonacci__minus_n__lit_2".into(),
                    key_name: "plus".into(),
                },
                Inst::Call {
                    name: "_fibonacci__minus_n__lit_2#plus".into(),
                    arguments: vec!["_fibonacci__minus_n__lit_3".into()].into(),
                    this: Some("_fibonacci__minus_n__lit_2".into()),
                },
                Inst::Alloc {
                    name: "_plus__fibonacci__minus_n__lit_2__fibonacci__minus_n__lit_3".into(),
                },
                Inst::PushCallResult {
                    name: "_plus__fibonacci__minus_n__lit_2__fibonacci__minus_n__lit_3".into(),
                },
                Inst::Return {
                    name: "_plus__fibonacci__minus_n__lit_2__fibonacci__minus_n__lit_3".into(),
                },
            ]
            .into(),
        },
        Inst::Alloc { name: "x".into() },
        Inst::Alloc { name: "y".into() },
        Inst::PushInt {
            name: "x".into(),
            value: n,
        },
        Inst::Call {
            name: "fibonacci".into(),
            arguments: vec!["x".into()].into(),
            this: None,
        },
        Inst::PushCallResult { name: "y".into() },
        Inst::Call {
            name: "print".into(),
            arguments: vec!["y".into()].into(),
            this: None,
        },
        Inst::Call {
            name: "exit".into(),
            arguments: vec![].into(),
            this: None,
        },
    ]);
    engine.push_task(main_context);
    ExecutionEngine::run(&new_syncmut(engine))
}

#[allow(dead_code)]
fn sum_text() -> Result<(), Error> {
    let mut engine: ExecutionEngine = Default::default();
    let main_context: SyncMut<ExecutionContext> = ExecutionContext::from_instructions(vec![
        Inst::Alloc { name: "x".into() },
        Inst::Alloc { name: "y".into() },
        Inst::Alloc { name: "_x".into() },
        Inst::Alloc { name: "z".into() },
        Inst::PushString {
            name: "x".into(),
            value: "hello".into(),
        },
        Inst::PushInt {
            name: "y".into(),
            value: 4,
        },
        Inst::PopObjectValue {
            pop_to_name: "_x".into(),
            object_name: "x".into(),
            key_name: "times".into(),
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

fn main() {
    fib(9).expect("works");
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
