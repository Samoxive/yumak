use pest::{self, Parser, iterators::{Pair}};

extern crate common;
use common::bytecode::Inst;
use common::{new_syncmut, SyncMut};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct YumakParser;

pub fn parse(source: &str) -> std::result::Result<Vec<Inst>, pest::error::Error<Rule>> {
    let mut insts = vec![];
    let pairs = YumakParser::parse(Rule::file, source)?;
    for pair in pairs {
        if let Rule::values = pair.as_rule() {
            insts.push(build_ast_from_values(pair));
        }
    }
    Ok(insts)
}

fn build_ast_from_values(pair: Pair<Rule>) -> Inst {
    match pair.as_rule() {
        Rule::values => build_ast_from_values(pair.into_inner().next().unwrap()),
        Rule::function_call => {
            let mut pair = pair.into_inner();
            let func = pair.next().unwrap();
            let args = pair.next().unwrap();
            build_ast_from_function_call(func, args)
        },
        Rule::assignment => {
            let mut pair = pair.into_inner();
            let func = pair.next().unwrap();
            let args = pair.next().unwrap();
            let mut var_name = "".to_string();
            println!{"func, {:?}", func};
            println!{ };
            println!{"args, {:?}", args};
            match (func.as_rule(), args.as_rule()) {
                (Rule::let_allocate, _) => {
                    println!{"letalloc"};
                    let mut pair = func.into_inner();
                    let op = pair.next().unwrap();
                    var_name = op.as_str().to_string();
                    build_ast_from_letallocate(op)
                },
                (Rule::variable, _) => {
                    println!{"variable"};
                    let mut pair = func.into_inner();
                    let op = pair.next().unwrap();
                    var_name = op.as_str().to_string();
                    // NOOP
                    Inst::Label{
                        name: op.as_str().to_string(),
                    }
                },
                // It won't trigger that part.
                (_, Rule::factor) => {
                    println!{"infac, {:?}", pair};
                    let mut pair = args.into_inner();
                    let op = pair.next().unwrap();
                    Inst::PushInt{
                        name: var_name,
                        value: op.as_str().parse::<i64>().unwrap()
                    }
                },
                _ => unreachable!()
            }
        },
        Rule::EOI => (
            Inst::Call{
                name: "exit".to_string(),
                arguments: vec!["".to_string()].into(),
                this: None
            }
        ),
        _ => unreachable!(),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_letallocate(pair: Pair<Rule>) -> Inst {
    Inst::Alloc{
        name: pair.as_str().to_string()
    }
}

fn build_ast_from_function_call(func: Pair<Rule>, args: Pair<Rule>) -> Inst{
    Inst::Call{
        name: func.as_str().to_string(),
        arguments: vec![args.as_str().to_string()].into(),
        this: None
    }
}