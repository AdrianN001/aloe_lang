use std::hash::{DefaultHasher, Hash, Hasher};

use crate::object::hashable::Hashable;
use crate::object::hashmap::HashKey;


#[derive(PartialEq, Eq, Clone)]
pub struct StringObj{
    pub value: String
}


impl StringObj{
    pub fn get_type(&self) -> String{
        "string".into()
    }

    pub fn inspect(&self) -> String{
        format!("\"{}\"", &self.value)
    }
}


impl Hashable for StringObj{
    fn hash(&self) -> HashKey{
        let mut hasher = DefaultHasher::new();
        self.value.hash(&mut hasher);

        HashKey { 
            obj_type: self.get_type(), 
            value: hasher.finish()
        }
    }
}
