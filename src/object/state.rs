use std::{cell::RefCell, rc::Rc};

use crate::object::state::call_frame::CallFrame;

mod call_frame;

pub type StateRef = Rc<RefCell<InterpreterState>>;

pub const DEFAULT_INTERPRETER_STATE: InterpreterState = InterpreterState { stack: Vec::new() };

#[derive(PartialEq, Eq, Clone, Default)]
pub struct InterpreterState {
    pub stack: Vec<CallFrame>,
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

    pub fn collect_as_stack_trace(&self) -> Vec<String> {
        self.stack
            .iter()
            .map(|call_frame| call_frame.name.clone())
            .collect()
    }
}
