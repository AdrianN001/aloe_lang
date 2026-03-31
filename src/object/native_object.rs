pub mod file;

use crate::object::{
    ObjectRef, native_object::file::FileWrapper, panic_obj::PanicObj, state::StateRef,
};

#[derive(PartialEq, Eq, Clone)]
pub enum NativeObject {
    File(FileWrapper),
}

impl NativeObject {
    pub fn get_type(&self) -> String {
        match self {
            NativeObject::File(file) => file.type_name(),
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            NativeObject::File(file) => file.inspect(),
        }
    }

    pub fn as_bool(&self) -> ObjectRef {
        match self {
            NativeObject::File(file) => file.get_is_open(),
        }
    }

    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match self {
            NativeObject::File(file) => file.apply_method(name, args, state),
        }
    }

    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match self {
            NativeObject::File(file) => file.apply_attribute(name, state),
        }
    }
}
