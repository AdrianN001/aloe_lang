use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, array::Array, buffer::Buffer, error::{error_type::ErrorType, panic_type::PanicType}, integer::Integer, iterator::{
        Iterator, enumerator_iterator::EnumeratorIterator, list_based_iterator::ListBasedIterator,
        zip_iterator::ZipIterator,
    }, new_objectref, null::Null, panic_obj::{PanicObj, RuntimeSignal}, stack_environment::EnvRef, state::StateRef, string_obj::StringObj
};

impl Array {
    pub fn apply_attribute(
        &self,
        name: &str,
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "length" => Ok(self.length()),

            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for array: '{}'", name),
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
            "push" => self.push(args, state),
            "extend" => self.extend(args, state),
            "clear" => Ok(self.clear()),

            "remove" => self.remove(args, state),
            "slice" => self.slice(args, state),
            "insert" => self.insert(args, state),

            "clone" => Ok(self.deep_copy()),
            "contains" => self.contains(args, state),

            "map" => self.map(args, state),
            "filter" => self.filter(args, state),

            "as_iter" => Ok(self.as_iter()),
            "as_buffer" => self.as_buffer(args, state),
            "enumerate" => self.enumerate(),
            "zip" => self.zip(args, state),
            "join" => self.join(args, state),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for array: '{}'", name),
                state,
            )),
        }
    }

    // Attributes

    fn length(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer {
            value: self.items.len() as i64,
        })))
    }

    // Methods

    fn reversed(&mut self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Array(Box::new(Array {
            items: self.items.iter().rev().map(|item| item.clone()).collect(),
        }))))
    }

    fn deep_copy(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Array(Box::new(Array {
            items: self
                .items
                .iter()
                .map(|item| Object::deep_copy(item.clone()))
                .collect(),
        }))))
    }

    fn push(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected at least {} arguments for array.push(), got: {}",
                    1,
                    args.len()
                ),
                state,
            ));
        }
        if !args.is_empty() {
            args.iter().for_each(|argument| {
                self.items.push(argument.clone());
            });
        }
        Ok(Rc::new(RefCell::new(Object::Null(Null {}))))
    }

    fn remove(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} arguments for array.remove(), got: {}",
                    1,
                    args.len()
                ),
                state,
            ));
        }
        let mut index = match &*args[0].borrow() {
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

        if index.is_negative() {
            index += self.items.len() as i64;
        }
        if index >= 0 && index < self.items.len() as i64 {
            return Ok(self.items.remove(index as usize).clone());
        }

        Ok(new_objectref(Object::new_error(
            ErrorType::IndexOutOfBound,
            format!(
                "array.remove(), array with size: {} was indexed with: {}",
                self.items.len(),
                index
            ),
            state,
        )))
    }

    fn insert(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} arguments for array.insert(), got: {}",
                    2,
                    args.len()
                ),
                state,
            ));
        }

        let mut insert_position = match &*args[0].borrow() {
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

        if insert_position.is_negative() {
            insert_position += self.items.len() as i64;
        }

        if insert_position as usize >= self.items.len() {
            return Ok(new_objectref(Object::new_error(
                ErrorType::IndexOutOfBound,
                "index position for insert >= array.length".into(),
                state,
            )));
        }

        self.items.insert(insert_position as usize, args[1].clone());

        Ok(new_objectref(Object::NULL_OBJECT))
    }

    fn slice(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} arguments for array.slice(), got: {}",
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
            start_index += self.items.len() as i64;
        }

        if end_index.is_negative() {
            end_index += self.items.len() as i64;
        }

        if start_index < 0 || start_index >= self.items.len() as i64 {
            return Ok(Rc::new(RefCell::new(Object::Array(Box::new(Array {
                items: Vec::new(),
            })))));
        }
        if end_index >= self.items.len() as i64 {
            end_index = self.items.len() as i64;
        }

        Ok(Rc::new(RefCell::new(Object::Array(Box::new(Array {
            items: if start_index < end_index {
                self.items[start_index as usize..end_index as usize].to_vec()
            } else {
                Vec::new()
            },
        })))))
    }

    fn extend(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!("expected {} arguments for array.extend(), got: {}", 1, 0),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();

        match &*arg_borrow {
            Object::Array(arr) => {
                let other_arr = &arr.items;
                other_arr.iter().for_each(|item| {
                    self.items.push(item.clone());
                })
            }
            other_type => {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentType,
                    format!(
                        "expected the first argument to be a list, got: {}",
                        other_type.get_type()
                    ),
                    state,
                ));
            }
        }

        Ok(Rc::new(RefCell::new(Object::Null(Null {}))))
    }

    fn clear(&mut self) -> ObjectRef {
        self.items.clear();

        Rc::new(RefCell::new(Object::Null(Null {})))
    }

    fn contains(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} arguments for array.contains(), got: {}",
                    1,
                    args.len()
                ),
                state,
            ));
        }

        Ok(new_objectref(Object::get_native_boolean_object(
            self.items.contains(&args[0]),
        )))
    }

    fn map(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                "no function was provided for map".into(),
                state,
            ));
        }

        if let Object::Func(function) = &*args[0].borrow() {
            if function.parameters.len() != 1 {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentCount,
                    "function provided to the map must have only one element".into(),
                    state,
                ));
            }
            let mut mapped_array_content = Vec::new();

            for item in self.items.clone() {
                let mapped_item = function.apply(
                    "(anonymm map function)".into(),
                    [item.clone()].as_ref(),
                    state.clone(),
                );
                match mapped_item {
                    Ok(ok_value) => mapped_array_content.push(ok_value.clone()),
                    Err(RuntimeSignal::Panic(error)) => return Err(error),
                    _ => todo!(),
                }
            }

            return Ok(Rc::new(RefCell::new(Object::Array(Box::new(Array {
                items: mapped_array_content,
            })))));
        }

        Err(PanicObj::new(
            PanicType::WrongArgumentType,
            "argument provided is not a function".into(),
            state,
        ))
    }

    fn filter(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                "expected 1 arguments for array.filter(), got: 0".into(),
                state,
            ));
        }

        if let Object::Func(function) = &*args[0].borrow() {
            if function.parameters.len() != 1 {
                return Err(PanicObj::new(
                    PanicType::WrongArgumentCount,
                    "function provided to the filter must have only one element".into(),
                    state,
                ));
            }
            let mut mapped_array_content = Vec::new();

            for item in self.items.clone() {
                let boolean_output = function.apply(
                    "(anonymm filter function)".into(),
                    [item.clone()].as_ref(),
                    state.clone(),
                );
                match boolean_output {
                    Ok(ok_value) => {
                        if ok_value.borrow().is_truthy() {
                            mapped_array_content.push(item.clone())
                        }
                    }
                    Err(RuntimeSignal::Panic(error)) => return Err(error),
                    _ => todo!(),
                }
            }

            return Ok(new_objectref(Object::Array(Box::new(Array {
                items: mapped_array_content,
            }))));
        }

        Err(PanicObj::new(
            PanicType::WrongArgumentType,
            "argument provided to array.filter() is not a function".into(),
            state,
        ))
    }

    fn as_iter(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Iterator(Box::new(
            self.build_iterator(),
        ))))
    }

    fn as_buffer(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj>{
        if !args.is_empty(){
            return Err(PanicObj::new(PanicType::WrongArgumentCount, format!("expected 0 argument for array.as_buffer(), got: {}", args.len()), state));
        }

        let bytes = self.items.iter().map(|item|{
            let val = match &*item.borrow(){
                Object::Int(integer) => integer.value,
                other_type => return Err(PanicObj::new(
                    PanicType::WrongType,
                    format!("expected an array of integers, got: '{}'", other_type.get_type()),
                    state.clone()
                ))
            };

            let byte = match u8::try_from(val){
                Ok(byte) => byte,
                Err(err_feedback) => return Err(PanicObj::new(
                    PanicType::IllegalTypeCasting,
                    err_feedback.to_string(),
                    state.clone()
                ))
            };

            Ok(byte)
        }).collect::<Result<Vec<u8>, PanicObj>>()?;

        let bytes_arr = new_objectref(Object::Buffer(Box::new(
            Buffer{
                size: bytes.len(),
                data: bytes.into_boxed_slice()
            }
        )));

        Ok(bytes_arr)
    }

    pub fn build_iterator(&self) -> Iterator {
        Iterator::ListBasedIterator(ListBasedIterator {
            list: self.items.iter().map(|item| item.clone()).collect(),
            index: 0,
        })
    }

    fn join(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        let join_str_value = if args.is_empty() {
            "".to_string()
        } else {
            match &*args[0].borrow() {
                Object::String(str) => str.value.clone(),
                other_type => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "expected to be the first paramter a 'str', got: {}",
                            other_type.get_type()
                        ),
                        state,
                    ));
                }
            }
        };

        let mut strings = Vec::new();

        for (index, item) in self.items.clone().iter().enumerate() {
            match &*item.borrow() {
                Object::String(str) => strings.push(str.value.clone()),
                Object::Int(int) => strings.push(int.value.to_string()),
                Object::FloatObj(float) => strings.push(float.val.to_string()),
                Object::Bool(bool) => strings.push(bool.value.to_string()),

                other_type => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "not all elements can be converted to str. Element {} is {}",
                            index,
                            other_type.get_type()
                        ),
                        state,
                    ));
                }
            }
        }

        Ok(Rc::new(RefCell::new(Object::String(Box::new(StringObj {
            value: strings.join(&join_str_value),
        })))))
    }

    fn enumerate(&self) -> Result<ObjectRef, PanicObj> {
        let enumerator_iterator = Iterator::EnumeratorIterator(EnumeratorIterator {
            list: self.items.clone(),
            index: 0,
        });

        Ok(new_objectref(Object::Iterator(Box::new(
            enumerator_iterator,
        ))))
    }

    fn zip(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        let mut minimum_length = self.items.len();

        for arg in args {
            let arg_borrow = arg.borrow();

            let arr_length = match &*arg_borrow {
                Object::Array(arr) => *&arr.items.len(),
                other_type => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        format!(
                            "expected array as parameter for array.zip(), got: {}",
                            other_type.get_type()
                        ),
                        state,
                    ));
                }
            };

            if minimum_length > arr_length {
                minimum_length = arr_length;
            }
        }

        let self_as_ref = new_objectref(Object::Array(Box::new(Array {
            items: self.items.clone(),
        })));

        let zip_iterator = Iterator::ZipIterator(ZipIterator {
            list: {
                let mut new_vec = Vec::with_capacity(args.len() + 1);
                new_vec.push(self_as_ref);
                args.iter().for_each(|arg| {
                    new_vec.push(arg.clone());
                });
                new_vec
            },
            index: 0,
            min_length: minimum_length,
        });

        Ok(new_objectref(Object::Iterator(Box::new(zip_iterator))))
    }
}
