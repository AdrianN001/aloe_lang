pub mod error_type;
pub mod panic_type;

use crate::object::{error::error_type::ErrorType, state::StateRef};

#[derive(Clone, Debug)]
pub struct Error {
    pub value: String,
    pub state: StateRef,
    pub type_of: ErrorType,
}

impl Error {
    pub fn get_type(&self) -> String {
        "error".into()
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

        buffer.push('\n');
        buffer.push_str(&format!("{:?}Error: {}", self.type_of, self.value));
        buffer
    }

    pub fn inspect_message(&self) -> String {
        self.value.clone()
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.type_of == other.type_of && self.value == other.value
    }
}

impl Eq for Error {}
