use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
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
                Ok(Self::convert_bytearr_to_objectref(&binary_data))
            }
            MessageOutput::Integer(value) => Ok(new_objectref(Object::Int(Integer { value }))),
            MessageOutput::EstablishedConnectionFromAsyncAccept(connection_stream) => {
                Ok(connection_stream.to_objecref())
            }
            MessageOutput::Error((errortype, message, origin)) => {
                let mut new_state = InterpreterState::default();
                new_state.push_to_stack(origin);

                let state_ref = Rc::new(RefCell::new(new_state));

                let error = Object::new_error(errortype, message, state_ref);

                Ok(new_objectref(error))
            }
            MessageOutput::Panic((panictype, message, origin)) => {
                let mut new_state = InterpreterState::default();
                new_state.push_to_stack(origin);

                let state_ref = Rc::new(RefCell::new(new_state));

                let panic = PanicObj::new(panictype, message, state_ref);

                Err(RuntimeSignal::Panic(panic))
            }
        }
    }

    fn convert_bytearr_to_objectref(data: &[u8]) -> ObjectRef {
        let arr = data
            .iter()
            .map(|byte| {
                new_objectref(Object::Int(Integer {
                    value: *byte as i64,
                }))
            })
            .collect();

        new_objectref(Object::Array(Box::new(Array { items: arr })))
    }
}
