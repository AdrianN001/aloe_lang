use crate::object::{hashable::Hashable, hashmap::HashKey};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Integer {
    pub value: i64,
}

impl Integer {
    pub fn get_type(&self) -> String {
        "integer".into()
    }

    pub fn inspect(&self) -> String {
        self.value.to_string()
    }
}

impl Hashable for Integer {
    fn hash(&self) -> HashKey {
        HashKey {
            obj_type: self.get_type(),
            value: self.value as u64,
        }
    }
}
