pub struct ModuleError {
    pub value: String,
    pub module_name: String,
}

impl ModuleError {
    pub fn new(name: &str, error: &str) -> Self {
        Self {
            value: error.to_string(),
            module_name: name.to_string(),
        }
    }
}
