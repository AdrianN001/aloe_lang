use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    stack_environment::EnvRef,
    state::StateRef,
};

pub mod array;
pub mod file_wrapper;
pub mod float;
pub mod hashmap;
pub mod int;
pub mod iterator;
pub mod path_wrapper;
pub mod string;
pub mod network_wrapper;
pub mod a_network_wrapper;

impl Object {
    pub fn apply_attribute(
        &self,
        name: &str,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        if let Some(result) = self.check_early_attributes(name) {
            return Ok(result);
        }

        let result = match self {
            Object::String(str) => str.apply_attribute(name, state),
            Object::Array(arr) => arr.apply_attribute(name, environ, state),
            Object::Int(int) => int.apply_attribute(name, state),
            Object::FloatObj(float) => float.apply_attribute(name, state),
            Object::Iterator(iterator) => iterator.apply_attribute(name, state),
            Object::HashMap(hashmap) => hashmap.apply_attribute(name, environ, state),
            Object::Native(native) => native.apply_attribute(name, state),

            Object::StructObject(struct_obj) => struct_obj.apply_attribute(name, environ, state),

            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("{} has no attribute", self.get_type()),
                state,
            )),
        };

        match result {
            Ok(ok_value) => Ok(ok_value),
            Err(panic_value) => Err(RuntimeSignal::Panic(panic_value)),
        }
    }

    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let result = match self {
            Object::String(str) => str.apply_method(name, args, environ, state),
            Object::Array(arr) => arr.apply_method(name, args, environ, state),
            Object::Int(int) => int.apply_method(name, args, environ, state),
            Object::FloatObj(float) => float.apply_method(name, args, environ, state),
            Object::Iterator(iterator) => iterator.apply_method(name, args, environ, state),
            Object::HashMap(hashmap) => hashmap.apply_method(name, args, environ, state),
            Object::Native(native) => native.apply_method(name, args, state),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("{} has no methods", self.get_type()),
                state,
            )),
        };

        match result {
            Ok(ok_value) => Ok(ok_value),
            Err(panic_value) => Err(RuntimeSignal::Panic(panic_value)),
        }
    }

    fn check_early_attributes(&self, name: &str) -> Option<ObjectRef> {
        match name {
            "is_ok" => match &self {
                Object::Err(_) => Some(new_objectref(Object::get_native_boolean_object(false))),
                _ => Some(new_objectref(Object::get_native_boolean_object(true))),
            },
            "is_err" => match &self {
                Object::Err(_) => Some(new_objectref(Object::get_native_boolean_object(true))),
                _ => Some(new_objectref(Object::get_native_boolean_object(false))),
            },

            _ => None,
        }
    }
}
