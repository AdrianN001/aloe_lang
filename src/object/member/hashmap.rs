use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
    error::{error_type::ErrorType, panic_type::PanicType},
    hashmap::{HashMap, HashPair},
    integer::Integer,
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
            "remove" => Ok(self.remove(args, state)),
            "clear" => Ok(self.clear()),
            "has_key" => Ok(self.has_key(args, state)),

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
        Rc::new(RefCell::new(Object::Array(Array {
            items: self.pairs.values().map(|value| value.key.clone()).collect(),
        })))
    }

    pub fn get_values(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Array(Array {
            items: self
                .pairs
                .values()
                .map(|value| value.value.clone())
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
        if args.len() != 1 {
            return Rc::new(RefCell::new(Object::new_error(
                ErrorType::WrongArgumentCount,
                format!("expected 1 argument for hashmap.get(), got: {}", args.len()),
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

        if let Some(value) = self.pairs.get(&hashed_key) {
            return value.value.clone();
        }

        Rc::new(RefCell::new(Object::NULL_OBJECT))
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

        Rc::new(RefCell::new(Object::HashMap(HashMap { pairs: new_pairs })))
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
}
