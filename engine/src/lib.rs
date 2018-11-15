extern crate common;

use common::bytecode::Inst;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct NativeFunctionData {
    fun: Box<Fn(Vec<Arc<Value>>) -> Arc<Value>>,
}

pub struct InterpretedFunctionData {
    argument_names: Vec<String>,
    label_points: HashMap<String, usize>,
    instructions: Vec<Inst>,
}

pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
    NativeFunction(NativeFunctionData),
    InterpretedFunction(Box<InterpretedFunctionData>),
    Nothing,
}

pub struct ExecutionContext {
    program_counter: usize,
    label_points: HashMap<String, usize>,
    program: Arc<Box<InterpretedFunctionData>>,
    stack: HashMap<String, Arc<Value>>,
    parent_context: Option<Arc<Mutex<ExecutionContext>>>,
    call_result: Option<Value>,
}

impl ExecutionContext {
    fn from_instructions(instructions: Vec<Inst>) -> ExecutionContext {
        let mut label_points: HashMap<String, usize> = HashMap::new();
        for (i, elem) in instructions.iter().enumerate() {
            if let Inst::GoTo { name } = elem {
                label_points.insert(name.clone(), i);
            }
        }

        let function_data = InterpretedFunctionData {
            argument_names: vec![],
            label_points,
            instructions,
        };

        let function_box = Box::from(function_data);
        let program = Arc::from(function_box);
        ExecutionContext {
            program_counter: 0,
            label_points,
            program,
            stack: HashMap::new(),
            parent_context: None,
            call_result: None,
        }
    }
}
