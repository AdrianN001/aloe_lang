use crate::object::{
    Object, ObjectRef, boolean::Boolean, error::panic_type::PanicType, new_objectref, panic_obj::PanicObj, state::StateRef
};

impl Boolean {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                 format!(
                    "unexpected operand types: {} {} {}",
                    "boolean",
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
                PanicType::OperatorIsNotSupported,
                 format!(
                    "unexpected operand types: {} {} {}",
                    "boolean",
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
                    "boolean",
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
                    "boolean",
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
                    "boolean",
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
                    "boolean",
                    "**",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, PanicObj> {
        Ok(new_objectref(Object::get_native_boolean_object(self.value)))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::Bool(bool) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.value == bool.value,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::Bool(bool) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.value != bool.value,
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
                    "boolean",
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
                    "boolean",
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
                    "boolean",
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
                    "boolean",
                    ">=",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    // boolean operations

    pub fn land(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Bool(other_bool) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value && other_bool.value,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                 format!(
                    "unexpected operand types: {} {} {}",
                    "boolean",
                    "&&",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn lor(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Bool(other_bool) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value || other_bool.value,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                 format!(
                    "unexpected operand types: {} {} {}",
                    "boolean",
                    "||",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn lxor(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Bool(other_bool) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value ^ other_bool.value,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                 format!(
                    "unexpected operand types: {} {} {}",
                    "boolean",
                    "^",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn band(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Bool(right_bool) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value & right_bool.value,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                 format!(
                    "unexpected operand types: {} {} {}",
                    "boolean",
                    "&",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn bor(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Bool(right_bool) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value | right_bool.value,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                 format!(
                    "unexpected operand types: {} {} {}",
                    "boolean",
                    "|",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }
}
