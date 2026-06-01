use std::{cell::RefCell, rc::Rc};

use crate::object::state::call_frame::CallFrame;

mod call_frame;

pub type StateRef = Rc<RefCell<InterpreterState>>;

#[derive(Debug)]
pub struct InterpreterState {
    pub stack: Vec<CallFrame>,
    pub current_line: usize,
}
impl InterpreterState {
    pub fn push_to_stack(&mut self, stack_name: String, line: usize) {
        self.stack.push(CallFrame {
            name: stack_name,
            function_call_line: line,
        });
    }

    pub fn set_current_line(&mut self, line: usize) {
        self.current_line = line;
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
            .rev()
            .map(|call_frame| {
                format!(
                    "{} at line {}",
                    call_frame.name.clone(),
                    call_frame.function_call_line
                )
            })
            .collect()
    }
}

impl Default for InterpreterState {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            current_line: 0,
        }
    }
}

impl PartialEq for InterpreterState {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for InterpreterState {}
