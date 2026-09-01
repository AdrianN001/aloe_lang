use std::hash::{DefaultHasher, Hash, Hasher};

use crate::object::{hashable::Hashable, hashmap::HashKey};

#[derive(Clone, Debug)]
pub struct Float {
    pub val: f64,
}

impl Float {
    pub fn get_type(&self) -> String {
        "<type float>".into()
    }

    pub fn inspect(&self) -> String {
        self.val.to_string()
    }
}

impl Hashable for Float {
    fn hash(&self) -> HashKey {
        let mut hasher = DefaultHasher::new();
        self.val.to_bits().hash(&mut hasher);

        HashKey {
            obj_type: self.get_type(),
            value: hasher.finish(),
        }
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Float) -> bool {
        self.val.to_bits() == other.val.to_bits()
    }
}

impl Eq for Float {}
