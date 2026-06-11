use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
    buffer::Buffer,
    error::{error_type::ErrorType, panic_type::PanicType},
    float_obj::FloatObj,
    integer::Integer,
    iterator::{Iterator, list_based_iterator::ListBasedIterator},
    new_objectref,
    panic_obj::PanicObj,
    stack_environment::EnvRef,
    state::StateRef,
    string_obj::StringObj,
};

impl StringObj {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "length" => Ok(self.length()),

            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for string: '{}'", name),
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
            "reversed" => self.reversed(args, state),
            "chars" => self.chars(args, state),
            "as_float" => self.as_float(args, state),
            "as_int" => self.as_int(args, state),
            "as_str" => self.as_str(args, state),

            "as_buffer" => self.as_buffer(args, state),
            "strip" => self.strip(args, state),
            "lstrip" => self.lstrip(args, state),
            "rstrip" => self.rstrip(args, state),
            "replace" => self.replace(args, state),

            "contains" => self.contains(args, state),
            "slice" => self.slice(args, state),
            "split" => self.split(args, state),
            "clone" => self.deep_copy(args, state),

            "to_lower" => self.to_lower(args, state),
            "to_upper" => self.to_upper(args, state),

            "is_empty" => self.is_empty(args, state),
            "is_ascii" => self.is_ascii(args, state),
            "is_digit" => self.is_digit(args, state),

