use std::collections::BTreeMap;

use crate::object::ObjectRef;

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

        buffer.push_str(
            &self
                .pairs
                .values()
                .map(|v| v.key.borrow().inspect() + ":" + &v.value.borrow().inspect())
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
