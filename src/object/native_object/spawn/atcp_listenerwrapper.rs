use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    native_object::{NativeObject, a_network::ATCPSocketListenerWrapper, spawn::Spawnable},
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

impl Spawnable for ATCPSocketListenerWrapper {
    fn spawn(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        if args.len() != 2 {
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

        let wrapper = match ATCPSocketListenerWrapper::new(port_as_u16, addr, state) {
            Ok(wrapper) => wrapper,
            Err(err_obj) => return Ok(err_obj),
        };

        Ok(new_objectref(Object::Native(Box::new(
            NativeObject::ATCPListener(wrapper),
        ))))
    }
}
