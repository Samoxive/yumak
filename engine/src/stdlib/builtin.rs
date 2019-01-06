use super::StdLibAdder;
use crate::value::{make_function, RcValue, Value};
use std::collections::HashMap;
use std::process;

fn print_val(_this: Option<RcValue>, arguments: Vec<RcValue>) -> RcValue {
    arguments
        .iter()
        .for_each(|argument| println!("{:?}", argument));
    Value::Nothing.into()
}

fn exit(_this: Option<RcValue>, arguments: Vec<RcValue>) -> RcValue {
    if arguments.is_empty() {
        process::exit(0);
    } else if let Value::Integer(i) = *arguments[0] {
        process::exit(i as i32);
    } else {
        process::exit(0);
    }
}

pub struct BuiltinAdder;

impl StdLibAdder for BuiltinAdder {
    fn add(&self, stack: &mut HashMap<String, RcValue>) {
        stack.insert("print".into(), make_function(Box::new(print_val)));
        stack.insert("exit".into(), make_function(Box::new(exit)));
    }
}
