use crate::object::{
    Object, ObjectRef, array::Array, error::panic_type::PanicType, new_objectref,
    panic_obj::PanicObj, state::StateRef,
};

impl Array {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
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

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "array",
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
                    "array",
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
                    "array",
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
                    "array",
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
                    "array",
                    "**",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, PanicObj> {
        Ok(new_objectref(Object::get_native_boolean_object(
            !self.items.is_empty(),
        )))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::Array(arr) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.items == arr.items,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::Array(arr) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                self.items != arr.items,
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
                    "array",
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
                    "array",
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
                    "array",
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
                    "array",
                    ">=",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }
}
