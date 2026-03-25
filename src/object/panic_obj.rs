use std::fmt;

use crate::object::{
    error::{Error, panic_type::PanicType},
    state::StateRef,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PanicObj {
    pub value: String,
    pub state: StateRef,
    pub panic_type: PanicType,
}

impl PanicObj {
    pub fn new(type_of: PanicType, value: String, state: StateRef) -> Self {
        Self {
            value,
            panic_type: type_of,
            state: state.clone(),
        }
    }
    pub fn new_simple(type_of: PanicType, value: &str, state: StateRef) -> Self {
        Self {
            panic_type: type_of,
            value: value.into(),
            state: state.clone(),
        }
    }
    pub fn from_error(error: &Error, state: StateRef) -> Self {
        Self {
            panic_type: PanicType::Propagation,
            value: error.value.to_string(),
            state: state.clone(),
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

impl fmt::Display for PanicObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.clone())
    }
}
