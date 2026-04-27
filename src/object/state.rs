use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::object::{
    future::task::TaskRef,
    state::{call_frame::CallFrame, scheduler::Scheduler},
};

mod call_frame;
pub mod scheduler;

pub type StateRef = Rc<RefCell<InterpreterState>>;

pub const DEFAULT_INTERPRETER_STATE: InterpreterState = InterpreterState {
    stack: Vec::new(),
    scheduler: Scheduler {
        queue: VecDeque::new(),
        current_task: None,
        sleeping: Vec::new(),
    },
};

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct InterpreterState {
    pub stack: Vec<CallFrame>,
    pub scheduler: Scheduler,
}

impl InterpreterState {
    pub fn push_to_stack(&mut self, stack_name: String) {
        self.stack.push(CallFrame { name: stack_name });
    }

    pub fn is_function_context(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn pop_from_stack(&mut self) {
        self.stack.pop();
    }

    pub fn add_to_scheduler(&mut self, task: TaskRef) {
        self.scheduler.queue.push_back(task);
    }

    pub fn collect_as_stack_trace(&self) -> Vec<String> {
        self.stack
            .iter()
            .map(|call_frame| call_frame.name.clone())
            .collect()
    }
}
