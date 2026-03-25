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
            "contains" => Ok(self.contains(args, state)),
            "slice" => Ok(self.slice(args, state)),
            "split" => Ok(self.split(args, state)),
            "clone" => Ok(self.deep_copy()),

            "to_lower" => Ok(self.to_lower()),
            "to_upper" => Ok(self.to_upper()),

            "is_empty" => Ok(self.is_empty()),
            "is_ascii" => Ok(self.is_ascii()),

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
        Rc::new(RefCell::new(Object::String(StringObj {
            value: self.value.chars().rev().collect(),
        })))
    }

    fn chars(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Array(Array {
            items: self
                .value
                .chars()
                .map(|character| {
                    Rc::new(RefCell::new(Object::String(StringObj {
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
                    Rc::new(RefCell::new(Object::String(StringObj {
                        value: char.to_string(),
                    })))
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
                    "expected {} arguments for array.slice(), got: {}",
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

        if start_index.is_negative() {
            start_index += self.value.len() as i64;
        }

        if end_index.is_negative() {
            end_index += self.value.len() as i64;
        }

        if start_index < 0 || start_index >= self.value.len() as i64 {
            return Rc::new(RefCell::new(Object::String(StringObj {
                value: String::new(),
            })));
        }
        if end_index >= self.value.len() as i64 {
            end_index = self.value.len() as i64;
        }

        Rc::new(RefCell::new(Object::String(StringObj {
            value: if start_index < end_index {
                self.value[start_index as usize..end_index as usize].to_string()
            } else {
                String::new()
            },
        })))
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

        Rc::new(RefCell::new(Object::Array(Array {
            items: self
                .value
                .split(&split_value)
                .map(|sub_str: &str| {
                    Rc::new(RefCell::new(Object::String(StringObj {
                        value: sub_str.to_string(),
                    })))
                })
                .collect(),
        })))
    }

    fn deep_copy(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::String(StringObj {
            value: self.value.clone(),
        })))
    }

    fn to_lower(&self) -> ObjectRef {
        new_objectref(Object::String(StringObj {
            value: self.value.to_lowercase(),
        }))
    }

    fn to_upper(&self) -> ObjectRef {
        new_objectref(Object::String(StringObj {
            value: self.value.to_uppercase(),
        }))
    }

    fn is_ascii(&self) -> ObjectRef {
        new_objectref(Object::get_native_boolean_object(self.value.is_ascii()))
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
