use std::{cell::RefCell, char, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    boolean::Boolean,
    buffer::Buffer,
    error::{error_type::ErrorType, panic_type::PanicType},
    float_obj::FloatObj,
    integer::Integer,
    new_objectref,
    panic_obj::PanicObj,
    stack_environment::EnvRef,
    state::StateRef,
    string_obj::StringObj,
};

impl Integer {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "is_negative" => Ok(self.is_negative()),
            "is_positive" => Ok(self.is_positive()),

            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for int: '{}'", name),
                state,
            )),
        }
    }
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "as_str" => self.as_str(args, state),
            "as_float" => Ok(self.as_float()),
            "as_int" => Ok(self.as_int()),
            "as_utf_char" => Ok(self.as_utf_char(state)),
            "as_le_bytes" => Ok(self.to_le_bytes()),
            "as_be_bytes" => Ok(self.to_be_bytes()),
            "clone" => Ok(self.deep_clone()),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for int: '{}'", name),
                state,
            )),
        }
    }
}

impl Integer {
    // Attributes

    pub fn is_negative(&self) -> ObjectRef {
        new_objectref(Object::Bool(Boolean {
            value: self.value.is_negative(),
        }))
    }

    pub fn is_positive(&self) -> ObjectRef {
        new_objectref(Object::Bool(Boolean {
            value: self.value.is_positive(),
        }))
    }

    // Methods

    pub fn as_str(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        let radix = match args.len() {
            0 => 10,
            _ => match &*args[0].borrow() {
                Object::Int(int) => {
                    if int.value == 2 || int.value == 8 || int.value == 16 {
                        int.value
                    } else {
                        return Ok(new_objectref(Object::new_error(
                            ErrorType::WrongRadix,
                            format!("expected radix 2,8 or 16, got: '{}'", int.value),
                            state,
                        )));
                    }
                }
                other_type => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "expected int for int.as_str() as argument, got: '{}'",
                            other_type.get_type()
                        ),
                        state,
                    ));
                }
            },
        };

        let int_as_str = match radix {
            2 => format!("{:b}", self.value),
            8 => format!("{:o}", self.value),
            16 => format!("{:x}", self.value),

            10 => format!("{}", self.value),
            _ => unreachable!(),
        };

        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: int_as_str,
        }))))
    }

    pub fn as_float(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::FloatObj(FloatObj {
            val: self.value as f64,
        })))
    }

    pub fn as_int(&self) -> ObjectRef {
        new_objectref(Object::Int(Integer { value: self.value }))
    }

    pub fn as_utf_char(&self, state: StateRef) -> ObjectRef {
        let bytes_value: u32 = match self.value.try_into() {
            Ok(bytes) => bytes,
            Err(_) => {
                return new_objectref(Object::new_error(
                    ErrorType::UTFValueCasting,
                    format!("{} can not be converted to character.", self.value),
                    state,
                ));
            }
        };

        let mapped_char = match char::from_u32(bytes_value) {
            Some(mapped_char) => mapped_char,
            None => {
                return new_objectref(Object::new_error(
                    ErrorType::UTFValueCasting,
                    format!("{} can not be converted to character.", self.value),
                    state,
                ));
            }
        };

        let str = String::from(mapped_char);

        new_objectref(Object::String(Box::new(StringObj { value: str })))
    }

    pub fn to_le_bytes(&self) -> ObjectRef {
        let bytes = self.value.to_le_bytes();

        new_objectref(Object::Buffer(Box::new(Buffer {
            data: Box::new(bytes),
            size: bytes.len(),
        })))
    }

    pub fn to_be_bytes(&self) -> ObjectRef {
        let bytes = self.value.to_be_bytes();

        new_objectref(Object::Buffer(Box::new(Buffer {
            data: Box::new(bytes),
            size: bytes.len(),
        })))
    }

    pub fn deep_clone(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer { value: self.value })))
    }
}
