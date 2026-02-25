use crate::object::Object;

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Bool(bool_obj) => bool_obj.value,
            Object::Int(int_obj) => int_obj.value != 0,
            Object::FloatObj(float_obj) => float_obj.val.to_bits() != 0.0_f64.to_bits(),
            Object::Null(_) => false,
            _ => false,
        }
    }
}