            "starts_with" => self.starts_with(args, state),
            "ends_with" => self.ends_with(args, state),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for string: '{}'", name),
                state,
            )),
        }
    }

    // Attributes

    fn length(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer {
            value: self.value.len() as i64,
        })))
    }

    // Methods

    fn reversed(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.reversed() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(Rc::new(RefCell::new(Object::String(Box::new(StringObj {
            value: self.value.chars().rev().collect(),
        })))))
    }

    fn chars(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.chars() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(new_objectref(Object::Array(Box::new(Array {
            items: self
                .value
                .chars()
                .map(|character| {
                    new_objectref(Object::String(Box::new(StringObj {
                        value: character.to_string(),
                    })))
                })
                .collect(),
        }))))
    }

    pub fn build_char_iterator(&self) -> Iterator {
        Iterator::ListBasedIterator(ListBasedIterator {
            list: self
                .value
                .chars()
                .map(|char| {
                    Rc::new(RefCell::new(Object::String(Box::new(StringObj {
                        value: char.to_string(),
                    }))))
                })
                .collect(),
            index: 0,
        })
    }

    fn contains(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} arguments for string.contains(), got: {}",
                    1,
                    args.len()
                ),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();
        let substr = match &*arg_borrow {
            Object::String(str) => &str.value,
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected as argument for string.contains() string, got: '{}'",
                        other_type.inspect()
                    ),
                    state,
                ));
            }
        };

        Ok(new_objectref(Object::get_native_boolean_object(
            self.value.contains(substr),
        )))
    }

    fn slice(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} arguments for string.slice(), got: {}",
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

        // Work with character indices to be UTF-8 safe
        let chars: Vec<char> = self.value.chars().collect();
        let len = chars.len() as i64;

        if start_index.is_negative() {
            start_index += len;
        }
        if end_index.is_negative() {
            end_index += len;
        }

        if start_index < 0 {
            start_index = 0;
        }
        if end_index > len {
            end_index = len;
        }

        if start_index >= len || start_index >= end_index {
            return Ok(Rc::new(RefCell::new(Object::String(Box::new(StringObj {
                value: String::new(),
            })))));
        }

        let start_us = start_index as usize;
        let end_us = end_index as usize;

        let result: String = chars[start_us..end_us].iter().collect();

        Ok(Rc::new(RefCell::new(Object::String(Box::new(StringObj {
            value: result,
        })))))
    }

    fn as_float(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.as_float() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(match self.value.parse::<f64>() {
            Ok(float_value) => Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: float_value,
            }))),
            Err(err) => Rc::new(RefCell::new(Object::new_error(
                ErrorType::IllegalCast,
                err.to_string(),
                state,
            ))),
        })
    }

    fn as_int(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
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
                            "expected int for str.as_int() as argument, got: '{}'",
                            other_type.get_type()
                        ),
                        state,
                    ));
                }
            },
        };

        match i64::from_str_radix(&self.value, radix as u32) {
            Ok(int_value) => Ok(new_objectref(Object::Int(Integer { value: int_value }))),
            Err(err) => Ok(new_objectref(Object::new_error(
                ErrorType::IllegalCast,
                err.to_string(),
                state,
            ))),
        }
    }

    fn as_str(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.as_str() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: self.value.clone(),
        }))))
    }

    fn as_buffer(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.as_buffer() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        let value = &self.value;

        let bytes = value.as_bytes().to_owned().into_boxed_slice();

        let size = bytes.len();

        Ok(new_objectref(Object::Buffer(Box::new(Buffer {
            data: bytes,
            size,
        }))))
    }

    fn strip(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.is_empty() {
            return Ok(new_objectref(Object::String(Box::new(StringObj {
                value: self.value.trim().to_string(),
            }))));
        }

        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 0 or 1 arguments for string.strip(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();
        let pattern = match &*arg_borrow {
            Object::String(s) => s.value.clone(),
            other => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected string as argument for string.strip(), got: {}",
                        other.get_type()
                    ),
                    state,
                ));
            }
        };

        let to_trim: Vec<char> = pattern.chars().collect();
        let result = self
            .value
            .trim_matches(|c| to_trim.contains(&c))
            .to_string();

        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: result,
        }))))
    }

    fn lstrip(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.is_empty() {
            return Ok(new_objectref(Object::String(Box::new(StringObj {
                value: self.value.trim_start().to_string(),
            }))));
        }
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 0 or 1 arguments for string.lstrip(), got: {}",
                    args.len()
                ),
                state,
            ));
        }
        let arg_borrow = args[0].borrow();
        let pattern = match &*arg_borrow {
            Object::String(s) => s.value.clone(),
            other => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected string as argument for string.lstrip(), got: {}",
                        other.get_type()
                    ),
                    state,
                ));
            }
        };
        let to_trim: Vec<char> = pattern.chars().collect();
        let result = self
            .value
            .trim_start_matches(|c| to_trim.contains(&c))
            .to_string();
        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: result,
        }))))
    }

    fn rstrip(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.is_empty() {
            return Ok(new_objectref(Object::String(Box::new(StringObj {
                value: self.value.trim_end().to_string(),
            }))));
        }
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 0 or 1 arguments for string.rstrip(), got: {}",
                    args.len()
                ),
                state,
            ));
        }
        let arg_borrow = args[0].borrow();
        let pattern = match &*arg_borrow {
            Object::String(s) => s.value.clone(),
            other => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected string as argument for string.rstrip(), got: {}",
                        other.get_type()
                    ),
                    state,
                ));
            }
        };
        let to_trim: Vec<char> = pattern.chars().collect();
        let result = self
            .value
            .trim_end_matches(|c| to_trim.contains(&c))
            .to_string();
        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: result,
        }))))
    }

    fn replace(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() < 2 || args.len() > 3 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 2 or 3 arguments for string.replace(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let old = match &*args[0].borrow() {
            Object::String(s) => s.value.clone(),
            other => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected string as first argument for string.replace(), got: {}",
                        other.get_type()
                    ),
                    state,
                ));
            }
        };

        let new = match &*args[1].borrow() {
            Object::String(s) => s.value.clone(),
            other => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected string as second argument for string.replace(), got: {}",
                        other.get_type()
                    ),
                    state,
                ));
            }
        };

        if args.len() == 2 {
            return Ok(new_objectref(Object::String(Box::new(StringObj {
                value: self.value.replace(&old, &new),
            }))));
        }

        // args.len() == 3 -> count
        let count = match &*args[2].borrow() {
            Object::Int(i) => i.value,
            other => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected int as third argument for string.replace(), got: {}",
                        other.get_type()
                    ),
                    state,
                ));
            }
        };

        if count <= 0 {
            return Ok(new_objectref(Object::String(Box::new(StringObj {
                value: self.value.replace(&old, &new),
            }))));
        }

        // replace up to count occurrences
        let parts: Vec<&str> = self.value.splitn((count + 1) as usize, &old).collect();
        let result = parts.join(&new);
        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: result,
        }))))
    }

    fn split(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        let split_value = if args.is_empty() {
            return self.chars(args, state);
        } else {
            match &*args[0].borrow() {
                Object::String(str) => str.value.clone(),
                other_type => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "expected to be the first parameter a 'str', got: {}",
                            other_type.get_type()
                        ),
                        state,
                    ));
                }
            }
        };

        if split_value.is_empty() {
            return self.chars([].as_slice(), state);
        }

        Ok(new_objectref(Object::Array(Box::new(Array {
            items: self
                .value
                .split(&split_value)
                .map(|sub_str: &str| {
                    new_objectref(Object::String(Box::new(StringObj {
                        value: sub_str.to_string(),
                    })))
                })
                .collect(),
        }))))
    }

    fn deep_copy(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.clone() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(Rc::new(RefCell::new(Object::String(Box::new(StringObj {
            value: self.value.clone(),
        })))))
    }

    fn to_lower(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.to_lower() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: self.value.to_lowercase(),
        }))))
    }

    fn to_upper(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.to_upper() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: self.value.to_uppercase(),
        }))))
    }

    fn is_ascii(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.is_ascii() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(new_objectref(Object::get_native_boolean_object(
            self.value.is_ascii(),
        )))
    }

    fn is_digit(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.is_digit() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(new_objectref(Object::get_native_boolean_object(
            self.value.chars().all(|c| c.is_ascii_digit()),
        )))
    }

    fn is_empty(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "string.is_empty() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(new_objectref(Object::get_native_boolean_object(
            self.value.is_empty(),
        )))
    }

    fn starts_with(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} argument for array.starts_with(), got: {}",
                    1,
                    args.len()
                ),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();
        let substr = match &*arg_borrow {
            Object::String(str) => &str.value,
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected as argument for string.starts_with() string, got: '{}'",
                        other_type.inspect()
                    ),
                    state,
                ));
            }
        };

        Ok(new_objectref(Object::get_native_boolean_object(
            self.value.starts_with(substr),
        )))
    }

    fn ends_with(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} argument for array.ends_with(), got: {}",
                    1,
                    args.len()
                ),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();
        let substr = match &*arg_borrow {
            Object::String(str) => &str.value,
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected as argument for string.ends_with() string, got: '{}'",
                        other_type.inspect()
                    ),
                    state,
                ));
            }
        };

        Ok(new_objectref(Object::get_native_boolean_object(
            self.value.ends_with(substr),
        )))
    }
}
