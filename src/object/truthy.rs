use crate::object::Object;

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Bool(bool_obj) => bool_obj.value,
            Object::Int(int_obj) => int_obj.value.is_positive(),
            Object::FloatObj(float_obj) => float_obj.val.is_sign_positive(),
            Object::String(str) => !str.value.is_empty(),
            Object::Array(arr) => !arr.items.is_empty(),
            Object::HashMap(hmap) => !hmap.pairs.is_empty(),
            Object::Iterator(iterator) => iterator._has_next_raw(),
            Object::Null(_) | Object::Err(_) => false,
            _ => false,
        }
    }
}
