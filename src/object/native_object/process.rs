use std::process::Command;

#[derive(Debug)]
pub struct CommandWrapper {
    pub native_cmd: Command,

    pub base_cmd: String,
}

impl CommandWrapper {
    pub fn new(cmd: &str) -> Self {
        let command = Command::new(cmd);
        Self {
            native_cmd: command,
            base_cmd: cmd.to_string(),
        }
    }

    pub fn inspect(&self) -> String {
        format!("[CommandWrapper for {:?}]", self.base_cmd)
    }

    pub fn type_name(&self) -> String {
        "<native object 'CommandWrapper'>".into()
    }
}

impl PartialEq for CommandWrapper {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Eq for CommandWrapper {}

impl Clone for CommandWrapper {
    fn clone(&self) -> Self {
        todo!()
    }
}
