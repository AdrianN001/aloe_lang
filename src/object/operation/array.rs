use crate::object::{Object, ObjectRef, array::Array, new_objectref, state::StateRef};

impl Array {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Array(arr) => {
                let mut new_arr: Vec<ObjectRef> =
                    self.items.iter().map(|item| item.clone()).collect();

                for other_element in &arr.items {
                    new_arr.push(other_element.clone());
                }

                Ok(new_objectref(Object::Array(Array { items: new_arr })))
            }
            _ => {
                let mut new_arr: Vec<ObjectRef> =
                    self.items.iter().map(|item| item.clone()).collect();

                new_arr.push(right.clone());

                Ok(new_objectref(Object::Array(Array { items: new_arr })))
            }
        }
    }

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "array",
                "-",
                other_type.get_type()
            )),
        }
    }

    pub fn mul(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "array",
                "*",
                other_type.get_type()
            )),
        }
    }

    pub fn div(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "array",
                "/",
                other_type.get_type()
            )),
        }
    }

    pub fn modulo(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "array",
                "%",
                other_type.get_type()
            )),
        }
    }

    pub fn power(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "array",
                "**",
                other_type.get_type()
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, String> {
        Ok(new_objectref(Object::get_native_boolean_object(
            !self.items.is_empty(),
        )))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        if let Object::Array(arr) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.items == arr.items,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        if let Object::Array(arr) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.items != arr.items,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(true)))
    }

    pub fn lt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "array",
                "<",
                other_type.get_type()
            )),
        }
    }

    pub fn le(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "array",
                "<=",
                other_type.get_type()
            )),
        }
    }

    pub fn gt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "array",
                ">",
                other_type.get_type()
            )),
        }
    }

    pub fn ge(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "array",
                ">=",
                other_type.get_type()
            )),
        }
    }
}
