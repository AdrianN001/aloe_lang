use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
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
            "reversed" => Ok(self.reversed()),
            "chars" => Ok(self.chars()),
            "as_float" => Ok(self.as_float(state)),
            "as_int" => Ok(self.as_int(state)),
            "as_str" => Ok(self.as_str()),

            "as_byte_array" => Ok(self.as_byte_array()),
            "strip" => Ok(self.strip(args, state)),
            "lstrip" => Ok(self.lstrip(args, state)),
            "rstrip" => Ok(self.rstrip(args, state)),
            "replace" => Ok(self.replace(args, state)),

            "contains" => Ok(self.contains(args, state)),
            "slice" => Ok(self.slice(args, state)),
            "split" => Ok(self.split(args, state)),
            "clone" => Ok(self.deep_copy()),

            "to_lower" => Ok(self.to_lower()),
            "to_upper" => Ok(self.to_upper()),

            "is_empty" => Ok(self.is_empty()),
            "is_ascii" => Ok(self.is_ascii()),
            "is_digit" => Ok(self.is_digit()),

            "starts_with" => Ok(self.starts_with(args, state)),
            "ends_with" => Ok(self.ends_with(args, state)),

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

    fn reversed(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::String(Box::new(StringObj {
            value: self.value.chars().rev().collect(),
        }))))
    }

    fn chars(&self) -> ObjectRef {
        new_objectref(Object::Array(Box::new(Array {
            items: self
                .value
                .chars()
                .map(|character| {
                    new_objectref(Object::String(Box::new(StringObj {
                        value: character.to_string(),
                    })))
                })
                .collect(),
        })))
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

    fn contains(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected {} arguments for string.contains(), got: {}",
                    1,
                    args.len()
                ),
                state,
            )));
        }

        let arg_borrow = args[0].borrow();
        let substr = match &*arg_borrow {
            Object::String(str) => &str.value,
            other_type => {
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
                    format!(
                        "expected as argument for string.contains() string, got: '{}'",
                        other_type.inspect()
                    ),
                    state,
                ));
            }
        };

        new_objectref(Object::get_native_boolean_object(
            self.value.contains(substr),
        ))
    }

    fn slice(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 2 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected {} arguments for string.slice(), got: {}",
                    2,
                    args.len()
                ),
                state,
            )));
        }
        let mut start_index = match &*args[0].borrow() {
            Object::Int(integer) => integer.value,
            other_type => {
                return Rc::new(RefCell::new(Object::new_error(
                    ErrorType::WrongArgumentType,
                    format!(
                        "expected the first argument to be int, got: {}",
                        other_type.get_type()
                    ),
                    state,
                )));
            }
        };
        let mut end_index = match &*args[1].borrow() {
            Object::Int(integer) => integer.value,
            other_type => {
                return Rc::new(RefCell::new(Object::new_error(
                    ErrorType::WrongArgumentType,
                    format!(
                        "expected the second argument to be int, got: {}",
                        other_type.get_type()
                    ),
                    state,
                )));
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
            return Rc::new(RefCell::new(Object::String(Box::new(StringObj {
                value: String::new(),
            }))));
        }

        let start_us = start_index as usize;
        let end_us = end_index as usize;

        let result: String = chars[start_us..end_us].iter().collect();

        Rc::new(RefCell::new(Object::String(Box::new(StringObj {
            value: result,
        }))))
    }

    fn as_float(&self, state: StateRef) -> ObjectRef {
        match self.value.parse::<f64>() {
            Ok(float_value) => Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: float_value,
            }))),
            Err(err) => Rc::new(RefCell::new(Object::new_error(
                ErrorType::IllegalCast,
                err.to_string(),
                state,
            ))),
        }
    }

    fn as_int(&self, state: StateRef) -> ObjectRef {
        match self.value.parse::<i64>() {
            Ok(int_value) => Rc::new(RefCell::new(Object::Int(Integer { value: int_value }))),
            Err(err) => Rc::new(RefCell::new(Object::new_error(
                ErrorType::IllegalCast,
                err.to_string(),
                state,
            ))),
        }
    }

    fn as_str(&self) -> ObjectRef {
        new_objectref(Object::String(Box::new(StringObj {
            value: self.value.clone(),
        })))
    }

    fn as_byte_array(&self) -> ObjectRef {
        let value = &self.value;
        let new_raw_array = value
            .as_bytes()
            .iter()
            .map(|byte| {
                new_objectref(Object::Int(Integer {
                    value: *byte as i64,
                }))
            })
            .collect();

        new_objectref(Object::Array(Box::new(Array {
            items: new_raw_array,
        })))
    }

    fn strip(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.is_empty() {
            return new_objectref(Object::String(Box::new(StringObj {
                value: self.value.trim().to_string(),
            })));
        }

        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 0 or 1 arguments for string.strip(), got: {}",
                    args.len()
                ),
                state,
            )));
        }

        let arg_borrow = args[0].borrow();
        let pattern = match &*arg_borrow {
            Object::String(s) => s.value.clone(),
            other => {
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
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

        new_objectref(Object::String(Box::new(StringObj { value: result })))
    }

    fn lstrip(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.is_empty() {
            return new_objectref(Object::String(Box::new(StringObj {
                value: self.value.trim_start().to_string(),
            })));
        }
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 0 or 1 arguments for string.lstrip(), got: {}",
                    args.len()
                ),
                state,
            )));
        }
        let arg_borrow = args[0].borrow();
        let pattern = match &*arg_borrow {
            Object::String(s) => s.value.clone(),
            other => {
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
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
        new_objectref(Object::String(Box::new(StringObj { value: result })))
    }

    fn rstrip(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.is_empty() {
            return new_objectref(Object::String(Box::new(StringObj {
                value: self.value.trim_end().to_string(),
            })));
        }
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 0 or 1 arguments for string.rstrip(), got: {}",
                    args.len()
                ),
                state,
            )));
        }
        let arg_borrow = args[0].borrow();
        let pattern = match &*arg_borrow {
            Object::String(s) => s.value.clone(),
            other => {
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
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
        new_objectref(Object::String(Box::new(StringObj { value: result })))
    }

    fn replace(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() < 2 || args.len() > 3 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 2 or 3 arguments for string.replace(), got: {}",
                    args.len()
                ),
                state,
            )));
        }

        let old = match &*args[0].borrow() {
            Object::String(s) => s.value.clone(),
            other => {
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
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
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
                    format!(
                        "expected string as second argument for string.replace(), got: {}",
                        other.get_type()
                    ),
                    state,
                ));
            }
        };

        if args.len() == 2 {
            return new_objectref(Object::String(Box::new(StringObj {
                value: self.value.replace(&old, &new),
            })));
        }

        // args.len() == 3 -> count
        let count = match &*args[2].borrow() {
            Object::Int(i) => i.value,
            other => {
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
                    format!(
                        "expected int as third argument for string.replace(), got: {}",
                        other.get_type()
                    ),
                    state,
                ));
            }
        };

        if count <= 0 {
            return new_objectref(Object::String(Box::new(StringObj {
                value: self.value.replace(&old, &new),
            })));
        }

        // replace up to count occurrences
        let parts: Vec<&str> = self.value.splitn((count + 1) as usize, &old).collect();
        let result = parts.join(&new);
        new_objectref(Object::String(Box::new(StringObj { value: result })))
    }

    fn split(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        let split_value = if args.is_empty() {
            return self.chars();
        } else {
            match &*args[0].borrow() {
                Object::String(str) => str.value.clone(),
                other_type => {
                    return Rc::new(RefCell::new(Object::new_error(
                        ErrorType::WrongArgumentType,
                        format!(
                            "expected to be the first paramter a 'str', got: {}",
                            other_type.get_type()
                        ),
                        state,
                    )));
                }
            }
        };

        if split_value.is_empty() {
            return self.chars();
        }

        new_objectref(Object::Array(Box::new(Array {
            items: self
                .value
                .split(&split_value)
                .map(|sub_str: &str| {
                    new_objectref(Object::String(Box::new(StringObj {
                        value: sub_str.to_string(),
                    })))
                })
                .collect(),
        })))
    }

    fn deep_copy(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::String(Box::new(StringObj {
            value: self.value.clone(),
        }))))
    }

    fn to_lower(&self) -> ObjectRef {
        new_objectref(Object::String(Box::new(StringObj {
            value: self.value.to_lowercase(),
        })))
    }

    fn to_upper(&self) -> ObjectRef {
        new_objectref(Object::String(Box::new(StringObj {
            value: self.value.to_uppercase(),
        })))
    }

    fn is_ascii(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(self.value.is_ascii()))
    }

    fn is_digit(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(
            self.value.chars().all(|c| c.is_ascii_digit()),
        ))
    }

    fn is_empty(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(self.value.is_empty()))
    }

    fn starts_with(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected {} argument for array.starts_with(), got: {}",
                    1,
                    args.len()
                ),
                state,
            )));
        }

        let arg_borrow = args[0].borrow();
        let substr = match &*arg_borrow {
            Object::String(str) => &str.value,
            other_type => {
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
                    format!(
                        "expected as argument for string.starts_with() string, got: '{}'",
                        other_type.inspect()
                    ),
                    state,
                ));
            }
        };

        new_objectref(Object::get_native_boolean_object(
            self.value.starts_with(substr),
        ))
    }

    fn ends_with(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected {} argument for array.ends_with(), got: {}",
                    1,
                    args.len()
                ),
                state,
            )));
        }

        let arg_borrow = args[0].borrow();
        let substr = match &*arg_borrow {
            Object::String(str) => &str.value,
            other_type => {
                return new_objectref(Object::new_error(
                    ErrorType::WrongArgumentType,
                    format!(
                        "expected as argument for string.ends_with() string, got: '{}'",
                        other_type.inspect()
                    ),
                    state,
                ));
            }
        };

        new_objectref(Object::get_native_boolean_object(
            self.value.ends_with(substr),
        ))
    }
}
