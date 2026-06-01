use std::{cell::RefCell, net::SocketAddr, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
    buffer::Buffer,
    error::{error_type::ErrorType, panic_type::PanicType},
    integer::Integer,
    native_object::a_network::ATCPSocketWrapper,
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::InterpreterState,
    string_obj::StringObj,
};

pub enum MessageOutput {
    BinaryData(Vec<u8>),
    PlainText(String),
    Integer(i64),

    UDPReceiveFrom(Vec<u8>, SocketAddr),

    EstablishedConnectionFromAsyncAccept(ATCPSocketWrapper),

    Error((ErrorType, String, String)),
    Panic((PanicType, String, String)),
}

impl MessageOutput {
    pub fn to_objectref(self) -> Result<ObjectRef, RuntimeSignal> {
        match self {
            MessageOutput::PlainText(text) => {
                Ok(new_objectref(Object::String(Box::new(StringObj {
                    value: text.clone(),
                }))))
            }
            MessageOutput::BinaryData(binary_data) => {
                Ok(Self::convert_bytearr_to_objectref(binary_data))
            }
            MessageOutput::Integer(value) => Ok(new_objectref(Object::Int(Integer { value }))),
            MessageOutput::EstablishedConnectionFromAsyncAccept(connection_stream) => {
                Ok(connection_stream.to_objecref())
            }
            MessageOutput::Error((errortype, message, origin)) => {
                let mut new_state = InterpreterState::default();
                new_state.push_to_stack(origin, 0);

                let state_ref = Rc::new(RefCell::new(new_state));

                let error = Object::new_error(errortype, message, state_ref);

                Ok(new_objectref(error))
            }
            MessageOutput::Panic((panictype, message, origin)) => {
                let mut new_state = InterpreterState::default();
                new_state.push_to_stack(origin, 0);

                let state_ref = Rc::new(RefCell::new(new_state));

                let panic = PanicObj::new(panictype, message, state_ref);

                Err(RuntimeSignal::Panic(panic))
            }
            MessageOutput::UDPReceiveFrom(buffer, addr) => {
                let buffer_obj = new_objectref(Object::Buffer(Box::new(Buffer {
                    size: buffer.len(),
                    data: buffer.into_boxed_slice(),
                })));

                let addr_str = new_objectref(Object::String(Box::new(StringObj {
                    value: addr.to_string(),
                })));

                Ok(new_objectref(Object::Array(Box::new(Array {
                    items: vec![buffer_obj, addr_str],
                }))))
            }
        }
    }

    fn convert_bytearr_to_objectref(data: Vec<u8>) -> ObjectRef {
        let size = data.len();
        let arr = data.into_boxed_slice();

        new_objectref(Object::Buffer(Box::new(Buffer { data: arr, size })))
    }
}
