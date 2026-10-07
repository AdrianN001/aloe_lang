use crate::{
    ast::syntax_error_report::syntax_error::SyntaxError,
    module::module_error::ModuleError,
    object::{
        ObjectRef,
        error::{Error, panic_type::PanicType},
        future::task::TaskRef,
        state::StateRef,
    },
    terminal::formatter::TerminalTextFormatter,
};

#[derive(Debug)]
pub enum RuntimeSignal {
    Panic(PanicObj),
    SyntaxError(SyntaxError),
    ModuleLoadError(ModuleError),
    GenericError(Box<dyn std::error::Error>),
    Yield(TaskRef),
    Propagation(ObjectRef),
    Return(ObjectRef),
    Break(ObjectRef),
    Continue,
    //TODO: Break(ObjectRef),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PanicObj {
    pub value: String,
    pub state: StateRef,
    pub panic_type: PanicType,

    tip: Option<String>,
}

impl PanicObj {
    pub fn new(type_of: PanicType, value: String, state: StateRef) -> Self {
        Self {
            value,
            panic_type: type_of,
            state: state.clone(),
            tip: None,
        }
    }
    pub fn new_simple(type_of: PanicType, value: &str, state: StateRef) -> Self {
        Self {
            panic_type: type_of,
            value: value.into(),
            state: state.clone(),
            tip: None,
        }
    }

    pub fn set_tip(mut self, tip: &str) -> Self {
        self.tip = Some(tip.into());
        self.clone()
    }

    pub fn from_error(error: &Error, state: StateRef) -> Self {
        Self {
            panic_type: PanicType::Propagation,
            value: error.value.to_string(),
            state: state.clone(),
            tip: None,
        }
    }

    pub fn inspect(&self) -> String {
        let mut buffer = String::new();
        let state_borrow = self.state.borrow();

        buffer.push_str(&TerminalTextFormatter::to_underlined("Stack trace:"));
        buffer.push_str("\n\t at ");
        if state_borrow.stack.is_empty() {
            buffer.push_str("<global>");
        } else {
            buffer.push_str(&state_borrow.collect_as_stack_trace().join("\n\t at "));
        }
        buffer.push('\n');
        buffer.push_str(&format!(
            "line {}, {:?}Panic: {}",
            TerminalTextFormatter::to_bold(&state_borrow.current_line.to_string()),
            self.panic_type,
            self.value
        ));

        buffer
    }

    pub fn inspect_message(&self) -> String {
        self.value.clone()
    }
}
