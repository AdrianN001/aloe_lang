use std::{
    io::{Read, Write},
    net::SocketAddr,
};

use crate::object::{
    Object, ObjectRef,
    array::Array,
    buffer::Buffer,
    error::{error_type::ErrorType, panic_type::PanicType},
    integer::Integer,
    native_object::{
        NativeObject,
        network::{TCPSocketListenerWrapper, TCPSocketWrapper, UDPSocketWrapper},
    },
    new_objectref,
    panic_obj::PanicObj,
    state::StateRef,
    string_obj::StringObj,
};

impl TCPSocketListenerWrapper {
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "accept" => self.accept(args, state),
            "set_nonblocking" => self.set_nonblocking(args, state),
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
            )),
        }
    }
}

impl TCPSocketListenerWrapper {
    // methods

    pub fn accept(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "listener.accept() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        match self.listener.accept() {
            Ok((stream, addr)) => {
                let wrapper = TCPSocketWrapper { stream, addr };
                Ok(new_objectref(Object::Native(Box::new(
                    NativeObject::TCPSocket(wrapper),
                ))))
            }
            Err(e) => Ok(new_objectref(Object::new_error(
                ErrorType::SocketAccept,
                format!("Failed to accept connection: {}", e),
                state,
            ))),
        }
    }

    pub fn set_nonblocking(
        &mut self,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
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
                ));
            }
        };

        match self.listener.set_nonblocking(blocking) {
            Ok(_) => {
                self.nonblocking = blocking;
                Ok(new_objectref(Object::NULL_OBJECT))
            }
            Err(e) => Ok(new_objectref(Object::new_error(
                ErrorType::NonBlockChange,
                format!("Failed to set non-blocking mode: {}", e),
                state,
            ))),
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
        new_objectref(Object::Int(Integer {
            value: self.port as i64,
        }))
    }

    pub fn get_addr(&self) -> ObjectRef {
        new_objectref(Object::String(Box::new(StringObj {
            value: self.addr.clone(),
        })))
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
            "read" => self.read(args, state),
            "write" => self.write(args, state),
            "close" => self.close(args, state),
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
            )),
        }
    }
}

impl TCPSocketWrapper {
    // methods

    pub fn read(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "socket.read() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        let mut buffer = [0; 1024];
        match self.stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let data = buffer[..bytes_read].to_vec().into_boxed_slice();
                let size = bytes_read;

                Ok(new_objectref(Object::Buffer(Box::new(Buffer {
                    size,
                    data,
                }))))
            }
            Err(e) => Ok(new_objectref(Object::new_error(
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
            Object::Buffer(buffer) => buffer.data.to_vec(),
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!("write expects an buffer, got: {}", other_type.get_type()),
                    state,
                ));
            }
        };

        match self.stream.write(&data) {
            Ok(bytes_written) => Ok(new_objectref(Object::Int(Integer {
                value: bytes_written as i64,
            }))),
            Err(e) => Ok(new_objectref(Object::new_error(
                ErrorType::SocketWrite,
                format!("Failed to write to socket: {}", e),
                state,
            ))),
        }
    }

    pub fn close(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "socket.close() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        match self.stream.shutdown(std::net::Shutdown::Both) {
            Ok(_) => Ok(new_objectref(Object::NULL_OBJECT)),
            Err(e) => Ok(new_objectref(Object::new_error(
                ErrorType::SocketClose,
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

impl UDPSocketWrapper {
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "send_to" => self.send_to(args, state),
            "recv_from" => self.recv_from(args, state),
            "set_nonblocking" => self.set_nonblocking(args, state),
            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("AUDPSocket has no method named '{}'", name),
                state,
            )),
        }
    }

    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "port" => Ok(self.get_port()),
            "addr" => Ok(self.get_addr()),
            "is_nonblocking" => Ok(self.get_nonblocking()),
            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("AUDPSocket has no attribute named '{}'", name),
                state,
            )),
        }
    }
}

impl UDPSocketWrapper {
    // attributes

    pub fn get_port(&self) -> ObjectRef {
        new_objectref(Object::Int(Integer {
            value: self.port as i64,
        }))
    }

    pub fn get_addr(&self) -> ObjectRef {
        new_objectref(Object::String(Box::new(StringObj {
            value: self.addr.clone(),
        })))
    }

    pub fn get_nonblocking(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(self.nonblocking))
    }

    // methods

    pub fn send_to(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 2 arguments for AUDPSocket.send_to(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let msg = match &*args[0].borrow() {
            Object::Buffer(buff) => buff.clone(),
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected buffer as the first argument for AUDPSocket.send_to(), got: '{}'",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };

        let addr_raw = match &*args[1].borrow() {
            Object::String(str) => str.value.clone(),
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected string as the second argument for AUDPSocket.send_to(), got: '{}'",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };

        let addr = match UDPSocketWrapper::parse_socket_addr(&addr_raw) {
            Ok(addr) => addr,
            Err(err_feedback) => {
                return Ok(new_objectref(Object::new_error(
                    ErrorType::IllegalAddress,
                    err_feedback.to_string(),
                    state,
                )));
            }
        };

        let buffer_content = msg.data;

        let bytes_sent = match self.socket.send_to(&buffer_content, addr) {
            Ok(bytes_written) => Ok(new_objectref(Object::Int(Integer {
                value: bytes_written as i64,
            }))),
            Err(e) => Ok(new_objectref(Object::new_error(
                ErrorType::SocketWrite,
                format!("Failed to write to socket: {}", e),
                state,
            ))),
        };

        bytes_sent
    }

    fn recv_from(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 0 arguments for AUDPSocket.recv_from(), got: '{}'",
                    args.len()
                ),
                state,
            ));
        }

        let mut buff = vec![0u8; 1024];
        let (bytes_read, addr) = match self.socket.recv_from(&mut buff) {
            Ok((bytes, addr)) => {
                buff.truncate(bytes);
                (bytes, addr)
            }
            Err(err) => {
                return Ok(new_objectref(Object::new_error(
                    ErrorType::SocketRead,
                    err.to_string(),
                    state,
                )));
            }
        };

        let buffer_obj = new_objectref(Object::Buffer(Box::new(Buffer {
            size: bytes_read,
            data: buff.into_boxed_slice(),
        })));
        let addr = new_objectref(Object::String(Box::new(StringObj {
            value: addr.to_string(),
        })));

        Ok(new_objectref(Object::Array(Box::new(Array {
            items: vec![buffer_obj, addr],
        }))))
    }

    pub fn set_nonblocking(
        &mut self,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
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
                ));
            }
        };

        match self.socket.set_nonblocking(blocking) {
            Ok(_) => {
                self.nonblocking = blocking;
                Ok(new_objectref(Object::NULL_OBJECT))
            }
            Err(e) => Ok(new_objectref(Object::new_error(
                ErrorType::NonBlockChange,
                format!("Failed to set non-blocking mode: {}", e),
                state,
            ))),
        }
    }

    pub fn to_bool_raw(&self) -> bool {
        true
    }

    pub fn to_bool(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(true))
    }

    fn parse_socket_addr(input: &str) -> Result<SocketAddr, std::net::AddrParseError> {
        input.parse()
    }
}
