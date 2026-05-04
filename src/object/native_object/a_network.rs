use std::{net::SocketAddr, sync::{Arc}};

use tokio::sync::Mutex;

use crate::object::{Object, ObjectRef, error::panic_type::PanicType, native_object::NativeObject, new_objectref, panic_obj::{PanicObj, RuntimeSignal}, state::{StateRef, scheduler::TOKIO_RUNTIME}};


#[derive( Debug)]
pub struct ATCPSocketWrapper {
    pub stream: Arc<Mutex<tokio::net::TcpStream>>,
    pub addr: SocketAddr,
}

#[derive(Debug)]
pub struct ATCPSocketListenerWrapper {
    pub listener: Arc<tokio::net::TcpListener>,
    pub port: u16,
    pub addr: String,

}


impl ATCPSocketListenerWrapper{
    pub fn new(port: u16, addr: String, state: StateRef) -> Result<Self, RuntimeSignal> {
        let listener = match std::net::TcpListener::bind(format!("{}:{}", addr, port)) {
            Ok(listener) => listener,
            Err(e) => return Err(RuntimeSignal::Panic(PanicObj::new( PanicType::SocketBind, format!("Failed to bind TCP listener: {}", e), state))),
        };

        match listener.set_nonblocking(true) {
            Ok(_) => (),
            Err(e) => return Err(RuntimeSignal::Panic(PanicObj::new( PanicType::SocketBind, format!("Failed to set TCP listener to non-blocking: {}", e), state))),
        }

        let tokio_listener = TOKIO_RUNTIME.with(|slot| {
            let rt = slot.borrow();

            rt.block_on(async {
                match tokio::net::TcpListener::from_std(listener) {
                    Ok(tokio_listener) => Ok(tokio_listener),
                    Err(e) => Err(RuntimeSignal::Panic(PanicObj::new( PanicType::SocketBind, format!("Failed to create Tokio TCP listener: {}", e), state))),
                }
            })
        })?;


        Ok(Self { listener: Arc::new(tokio_listener), port, addr })
    }

    pub fn type_name(&self) -> String {
        "<native object 'ATCPSocketListener'>".into()
    }

    pub fn inspect(&self) -> String {
        format!("[ATCPSocketListenerWrapper for {}:{}]", self.addr, self.port)
    }
}

impl ATCPSocketWrapper {

    pub fn new_with_connect(addr: String, port: u16, state: StateRef) -> Result<Self, RuntimeSignal> {
        let stream = match std::net::TcpStream::connect(format!("{}:{}", addr, port)) {
            Ok(stream) => stream,
            Err(e) => return Err(RuntimeSignal::Panic(PanicObj::new( PanicType::SocketBind, format!("Failed to connect TCP socket: {}", e), state))),
        };

        let peer_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => return Err(RuntimeSignal::Panic(PanicObj::new( PanicType::SocketBind, format!("Failed to get peer address: {}", e), state))),
        };

        match stream.set_nonblocking(true){
            Ok(_) => (),
            Err(e) => return Err(RuntimeSignal::Panic(PanicObj::new( PanicType::SocketBind, format!("Failed to set TCP socket to non-blocking: {}", e), state))),
        }

        let tokio_stream = TOKIO_RUNTIME.with(|slot| {
            let rt = slot.borrow();

            rt.block_on(async {
                match tokio::net::TcpStream::from_std(stream) {
                    Ok(tokio_stream) => Ok(tokio_stream),
                    Err(e) => Err(RuntimeSignal::Panic(PanicObj::new( PanicType::SocketBind, format!("Failed to create Tokio TCP stream: {}", e), state))),
                }
            })
        })?;

        Ok(Self { stream: Arc::new(Mutex::new(tokio_stream)), addr: peer_addr })
    }

    pub fn to_objecref(self) -> ObjectRef {
        new_objectref(Object::Native(NativeObject::ATCPSocket(self)))
    }

    pub fn type_name(&self) -> String {
        "<native object 'ATCPSocket'>".into()
    }

    pub fn inspect(&self) -> String {
        format!("[ATCPSocketWrapper for stream {:?}]", self.stream)
    }
}

impl PartialEq for ATCPSocketWrapper {
    fn eq(&self, other: &Self) -> bool {
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