use crate::object::{hashable::Hashable, hashmap::HashKey};


#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub struct Boolean{
    pub value: bool
}

impl Boolean{
    pub fn get_type(&self) -> String{
        "boolean".into()
    }

    pub fn inspect(&self) -> String{
        self.value.to_string()
    }
}

impl Hashable for Boolean{
    fn hash(&self) -> HashKey{
        HashKey { 
            obj_type: self.get_type(), 
            value: if self.value{ 1 } else { 0 } 
        }
    }
}
