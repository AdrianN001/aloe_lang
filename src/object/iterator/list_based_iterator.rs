use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef};


#[derive(PartialEq, Eq, Clone)]
pub struct ListBasedIterator{
    pub list: Vec<ObjectRef>,
    pub index: usize,
}


impl ListBasedIterator{

    pub fn _has_next(&self) -> ObjectRef{
        Rc::new(RefCell::new(Object::get_native_boolean_object(
            self.index < self.list.len()
        )))
    }
    pub fn _next(&mut self) -> Option<ObjectRef>{
        let current_index = self.index;

        if current_index >= self.list.len(){
            return None;
        }

        self.index += 1;

        Some(self.list[current_index].clone())
    }
}
