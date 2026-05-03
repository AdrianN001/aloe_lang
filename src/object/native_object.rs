pub mod file;
pub mod path;
pub mod network;

use crate::object::{
    ObjectRef,
    native_object::{file::FileWrapper, network::{TCPSocketListenerWrapper, TCPSocketWrapper}, path::PathWrapper},
    panic_obj::PanicObj,
    state::StateRef,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum NativeObject {
    File(Box<FileWrapper>),
    Path(PathWrapper),

    TCPListener(TCPSocketListenerWrapper),
    TCPSocket(TCPSocketWrapper),
}

impl NativeObject {
    pub fn get_type(&self) -> String {
        match self {
            NativeObject::File(file) => file.type_name(),
            NativeObject::Path(path) => path.type_name(),
            NativeObject::TCPListener(listener) => listener.type_name(),
            NativeObject::TCPSocket(socket) => socket.type_name(),
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            NativeObject::File(file) => file.inspect(),
            NativeObject::Path(path) => path.inspect(),
            NativeObject::TCPListener(listener) => listener.inspect(),
            NativeObject::TCPSocket(socket) => socket.inspect(),
        }
    }

    pub fn as_bool(&self) -> ObjectRef {
        match self {
            NativeObject::File(file) => file.get_is_open(),
            NativeObject::Path(path) => path.exists(),
            NativeObject::TCPListener(listener) => listener.to_bool(),
            NativeObject::TCPSocket(socket) => socket.to_bool(),
            _ => panic!()
        }
    }

    pub fn as_bool_raw(&self) -> bool {
        match self {
            NativeObject::File(file) => file.get_is_open_raw(),
            NativeObject::Path(path) => path.exists_raw(),
            NativeObject::TCPListener(listener) => listener.to_bool_raw(),
            NativeObject::TCPSocket(socket) => !socket.is_closed(),

            _ => panic!()
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
            NativeObject::TCPListener(listener) => listener.apply_method(name, args, state),
            NativeObject::TCPSocket(socket) => socket.apply_method(name, args, state),
            _ => panic!()
        }
    }

    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match self {
            NativeObject::File(file) => file.apply_attribute(name, state),
            NativeObject::Path(path) => path.apply_attribute(name, state),
            NativeObject::TCPListener(listener) => listener.apply_attribute(name, state),
            NativeObject::TCPSocket(socket) => socket.apply_attribute(name, state),

            _ => panic!()
        }
    }
}
