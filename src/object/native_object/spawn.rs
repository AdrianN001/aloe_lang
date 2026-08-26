mod atcp_listenerwrapper;
mod atcp_socketwrapper;
mod audp_socketwrapper;
mod command_wrapper;
mod filewrapper;
mod pathwrapper;
mod tcp_listenerwrapper;
mod tcp_socketwrapper;
mod udp_socketwrapper;

use crate::object::{
    ObjectRef,
    error::panic_type::PanicType,
    native_object::{
        a_network::{ATCPSocketListenerWrapper, ATCPSocketWrapper, AUDPSocketWrapper},
        file::FileWrapper,
        network::{TCPSocketListenerWrapper, TCPSocketWrapper, UDPSocketWrapper},
        path::PathWrapper,
        process::CommandWrapper,
    },
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

pub trait Spawnable {
    fn spawn(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, RuntimeSignal>;
}

pub fn generate_native_object(
    name: &str,
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    match name {
        "File" => FileWrapper::spawn(args, state),
        "Path" => PathWrapper::spawn(args, state),

        "TCPListener" => TCPSocketListenerWrapper::spawn(args, state),
        "TCPSocket" => TCPSocketWrapper::spawn(args, state),
        "UDPSocket" => UDPSocketWrapper::spawn(args, state),

        "ATCPListener" => ATCPSocketListenerWrapper::spawn(args, state),
        "ATCPSocket" => ATCPSocketWrapper::spawn(args, state),
        "AUDPSocket" => AUDPSocketWrapper::spawn(args, state),

        "Command" => CommandWrapper::spawn(args, state),

        other => Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!("unexpected native object name: '{}'", other),
            state,
        ))),
    }
}
