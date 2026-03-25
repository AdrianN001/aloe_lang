use crate::object::{
    Object, ObjectRef, error::panic_type::PanicType, float_obj::FloatObj, new_objectref,
    panic_obj::PanicObj, state::StateRef,
};

impl FloatObj {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val + right_integer.value as f64,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val + right_float.val,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "float",
                    "+",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val - right_integer.value as f64,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val - right_float.val,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "float",
                    "-",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn mul(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val * right_integer.value as f64,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val * right_float.val,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "float",
                    "*",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn div(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => {
                if right_integer.value != 0 {
                    Ok(new_objectref(Object::FloatObj(FloatObj {
                        val: self.val / right_integer.value as f64,
                    })))
                } else {
                    Err(PanicObj::new(
                        PanicType::DivisionByNull,
                        "division by 0 is not allowed".to_string(),
                        _state,
                    ))
                }
            }
            Object::FloatObj(right_float) => {
                if right_float.val != 0 as f64 {
                    Ok(new_objectref(Object::FloatObj(FloatObj {
                        val: self.val / right_float.val,
                    })))
                } else {
                    Err(PanicObj::new(
                        PanicType::DivisionByNull,
                        "division by 0 is not allowed".to_string(),
                        _state,
                    ))
                }
            }
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "float",
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
                    "float",
                    "%",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn power(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val.powf(right_integer.value as f64),
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val.powf(right_float.val),
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "float",
                    "**",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, PanicObj> {
        Ok(new_objectref(Object::get_native_boolean_object(
            self.val.is_sign_positive(),
        )))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::FloatObj(float) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                float.val.to_bits() == self.val.to_bits(),
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        if let Object::FloatObj(float) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                float.val.to_bits() != self.val.to_bits(),
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(true)))
    }

    pub fn lt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val < right_integer.value as f64,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val < right_float.val,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "float",
                    "<",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn le(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val <= right_integer.value as f64,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val <= right_float.val,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "float",
                    "<=",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn gt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val > right_integer.value as f64,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val > right_float.val,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "float",
                    ">",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn ge(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val >= right_integer.value as f64,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val >= right_float.val,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "float",
                    ">=",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }
}
