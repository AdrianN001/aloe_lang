use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    object::{
        Object, ObjectRef,
        error::{error_type::ErrorType, panic_type::PanicType},
        future::{FutureObj, future_kind::FutureKind, future_state::FutureState},
        integer::Integer,
        native_object::a_network::{
            ATCPSocketListenerWrapper, ATCPSocketWrapper, AUDPSocketWrapper,
        },
        new_objectref,
        panic_obj::PanicObj,
        state::StateRef,
        string_obj::StringObj,
    },
    scheduler::{SCHEDULER_CHANNEL, TOKIO_RUNTIME, add_io_future, message_output::MessageOutput},
};

impl ATCPSocketListenerWrapper {
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "accept" => self.accept(args, state),
            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("ATCPSocketListener has no method named '{}'", name),
                state,
            )),
        }
    }

    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "port" => Ok(self.get_port()),
            "addr" => Ok(self.get_addr()),
            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("ATCPSocketListener has no attribute named '{}'", name),
                state,
            )),
        }
    }
}

impl ATCPSocketListenerWrapper {
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
        let future = new_objectref(Object::Future(Box::new(FutureObj::new(
            FutureState::Pending(FutureKind::IO),
        ))));

        let future_id = {
            let future_borrow = future.borrow();
            if let Object::Future(future_obj) = &*future_borrow {
                future_obj.get_id()
            } else {
                panic!("Expected a Future object");
            }
        };

        add_io_future(future_id, future.clone());

        let listener = self.listener.clone();

        let tx = SCHEDULER_CHANNEL.with(|slot| slot.borrow().0.clone());

        TOKIO_RUNTIME.with(|slot| {
            let runtime = slot.borrow();

            runtime.spawn(async move {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        let socket_wrapper = ATCPSocketWrapper::new_with_stream(stream, addr);
                        let _ = tx.send((
                            future_id,
                            MessageOutput::EstablishedConnectionFromAsyncAccept(socket_wrapper),
                        ));
                    }
                    Err(e) => {
                        let _ = tx.send((
                            future_id,
                            MessageOutput::Error((
                                ErrorType::SocketAccept,
                                format!("Failed to accept TCP connection: {}", e),
                                String::from("NonBlockingTCPListener::accept"),
                            )),
                        ));
                    }
                }
            })
        });

        Ok(future)
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
}

impl ATCPSocketWrapper {
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "read" => self.read(args, state),
            "write" => self.write(args, state),
            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("ATCPSocket has no method named '{}'", name),
                state,
            )),
        }
    }

    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "port" => Ok(self.get_port()),
            "addr" => Ok(self.get_addr()),
            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("ATCPSocket has no attribute named '{}'", name),
                state,
            )),
        }
    }
}

impl ATCPSocketWrapper {
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
        let future = new_objectref(Object::Future(Box::new(FutureObj::new(
            FutureState::Pending(FutureKind::IO),
        ))));

        let future_id = {
            let future_borrow = future.borrow();
            if let Object::Future(future_obj) = &*future_borrow {
                future_obj.get_id()
            } else {
                panic!("Expected a Future object");
            }
        };

        add_io_future(future_id, future.clone());

        let reader = self.reader.clone();

        let tx = SCHEDULER_CHANNEL.with(|slot| slot.borrow().0.clone());

        TOKIO_RUNTIME.with(|slot| {
            let runtime = slot.borrow();

            runtime.spawn(async move {
                let mut buf = vec![0u8; 1024];
                let mut reader_lock = reader.lock().await;
                match (*reader_lock).read(&mut buf).await {
                    Ok(bytes_read) => {
                        buf.truncate(bytes_read);
                        let _ = tx.send((future_id, MessageOutput::BinaryData(buf.to_vec())));
                    }
                    Err(e) => {
                        let _ = tx.send((
                            future_id,
                            MessageOutput::Error((
                                ErrorType::SocketRead,
                                format!("Failed to read from TCP socket: {}", e),
                                String::from("NonBlockingTCPConn::read"),
                            )),
                        ));
                    }
                }
            })
        });

