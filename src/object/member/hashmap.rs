use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
    error::{error_type::ErrorType, panic_type::PanicType},
    hashmap::{HashMap, HashPair},
    integer::Integer,
    iterator::{Iterator, list_based_iterator::ListBasedIterator},
    new_objectref,
    panic_obj::PanicObj,
    stack_environment::EnvRef,
    state::StateRef,
};

impl HashMap {
    pub fn apply_attribute(
        &self,
        name: &str,
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "length" => Ok(self.get_length()),
            "keys" => Ok(self.get_keys()),
            "values" => Ok(self.get_values()),
            "items" => Ok(self.items()),

            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for hashmap: '{}'", name),
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
            "set" => Ok(self.set(args, state)),
            "get" => Ok(self.get(args, state)),
            "pop" => Ok(self.pop(args, state)),
            "setdefault" => Ok(self.setdefault(args, state)),
            "update" => Ok(self.update(args, state)),
            "remove" => Ok(self.remove(args, state)),
            "clear" => Ok(self.clear()),
            "has_key" => Ok(self.has_key(args, state)),
            "as_iter" => Ok(self.as_iter()),

            "clone" => Ok(self.deep_copy()),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for hashmap: '{}'", name),
                state,
            )),
        }
    }

    // Attributes

    pub fn get_length(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer {
            value: self.pairs.len() as i64,
        })))
    }

    pub fn get_keys(&self) -> ObjectRef {
        new_objectref(Object::Array(Box::new(Array {
            items: self.pairs.values().map(|value| value.key.clone()).collect(),
        })))
    }

    pub fn get_values(&self) -> ObjectRef {
        new_objectref(Object::Array(Box::new(Array {
            items: self
                .pairs
                .values()
                .map(|value| value.value.clone())
                .collect(),
        })))
    }

    pub fn items(&self) -> ObjectRef {
        new_objectref(Object::Array(Box::new(Array {
            items: self
                .pairs
                .values()
                .map(|value| {
                    let key = value.key.clone();
                    let value = value.value.clone();

                    new_objectref(Object::Array(Box::new(Array {
                        items: vec![key, value],
                    })))
                })
                .collect(),
        })))
    }

    // Methods

    pub fn set(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 2 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 2 arguments for hashmap.set(), got: {}",
                    args.len()
                ),
                state,
            )));
        }

        let hashed_key = match args[0].borrow().hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                )));
            }
        };

        self.pairs.insert(
            hashed_key,
            HashPair {
                key: args[0].clone(),
                value: args[1].clone(),
            },
        );

        args[1].clone()
    }

    pub fn get(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 && args.len() != 2 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 1 or 2 arguments for hashmap.get(), got: {}",
                    args.len()
                ),
                state,
            )));
        }

        let arg_borrow = args[0].borrow();

        let hashed_key = match arg_borrow.hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                )));
            }
        };

        if let Some(value) = self.pairs.get(&hashed_key) {
            return value.value.clone();
        }

        if args.len() == 2 {
            return args[1].clone();
        }

        new_objectref(Object::new_error(
            ErrorType::ItemNotFound,
            format!("hashmap has no key: '{}'", &*arg_borrow.inspect()),
            state,
        ))
    }

    pub fn clear(&mut self) -> ObjectRef {
        self.pairs.clear();

        Rc::new(RefCell::new(Object::NULL_OBJECT))
    }

    pub fn deep_copy(&self) -> ObjectRef {
        let mut new_pairs = BTreeMap::new();

        for (key_hash, pair) in &self.pairs {
            new_pairs.insert(
                key_hash.clone(),
                HashPair {
                    key: Object::deep_copy(pair.key.clone()),
                    value: Object::deep_copy(pair.value.clone()),
                },
            );
        }

        Rc::new(RefCell::new(Object::HashMap(Box::new(HashMap {
            pairs: new_pairs,
        }))))
    }

    pub fn as_iter(&self) -> ObjectRef {
        new_objectref(Object::Iterator(Box::new(self.build_iterator())))
    }

    pub fn build_iterator(&self) -> Iterator {
        Iterator::ListBasedIterator(ListBasedIterator {
            list: self
                .pairs
                .values()
                .map(|value| {
                    let key = value.key.clone();
                    let value = value.value.clone();

                    new_objectref(Object::Array(Box::new(Array {
                        items: vec![key, value],
                    })))
                })
                .collect(),
            index: 0,
        })
    }

    pub fn has_key(&self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 1 argument for hashmap.has_key(), got: {}",
                    args.len()
                ),
                state,
            )));
        }

        let hashed_key = match args[0].borrow().hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                )));
            }
        };

        Rc::new(RefCell::new(Object::get_native_boolean_object(
            self.pairs.contains_key(&hashed_key),
        )))
    }

    pub fn remove(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 1 argument for hashmap.remove(), got: {}",
                    args.len()
                ),
                state,
            )));
        }

        let hashed_key = match args[0].borrow().hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                )));
            }
        };

        Rc::new(RefCell::new(Object::get_native_boolean_object(
            self.pairs.remove(&hashed_key).is_some(),
        )))
    }

    pub fn pop(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 && args.len() != 2 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 1 or 2 arguments for hashmap.pop(), got: {}",
                    args.len()
                ),
                state,
            )));
        }

        let hashed_key = match args[0].borrow().hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                )));
            }
        };

        if let Some(pair) = self.pairs.remove(&hashed_key) {
            return pair.value.clone();
        }

        if args.len() == 2 {
            return args[1].clone();
        }

        new_objectref(Object::new_error(
            ErrorType::ItemNotFound,
            format!("hashmap has no key: '{}'", &*args[0].borrow().inspect()),
            state,
        ))
    }

    pub fn setdefault(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 && args.len() != 2 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 1 or 2 arguments for hashmap.setdefault(), got: {}",
                    args.len()
                ),
                state,
            )));
        }

        let hashed_key = match args[0].borrow().hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                )));
            }
        };

        if let Some(pair) = self.pairs.get(&hashed_key) {
            return pair.value.clone();
        }

        let default_val = if args.len() == 2 {
            args[1].clone()
        } else {
            new_objectref(Object::NULL_OBJECT)
        };

        self.pairs.insert(
            hashed_key,
            HashPair {
                key: args[0].clone(),
                value: default_val.clone(),
            },
        );

        default_val
    }

    pub fn update(&mut self, args: &[ObjectRef], state: StateRef) -> ObjectRef {
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!(
                    "expected 1 argument for hashmap.update(), got: {}",
                    args.len()
                ),
                state,
            )));
        }

        match &*args[0].borrow() {
            Object::HashMap(hmap) => {
                for (k, pair) in &hmap.pairs {
                    self.pairs.insert(
                        k.clone(),
                        HashPair {
                            key: Object::deep_copy(pair.key.clone()),
                            value: Object::deep_copy(pair.value.clone()),
                        },
                    );
                }
                new_objectref(Object::NULL_OBJECT)
            }
            Object::Array(arr) => {
                for item in &arr.items {
                    match &*item.borrow() {
                        Object::Array(pair_arr) => {
                            if pair_arr.items.len() == 2 {
                                let key = pair_arr.items[0].clone();
                                let value = pair_arr.items[1].clone();
                                let hashed_key = match key.borrow().hash() {
                                    Ok(v) => v,
                                    Err(err_feedback) => {
                                        return Rc::new(RefCell::new(Object::new_error(
                                            ErrorType::ErrorFromPanic,
                                            err_feedback,
                                            state,
                                        )));
                                    }
                                };
                                self.pairs.insert(hashed_key, HashPair { key, value });
                            } else {
                                return Rc::new(RefCell::new(Object::new_error(
                                    ErrorType::WrongArgumentType,
                                    "expected list of pairs for hashmap.update()".into(),
                                    state,
                                )));
                            }
                        }
                        _ => {
                            return Rc::new(RefCell::new(Object::new_error(
                                ErrorType::WrongArgumentType,
                                "expected list of pairs for hashmap.update()".into(),
                                state,
                            )));
                        }
                    }
                }
                new_objectref(Object::NULL_OBJECT)
            }
            other => Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentType,
                format!(
                    "expected hashmap or array for hashmap.update(), got: {}",
                    other.get_type()
                ),
                state,
            ))),
        }
    }
}
