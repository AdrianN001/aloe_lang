use crate::object::{
    Object, ObjectRef, error::panic_type::PanicType, float_obj::FloatObj, integer::Integer,
    native_object::NativeObject, new_objectref, panic_obj::PanicObj, state::StateRef,
};

impl NativeObject {
    pub fn bool(&self) -> Result<ObjectRef, PanicObj> {
        Ok(self.as_bool())
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::Native(native_obj_raw) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self == native_obj_raw,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::Native(native_obj_raw) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self != native_obj_raw,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(true)))
    }
}
