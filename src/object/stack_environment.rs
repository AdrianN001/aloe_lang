use std::collections::HashMap;

use crate::object::Object;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct StackEnvironment{
    map: HashMap<String, Object>
}

impl StackEnvironment{
    pub fn new() -> Self{
        StackEnvironment { 
            map: HashMap::new()  
        }
    }

    pub fn set(&mut self, identifier: &str, value: Object){
        self.map.insert(identifier.into(), value);
    }

    pub fn get(&self, identifier: &str) -> Option<&Object>{
        self.map.get(identifier)
    }

    pub fn get_owned(&self, identifier: &str) -> Option<Object>{
        match self.map.get(identifier){
            Some(obj) => Some(obj.clone()),
            None => None,
        }
    }
}


