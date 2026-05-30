use std::{net::SocketAddr, sync::Arc};

use tokio::{net::UdpSocket, sync::Mutex};

use crate::{
    object::{
        Object, ObjectRef, error::error_type::ErrorType, native_object::NativeObject,
        new_objectref, state::StateRef,
    },
    scheduler::TOKIO_RUNTIME,
};

#[derive(Debug)]
pub struct ATCPSocketWrapper {
    pub reader: Arc<Mutex<tokio::net::tcp::OwnedReadHalf>>,
    pub writer: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
    pub addr: SocketAddr,
}

#[derive(Debug)]
pub struct ATCPSocketListenerWrapper {
    pub listener: Arc<tokio::net::TcpListener>,
    pub port: u16,
    pub addr: String,
}

#[derive(Debug)]
pub struct AUDPSocketWrapper {
    pub socket: Arc<tokio::net::UdpSocket>,
    pub port: u16,
    pub addr: String,
}

impl ATCPSocketListenerWrapper {
    pub fn new(port: u16, addr: String, state: StateRef) -> Result<Self, ObjectRef> {
        let listener = match std::net::TcpListener::bind(format!("{}:{}", addr, port)) {
            Ok(listener) => listener,
            Err(e) => {
                return Err(new_objectref(Object::new_error(
                    ErrorType::SocketBind,
                    format!("Failed to bind TCP server: {}", e),
                    state,
                )));
            }
        };

        match listener.set_nonblocking(true) {
            Ok(_) => (),
            Err(e) => {
                return Err(new_objectref(Object::new_error(
                    ErrorType::SocketBind,
                    format!("Failed to set TCP listener to non-blocking: {}", e),
                    state,
                )));
            }
        }

        let tokio_listener = TOKIO_RUNTIME.with(|slot| {
            let rt = slot.borrow();

            rt.block_on(async {
                match tokio::net::TcpListener::from_std(listener) {
                    Ok(tokio_listener) => Ok(tokio_listener),
                    Err(e) => Err(new_objectref(Object::new_error(
                        ErrorType::SocketBind,
                        format!("Failed to create Tokio TCP listener: {}", e),
                        state,
                    ))),
                }
            })
        })?;

        Ok(Self {
            listener: Arc::new(tokio_listener),
            port,
            addr,
        })
    }

    pub fn type_name(&self) -> String {
        "<native object 'ATCPSocketListener'>".into()
    }

    pub fn inspect(&self) -> String {
        format!(
            "[ATCPSocketListenerWrapper for {}:{}]",
            self.addr, self.port
        )
    }
}

impl ATCPSocketWrapper {
    pub fn new_with_connect(addr: String, port: u16, state: StateRef) -> Result<Self, ObjectRef> {
        let stream = match std::net::TcpStream::connect(format!("{}:{}", addr, port)) {
            Ok(stream) => stream,
            Err(e) => {
                return Err(new_objectref(Object::new_error(
                    ErrorType::SocketConnect,
                    format!("Failed to connect TCP socket: {}", e),
                    state,
                )));
            }
        };

        let peer_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                return Err(new_objectref(Object::new_error(
                    ErrorType::SocketConnect,
                    format!("Failed to connect TCP socket: {}", e),
                    state,
                )));
            }
        };

        match stream.set_nonblocking(true) {
            Ok(_) => (),
            Err(e) => {
                return Err(new_objectref(Object::new_error(
                    ErrorType::SocketConnect,
                    format!("Failed to connect TCP socket: {}", e),
                    state,
                )));
            }
        }

        let tokio_stream = TOKIO_RUNTIME.with(|slot| {
            let rt = slot.borrow();

            rt.block_on(async {
                match tokio::net::TcpStream::from_std(stream) {
                    Ok(tokio_stream) => Ok(tokio_stream),
                    Err(e) => Err(new_objectref(Object::new_error(
                        ErrorType::SocketConnect,
                        format!("Failed to create Tokio TCP stream: {}", e),
                        state,
                    ))),
                }
            })
        })?;

        let (reader, writer) = tokio_stream.into_split();

        Ok(Self {
            addr: peer_addr,
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        })
    }

    pub fn new_with_stream(stream: tokio::net::TcpStream, addr: SocketAddr) -> Self {
        let (reader, writer) = stream.into_split();

        Self {
            addr,
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        }
    }
    pub fn to_objecref(self) -> ObjectRef {
        new_objectref(Object::Native(Box::new(NativeObject::ATCPSocket(self))))
    }

    pub fn type_name(&self) -> String {
        "<native object 'ATCPSocket'>".into()
    }

    pub fn inspect(&self) -> String {
        format!(
            "[ATCPSocketWrapper for reader {:?}, writer {:?}]",
            self.reader, self.writer
        )
    }
}

impl AUDPSocketWrapper {
    pub fn new(address: &str, port: u16, state: StateRef) -> Result<Self, ObjectRef> {
        let tokio_socket = TOKIO_RUNTIME.with(|slot| {
            let rt = slot.borrow();

            rt.block_on(async {
                match UdpSocket::bind(format!("{}:{}", address, port)).await {
                    Ok(tokio_listener) => Ok(tokio_listener),
                    Err(e) => Err(new_objectref(Object::new_error(
                        ErrorType::SocketBind,
                        format!("Failed to create Tokio TCP listener: {}", e),
                        state,
                    ))),
                }
            })
        })?;

        Ok(Self {
            addr: address.to_string(),
            socket: Arc::new(tokio_socket),
            port,
        })
    }

    pub fn type_name(&self) -> String {
        "<native object 'AUDPSocket'>".into()
    }

    pub fn inspect(&self) -> String {
        format!("[AUDPSocket, binded to: {}:{}]", self.addr, self.port)
    }
}

impl PartialEq for AUDPSocketWrapper {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for AUDPSocketWrapper {}

impl Clone for AUDPSocketWrapper {
    fn clone(&self) -> Self {
        todo!();
    }
}

impl PartialEq for ATCPSocketWrapper {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialEq for ATCPSocketListenerWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port && self.addr == other.addr
    }
}

impl Eq for ATCPSocketWrapper {}
impl Eq for ATCPSocketListenerWrapper {}

impl Clone for ATCPSocketWrapper {
    fn clone(&self) -> Self {
        todo!();
    }
}

impl Clone for ATCPSocketListenerWrapper {
    fn clone(&self) -> Self {
        todo!();
    }
}
