use std::{
    io::{Read, Write},
    sync::Arc,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};

use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    future::{FutureObj, future_kind::FutureKind, future_state::FutureState},
    integer::Integer,
    native_object::a_network::{ATCPSocketListenerWrapper, ATCPSocketWrapper},
    new_objectref,
    panic_obj::PanicObj,
    state::{
        StateRef,
        scheduler::{
            SCHEDULER_CHANNEL, TOKIO_RUNTIME, add_io_future, message_output::MessageOutput,
        },
    },
    string_obj::StringObj,
};

impl ATCPSocketListenerWrapper {
    pub fn apply_method(
        &mut self,
        name: &str,
        _args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "accept" => self.accept(state),
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

    pub fn accept(&self, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        let future = new_objectref(Object::Future(FutureObj::new(FutureState::Pending(
            FutureKind::IO,
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
                        let socket_wrapper = ATCPSocketWrapper {
                            stream: Arc::new(Mutex::new(stream)),
                            addr,
                        };
                        let _ = tx.send((
                            future_id,
                            MessageOutput::EstablishedConnectionFromAsyncAccept(socket_wrapper),
                        ));
                    }
                    Err(e) => {
                        let _ = tx.send((
                            future_id,
                            MessageOutput::Panic((
                                PanicType::SocketAccept,
                                format!("Failed to accept TCP connection: {}", e),
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
        new_objectref(Object::String(StringObj {
            value: self.addr.clone(),
        }))
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
            "read" => self.read(state),
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

    pub fn read(&mut self, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        let future = new_objectref(Object::Future(FutureObj::new(FutureState::Pending(
            FutureKind::IO,
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

        let stream = self.stream.clone();

        let tx = SCHEDULER_CHANNEL.with(|slot| slot.borrow().0.clone());

        TOKIO_RUNTIME.with(|slot| {
            let runtime = slot.borrow();

            runtime.spawn(async move {
                let mut buf = vec![0u8; 1024];
                let mut stream_lock = stream.lock().await;
                match stream_lock.read(&mut buf).await {
                    Ok(bytes_read) => {
                        buf.truncate(bytes_read);
                        let _ = tx.send((future_id, MessageOutput::BinaryData(buf.to_vec())));
                    }
                    Err(e) => {
                        let _ = tx.send((
                            future_id,
                            MessageOutput::Panic((
                                PanicType::SocketRead,
                                format!("Failed to read from TCP socket: {}", e),
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

        let data = match &*args[0].borrow() {
            Object::Array(arr) => arr
                .items
                .iter()
                .map(|item| match &*item.borrow() {
                    Object::Int(i) => Ok(i.value as u8),
                    _ => Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        "write expects an array of integers (bytes)".into(),
                        state.clone(),
                    )),
                })
                .collect::<Result<Vec<u8>, PanicObj>>()?,
            _ => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    "write expects an array of integers (bytes)".into(),
                    state,
                ));
            }
        };

        let future = new_objectref(Object::Future(FutureObj::new(FutureState::Pending(
            FutureKind::IO,
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

        let stream = self.stream.clone();

        let tx = SCHEDULER_CHANNEL.with(|slot| slot.borrow().0.clone());

        TOKIO_RUNTIME.with(|slot| {
            let runtime = slot.borrow();

            runtime.spawn(async move {
                let mut stream_lock = stream.lock().await;
                match stream_lock.write_all(&data).await {
                    Ok(_) => {
                        let _ = tx.send((future_id, MessageOutput::Integer(data.len() as i64)));
                    }
                    Err(e) => {
                        let _ = tx.send((
                            future_id,
                            MessageOutput::Panic((
                                PanicType::SocketWrite,
                                format!("Failed to write to TCP socket: {}", e),
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
        new_objectref(Object::String(StringObj {
            value: self.addr.to_string(),
        }))
    }

    pub fn to_bool(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(true))
    }

    pub fn to_bool_raw(&self) -> bool {
        true
    }
}
