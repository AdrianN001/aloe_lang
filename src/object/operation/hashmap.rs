use crate::object::{
    Object, ObjectRef, error::panic_type::PanicType, hashmap::HashMap, new_objectref,
    panic_obj::PanicObj, state::StateRef,
};

impl HashMap {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "hashmap",
                    "-",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "hashmap",
                    "-",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn mul(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "hashmap",
                    "*",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn div(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "hashmap",
                    "/",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn modulo(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "hashmap",
                    "%",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn power(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "hashmap",
                    "**",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, PanicObj> {
        Ok(new_objectref(Object::get_native_boolean_object(
            !self.pairs.is_empty(),
        )))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::HashMap(hmap) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.pairs == hmap.pairs,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::HashMap(hmap) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.pairs != hmap.pairs,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(true)))
    }

    pub fn lt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "hashmap",
                    "<",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn le(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "hashmap",
                    "<=",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn gt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "hashmap",
                    ">",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn ge(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "hashmap",
                    ">=",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }
}
