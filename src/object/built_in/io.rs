use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    native_object::{NativeObject, a_network::{ATCPSocketListenerWrapper, ATCPSocketWrapper}, file::FileWrapper, network::{TCPSocketListenerWrapper, TCPSocketWrapper}, path::PathWrapper},
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

pub fn open_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    match args.len() {
        1 => {
            // open("file_name");
            let file_name_borrow = args[0].borrow();

            let file_name_raw = match &*file_name_borrow {
                Object::String(str) => str.value.clone(),
                other_type => {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "unexpected parameter type for __open(). Expected: 'str', got: '{}'",
                            other_type.get_type()
                        ),
                        state,
                    )));
                }
            };

            let mode = "r";

            let wrapper = match FileWrapper::new(file_name_raw, mode) {
                Ok(wrapper) => wrapper,
                Err(err_feedback) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::FileOpen,
                        err_feedback,
                        state,
                    )));
                }
            };

            Ok(new_objectref(Object::Native(NativeObject::File(Box::new(
                wrapper,
            )))))
        }

        2 => {
            // open("file_name", "mode");
            let file_name_borrow = args[0].borrow();
            let mode_borrow = args[1].borrow();

            let file_name_raw = match &*file_name_borrow {
                Object::String(str) => str.value.clone(),
                other_type => {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "unexpected parameter type for __open(). Expected: 'str', got: '{}'",
                            other_type.get_type()
                        ),
                        state,
                    )));
                }
            };

            let mode_raw = match &*mode_borrow {
                Object::String(str) => str.value.clone(),
                other_type => {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "unexpected parameter type for __open(). Expected: 'str', got: '{}'",
                            other_type.get_type()
                        ),
                        state,
                    )));
                }
            };

            let wrapper = match FileWrapper::new(file_name_raw, &mode_raw) {
                Ok(wrapper) => wrapper,
                Err(err_feedback) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::FileOpen,
                        err_feedback,
                        state,
                    )));
                }
            };

            Ok(new_objectref(Object::Native(NativeObject::File(Box::new(
                wrapper,
            )))))
        }
        other_n_of_args => Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!(
                "unexpected number of parameter for __open(). Expected: 1 or 2, got: '{}'",
                other_n_of_args
            ),
            state,
        ))),
    }
}

pub fn path_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!(
                "unexpected number of parameter for __path(). Expected: 1, got: '{}'",
                args.len()
            ),
            state,
        )));
    }

    let arg_borrow = args[0].borrow();

    let path_arg = match &*arg_borrow {
        Object::String(str_obj) => &str_obj.value,
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unexpected parameter type for __path(). Expected: 'str', got: '{}'",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let wrapper = match PathWrapper::new(path_arg) {
        Ok(wrapper) => wrapper,
        Err(err_feedback) => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::PathResolve,
                err_feedback,
                state,
            )));
        }
    };

    Ok(new_objectref(Object::Native(NativeObject::Path(wrapper))))
}


pub fn tcp_bind_builtin_function(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, RuntimeSignal>{

    if args.len() != 2{
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!(
                "unexpected number of parameter for __tcp_bind(). Expected: 2, got: '{}'",
                args.len()
            ),
            state,
        )));
    }

    let addr = match &*args[0].borrow() {
        Object::String(str_obj) => str_obj.value.to_string(),
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unexpected parameter type for __tcp_bind() address parameter. Expected: 'str', got: '{}'",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let port = match &*args[1].borrow() {
        Object::Int(int_obj) => int_obj.value,
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unexpected parameter type for __tcp_bind() port parameter. Expected: 'int', got: '{}'",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let port_as_u16 = match u16::try_from(port) {
        Ok(port_u16) => port_u16,
        Err(_) => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "port number out of range for __tcp_bind(). Expected: 0-65535, got: '{}'",
                    port
                ),
                state,
            )));
        }
    };

    let wrapper = TCPSocketListenerWrapper::new(port_as_u16, addr, state)?;

    Ok(new_objectref(Object::Native(NativeObject::TCPListener(wrapper))))
}

pub fn tcp_connect_builtin_function(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, RuntimeSignal>{
    if args.len() != 2{
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!(
                "unexpected number of parameter for __tcp_connect(). Expected: 2, got: '{}'",
                args.len()
            ),
            state,
        )));
    }

    let addr = match &*args[0].borrow() {
        Object::String(str_obj) => str_obj.value.to_string(),
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unexpected parameter type for __tcp_connect() address parameter. Expected: 'str', got: '{}'",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let port = match &*args[1].borrow() {
        Object::Int(int_obj) => int_obj.value,
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unexpected parameter type for __tcp_connect() port parameter. Expected: 'int', got: '{}'",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let port_as_u16 = match u16::try_from(port) {
        Ok(port_u16) => port_u16,
        Err(_) => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "port number out of range for __tcp_connect(). Expected: 0-65535, got: '{}'",
                    port
                ),
                state,
            )));
        }
    };

    let wrapper = TCPSocketWrapper::new_with_connect(addr, port_as_u16, state)?;

    Ok(new_objectref(Object::Native(NativeObject::TCPSocket(wrapper))))
}


pub fn async_tcp_bind_builtin_function(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, RuntimeSignal>{
    if args.len() != 2{
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!(
                "unexpected number of parameter for __async_tcp_bind(). Expected: 2, got: '{}'",
                args.len()
            ),
            state,
        )));
    }

    let addr = match &*args[0].borrow() {
        Object::String(str_obj) => str_obj.value.to_string(),
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unexpected parameter type for __async_tcp_bind() address parameter. Expected: 'str', got: '{}'",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let port = match &*args[1].borrow() {
        Object::Int(int_obj) => int_obj.value,
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unexpected parameter type for __async_tcp_bind() port parameter. Expected: 'int', got: '{}'",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let port_as_u16 = match u16::try_from(port) {
        Ok(port_u16) => port_u16,
        Err(_) => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "port number out of range for __async_tcp_bind(). Expected: 0-65535, got: '{}'",
                    port
                ),
                state,
            )));
        }
    };

    let wrapper = ATCPSocketListenerWrapper::new(port_as_u16, addr, state)?;

    Ok(new_objectref(Object::Native(NativeObject::ATCPListener(wrapper))))
}

pub fn async_tcp_connect_builtin_function(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, RuntimeSignal>{
    if args.len() != 2{
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!(
                "unexpected number of parameter for __async_tcp_connect(). Expected: 2, got: '{}'",
                args.len()
            ),
            state,
        )));
    }

    let addr = match &*args[0].borrow() {
        Object::String(str_obj) => str_obj.value.to_string(),
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unexpected parameter type for __async_tcp_connect() address parameter. Expected: 'str', got: '{}'",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let port = match &*args[1].borrow() {
        Object::Int(int_obj) => int_obj.value,
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "unexpected parameter type for __async_tcp_connect() port parameter. Expected: 'int', got: '{}'",
                    other_type.get_type()
                ),
                state,
            )));
        }
    };

    let port_as_u16 = match u16::try_from(port) {
        Ok(port_u16) => port_u16,
        Err(_) => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "port number out of range for __async_tcp_connect(). Expected: 0-65535, got: '{}'",
                    port
                ),
                state,
            )));
        }
    };

    let wrapper = ATCPSocketWrapper::new_with_connect(addr, port_as_u16, state)?;

    Ok(new_objectref(Object::Native(NativeObject::ATCPSocket(wrapper))))
}