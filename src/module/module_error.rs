use core::fmt;

#[derive(Debug)]
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

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nError in file: '{}' -> {}\n",
            self.module_name, self.value
        )
    }
}
