use std::hash::{DefaultHasher, Hash, Hasher};

use crate::object::{hashable::Hashable, hashmap::HashKey};

#[derive(Clone)]
pub struct FloatObj {
    pub val: f64,
}

impl FloatObj {
    pub fn get_type(&self) -> String {
        "float".into()
    }

    pub fn inspect(&self) -> String {
        self.val.to_string()
    }
}

impl Hashable for FloatObj {
    fn hash(&self) -> HashKey {
        let mut hasher = DefaultHasher::new();
        self.val.to_bits().hash(&mut hasher);

        HashKey {
            obj_type: self.get_type(),
            value: hasher.finish(),
        }
    }
}

impl PartialEq for FloatObj {
    fn eq(&self, other: &FloatObj) -> bool {
        self.val.to_bits() == other.val.to_bits()
    }
}

impl Eq for FloatObj {}
