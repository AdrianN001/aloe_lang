use crate::object::Object;




impl Object{

    pub fn is_truthy(&self) -> bool{
        match self{
            Object::Bool(bool_obj) => bool_obj.value,
            Object::Int(int_obj) => int_obj.value != 0,
            Object::Null(_) => false,
            _ => false
        }
    }
} 
