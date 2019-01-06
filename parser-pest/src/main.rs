extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::fs;

extern crate common;
extern crate engine;

use common::bytecode::Inst;
use common::{new_syncmut, SyncMut};
use engine::{ExecutionContext, ExecutionEngine};
use pest::{Parser,
    iterators::{Pair, Pairs},
    prec_climber::{Operator, PrecClimber, Assoc}
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct YumakParser;


#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        PrecClimber::new(vec![
            Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
            Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left) | Operator::new(Rule::modulus, Assoc::Left),
            Operator::new(Rule::power, Assoc::Right)
        ])
    };
}


fn consume<'i>(pair: Pair<'i, Rule>, climber: &PrecClimber<Rule>) -> Vec<common::bytecode::Inst> {
    let mut insts: Vec<Inst> = vec![];
    let primary = |pair| consume(pair, climber);
    let infix = |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
        Rule::add => {
            insts.push(Inst::Alloc{
                name: "lhs".to_string()
            });
            insts.push(Inst::Alloc{
                name: "rhs".to_string()
            });
            insts.push(Inst::PushFloat{
                name: "lhs".to_string(),
                value: lhs
            });
            insts.push(Inst::PushFloat{
                name: "rhs".to_string(),
                value: rhs
            });
            insts.push(Inst::Alloc{
                name: "lhs#add".to_string()
            });
            insts.push(Inst::PopObjectValue{
                pop_to_name: "lhs#add".to_string(),
                object_name: "lhs".to_string(),
                key_name: "add".to_string(),
            });
            insts.push(Inst::Call{
                name: "lhs#add".to_string(),
                arguments: vec!["rhs".into()].into(),
                this: Some("lhs".to_string()),
            });
        },
        Rule::subtract => {
            insts.push(Inst::Alloc{
                name: "lhs".to_string()
            });
            insts.push(Inst::Alloc{
                name: "rhs".to_string()
            });
            insts.push(Inst::PushFloat{
                name: "lhs".to_string(),
                value: lhs
            });
            insts.push(Inst::PushFloat{
                name: "rhs".to_string(),
                value: rhs
            });
            insts.push(Inst::Alloc{
                name: "lhs#sub".to_string()
            });
            insts.push(Inst::PopObjectValue{
                pop_to_name: "lhs#sub".to_string(),
                object_name: "lhs".to_string(),
                key_name: "sub".to_string(),
            });
            insts.push(Inst::Call{
                name: "lhs#sub".to_string(),
                arguments: vec!["rhs"].into(),
                this: "lhs".to_string(),
            });
        },
        Rule::multiply => {
            insts.push(Inst::Alloc{
                name: "lhs".to_string()
            });
            insts.push(Inst::Alloc{
                name: "rhs".to_string()
            });
            insts.push(Inst::PushFloat{
                name: "lhs".to_string(),
                value: lhs
            });
            insts.push(Inst::PushFloat{
                name: "rhs".to_string(),
                value: rhs
            });
            insts.push(Inst::Alloc{
                name: "lhs#mul".to_string()
            });
            insts.push(Inst::PopObjectValue{
                pop_to_name: "lhs#mul".to_string(),
                object_name: "lhs".to_string(),
                key_name: "mul".to_string(),
            });
            insts.push(Inst::Call{
                name: "lhs#mul".to_string(),
                arguments: vec!["rhs"].into(),
                this: "lhs".to_string(),
            });
        },
        Rule::divide => {
            insts.push(Inst::Alloc{
                name: "lhs".to_string()
            });
            insts.push(Inst::Alloc{
                name: "rhs".to_string()
            });
            insts.push(Inst::PushFloat{
                name: "lhs".to_string(),
                value: lhs
            });
            insts.push(Inst::PushFloat{
                name: "rhs".to_string(),
                value: rhs
            });
            insts.push(Inst::Alloc{
                name: "lhs#div".to_string()
            });
            insts.push(Inst::PopObjectValue{
                pop_to_name: "lhs#div".to_string(),
                object_name: "lhs".to_string(),
                key_name: "div".to_string(),
            });
            insts.push(Inst::Call{
                name: "lhs#div".to_string(),
                arguments: vec!["rhs"].into(),
                this: "lhs".to_string(),
            });
        },
        Rule::modulus => {
            insts.push(Inst::Alloc{
                name: "lhs".to_string()
            });
            insts.push(Inst::Alloc{
                name: "rhs".to_string()
            });
            insts.push(Inst::PushFloat{
                name: "lhs".to_string(),
                value: lhs
            });
            insts.push(Inst::PushFloat{
                name: "rhs".to_string(),
                value: rhs
            });
            insts.push(Inst::Alloc{
                name: "lhs#mod".to_string()
            });
            insts.push(Inst::PopObjectValue{
                pop_to_name: "lhs#mod".to_string(),
                object_name: "lhs".to_string(),
                key_name: "mod".to_string(),
            });
            insts.push(Inst::Call{
                name: "lhs#mod".to_string(),
                arguments: vec!["rhs"].into(),
                this: "lhs".to_string(),
            });
        },
            //Rule::power    => lhs.powf(rhs),
        Rule::power => {
            insts.push(Inst::Alloc{
                name: "lhs".to_string()
            });
            insts.push(Inst::Alloc{
                name: "rhs".to_string()
            });
            insts.push(Inst::PushFloat{
                name: "lhs".to_string(),
                value: lhs
            });
            insts.push(Inst::PushFloat{
                name: "rhs".to_string(),
                value: rhs
            });
            insts.push(Inst::Alloc{
                name: "lhs#pow".to_string()
            });
            insts.push(Inst::PopObjectValue{
                pop_to_name: "lhs#pow".to_string(),
                object_name: "lhs".to_string(),
                key_name: "pow".to_string(),
            });
            insts.push(Inst::Call{
                name: "lhs#pow".to_string(),
                arguments: vec!["rhs"].into(),
                this: "lhs".to_string(),
            });
        },
        _ => unreachable!(),
    };
    //println!("{:?}",pair);
    match pair.as_rule() {
        Rule::expr => climber.climb(pair.into_inner(), primary, infix),
        Rule::integer => {
            /*let result = args.as_str().parse::<i64>().unwrap();
            insts.push(Inst::PushInt{
                name: var_name,
                value: result
            });*/
        },
        Rule::float => {
            /*let result = args.as_str().parse::<f64>().unwrap();
            insts.push(Inst::PushFloat{
                name: var_name,
                value: result
            });*/
        },
        _ => 0,
    }
    return insts;
}


