use crate::object::{Object, ObjectRef, float_obj::FloatObj, new_objectref};

use rand::Rng;

pub fn random_builtin_function() -> ObjectRef {
    let random_float = rand::thread_rng().gen_range(0.0..1.0);

    new_objectref(Object::FloatObj(FloatObj { val: random_float }))
}
