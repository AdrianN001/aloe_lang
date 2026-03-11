use crate::object::{
    Object, ObjectRef,
    array::Array,
    float_obj::FloatObj,
    hashmap::HashMap,
    integer::Integer,
    iterator::Iterator,
    new_objectref,
    state::{self, StateRef},
    string_obj::StringObj,
};

impl Iterator {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                "-",
                other_type.get_type()
            )),
        }
    }

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                "-",
                other_type.get_type()
            )),
        }
    }

    pub fn mul(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                "*",
                other_type.get_type()
            )),
        }
    }

    pub fn div(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                "*",
                other_type.get_type()
            )),
        }
    }

    pub fn modulo(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                "*",
                other_type.get_type()
            )),
        }
    }

    pub fn power(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                "*",
                other_type.get_type()
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, String> {
        Ok(self._has_next())
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                "==",
                other_type.get_type()
            )),
        }
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                "!=",
                other_type.get_type()
            )),
        }
    }

    pub fn lt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                "<",
                other_type.get_type()
            )),
        }
    }

    pub fn le(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                "<=",
                other_type.get_type()
            )),
        }
    }

    pub fn gt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                ">",
                other_type.get_type()
            )),
        }
    }

    pub fn ge(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "iterator",
                ">=",
                other_type.get_type()
            )),
        }
    }
}
