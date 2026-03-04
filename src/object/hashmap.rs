use std::collections::{BTreeMap, HashSet};

use crate::object::{Object, ObjectRef};

#[derive(PartialEq, Eq, Clone)]
pub struct HashMap {
    pub pairs: BTreeMap<HashKey, HashPair>,
}

impl HashMap {
    pub fn get_type(&self) -> String {
        "hash map".into()
    }

    pub fn inspect(&self) -> String {
        let mut buffer = String::new();
        buffer.push('{');

        let current_ptr = self as *const _ as usize;
        let mut visited = HashSet::new();
        visited.insert(current_ptr);

        buffer.push_str(
            &self
                .pairs
                .values()
                .map(|v| {
                    v.key.borrow().inspect()
                        + ":"
                        + &match &*v.value.borrow() {
                            Object::HashMap(hashmap) => hashmap.inspect_with_memory(&mut visited),
                            other => other.inspect(),
                        }
                })
                .collect::<Vec<String>>()
                .join(", "),
        );

        buffer.push('}');
        buffer
    }

    fn inspect_with_memory(&self, visited: &mut HashSet<usize>) -> String {
        let current_ptr = self as *const _ as usize;

        if visited.get(&current_ptr).is_some() {
            return "{...}".into();
        } else {
            visited.insert(current_ptr);
        }

        let mut buffer = String::new();
        buffer.push('{');

        buffer.push_str(
            &self
                .pairs
                .values()
                .map(|v| {
                    v.key.borrow().inspect()
                        + ":"
                        + &match &*v.value.borrow() {
                            Object::HashMap(hashmap) => hashmap.inspect_with_memory(visited),
                            other => other.inspect(),
                        }
                })
                .collect::<Vec<String>>()
                .join(", "),
        );

        buffer.push('}');
        buffer
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct HashKey {
    pub obj_type: String,
    pub value: u64,
}

#[derive(PartialEq, Eq, Clone)]
pub struct HashPair {
    pub key: ObjectRef,
    pub value: ObjectRef,
}
