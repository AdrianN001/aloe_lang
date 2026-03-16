use crate::object::state::StateRef;

#[derive(PartialEq, Eq, Clone)]
pub struct Error {
    pub value: String,
    pub state: StateRef,
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

        buffer.push_str("\nError: ");
        buffer.push_str(&self.value);

        buffer
    }

    pub fn inspect_message(&self) -> String {
        self.value.clone()
    }
}
