use std::io::{Read, Write};

use crate::object::{Object, ObjectRef, array::Array, error::{error_type::ErrorType, panic_type::PanicType}, integer::Integer, native_object::{NativeObject, network::{TCPSocketListenerWrapper, TCPSocketWrapper}}, new_objectref, panic_obj::PanicObj, state::{StateRef, scheduler::TOKIO_RUNTIME}, string_obj::StringObj};



impl TCPSocketListenerWrapper{
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "accept" => {
                self.accept(state)
            }
            "set_nonblocking" => {
                self.set_nonblocking(args, state)
            }
            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("TCPSocketListener has no method named '{}'", name),
                state,
            )),
        }
    }

    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        
        match name {
            "port" => Ok(self.get_port()),
            "addr" => Ok(self.get_addr()),
            "is_nonblocking" => Ok(self.get_is_nonblocking()),
            _ => Err(PanicObj::new(
            PanicType::UnknownAttribute,
            format!("TCPSocketListener has no attribute named '{}'", name),
            state,
            ))
        }
    }
}

impl TCPSocketListenerWrapper{
    // methods

    pub fn accept(&self, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match self.listener.accept() {
            Ok((stream, addr)) => {
                let wrapper = TCPSocketWrapper { stream, addr };
                Ok(new_objectref(Object::Native(NativeObject::TCPSocket(wrapper))))
            }
            Err(e) => Ok(new_objectref( Object::new_error(
                ErrorType::SocketAccept,
                format!("Failed to accept connection: {}", e),
                state,
            ))),
        }
    }

    pub fn set_nonblocking(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                "set_nonblocking expects exactly 1 argument".into(),
                state,
            ));
        }

        let blocking = match &*args[0].borrow() {
            Object::Bool(b) => b.value,
            _ => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    "set_nonblocking expects a boolean argument".into(),
                    state,
                ))
            }
        };

        match self.listener.set_nonblocking(blocking) {
            Ok(_) => {
                self.nonblocking = blocking;
                Ok(new_objectref(Object::NULL_OBJECT))
            },
            Err(e) => Err(PanicObj::new(
                PanicType::Listener,
                format!("Failed to set non-blocking mode: {}", e),
                state,
            )),
        }
    }

    pub fn to_bool_raw(&self) -> bool {
        true
    }

    pub fn to_bool(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(true))
    }

    // attributes

    pub fn get_port(&self) -> ObjectRef {
        new_objectref(Object::Int(Integer{value: self.port as i64}))
    }

    pub fn get_addr(&self) -> ObjectRef {
        new_objectref(Object::String(StringObj{value: self.addr.clone()}))
    }

    pub fn get_is_nonblocking(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(self.nonblocking))
    }

}

impl TCPSocketWrapper {
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "read" => self.read(state),
            "write" => self.write(args, state),
            "close" => self.close(state),
            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("TCPSocket has no method named '{}'", name),
                state,
            )),
        }
    }

    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        
        match name {
            _ => Err(PanicObj::new(
            PanicType::UnknownAttribute,
            format!("TCPSocket has no attribute named '{}'", name),
            state,
            ))
        }
    }
}

impl TCPSocketWrapper {
    // methods

    pub fn read(&mut self, state: StateRef) -> Result<ObjectRef, PanicObj> {
        let mut buffer = [0; 1024];
        match self.stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let data = buffer[..bytes_read].iter().map(|byte|{
                    new_objectref(Object::Int(Integer { value: *byte as i64 }))
                }).collect();
                Ok(new_objectref(Object::Array(Array{ items: data })))
            }
            Err(e) => Ok(new_objectref( Object::new_error(
                ErrorType::SocketRead,
                format!("Failed to read from socket: {}", e),
                state,
            ))),
        }
    }

    pub fn write(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                "write expects exactly 1 argument".into(),
                state,
            ));
        }

        let data = match &*args[0].borrow() {
            Object::Array(arr) => arr.items.iter().map(|item| {
                match &*item.borrow() {
                    Object::Int(i) => Ok(i.value as u8),
                    _ => Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        "write expects an array of integers (bytes)".into(),
                        state.clone(),
                    )),
                }
            }).collect::<Result<Vec<u8>, PanicObj>>()?,
            _ => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    "write expects an array of integers (bytes)".into(),
                    state,
                ))
            }
        };

        match self.stream.write(&data) {
            Ok(bytes_written) => Ok(new_objectref(Object::Int(Integer { value: bytes_written as i64 }))),
            Err(e) => Ok(new_objectref( Object::new_error(
                ErrorType::SocketRead,
                format!("Failed to write to socket: {}", e),
                state,
            ))),
        }
    }

    pub fn close(&mut self, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match self.stream.shutdown(std::net::Shutdown::Both) {
            Ok(_) => Ok(new_objectref(Object::NULL_OBJECT)),
            Err(e) => Ok(new_objectref( Object::new_error(
                ErrorType::SocketRead,
                format!("Failed to close socket: {}", e),
                state,
            ))),
        }
    }
    // attributes

    pub fn is_closed(&self) -> bool {
        self.stream.peer_addr().is_err()
    }

    pub fn to_bool(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(!self.is_closed()))
    }
}