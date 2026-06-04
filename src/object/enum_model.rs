use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct EnumModel {
    pub name: String,

    pub values: HashMap<String, usize>,
}

impl EnumModel {
    pub fn get_type(&self) -> String {
        "<type enum>".to_string()
    }

    pub fn inspect(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("enum ");
        buffer.push_str(&self.name);
        buffer.push('{');

        if !self.values.is_empty() {
            buffer.push('\n');
        }
        self.values.keys().for_each(|key| {
            buffer.push('\t');
            buffer.push_str(key);
            buffer.push(';');
            buffer.push('\n');
        });
        buffer.push('}');
        buffer.push(';');

        buffer
    }
}
