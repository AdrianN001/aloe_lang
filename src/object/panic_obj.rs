use std::fmt;

use crate::{object::{error::Error, state::StateRef}};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PanicObj {
pub value: String,
    pub state: StateRef,
}

impl PanicObj {
    pub fn new(value: String, state: StateRef) -> Self{
        Self{
            value,
            state: state.clone()
        }
    }
    pub fn new_simple(value: &str, state: StateRef) -> Self{
        Self{
            value: value.into(),
            state: state.clone()
        }
    }
    pub fn from_error(error: &Error, state: StateRef) -> Self{
        Self{
            value: error.value.to_string(),
            state: state.clone()
        }
    }
    
    pub fn inspect(&self) -> String {
        let mut buffer = String::new();
        let state_borrow = self.state.borrow();

        buffer.push_str("Stack trace:");
        buffer.push_str("\n\t at ");
        if state_borrow.stack.is_empty() {
            buffer.push_str("<global>");
        } else {
            buffer.push_str(&state_borrow.collect_as_stack_trace().join("\n\t at "));
        }

        buffer.push_str("\nPanic: ");
        buffer.push_str(&self.value);

        buffer
    }

    pub fn inspect_message(&self) -> String {
        self.value.clone()
    }
}


impl fmt::Display for PanicObj{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}", self.value.clone())
    }
}
