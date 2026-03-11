use crate::object::{Object, ObjectRef, boolean::Boolean, new_objectref, state::StateRef};

impl Boolean {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "+",
                other_type.get_type()
            )),
        }
    }

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "-",
                other_type.get_type()
            )),
        }
    }

    pub fn mul(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "*",
                other_type.get_type()
            )),
        }
    }

    pub fn div(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "/",
                other_type.get_type()
            )),
        }
    }

    pub fn modulo(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "%",
                other_type.get_type()
            )),
        }
    }

    pub fn power(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "**",
                other_type.get_type()
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, String> {
        Ok(new_objectref(Object::get_native_boolean_object(self.value)))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        if let Object::Bool(bool) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.value == bool.value,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        if let Object::Bool(bool) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.value != bool.value,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(true)))
    }

    pub fn lt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "<",
                other_type.get_type()
            )),
        }
    }

    pub fn le(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "<=",
                other_type.get_type()
            )),
        }
    }

    pub fn gt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                ">",
                other_type.get_type()
            )),
        }
    }

    pub fn ge(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                ">=",
                other_type.get_type()
            )),
        }
    }

    // boolean operations

    pub fn land(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Bool(other_bool) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value && other_bool.value,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "&&",
                other_type.get_type()
            )),
        }
    }

    pub fn lor(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Bool(other_bool) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value || other_bool.value,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "||",
                other_type.get_type()
            )),
        }
    }

    pub fn lxor(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Bool(other_bool) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value ^ other_bool.value,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "^",
                other_type.get_type()
            )),
        }
    }

    pub fn band(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Bool(right_bool) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value & right_bool.value,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "&",
                other_type.get_type()
            )),
        }
    }

    pub fn bor(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Bool(right_bool) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value | right_bool.value,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                "|",
                other_type.get_type()
            )),
        }
    }
}
