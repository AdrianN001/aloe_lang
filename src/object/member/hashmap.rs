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
            "set" => self.set(args, state),
            "get" => self.get(args, state),
            "pop" => self.pop(args, state),
            "setdefault" => self.setdefault(args, state),
            "update" => self.update(args, state),
            "remove" => self.remove(args, state),
            "clear" => self.clear(args, state),
            "has_key" => self.has_key(args, state),
            "as_iter" => self.as_iter(args, state),

            "clone" => self.deep_copy(args, state),

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

    pub fn set(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 2 arguments for hashmap.set(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let hashed_key = match args[0].borrow().hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Ok(Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                ))));
            }
        };

        self.pairs.insert(
            hashed_key,
            HashPair {
                key: args[0].clone(),
                value: args[1].clone(),
            },
        );

        Ok(args[1].clone())
    }

    pub fn get(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 && args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 1 or 2 arguments for hashmap.get(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();

        let hashed_key = match arg_borrow.hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Ok(Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                ))));
            }
        };

        if let Some(value) = self.pairs.get(&hashed_key) {
            return Ok(value.value.clone());
        }

        if args.len() == 2 {
            return Ok(args[1].clone());
        }

        Ok(new_objectref(Object::new_error(
            ErrorType::ItemNotFound,
            format!("hashmap has no key: '{}'", &*arg_borrow.inspect()),
            state,
        )))
    }

    pub fn get_panic(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 && args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 1 or 2 arguments for hashmap.get(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let arg_borrow = args[0].borrow();

        let hashed_key = match arg_borrow.hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Ok(Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                ))));
            }
        };

        if let Some(value) = self.pairs.get(&hashed_key) {
            return Ok(value.value.clone());
        }

        if args.len() == 2 {
            return Ok(args[1].clone());
        }

        Err(PanicObj::new(
            PanicType::KeyNotFound,
            format!("hashmap has no key: '{}'", &*arg_borrow.inspect()),
            state,
        ))
    }

    pub fn clear(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "hashmap.clear() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        self.pairs.clear();

        Ok(Rc::new(RefCell::new(Object::NULL_OBJECT)))
    }

    pub fn deep_copy(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "hashmap.clone() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
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

        Ok(Rc::new(RefCell::new(Object::HashMap(Box::new(HashMap {
            pairs: new_pairs,
        })))))
    }

    pub fn as_iter(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "hashmap.as_iter() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(new_objectref(Object::Iterator(Box::new(
            self.build_iterator(),
        ))))
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

    pub fn has_key(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 1 argument for hashmap.has_key(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let hashed_key = match args[0].borrow().hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Ok(Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                ))));
            }
        };

        Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
            self.pairs.contains_key(&hashed_key),
        ))))
    }

    pub fn remove(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 1 argument for hashmap.remove(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let hashed_key = match args[0].borrow().hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Ok(Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                ))));
            }
        };

        Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
            self.pairs.remove(&hashed_key).is_some(),
        ))))
    }

    pub fn pop(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 && args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 1 or 2 arguments for hashmap.pop(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let hashed_key = match args[0].borrow().hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Ok(Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                ))));
            }
        };

        if let Some(pair) = self.pairs.remove(&hashed_key) {
            return Ok(pair.value.clone());
        }

        if args.len() == 2 {
            return Ok(args[1].clone());
        }

        Ok(new_objectref(Object::new_error(
            ErrorType::ItemNotFound,
            format!("hashmap has no key: '{}'", &*args[0].borrow().inspect()),
            state,
        )))
    }

    pub fn setdefault(
        &mut self,
        args: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 && args.len() != 2 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 1 or 2 arguments for hashmap.setdefault(), got: {}",
                    args.len()
                ),
                state,
            ));
        }

        let hashed_key = match args[0].borrow().hash() {
            Ok(val) => val,
            Err(err_feedback) => {
                return Ok(Rc::new(RefCell::new(Object::new_error(
                    ErrorType::ErrorFromPanic,
                    err_feedback,
                    state,
                ))));
            }
        };

        if let Some(pair) = self.pairs.get(&hashed_key) {
            return Ok(pair.value.clone());
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

        Ok(default_val)
    }

    pub fn update(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if args.len() != 1 {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected 1 argument for hashmap.update(), got: {}",
                    args.len()
                ),
                state,
            ));
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
                Ok(new_objectref(Object::NULL_OBJECT))
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
                                        return Ok(Rc::new(RefCell::new(Object::new_error(
                                            ErrorType::ErrorFromPanic,
                                            err_feedback,
                                            state,
                                        ))));
                                    }
                                };
                                self.pairs.insert(hashed_key, HashPair { key, value });
                            } else {
                                return Err(PanicObj::new(
                                    PanicType::WrongArgumentType,
                                    "expected list of pairs for hashmap.update()".into(),
                                    state,
                                ));
                            }
                        }
                        _ => {
                            return Err(PanicObj::new(
                                PanicType::WrongArgumentType,
                                "expected list of pairs for hashmap.update()".into(),
                                state,
                            ));
                        }
                    }
                }
                Ok(new_objectref(Object::NULL_OBJECT))
            }
            other => Err(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "expected hashmap or array for hashmap.update(), got: {}",
                    other.get_type()
                ),
                state,
            )),
        }
    }
}
