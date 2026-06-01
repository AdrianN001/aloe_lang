#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CallFrame {
    pub name: String,
    pub function_call_line: usize,
}
