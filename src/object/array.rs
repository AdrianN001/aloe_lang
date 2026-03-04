use std::collections::HashSet;

use crate::object::{Object, ObjectRef};

#[derive(PartialEq, Eq, Clone)]
pub struct Array {
    pub items: Vec<ObjectRef>,
}

impl Array {
    pub fn get_type(&self) -> String {
        "array".into()
    }

    pub fn inspect(&self) -> String {
        let mut buffer = String::new();

        buffer.push('[');

        let mut visited = HashSet::new();
        let current_ptr = self as *const _ as usize;

        visited.insert(current_ptr);

        buffer.push_str(
            &self
                .items
                .iter()
                .map(|item| match &*item.borrow() {
                    Object::Array(arr) => arr.inspect_with_memory(&mut visited),
                    other_type => other_type.inspect(),
                })
                .collect::<Vec<_>>()
                .join(", "),
        );

        buffer.push(']');

        buffer
    }

    fn inspect_with_memory(&self, visited: &mut HashSet<usize>) -> String {
        let current_ptr = self as *const _ as usize;

        if visited.get(&current_ptr).is_some() {
            return "[...]".to_string();
        } else {
            visited.insert(current_ptr);
        }

        let mut buffer = String::new();

        buffer.push('[');

        buffer.push_str(
            &self
                .items
                .iter()
                .map(|item| match &*item.borrow() {
                    Object::Array(arr) => arr.inspect_with_memory(visited),
                    other_type => other_type.inspect(),
                })
                .collect::<Vec<_>>()
                .join(", "),
        );

        buffer.push(']');

        buffer
    }
}
