use crate::object::{
    Object, ObjectRef, buffer::Buffer, error::panic_type::PanicType, new_objectref,
    panic_obj::PanicObj, state::StateRef,
};

impl Buffer {
    pub fn add(&self, right: ObjectRef, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Buffer(buffer) => {
                let first_buffer = &self.data;
                let second_buffer = &buffer.data;

                let big_buffer = [&**first_buffer, &**second_buffer]
                    .concat()
                    .into_boxed_slice();

                Ok(new_objectref(Object::Buffer(Box::new(Buffer {
                    data: big_buffer,
                    size: first_buffer.len() + second_buffer.len(),
                }))))
            }

            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "buffer",
                    "+",
                    other_type.get_type()
                ),
                state.clone(),
            )),
        }
    }
}