fn main() {
    let unparsed_file = fs::read_to_string("./src/asd.yumak").expect("cannot read file");
    let file = YumakParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    
    //let mut current_section_name = "";
    let mut insts: Vec<Inst> = vec![];
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::let_allocate => {
                let mut inner_rules = line.into_inner();

                let result: &str = inner_rules.next().unwrap().as_str();
                insts.push(Inst::Alloc{
                    name: result.to_string()
                })
            }
            Rule::function_call=>{
                let mut inner_rules = line.into_inner();

                let func: &str = inner_rules.next().unwrap().as_str();
                let args: &str = inner_rules.next().unwrap().as_str();
                insts.push(Inst::Call{
                    name: func.to_string(),
                    arguments: vec![args.to_string()].into(),
                    this: None
                })
            }
            Rule::assignment=>{
                let mut inner_rules = line.into_inner();
                let variable = inner_rules.next().unwrap();
                let mut var_name;
                match variable.as_rule() {
                    Rule::let_allocate => {
                        let mut inner_rules = variable.into_inner();
                        let result: &str = inner_rules.next().unwrap().as_str();
                        insts.push(Inst::Alloc{
                            name: result.to_string()
                        });
                        var_name = result.to_string();
                        },
                    Rule::variable => {
                        let mut inner_rules = variable.into_inner();
                        var_name = inner_rules.next().unwrap().as_str().to_string();
                        },
                    _ => unreachable!()
                }
                //println!("{:?}",variable);
                //let expr = 
                let args = inner_rules.next().unwrap();
                match args.as_rule() {
                    Rule::expr => {
                        //let expr = args.inner_rules().next().unwrap();
                        insts.append(&mut consume(args, &PREC_CLIMBER));
                        //println!("{:?}",consume(args, &PREC_CLIMBER));
                        insts.push(Inst::PushCallResult{
                            name: var_name,
                        });
                    }
                    Rule::integer => {
                        let result = args.as_str().parse::<i64>().unwrap();
                        insts.push(Inst::PushInt{
                            name: var_name,
                            value: result
                        });
                        },
                    Rule::float => {
                        let result = args.as_str().parse::<f64>().unwrap();
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

    let mut engine: ExecutionEngine = ExecutionEngine::new();
    let main_context: SyncMut<ExecutionContext> = ExecutionContext::from_instructions(insts);
    engine.push_task(main_context);
    ExecutionEngine::run(new_syncmut(engine));
    //println!("{:?}",insts);
}