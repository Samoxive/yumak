use crate::value::RcValue;
use std::collections::HashMap;

pub mod types;

mod builtin;
use crate::stdlib::builtin::BuiltinAdder;

pub trait StdLibAdder {
    fn add(&self, stack: &mut HashMap<String, RcValue>);
}

pub fn get_stdlib() -> HashMap<String, RcValue> {
    let adders: Vec<&StdLibAdder> = vec![&BuiltinAdder];
    let mut stack = HashMap::new();

    for adder in &adders {
        adder.add(&mut stack);
    }

    stack
}
