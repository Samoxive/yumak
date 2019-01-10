use common::bytecode::Inst;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

pub type NativeFn = Fn(Option<RcValue>, Vec<RcValue>) -> RcValue;

pub fn make_function(fun: Box<NativeFn>) -> RcValue {
    Value::NativeFunction(NativeFunctionData { fun }).into()
}

pub struct NativeFunctionData {
    pub fun: Box<NativeFn>,
}

impl Debug for NativeFunctionData {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "NativeFunction {{ fun }}")
    }
}

#[derive(Clone, Debug)]
pub struct InterpretedFunctionData {
    pub argument_names: Arc<Vec<String>>,
    pub label_points: Arc<HashMap<String, usize>>,
    pub instructions: Arc<Vec<Inst>>,
}

impl InterpretedFunctionData {
    pub fn from_argument_and_instructions(
        argument_names: Arc<Vec<String>>,
        instructions: Arc<Vec<Inst>>,
    ) -> Self {
        let mut label_points: HashMap<String, usize> = HashMap::new();
        for (i, elem) in instructions.iter().enumerate() {
            if let Inst::Label { name } = elem {
                label_points.insert(name.clone(), i);
            }
        }

        InterpretedFunctionData {
            argument_names,
            label_points: label_points.into(),
            instructions,
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(Mutex<Vec<Value>>),
    Object(Mutex<HashMap<String, RcValue>>),
    NativeFunction(NativeFunctionData),
    InterpretedFunction(InterpretedFunctionData),
    Nothing,
}

pub type RcValue = Arc<Value>;