        Ok(future)
    }

    pub fn write(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                "write expects exactly 1 argument".into(),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();
        let data = match &*arg_borrow {
            Object::Buffer(buffer) => buffer.data.to_vec(),
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!("write expects buffer, got: {}", other_type.get_type()),
                    state,
                ));
            }
        };

        let future = new_objectref(Object::Future(Box::new(FutureObj::new(
            FutureState::Pending(FutureKind::IO),
        ))));

        let future_id = {
            let future_borrow = future.borrow();
            if let Object::Future(future_obj) = &*future_borrow {
                future_obj.get_id()
            } else {
                panic!("Expected a Future object");
            }
        };

        add_io_future(future_id, future.clone());

        let writer = self.writer.clone();

        let tx = SCHEDULER_CHANNEL.with(|slot| slot.borrow().0.clone());

        TOKIO_RUNTIME.with(|slot| {
            let runtime = slot.borrow();

            runtime.spawn(async move {
                let mut writer_lock = writer.lock().await;
                match (*writer_lock).write_all(&data).await {
                    Ok(_) => {
                        let _ = tx.send((future_id, MessageOutput::Integer(data.len() as i64)));
                    }
                    Err(e) => {
                        let _ = tx.send((
                            future_id,
                            MessageOutput::Error((
                                ErrorType::SocketWrite,
                                format!("Failed to write to TCP socket: {}", e),
                                String::from("NonBlockingTCPConn::write"),
                            )),
                        ));
                    }
                }
            })
        });

        Ok(future)
    }

    // attributes

    pub fn get_port(&self) -> ObjectRef {
        new_objectref(Object::Int(Integer {
            value: self.addr.port() as i64,
        }))
    }

    pub fn get_addr(&self) -> ObjectRef {
        new_objectref(Object::String(Box::new(StringObj {
            value: self.addr.to_string(),
        })))
    }

    pub fn to_bool(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(true))
    }

    pub fn to_bool_raw(&self) -> bool {
        true
    }
}

impl AUDPSocketWrapper {
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "send_to_async" => self.send_to(args, state),
            "recv_from_async" => self.recv_from(args, state),
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
            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("AUDPSocket has no attribute named '{}'", name),
                state,
            )),
        }
    }
}

impl AUDPSocketWrapper {
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

        let addr = match AUDPSocketWrapper::parse_socket_addr(&addr_raw) {
            Ok(addr) => addr,
            Err(err_feedback) => {
                return Ok(new_objectref(Object::new_error(
                    ErrorType::IllegalAddress,
                    err_feedback.to_string(),
                    state,
                )));
            }
        };

        let socket = self.socket.clone();
        let buffer_content = msg.data.to_vec();

        let future = new_objectref(Object::Future(Box::new(FutureObj::new(
            FutureState::Pending(FutureKind::IO),
        ))));

        let future_id = {
            let future_borrow = future.borrow();
            if let Object::Future(future_obj) = &*future_borrow {
                future_obj.get_id()
            } else {
                panic!("Expected a Future object");
            }
        };

        add_io_future(future_id, future.clone());

        let tx = SCHEDULER_CHANNEL.with(|slot| slot.borrow().0.clone());

        TOKIO_RUNTIME.with(|slot| {
            let runtime = slot.borrow();

            runtime.spawn(async move {
                match socket.send_to(&buffer_content, addr).await {
                    Ok(bytes_sent) => {
                        let _ = tx.send((future_id, MessageOutput::Integer(bytes_sent as i64)));
                    }
                    Err(e) => {
                        let _ = tx.send((
                            future_id,
                            MessageOutput::Error((
                                ErrorType::SocketWrite,
                                format!("Failed to write to UDP socket: {}", e),
                                String::from("NonBlockingUDPConn::send_to"),
                            )),
                        ));
                    }
                }
            })
        });

        Ok(future)
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

        let socket = self.socket.clone();

        let future = new_objectref(Object::Future(Box::new(FutureObj::new(
            FutureState::Pending(FutureKind::IO),
        ))));

        let future_id = {
            let future_borrow = future.borrow();
            if let Object::Future(future_obj) = &*future_borrow {
                future_obj.get_id()
            } else {
                panic!("Expected a Future object");
            }
        };

        add_io_future(future_id, future.clone());

        let tx = SCHEDULER_CHANNEL.with(|slot| slot.borrow().0.clone());

        TOKIO_RUNTIME.with(|slot| {
            let runtime = slot.borrow();

            runtime.spawn(async move {
                let mut buf = vec![0u8; 1024];
                match socket.recv_from(&mut buf).await {
                    Ok((bytes, socket_addr)) => {
                        buf.truncate(bytes);
                        let _ =
                            tx.send((future_id, MessageOutput::UDPReceiveFrom(buf, socket_addr)));
                    }
                    Err(e) => {
                        let _ = tx.send((
                            future_id,
                            MessageOutput::Error((
                                ErrorType::SocketRead,
                                format!("Failed to read from UDP socket: {}", e),
                                String::from("NonBlockingUDPConn::recv_from"),
                            )),
                        ));
                    }
                }
            })
        });

        Ok(future)
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
