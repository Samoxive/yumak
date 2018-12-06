#![allow(dead_code)]

extern crate common;
extern crate either;

use common::bytecode::Inst;
use common::SyncMut;
use either::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem::replace;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

pub struct ExecutionEngine {
    tasks: VecDeque<ExecutionContext>,
}

pub struct NativeFunctionData {
    fun: Arc<Fn(Option<SyncValue>, Vec<SyncValue>) -> SyncValue>,
}

pub struct InterpretedFunctionData {
    argument_names: Arc<Vec<String>>,
    label_points: HashMap<String, usize>,
    instructions: Arc<Vec<Inst>>,
}

impl InterpretedFunctionData {
    fn from_argument_and_instructions(
        argument_names: Arc<Vec<String>>,
        instructions: Arc<Vec<Inst>>,
    ) -> Self {
        let mut label_points: HashMap<String, usize> = HashMap::new();
        for (i, elem) in instructions.iter().enumerate() {
            if let Inst::GoTo { name } = elem {
                label_points.insert(name.clone(), i);
            }
        }

        InterpretedFunctionData {
            argument_names,
            label_points,
            instructions,
        }
    }
}

pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Object(HashMap<String, SyncValue>),
    NativeFunction(Arc<NativeFunctionData>),
    InterpretedFunction(InterpretedFunctionData),
    Nothing,
}

type SyncValue = SyncMut<Value>;

impl From<Value> for SyncValue {
    fn from(value: Value) -> Self {
        Arc::from(Mutex::new(value))
    }
}

fn search_value_from_context(
    context: SyncMut<ExecutionContext>,
    name: &str,
) -> Option<(SyncMut<ExecutionContext>, SyncValue)> {
    let result: Option<SyncValue> = {
        context
            .lock()
            .expect("Could not lock execution context!")
            .stack
            .get(name)
            .map(|value| value.clone())
    };

    match result {
        Some(value_ref) => return Some((context, value_ref)),
        _ => (),
    };

    let parent_context: Option<SyncMut<ExecutionContext>> = {
        context
            .lock()
            .expect("Could not lock execution context!")
            .parent_context
            .as_ref()
            .map(|context| context.clone())
    };

    match parent_context {
        Some(parent) => search_value_from_context(parent, name),
        None => None,
    }
}

