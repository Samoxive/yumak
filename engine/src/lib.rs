#![allow(dead_code)]

extern crate common;
extern crate either;
extern crate failure;

use common::bytecode::Inst;
use common::{new_syncmut, SyncMut};
use either::*;
use failure::{format_err, Error};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::mem::replace;
use std::process::exit;
use std::sync::{Arc, Mutex};

pub type ExecutionResult = Result<(), Error>;

#[derive(Default)]
pub struct ExecutionEngine {
    tasks: VecDeque<SyncMut<ExecutionContext>>,
}

impl ExecutionEngine {
    pub fn run(engine: &SyncMut<ExecutionEngine>) -> Result<(), Error> {
        loop {
            let task_option = {
                let mut engine_lock = engine.lock().unwrap();
                engine_lock.tasks.pop_front()
            };

            if let Some(task) = task_option {
                task.lock().unwrap().run(engine, &task)?;
            }
        }

        Ok(())
    }

    pub fn push_task(&mut self, task: SyncMut<ExecutionContext>) {
        self.tasks.push_back(task);
    }
}

pub struct NativeFunctionData {
    fun: Arc<Fn(Option<RcValue>, Vec<RcValue>) -> RcValue>,
}

impl Debug for NativeFunctionData {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "NativeFunction {{ fun }}")
    }
}

fn print_val(_this: Option<RcValue>, arguments: Vec<RcValue>) -> RcValue {
    arguments
        .iter()
        .for_each(|argument| println!("{:?}", argument));
    Value::Nothing.into()
}

fn yumak_exit(_this: Option<RcValue>, arguments: Vec<RcValue>) -> RcValue {
    if arguments.is_empty() {
        exit(0);
    } else if let Value::Integer(i) = *arguments[0] {
        exit(i as i32);
    } else {
        exit(0);
    }
}

#[derive(Clone, Debug)]
pub struct InterpretedFunctionData {
    argument_names: Arc<Vec<String>>,
    label_points: Arc<HashMap<String, usize>>,
    instructions: Arc<Vec<Inst>>,
}

impl InterpretedFunctionData {
    pub fn from_argument_and_instructions(
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

#[derive(Debug)]
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
            .cloned()
    };

    if let Some(value_ref) = result {
        return Some((context, value_ref));
    };

