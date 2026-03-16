use crate::object::{
    Object, ObjectRef, new_objectref, panic_obj::PanicObj, state::StateRef, string_obj::StringObj,
};

impl StringObj {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::String(str_obj) => Ok(new_objectref(Object::String(StringObj {
                value: self.value.clone() + &str_obj.value,
            }))),
            other_type => Err(PanicObj::new(
                format!(
                    "unexpected operand types: {} {} {}",
                    "string",
                    "+",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                format!(
                    "unexpected operand types: {} {} {}",
                    "string",
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
                format!(
                    "unexpected operand types: {} {} {}",
                    "string",
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
                format!(
                    "unexpected operand types: {} {} {}",
                    "string",
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
                format!(
                    "unexpected operand types: {} {} {}",
                    "string",
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
                format!(
                    "unexpected operand types: {} {} {}",
                    "string",
                    "**",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, PanicObj> {
        Ok(new_objectref(Object::get_native_boolean_object(
            !self.value.is_empty(),
        )))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::String(string) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                string.value == self.value,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::String(string) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                string.value != self.value,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(true)))
    }

    pub fn lt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                format!(
                    "unexpected operand types: {} {} {}",
                    "string",
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
                format!(
                    "unexpected operand types: {} {} {}",
                    "string",
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
                format!(
                    "unexpected operand types: {} {} {}",
                    "string",
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
                format!(
                    "unexpected operand types: {} {} {}",
                    "string",
                    ">=",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }
}
