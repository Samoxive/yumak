#![allow(dead_code)]

pub mod context;
pub mod stdlib;
pub mod value;

use crate::context::ExecutionContext;

use common::SyncMut;
use failure::Error;
use std::collections::VecDeque;

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
    }

    pub fn push_task(&mut self, task: SyncMut<ExecutionContext>) {
        self.tasks.push_back(task);
    }
}
