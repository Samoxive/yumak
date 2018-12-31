extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::fs;
use common::bytecode::Inst;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct YumakParser;

fn main() {
    let unparsed_file = fs::read_to_string("./src/asd.yumak").expect("cannot read file");
    let file = YumakParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    
    //let mut current_section_name = "";
    let mut insts: Vec<Inst> = vec![];
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::letalloc => {
                let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

                let result: &str = inner_rules.next().unwrap().as_str();
                insts.push(Inst::Alloc{
                    name: result.to_string()
                })
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:?}",insts);
}