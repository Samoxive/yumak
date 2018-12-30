#![feature(box_syntax, box_patterns, slice_patterns, plugin, rustc_private,
           quote, question_mark, try_from)]

#[macro_use] 
extern crate lalrpop_util;
extern crate syntax;

use lazy_static::lazy_static;

use crate::grammar::ToplevelParser;
use crate::grammar::AllocParser;
use common::bytecode::Inst;

use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::fs;

pub mod token;
pub mod lexer;
lalrpop_mod!(grammar);

enum Status<E> {
    Ok,
    Quit,
    Err(E),
}

lazy_static! {
    static ref PARSER: ToplevelParser = ToplevelParser::new();
    static ref testPARSER: AllocParser = AllocParser::new();
}

fn compile(filename: &str) -> common::bytecode::Inst{
    let mut file = File::open(filename).expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let res = testPARSER.parse(&contents, lexer::Lexer::new(&contents)).unwrap();
    return res;
}

#[test]  
fn main(){
    //let comp = compile("./src/asd.yumak");
    //assert_eq!(&format!("{:?}", comp), "ERR");
    let mut file = File::open("./src/asd.yumak").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let res = testPARSER.parse(&contents, lexer::Lexer::new(&contents)).unwrap();
    //println!("{}",res);
    assert_eq!(res, common::bytecode::Inst::Alloc{name: "testing".to_string()});
}