pub struct ExecutionContext {
    program_counter: usize,
    program: Arc<InterpretedFunctionData>,
    stack: HashMap<String, SyncValue>,
    parent_context: Option<SyncMut<ExecutionContext>>,
    call_result: SyncValue,
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
            argument_names: Arc::new(vec![]),
            label_points,
            instructions: Arc::new(instructions),
        };

        let function_box = Box::from(function_data);
        let program = Arc::from(function_box);
        ExecutionContext {
            program_counter: 0,
            program,
            stack: HashMap::new(),
            parent_context: None,
            call_result: Value::Nothing.into(),
        }
    }

    fn get_value(
        &self,
        name: &str,
    ) -> Option<Either<SyncValue, (SyncMut<ExecutionContext>, SyncValue)>> {
        let result = self.stack.get(name);
        match result {
            Some(value_ref) => return Some(Left(value_ref.clone())),
            _ => (),
        }

        let parent_context = self.parent_context.as_ref().map(|parent| parent.clone());
        match parent_context {
            Some(parent) => search_value_from_context(parent, name).map(|val| Right(val)),
            None => None,
        }
    }

    fn set_value(&mut self, name: String, value: SyncValue) -> Result<(), ()> {
        if self.stack.get(&name).is_some() {
            self.stack.insert(name, value);
            return Ok(());
        }

        if let Some(ref context_mutex) = self.parent_context {
            context_mutex
                .lock()
                .expect("Could not lock execution context!")
                .set_value(name, value)
        } else {
            Err(())
        }
    }

    fn insert_value(&mut self, name: String, value: SyncValue) {
        self.stack.insert(name, value);
    }

    fn run(&mut self, engine: SyncMut<ExecutionEngine>) {
        let current_instruction: Inst = self
            .program
            .instructions
            .get(self.program_counter)
            .expect("Invalid PC!")
            .clone();

        match current_instruction {
            Inst::Alloc { name } => self.handle_alloc(name),
            Inst::PushInt { name, value } => self.handle_push_int(name, value),
            Inst::PushFloat { name, value } => self.handle_push_float(name, value),
            Inst::PushBoolean { name, value } => self.handle_push_boolean(name, value),
            Inst::PushFunction {
                name,
                argument_names,
                instructions,
            } => self.handle_push_function(name, argument_names, instructions),
            Inst::PushList { name } => self.handle_push_list(name),
            Inst::PushObject { name } => self.handle_push_object(name),
            Inst::PopObjectValue {
                pop_to_name,
                object_name,
                key_name,
            } => self.handle_pop_object_value(pop_to_name, object_name, key_name),
            Inst::PushObjectValue {
                object_name,
                key_name,
                value_name,
            } => self.handle_push_object_value(object_name, key_name, value_name),
            Inst::Call {
                name,
                arguments,
                this,
            } => self.handle_call(name, arguments, this),
            Inst::PushCallResult { name } => self.handle_push_call_result(name),
            Inst::Label { name } => self.handle_label(name),
            Inst::GoTo { name } => self.handle_goto(name),
            Inst::Branch {
                name,
                true_label,
                false_label,
            } => self.handle_branch(name, true_label, false_label),
            Inst::Return { name } => self.handle_return(name),
        }
    }

    fn handle_alloc(&mut self, name: String) {
        self.insert_value(name, Value::Nothing.into());
    }

    fn handle_push_int(&mut self, name: String, value: i64) {
        self.set_value(name, Value::Integer(value).into()).unwrap();
    }

    fn handle_push_float(&mut self, name: String, value: f64) {
        self.set_value(name, Value::Float(value).into()).unwrap();
    }

    fn handle_push_boolean(&mut self, name: String, value: bool) {
        self.set_value(name, Value::Boolean(value).into()).unwrap();
    }

    fn handle_push_function(
        &mut self,
        name: String,
        argument_names: Arc<Vec<String>>,
        instructions: Arc<Vec<Inst>>,
    ) {
        self.set_value(
            name,
            Value::InterpretedFunction(InterpretedFunctionData::from_argument_and_instructions(
                argument_names,
                instructions,
            ))
            .into(),
        )
        .unwrap();
    }

    fn handle_push_list(&mut self, name: String) {
        self.set_value(name, Value::List(Vec::new()).into())
            .unwrap();
    }

    fn handle_push_object(&mut self, name: String) {
        self.set_value(name, Value::Object(HashMap::new()).into())
            .unwrap();
    }

    fn handle_pop_object_value(
        &mut self,
        pop_to_name: String,
        object_name: String,
        key_name: String,
    ) {
        let object_value = self
            .get_value(&object_name)
            .expect("Could not find object to pop key from!")
            .either(|value| value, |value_and_stack| value_and_stack.1);

        let target_value_and_stack = self
            .get_value(&pop_to_name)
            .expect("Could not find value to place popped value into!");

        if let Left(value) = target_value_and_stack {
            let object_lock = object_value.lock().expect("Could not lock object!");
            let popped_value = if let Value::Object(map) = object_lock.deref() {
                map.get(&key_name)
                    .map(|popped| popped.clone())
                    .unwrap_or_else(|| Value::Nothing.into())
            } else {
                unimplemented!("Handle method returning etc for non-object values!")
            };

            self.stack.insert(pop_to_name, popped_value);
        } else if let Right((stack, target_value)) = target_value_and_stack {
            let object_lock = object_value.lock().expect("Could not lock object!");
            let popped_value = if let Value::Object(map) = object_lock.deref() {
                map.get(&key_name)
                    .map(|popped| popped.clone())
                    .unwrap_or_else(|| Value::Nothing.into())
            } else {
                unimplemented!("Handle method returning etc for non-object values!")
            };

            stack
                .lock()
                .expect("Could not lock execution context!")
                .stack
                .insert(pop_to_name, popped_value);;
        }
    }

    fn handle_push_object_value(
        &mut self,
        object_name: String,
        key_name: String,
        value_name: String,
    ) {
        let object_value = self
            .get_value(&object_name)
            .expect("Could not find object to be pushed into!")
            .either(|value| value, |value_and_stack| value_and_stack.1);
        let mut object_lock = object_value.lock().expect("Could not lock object!");

        let target_value = self
            .get_value(&value_name)
            .expect("Could not find value to push into object!")
            .either(|value| value, |value_and_stack| value_and_stack.1);

        if let Value::Object(ref mut map) = *object_lock {
            map.insert(key_name, target_value);
        } else {
            panic!("Selected object is no object at all!");
        }
    }

    fn handle_call(&mut self, name: String, arguments: Arc<Vec<String>>, this: Option<String>) {}

    fn handle_push_call_result(&mut self, name: String) {
        let call_result = replace(&mut self.call_result, Value::Nothing.into());
        self.set_value(name, call_result).unwrap()
    }

    fn handle_label(&mut self, name: String) {
        // nada
    }

    fn handle_goto(&mut self, name: String) {
        self.program_counter = *self
            .program
            .label_points
            .get(&name)
            .expect("Label to goto could not be found!");
    }

    fn handle_branch(
        &mut self,
        name: String,
        true_label: Option<String>,
        false_label: Option<String>,
    ) {
        let value_and_stack = self
            .get_value(&name)
            .expect("Could not find value to branch on!");
        let value = value_and_stack.either(|value| value, |value_and_stack| value_and_stack.1);
        let value_lock = value.lock().expect("Could not lock value!");
        let bool_value = match *value_lock {
            Value::Boolean(ref value) => *value,
            _ => panic!("Value to branch on is not a boolean!"),
        };

        if true_label.is_none() && false_label.is_none() {
            panic!("There is no labels to branch to!");
        }

        if bool_value {
            match true_label {
                Some(label) => self.handle_goto(label),
                None => self.program_counter += 1,
            }
        } else {
            match false_label {
                Some(label) => self.handle_goto(label),
                None => self.program_counter += 1,
            }
        }
    }

    fn handle_return(&mut self, name: String) {}
}
