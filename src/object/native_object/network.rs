use std::net::SocketAddr;

use crate::object::{
    error::panic_type::PanicType,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

#[derive(Debug)]
pub struct TCPSocketWrapper {
    pub stream: std::net::TcpStream,
    pub addr: SocketAddr,
}

#[derive(Debug)]
pub struct TCPSocketListenerWrapper {
    pub listener: std::net::TcpListener,
    pub port: u16,
    pub addr: String,

    pub nonblocking: bool,
}

impl TCPSocketListenerWrapper {
    pub fn new(port: u16, addr: String, state: StateRef) -> Result<Self, RuntimeSignal> {
        let listener = match std::net::TcpListener::bind(format!("{}:{}", addr, port)) {
            Ok(listener) => listener,
            Err(e) => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::SocketBind,
                    format!("Failed to bind TCP listener: {}", e),
                    state,
                )));
            }
        };
        Ok(Self {
            listener,
            port,
            addr,
            nonblocking: false,
        })
    }

    pub fn type_name(&self) -> String {
        "<native object 'TCPSocketListener'>".into()
    }

    pub fn inspect(&self) -> String {
        format!("[TCPSocketListenerWrapper for {}:{}]", self.addr, self.port)
    }
}

impl TCPSocketWrapper {
    pub fn new_with_connect(
        addr: String,
        port: u16,
        state: StateRef,
    ) -> Result<Self, RuntimeSignal> {
        let stream = match std::net::TcpStream::connect(format!("{}:{}", addr, port)) {
            Ok(stream) => stream,
            Err(e) => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::SocketBind,
                    format!("Failed to connect TCP socket: {}", e),
                    state,
                )));
            }
        };
        let addr = stream
            .peer_addr()
            .unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], 0)));
        Ok(Self { stream, addr })
    }

    pub fn type_name(&self) -> String {
        "<native object 'TCPSocket'>".into()
    }

    pub fn inspect(&self) -> String {
        format!("[TCPSocketWrapper for stream {:?}]", self.stream)
    }
}

impl PartialEq for TCPSocketWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.stream.peer_addr().ok() == other.stream.peer_addr().ok()
    }
}

impl PartialEq for TCPSocketListenerWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port && self.addr == other.addr
    }
}

impl Eq for TCPSocketWrapper {}
impl Eq for TCPSocketListenerWrapper {}

impl Clone for TCPSocketWrapper {
    fn clone(&self) -> Self {
        // Note: Cloning a TcpStream creates a new handle to the same underlying socket
        TCPSocketWrapper {
            stream: self.stream.try_clone().expect("Failed to clone TCP stream"),
            addr: self.addr,
        }
    }
}

impl Clone for TCPSocketListenerWrapper {
    fn clone(&self) -> Self {
        // Note: Cloning a TcpListener creates a new handle to the same underlying socket
        let listener = self
            .listener
            .try_clone()
            .expect("Failed to clone TCP listener");
        TCPSocketListenerWrapper {
            listener,
            port: self.port,
            addr: self.addr.clone(),
            nonblocking: self.nonblocking,
        }
    }
}
