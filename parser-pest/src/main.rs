extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::fs;

extern crate common;
extern crate engine;

use common::bytecode::Inst;
use common::{new_syncmut, SyncMut};
use engine::{ExecutionEngine};
use engine::context::ExecutionContext;

use pest::{Parser,
    //error::Error as PestError,
    //iterators::{Pair, Pairs},
    //prec_climber::{Operator, PrecClimber, Assoc}
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct YumakParser;

fn main() {
    let unparsed_file = fs::read_to_string("./src/asd.yumak").expect("cannot read file");
    let file = YumakParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut insts: Vec<Inst> = vec![];
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::let_allocate => {
                let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

                let result: &str = inner_rules.next().unwrap().as_str();
                insts.push(Inst::Alloc{
                    name: result.to_string()
                })
            }
            Rule::function_call=>{
                let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

                let func: &str = inner_rules.next().unwrap().as_str();
                let args: &str = inner_rules.next().unwrap().as_str();
                insts.push(Inst::Call{
                    name: func.to_string(),
                    arguments: vec![args.to_string()].into(),
                    this: None
                })
            }
            Rule::assignment=>{
                let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }
                let variable = inner_rules.next().unwrap();
                let mut var_name;
                match variable.as_rule() {
                    Rule::let_allocate => {
                        let mut inner_rules = variable.into_inner(); // { name ~ "=" ~ value }
                        let result: &str = inner_rules.next().unwrap().as_str();
                        insts.push(Inst::Alloc{
                            name: result.to_string()
                        });
                        var_name = result.to_string();
                        },
                    Rule::variable => {
                        let mut inner_rules = variable.into_inner(); // { name ~ "=" ~ value }
                        var_name = inner_rules.next().unwrap().as_str().to_string();
                        },
                    _ => unreachable!()
                }
                //println!("{:?}",variable);
                let args = inner_rules.next().unwrap();
                match args.as_rule() {
                    Rule::array => {
                        let mut inside = args.into_inner();
                        let mut variable = inside.next();
                        let mut name = &var_name;
                        insts.push(Inst::PushList{
                            name: name.to_string()
                        });
                        insts.push(Inst::PopObjectValue{
                            pop_to_name:  format!("_{}#pop", &var_name), 
                            object_name: format!("{}", &var_name), 
                            key_name: "push".into()
                        });
                        let mut arrayCtr = 0;
                        while variable!=None {
                            let variableUnwrap = variable.unwrap();
                            match variableUnwrap.as_rule() {
                                Rule::integer => {
                                    let result = variableUnwrap.as_str().parse::<i64>().unwrap(); // { name ~ "=" ~ value }
                                    insts.push(Inst::Alloc{
                                        name: format!("_lit_{}", arrayCtr)
                                    });
                                    insts.push(Inst::PushInt{
                                        name: format!("_lit_{}", arrayCtr),
                                        value: result
                                    });
                                    insts.push(Inst::Call{
                                        name: format!("_{}#pop", &var_name), 
                                        arguments: vec![format!("_lit_{}", arrayCtr)].into(), 
                                        this: Some(format!("{}", &var_name))
                                    });
                                },
                                Rule::float => {
                                    let result = variableUnwrap.as_str().parse::<f64>().unwrap(); // { name ~ "=" ~ value }
                                    insts.push(Inst::Alloc{
                                        name: format!("_lit_{}", arrayCtr)
                                    });
                                    insts.push(Inst::PushFloat{
                                        name: format!("_lit_{}", arrayCtr),
                                        value: result
                                    });
                                    insts.push(Inst::Call{
                                        name: format!("_{}#pop", &var_name), 
                                        arguments: vec![format!("_lit_{}", arrayCtr)].into(), 
                                        this: Some(format!("{}", &var_name))
                                    });
                                }
                                _=>unreachable!()
                            }
                            variable = inside.next();
                            arrayCtr = arrayCtr+1;
                        } 
                    },
                    Rule::integer => {
                        let result = args.as_str().parse::<i64>().unwrap(); // { name ~ "=" ~ value }
                        insts.push(Inst::PushInt{
                            name: var_name,
                            value: result
                        });
                    },
                    Rule::float => {
                        let result = args.as_str().parse::<f64>().unwrap(); // { name ~ "=" ~ value }
                        insts.push(Inst::PushFloat{
                            name: var_name,
                            value: result
                        });
                    }
                    _ => unreachable!()
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:?}",insts);
    /*
    let mut engine: ExecutionEngine = Default::default();
    let main_context: SyncMut<ExecutionContext> = ExecutionContext::from_instructions(insts);
    engine.push_task(main_context);
    ExecutionEngine::run(&new_syncmut(engine));
    */
}
