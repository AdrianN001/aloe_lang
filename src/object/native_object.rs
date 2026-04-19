pub mod file;
pub mod path;

use crate::object::{
    ObjectRef,
    native_object::{file::FileWrapper, path::PathWrapper},
    panic_obj::PanicObj,
    state::StateRef,
};

#[derive(PartialEq, Eq, Clone)]
pub enum NativeObject {
    File(Box<FileWrapper>),
    Path(PathWrapper),
}

impl NativeObject {
    pub fn get_type(&self) -> String {
        match self {
            NativeObject::File(file) => file.type_name(),
            NativeObject::Path(path) => path.type_name(),
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            NativeObject::File(file) => file.inspect(),
            NativeObject::Path(path) => path.inspect(),
        }
    }

    pub fn as_bool(&self) -> ObjectRef {
        match self {
            NativeObject::File(file) => file.get_is_open(),
            NativeObject::Path(path) => path.exists(),
        }
    }

    pub fn as_bool_raw(&self) -> bool {
        match self {
            NativeObject::File(file) => file.get_is_open_raw(),
            NativeObject::Path(path) => path.exists_raw(),
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
            NativeObject::Path(path) => path.apply_method(name, args, state),
        }
    }

    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match self {
            NativeObject::File(file) => file.apply_attribute(name, state),
            NativeObject::Path(path) => path.apply_attribute(name, state),
        }
    }
}
