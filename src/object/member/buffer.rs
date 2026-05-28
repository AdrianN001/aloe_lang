use crate::object::{
    Object, ObjectRef, array::Array, buffer::Buffer, error::panic_type::PanicType,
    integer::Integer, new_objectref, panic_obj::PanicObj, stack_environment::EnvRef,
    state::StateRef, string_obj::StringObj,
};

impl Buffer {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "length" => Ok(self.length()),

            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for buffer: '{}'", name),
                state,
            )),
        }
    }
    pub fn apply_method(
        &self,
        name: &str,
        args: &[ObjectRef],
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "clone" => self.clone(),
            "as_str" => self.to_str(state),
            "as_arr" => self.to_arr(),
            "slice" => self.slice(args, state),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for buffer: '{}'", name),
                state,
            )),
        }
    }

    // attributes
    pub fn length(&self) -> ObjectRef {
        new_objectref(Object::Int(Integer {
            value: self.size as i64,
        }))
    }

    // methods
    fn clone(&self) -> Result<ObjectRef, PanicObj> {
        Ok(new_objectref(Object::Buffer(Box::new(Buffer {
            data: self.data.clone(),
            size: self.size,
        }))))
    }

    fn to_str(&self, state: StateRef) -> Result<ObjectRef, PanicObj> {
        let str = match str::from_utf8(&self.data) {
            Ok(str) => str.to_string(),
            Err(err_feedback) => {
                return Err(PanicObj::new(
                    PanicType::UTF8Conversion,
                    err_feedback.to_string(),
                    state,
                ));
            }
        };

        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: str,
        }))))
    }

    fn to_arr(&self) -> Result<ObjectRef, PanicObj> {
        let arr_of_ints = self
            .data
            .iter()
            .map(|byte| {
                new_objectref(Object::Int(Integer {
                    value: *byte as i64,
                }))
            })
            .collect();

        Ok(new_objectref(Object::Array(Box::new(Array {
            items: arr_of_ints,
        }))))
    }

    fn slice(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} arguments for buffer.slice(), got: {}",
                    2,
                    args.len()
                ),
                state,
            ));
        }
        let mut start_index = match &*args[0].borrow() {
            Object::Int(integer) => integer.value,
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected the first argument to be int, got: {}",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };
        let mut end_index = match &*args[1].borrow() {
            Object::Int(integer) => integer.value,
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected the second argument to be int, got: {}",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        };

        if start_index.is_negative() {
            start_index += self.size as i64;
        }

        if end_index.is_negative() {
            end_index += self.size as i64;
        }

        if start_index < 0 || start_index >= self.size as i64 {
            return Ok(new_objectref(Object::Buffer(Box::new(Buffer::new_empty()))));
        }
        if end_index >= self.size as i64 {
            end_index = self.size as i64;
        }

        Ok(new_objectref(Object::Buffer(Box::new(
            if start_index < end_index {
                Buffer {
                    data: self.data[start_index as usize..end_index as usize]
                        .to_vec()
                        .into_boxed_slice(),
                    size: (end_index - start_index) as usize,
                }
            } else {
                Buffer::new_empty()
            },
        ))))
    }
}
