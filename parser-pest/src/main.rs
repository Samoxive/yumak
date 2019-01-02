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


fn consume<'i>(pair: Pair<'i, Rule>, climber: &PrecClimber<Rule>) -> i32 {
    let primary = |pair| consume(pair, climber);
    let infix = |lhs: i32, op: Pair<Rule>, rhs: i32| match op.as_rule() {
        Rule::add => lhs + rhs,
        Rule::subtract => lhs - rhs,
        Rule::multiply => lhs * rhs,
        Rule::divide   => lhs / rhs,
        Rule::modulus  => lhs % rhs,
            //Rule::power    => lhs.powf(rhs),
        Rule::power => lhs.pow(rhs as u32),
        _ => unreachable!(),
    };
    //println!("{:?}",pair);
    match pair.as_rule() {
        Rule::expr => climber.climb(pair.into_inner(), primary, infix),
        //Rule::number => pair.as_rule()//as_str().parse::<i32>().unwrap(),
        Rule::integer => pair.as_str().parse::<i32>().unwrap(),
        Rule::float => pair.as_str().parse::<i32>().unwrap(),
        _ => 0,
    }
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
                //let expr = 
                let args = inner_rules.next().unwrap();
                match args.as_rule() {
                    Rule::expr => {
                        //let expr = args.inner_rules().next().unwrap();
                        println!("{:?}",consume(args, &PREC_CLIMBER));
                    }
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

    let mut engine: ExecutionEngine = ExecutionEngine::new();
    let main_context: SyncMut<ExecutionContext> = ExecutionContext::from_instructions(insts);
    engine.push_task(main_context);
    ExecutionEngine::run(new_syncmut(engine));
    //println!("{:?}",insts);
}