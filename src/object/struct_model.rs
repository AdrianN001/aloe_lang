use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::ObjectRef;

pub type MethodTableRef = Rc<RefCell<HashMap<String, ObjectRef>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct StructModel {
    pub name: String,

    pub attributes: Vec<String>,
    pub methods: MethodTableRef,
}

impl StructModel {
    pub fn get_type(&self) -> String {
        "<type struct>".to_string()
    }

    pub fn inspect(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("struct ");
        buffer.push_str(&self.name);
        buffer.push('{');

        if !self.attributes.is_empty() {
            buffer.push('\n');
        }
        self.attributes.iter().for_each(|attribute| {
            buffer.push('\t');
            buffer.push_str(attribute);
            buffer.push(',');
            buffer.push('\n');
        });
        buffer.push('}');
        buffer.push(';');

        buffer
    }
}
