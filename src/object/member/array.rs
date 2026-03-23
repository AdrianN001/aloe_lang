use std::{cell::RefCell, env::args, rc::Rc};

use crate::object::{
    Object, ObjectRef, array::Array, integer::Integer, iterator::{Iterator, list_based_iterator::ListBasedIterator}, new_objectref, null::Null, stack_environment::EnvRef, state::StateRef, string_obj::StringObj
};

impl Array {
    pub fn apply_attribute(&self, name: &str, _environ: EnvRef, state: StateRef) -> ObjectRef {
        match name {
            "length" => self.length(),

            _ => Rc::new(RefCell::new(Object::new_error(
                format!("unknown attribute for array: '{}'", name),
                state,
            ))),
        }
    }
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        _environ: EnvRef,
        state: StateRef,
    ) -> ObjectRef {
        match name {
            "reversed" => self.reversed(),
            "push" => self.push(args),
            "extend" => self.extend(args),
            "clear" => self.clear(),

            "remove" => self.remove(args, state),
            "slice" => self.slice(args, state),
            "insert"  => self.insert(args, state),

            "clone" => self.deep_copy(),
            "contains" => self.contains(args, state),

            "map" => self.map(args, state),
            "filter" => self.filter(args, state),

            "as_iter" => self.as_iter(),
            "join" => self.join(args, state),

            _ => Rc::new(RefCell::new(Object::new_error(
                format!("unknown method for array: '{}'", name),
                state,
            ))),
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
        Rc::new(RefCell::new(Object::Array(Array {
            items: self.items.iter().rev().map(|item| item.clone()).collect(),
        })))
    }

    fn deep_copy(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Array(Array {
            items: self
                .items
                .iter()
                .map(|item| Object::deep_copy(item.clone()))
                .collect(),
        })))
    }

    fn push(&mut self, args: &[ObjectRef]) -> ObjectRef {
        if !args.is_empty() {
            args.iter().for_each(|argument| {
                self.items.push(argument.clone());
            });
        }
        Rc::new(RefCell::new(Object::Null(Null {})))
    }

    fn remove(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                format!(
                    "expected {} arguments for array.remove(), got: {}",
                    1,
                    args.len()
                ),
                state,
            )));
        }
        let mut index = match &*args[0].borrow() {
            Object::Int(integer) => integer.value,
            other_type => {
                return Rc::new(RefCell::new(Object::new_error(
                    format!(
                        "expected the first argument to be int, got: {}",
                        other_type.get_type()
                    ),
                    state,
                )));
            }
        };

        if index.is_negative() {
            index += self.items.len() as i64;
        }
        if index >= 0 && index < self.items.len() as i64 {
            return self.items.remove(index as usize).clone();
        }

        Rc::new(RefCell::new(Object::NULL_OBJECT))
    }

    fn insert(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef{
        if args.len() != 2{ 
            return Rc::new(RefCell::new(Object::new_error(
                format!(
                    "expected {} arguments for array.insert(), got: {}",
                    2,
                    args.len()
                ),
                state,
            )));
        }

        let mut insert_position = match &*args[0].borrow() {
            Object::Int(integer) => integer.value,
            other_type => {
                return Rc::new(RefCell::new(Object::new_error(
                    format!(
                        "expected the first argument to be int, got: {}",
                        other_type.get_type()
                    ),
                    state,
                )));
            }
        };

        if insert_position.is_negative(){
            insert_position += self.items.len() as i64;
        }

        if insert_position as usize >= self.items.len(){
            return new_objectref(Object::new_error("index position for insert >= array.length".into(), state))
        }

        self.items.insert(insert_position as usize, args[1].clone());

        new_objectref(Object::NULL_OBJECT)
    }

    fn slice(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 2 {
            return Rc::new(RefCell::new(Object::new_error(
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
                    format!(
                        "expected the second argument to be int, got: {}",
                        other_type.get_type()
                    ),
                    state,
                )));
            }
        };

        if start_index.is_negative() {
            start_index += self.items.len() as i64;
        }

        if end_index.is_negative() {
            end_index += self.items.len() as i64;
        }

        if start_index < 0 || start_index >= self.items.len() as i64 {
            return Rc::new(RefCell::new(Object::Array(Array { items: Vec::new() })));
        }
        if end_index >= self.items.len() as i64 {
            end_index = self.items.len() as i64;
        }

        Rc::new(RefCell::new(Object::Array(Array {
            items: if start_index < end_index {
                self.items[start_index as usize..end_index as usize]
                    .iter()
                    .map(|item| item.clone())
                    .collect()
            } else {
                Vec::new()
            },
        })))
    }

    fn extend(&mut self, args: &[ObjectRef]) -> ObjectRef {
        if !args.is_empty()
            && let Object::Array(other_arr) = &*args[0].borrow()
        {
            other_arr.items.iter().for_each(|item| {
                self.items.push(item.clone());
            })
        }

        Rc::new(RefCell::new(Object::Null(Null {})))
    }

    fn clear(&mut self) -> ObjectRef {
        self.items.clear();

        Rc::new(RefCell::new(Object::Null(Null {})))
    }

    fn contains(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef{
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                format!(
                    "expected {} arguments for array.contains(), got: {}",
                    1,
                    args.len()
                ),
                state,
            )));
        }

        new_objectref(Object::get_native_boolean_object( self.items.contains(&args[0]) ) )
    }

    fn map(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.is_empty() {
            return Rc::new(RefCell::new(Object::new_error(
                "no function was provided for map".into(),
                state,
            )));
        }

        if let Object::Func(function) = &*args[0].borrow() {
            if function.parameters.len() > 1 {
                return Rc::new(RefCell::new(Object::new_error(
                    "function provided to the map needs more than 1 argument".into(),
                    state,
                )));
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
                    Err(error) => {
                        return Rc::new(RefCell::new(Object::new_error(error.to_string(), state)));
                    }
                }
            }

            return Rc::new(RefCell::new(Object::Array(Array {
                items: mapped_array_content,
            })));
        }

        Rc::new(RefCell::new(Object::new_error(
            "argument provided is not a function".into(),
            state,
        )))
    }

    fn filter(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.is_empty() {
            return Rc::new(RefCell::new(Object::new_error(
                "no function was provided for filter".into(),
                state,
            )));
        }

        if let Object::Func(function) = &*args[0].borrow() {
            if function.parameters.len() > 1 {
                return Rc::new(RefCell::new(Object::new_error(
                    "function provided to the filter needs more than 1 argument".into(),
                    state,
                )));
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
                    Err(error) => {
                        return Rc::new(RefCell::new(Object::new_error(error.to_string(), state)));
                    }
                }
            }

            return Rc::new(RefCell::new(Object::Array(Array {
                items: mapped_array_content,
            })));
        }

        Rc::new(RefCell::new(Object::new_error(
            "argument provided is not a function".into(),
            state,
        )))
    }

    fn as_iter(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Iterator(self.build_iterator())))
    }

    pub fn build_iterator(&self) -> Iterator {
        Iterator::ListBasedIterator(ListBasedIterator {
            list: self.items.iter().map(|item| item.clone()).collect(),
            index: 0,
        })
    }

    fn join(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        let join_str_value = if args.is_empty() {
            "".to_string()
        } else {
            match &*args[0].borrow() {
                Object::String(str) => str.value.clone(),
                other_type => {
                    return Rc::new(RefCell::new(Object::new_error(
                        format!(
                            "expected to be the first paramter a 'str', got: {}",
                            other_type.get_type()
                        ),
                        state,
                    )));
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
                    return Rc::new(RefCell::new(Object::new_error(
                        format!(
                            "not all elements can be converted to str. Element {} is {}",
                            index,
                            other_type.get_type()
                        ),
                        state,
                    )));
                }
            }
        }

        Rc::new(RefCell::new(Object::String(StringObj {
            value: strings.join(&join_str_value),
        })))
    }
}
