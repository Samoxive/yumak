#![allow(dead_code)]

extern crate common;
extern crate either;

use common::bytecode::Inst;
use common::SyncMut;
use either::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem::replace;
use std::sync::{Arc, Mutex};

pub struct ExecutionEngine {
    tasks: VecDeque<SyncMut<ExecutionContext>>,
}

pub struct NativeFunctionData {
    fun: Arc<Fn(Option<RcValue>, Vec<RcValue>) -> RcValue>,
}

#[derive(Clone)]
pub struct InterpretedFunctionData {
    argument_names: Arc<Vec<String>>,
    label_points: Arc<HashMap<String, usize>>,
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
            label_points: label_points.into(),
            instructions,
        }
    }
}

pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(Mutex<Vec<Value>>),
    Object(Mutex<HashMap<String, RcValue>>),
    NativeFunction(Arc<NativeFunctionData>),
    InterpretedFunction(InterpretedFunctionData),
    Nothing,
}

type RcValue = Arc<Value>;

fn search_value_from_context(
    context: SyncMut<ExecutionContext>,
    name: &str,
) -> Option<(SyncMut<ExecutionContext>, RcValue)> {
    let result: Option<RcValue> = {
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
    program: InterpretedFunctionData,
    stack: HashMap<String, RcValue>,
    parent_context: Option<SyncMut<ExecutionContext>>,
    call_result: RcValue,
}

impl ExecutionContext {
    fn from_instructions(instructions: Vec<Inst>) -> ExecutionContext {
        let mut label_points: HashMap<String, usize> = HashMap::new();
        for (i, elem) in instructions.iter().enumerate() {
            if let Inst::GoTo { name } = elem {
                label_points.insert(name.clone(), i);
            }
        }

        let program = InterpretedFunctionData {
            argument_names: Arc::new(vec![]),
            label_points: label_points.into(),
            instructions: Arc::new(instructions),
        };

        ExecutionContext {
            program_counter: 0,
            program,
            stack: HashMap::new(),
            parent_context: None,
            call_result: Value::Nothing.into(),
        }
    }

    fn from_interpreted_function_call(
        interpreted_function: InterpretedFunctionData,
        parent_context: Option<SyncMut<ExecutionContext>>,
    ) -> ExecutionContext {
        ExecutionContext {
            program_counter: 0,
            program: interpreted_function,
            stack: HashMap::new(),
            parent_context,
            call_result: Value::Nothing.into(),
        }
    }

    fn get_value_and_stack(
        &self,
        name: &str,
    ) -> Option<Either<RcValue, (SyncMut<ExecutionContext>, RcValue)>> {
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

    fn get_value(&self, name: &str) -> Option<RcValue> {
        self.get_value_and_stack(name).map(|value_and_stack| {
            value_and_stack.either(|value| value, |value_and_stack| value_and_stack.1)
        })
    }

    fn set_value(&mut self, name: String, value: RcValue) -> Result<(), ()> {
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

    fn insert_value(&mut self, name: String, value: RcValue) {
        self.stack.insert(name, value);
    }

    fn run(&mut self, engine: SyncMut<ExecutionEngine>, this_context: SyncMut<ExecutionContext>) {
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
            } => {
                self.handle_call(engine.clone(), this_context, name, arguments, this);
            }
            Inst::PushCallResult { name } => self.handle_push_call_result(name),
            Inst::Label { name } => self.handle_label(name),
            Inst::GoTo { name } => self.handle_goto(name),
            Inst::Branch {
                name,
                true_label,
                false_label,
            } => self.handle_branch(name, true_label, false_label),
            Inst::Return { name } => self.handle_return(engine.clone(), name),
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
        self.set_value(name, Value::List(Mutex::new(Vec::new())).into())
            .unwrap();
    }

    fn handle_push_object(&mut self, name: String) {
        self.set_value(name, Value::Object(Mutex::new(HashMap::new())).into())
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
            .expect("Could not find object to pop key from!");

        let target_value_and_context = self
            .get_value_and_stack(&pop_to_name)
            .expect("Could not find value to place popped value into!");

        if let Left(_) = target_value_and_context {
            let popped_value = if let Value::Object(ref map) = *object_value {
                let map_lock = map.lock().expect("Could not lock object's internal map!");
                map_lock
                    .get(&key_name)
                    .map(|popped| popped.clone())
                    .unwrap_or_else(|| Value::Nothing.into())
            } else {
                unimplemented!("Handle method returning etc for non-object values!")
            };

            self.stack.insert(pop_to_name, popped_value);
        } else if let Right((context, _)) = target_value_and_context {
            let popped_value = if let Value::Object(ref map) = *object_value {
                let map_lock = map.lock().expect("Could not lock object's internal map!");
                map_lock
                    .get(&key_name)
                    .map(|popped| popped.clone())
                    .unwrap_or_else(|| Value::Nothing.into())
            } else {
                unimplemented!("Handle method returning etc for non-object values!")
            };

            context
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
            .expect("Could not find object to be pushed into!");

        let target_value = self
            .get_value(&value_name)
            .expect("Could not find value to push into object!");

        if let Value::Object(ref map) = *object_value {
            let mut map_lock = map.lock().expect("Could not lock object's internal map!");
            map_lock.insert(key_name, target_value);
        } else {
            panic!("Selected object is no object at all!");
        }
    }

    fn handle_call(
        &mut self,
        engine: SyncMut<ExecutionEngine>,
        this_context: SyncMut<ExecutionContext>,
        name: String,
        arguments: Arc<Vec<String>>,
        this: Option<String>,
    ) -> bool {
        let call_value = self
            .get_value(&name)
            .expect("Could not find value to call!");

        let this_value_option: Option<RcValue> = if let Some(this_value_name) = this {
            self.get_value(&this_value_name)
                .expect("Could not find value to set as `this` for called function!")
                .into()
        } else {
            None
        };

        self.program_counter += 1;
        if let Value::InterpretedFunction(ref interpreted_function) = *call_value {
            let mut engine_lock = engine.lock().expect("Could not lock execution engine!");
            let function_clone = interpreted_function.clone();
            let mut new_context = ExecutionContext::from_interpreted_function_call(
                function_clone,
                this_context.into(),
            );
            for (i, pass_value_name) in arguments.iter().enumerate() {
                if let Some(argument_name) = interpreted_function.argument_names.get(i) {
                    let pass_value = self
                        .get_value(pass_value_name)
                        .expect("Could not find value to pass to function!");
                    new_context.stack.insert(argument_name.clone(), pass_value);
                } else {
                    break;
                }
            }

            if let Some(this_value) = this_value_option {
                new_context.stack.insert("this".into(), this_value);
            }

            engine_lock
                .tasks
                .push_back(Arc::new(Mutex::new(new_context)));
            return false;
        } else if let Value::NativeFunction(ref native_function) = *call_value {
            let result_value = (native_function.fun)(
                this_value_option,
                arguments
                    .iter()
                    .map(|argument_name| {
                        self.get_value(&argument_name)
                            .expect("Could not find value to pass to function!")
                    })
                    .collect(),
            );
            self.call_result = result_value;
            return true;
        } else {
            panic!("Value called wasn't a function!");
        }
    }

    fn handle_push_call_result(&mut self, name: String) {
        let call_result = replace(&mut self.call_result, Value::Nothing.into());
        self.set_value(name, call_result).unwrap()
    }

    fn handle_label(&mut self, _name: String) {
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
        let value = self
            .get_value(&name)
            .expect("Could not find value to branch on!");

        let bool_value = match *value {
            Value::Boolean(ref bool_value) => *bool_value,
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

    fn ret(&mut self, engine: SyncMut<ExecutionEngine>, return_value_option: Option<RcValue>) {
        let mut engine_lock = engine.lock().expect("Could not lock execution engine!");
        if let Some(ref context) = self.parent_context {
            let mut context_lock = context.lock().expect("Could not lock parent context!");
            context_lock.call_result = if let Some(return_value) = return_value_option {
                return_value
            } else {
                Value::Nothing.into()
            };

            engine_lock.tasks.push_back(context.clone())
        }
    }

    fn handle_return(&mut self, engine: SyncMut<ExecutionEngine>, name: String) {
        let return_value = self
            .get_value(&name)
            .expect("Could not get value to return!");
        self.ret(engine, return_value.into())
    }
}