    let parent_context: Option<SyncMut<ExecutionContext>> = {
        context
            .lock()
            .expect("Could not lock execution context!")
            .parent_context
            .as_ref()
            .cloned()
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
    pub fn from_instructions(instructions: Vec<Inst>) -> SyncMut<ExecutionContext> {
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

        let mut stack: HashMap<String, RcValue> = HashMap::new();
        stack.insert(
            "print".into(),
            Value::NativeFunction(Arc::new(NativeFunctionData {
                fun: Arc::new(print_val),
            }))
            .into(),
        );
        stack.insert(
            "exit".into(),
            Value::NativeFunction(Arc::new(NativeFunctionData {
                fun: Arc::new(yumak_exit),
            }))
            .into(),
        );

        new_syncmut(ExecutionContext {
            program_counter: 0,
            program,
            stack,
            parent_context: None,
            call_result: Value::Nothing.into(),
        })
    }

    pub fn from_interpreted_function_call(
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
        if let Some(value_ref) = result {
            return Some(Left(value_ref.clone()));
        }

        let parent_context = self.parent_context.as_ref().cloned();
        match parent_context {
            Some(parent) => search_value_from_context(parent, name).map(Right),
            None => None,
        }
    }

    fn get_value(&self, name: &str) -> Result<RcValue, Error> {
        let value_option = self.get_value_and_stack(name).map(|value_and_stack| {
            value_and_stack.either(|value| value, |value_and_stack| value_and_stack.1)
        });
        if let Some(value) = value_option {
            Ok(value)
        } else {
            Err(format_err!("Could not find value with given name!"))
        }
    }

    fn set_value(&mut self, name: String, value: RcValue) -> ExecutionResult {
        if self.stack.get(&name).is_some() {
            self.stack.insert(name, value);
            return Ok(());
        }

        if let Some(ref context_mutex) = self.parent_context {
            context_mutex.lock().unwrap().set_value(name, value)
        } else {
            Err(format_err!("No such value exists!"))
        }
    }

    fn insert_value(&mut self, name: String, value: RcValue) {
        self.stack.insert(name, value);
    }

    pub fn run(
        &mut self,
        engine: &SyncMut<ExecutionEngine>,
        this_context: &SyncMut<ExecutionContext>,
    ) -> ExecutionResult {
        loop {
            if self.program_counter >= self.program.instructions.len() {
                self.ret(&engine, None)?;
                return Ok(());
            }

            let current_instruction: Inst = self
                .program
                .instructions
                .get(self.program_counter)
                .expect("Invalid PC!")
                .clone();

            match current_instruction {
                Inst::Alloc { name } => self.handle_alloc(name),
                Inst::PushInt { name, value } => self.handle_push_int(name, value)?,
                Inst::PushFloat { name, value } => self.handle_push_float(name, value)?,
                Inst::PushBoolean { name, value } => self.handle_push_boolean(name, value)?,
                Inst::PushFunction {
                    name,
                    argument_names,
                    instructions,
                } => self.handle_push_function(name, argument_names, instructions)?,
                Inst::PushList { name } => self.handle_push_list(name)?,
                Inst::PushObject { name } => self.handle_push_object(name)?,
                Inst::PopObjectValue {
                    pop_to_name,
                    object_name,
                    key_name,
                } => self.handle_pop_object_value(&pop_to_name, &object_name, &key_name)?,
                Inst::PushObjectValue {
                    object_name,
                    key_name,
                    value_name,
                } => self.handle_push_object_value(&object_name, &key_name, &value_name)?,
                Inst::Call {
                    name,
                    arguments,
                    this,
                } => {
                    let should_continue =
                        self.handle_call(&engine, this_context.clone(), &name, &arguments, this)?;
                    if should_continue {
                        continue;
                    } else {
                        return Ok(());
                    }
                }
                Inst::PushCallResult { name } => self.handle_push_call_result(name)?,
                Inst::Label { name } => self.handle_label(&name),
                Inst::GoTo { name } => {
                    self.handle_goto(&name)?;
                    continue;
                }
                Inst::Branch {
                    name,
                    true_label,
                    false_label,
                } => {
                    self.handle_branch(&name, true_label, false_label)?;
                    continue;
                }
                Inst::Return { name } => {
                    self.handle_return(&engine, &name)?;
                    return Ok(());
                }
            }

            self.program_counter += 1
        }

        Ok(())
    }

    fn handle_alloc(&mut self, name: String) {
        self.insert_value(name, Value::Nothing.into());
    }

    fn handle_push_int(&mut self, name: String, value: i64) -> ExecutionResult {
        self.set_value(name, Value::Integer(value).into())
    }

    fn handle_push_float(&mut self, name: String, value: f64) -> ExecutionResult {
        self.set_value(name, Value::Float(value).into())
    }

    fn handle_push_boolean(&mut self, name: String, value: bool) -> ExecutionResult {
        self.set_value(name, Value::Boolean(value).into())
    }

    fn handle_push_function(
        &mut self,
        name: String,
        argument_names: Arc<Vec<String>>,
        instructions: Arc<Vec<Inst>>,
    ) -> ExecutionResult {
        self.set_value(
            name,
            Value::InterpretedFunction(InterpretedFunctionData::from_argument_and_instructions(
                argument_names,
                instructions,
            ))
            .into(),
        )
    }

    fn handle_push_list(&mut self, name: String) -> ExecutionResult {
        self.set_value(name, Value::List(Mutex::new(Vec::new())).into())
    }

    fn handle_push_object(&mut self, name: String) -> ExecutionResult {
        self.set_value(name, Value::Object(Mutex::new(HashMap::new())).into())
    }

    fn handle_pop_object_value(
        &mut self,
        pop_to_name: &str,
        object_name: &str,
        key_name: &str,
    ) -> ExecutionResult {
        let object_value = self.get_value(&object_name)?;

        let popped_value = if let Value::Object(ref map) = *object_value {
            let map_lock = map.lock().unwrap();
            map_lock
                .get(key_name)
                .cloned()
                .unwrap_or_else(|| Value::Nothing.into())
        } else {
            unimplemented!("Handle methods of non-object values!");
        };

        self.set_value(pop_to_name.to_string(), popped_value)
    }

    fn handle_push_object_value(
        &mut self,
        object_name: &str,
        key_name: &str,
        value_name: &str,
    ) -> ExecutionResult {
        let object_value = self.get_value(&object_name)?;

        let target_value = self.get_value(&value_name)?;

        if let Value::Object(ref map) = *object_value {
            let mut map_lock = map.lock().unwrap();
            map_lock.insert(key_name.to_string(), target_value);
            Ok(())
        } else {
            Err(format_err!("Selected object is no object at all!"))
        }
    }

    fn handle_call(
        &mut self,
        engine: &SyncMut<ExecutionEngine>,
        this_context: SyncMut<ExecutionContext>,
        name: &str,
        arguments: &Arc<Vec<String>>,
        this: Option<String>,
    ) -> Result<bool, Error> {
        let call_value = self.get_value(&name)?;

        let this_value_option: Option<RcValue> = if let Some(this_value_name) = this {
            self.get_value(&this_value_name)?.into()
        } else {
            None
        };

        self.program_counter += 1;
        if let Value::InterpretedFunction(ref interpreted_function) = *call_value {
            let mut engine_lock = engine.lock().unwrap();
            let function_clone = interpreted_function.clone();
            let mut new_context = ExecutionContext::from_interpreted_function_call(
                function_clone,
                this_context.into(),
            );
            for (i, pass_value_name) in arguments.iter().enumerate() {
                if let Some(argument_name) = interpreted_function.argument_names.get(i) {
                    let pass_value = self.get_value(pass_value_name)?;
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
            return Ok(false);
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
            Ok(true)
        } else {
            Err(format_err!("Value called wasn't a function!"))
        }
    }

    fn handle_push_call_result(&mut self, name: String) -> ExecutionResult {
        let call_result = replace(&mut self.call_result, Value::Nothing.into());
        self.set_value(name, call_result)
    }

    fn handle_label(&mut self, _name: &str) {
        // nada
    }

    fn handle_goto(&mut self, name: &str) -> ExecutionResult {
        let pc_option = self.program.label_points.get(name);
        if let Some(pc) = pc_option {
            self.program_counter = *pc;
            Ok(())
        } else {
            Err(format_err!("Invalid label to goto!"))
        }
    }

    fn handle_branch(
        &mut self,
        name: &str,
        true_label: Option<String>,
        false_label: Option<String>,
    ) -> ExecutionResult {
        let value = self.get_value(&name)?;

        let bool_value = match *value {
            Value::Boolean(ref bool_value) => *bool_value,
            _ => return Err(format_err!("Value to branch on is not a boolean!")),
        };

        if true_label.is_none() && false_label.is_none() {
            return Err(format_err!("There is no labels to branch to!"));
        }

        if bool_value {
            match true_label {
                Some(label) => self.handle_goto(&label)?,
                None => self.program_counter += 1,
            }
        } else {
            match false_label {
                Some(label) => self.handle_goto(&label)?,
                None => self.program_counter += 1,
            }
        }

        Ok(())
    }

    fn ret(
        &mut self,
        engine: &SyncMut<ExecutionEngine>,
        return_value_option: Option<RcValue>,
    ) -> ExecutionResult {
        let mut engine_lock = engine.lock().unwrap();
        if let Some(ref context) = self.parent_context {
            let mut context_lock = context.lock().unwrap();
            context_lock.call_result = if let Some(return_value) = return_value_option {
                return_value
            } else {
                Value::Nothing.into()
            };

            engine_lock.tasks.push_back(context.clone())
        }

        Ok(())
    }

    fn handle_return(&mut self, engine: &SyncMut<ExecutionEngine>, name: &str) -> ExecutionResult {
        let return_value = self.get_value(name)?;
        self.ret(engine, return_value.into())
    }
}
