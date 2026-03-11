use crate::object::{
    Object, ObjectRef,
    array::Array,
    float_obj::FloatObj,
    hashmap::HashMap,
    integer::Integer,
    new_objectref,
    state::{self, StateRef},
    string_obj::StringObj,
};

impl HashMap {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                "-",
                other_type.get_type()
            )),
        }
    }

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                "-",
                other_type.get_type()
            )),
        }
    }

    pub fn mul(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                "*",
                other_type.get_type()
            )),
        }
    }

    pub fn div(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                "/",
                other_type.get_type()
            )),
        }
    }

    pub fn modulo(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                "%",
                other_type.get_type()
            )),
        }
    }

    pub fn power(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                "**",
                other_type.get_type()
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, String> {
        Ok(new_objectref(Object::get_native_boolean_object(
            !self.pairs.is_empty(),
        )))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        if let Object::HashMap(hmap) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.pairs == hmap.pairs,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        if let Object::HashMap(hmap) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.pairs != hmap.pairs,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(true)))
    }

    pub fn lt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                "<",
                other_type.get_type()
            )),
        }
    }

    pub fn le(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                "<=",
                other_type.get_type()
            )),
        }
    }

    pub fn gt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                ">",
                other_type.get_type()
            )),
        }
    }

    pub fn ge(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                ">=",
                other_type.get_type()
            )),
        }
    }
}
