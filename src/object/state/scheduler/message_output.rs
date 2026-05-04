use crate::object::{
    Object, ObjectRef, array::Array, error::panic_type::PanicType, integer::Integer,
    native_object::a_network::ATCPSocketWrapper, new_objectref, panic_obj::RuntimeSignal,
    string_obj::StringObj,
};

pub enum MessageOutput {
    BinaryData(Vec<u8>),
    PlainText(String),
    Integer(i64),

    EstablishedConnectionFromAsyncAccept(ATCPSocketWrapper),

    Panic((PanicType, String)),
}

impl MessageOutput {
    pub fn to_objectref(self) -> Result<ObjectRef, RuntimeSignal> {
        match self {
            MessageOutput::PlainText(text) => Ok(new_objectref(Object::String(StringObj {
                value: text.clone(),
            }))),
            MessageOutput::BinaryData(binary_data) => {
                Ok(Self::convert_bytearr_to_objectref(&binary_data))
            }
            MessageOutput::Integer(value) => Ok(new_objectref(Object::Int(Integer { value }))),
            MessageOutput::EstablishedConnectionFromAsyncAccept(connection_stream) => {
                Ok(connection_stream.to_objecref())
            }

            _ => todo!(),
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

        new_objectref(Object::Array(Array { items: arr }))
    }
}
