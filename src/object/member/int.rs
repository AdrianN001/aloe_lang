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
            "as_float" => self.as_float(args, state),
            "as_int" => self.as_int(args, state),
            "as_utf_char" => self.as_utf_char(args, state),
            "as_le_bytes" => self.to_le_bytes(args, state),
            "as_be_bytes" => self.to_be_bytes(args, state),
            "clone" => self.deep_clone(args, state),

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

    pub fn as_float(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "int.as_float() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
            val: self.value as f64,
        }))))
    }

    pub fn as_int(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "int.as_int() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(new_objectref(Object::Int(Integer { value: self.value })))
    }

    pub fn as_utf_char(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "int.as_utf_char() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        let bytes_value: u32 = match self.value.try_into() {
            Ok(bytes) => bytes,
            Err(_) => {
                return Ok(new_objectref(Object::new_error(
                    ErrorType::UTFValueCasting,
                    format!("{} can not be converted to character.", self.value),
                    state,
                )));
            }
        };

        let mapped_char = match char::from_u32(bytes_value) {
            Some(mapped_char) => mapped_char,
            None => {
                return Ok(new_objectref(Object::new_error(
                    ErrorType::UTFValueCasting,
                    format!("{} can not be converted to character.", self.value),
                    state,
                )));
            }
        };

        let str = String::from(mapped_char);

        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: str,
        }))))
    }

    pub fn to_le_bytes(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "int.as_le_bytes() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        let bytes = self.value.to_le_bytes();

        Ok(new_objectref(Object::Buffer(Box::new(Buffer {
            data: Box::new(bytes),
            size: bytes.len(),
        }))))
    }

    pub fn to_be_bytes(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "int.as_be_bytes() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        let bytes = self.value.to_be_bytes();

        Ok(new_objectref(Object::Buffer(Box::new(Buffer {
            data: Box::new(bytes),
            size: bytes.len(),
        }))))
    }

    pub fn deep_clone(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "int.clone() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(Rc::new(RefCell::new(Object::Int(Integer {
            value: self.value,
        }))))
    }
}
