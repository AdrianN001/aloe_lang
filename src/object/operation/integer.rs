use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    float_obj::FloatObj,
    integer::Integer,
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

impl Integer {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value + right_integer.value,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.value as f64 + right_float.val,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
                    "+",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value - right_integer.value,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.value as f64 - right_float.val,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
                    "-",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn mul(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value * right_integer.value,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.value as f64 * right_float.val,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
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
                        val: self.value as f64 / right_integer.value as f64,
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
                        val: self.value as f64 / right_float.val,
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
                    "int",
                    "/",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn modulo(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value % right_integer.value,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
                    "%",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn power(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value.pow(right_integer.value as u32),
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: (self.value as f64).powf(right_float.val),
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
                    "**",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, RuntimeSignal> {
        Ok(new_objectref(Object::get_native_boolean_object(
            self.value.is_positive(),
        )))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        let objects_matches = match &*right.borrow() {
            Object::Int(integer) => integer.value == self.value,
            Object::FloatObj(float) => {
                if float.val.fract() == 0.0 {
                    self.value == (float.val as i64)
                } else {
                    false
                }
            }

            _ => false,
        };

        Ok(new_objectref(Object::get_native_boolean_object(
            objects_matches,
        )))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, PanicObj> {
        let objects_matches = match &*right.borrow() {
            Object::Int(integer) => integer.value == self.value,
            Object::FloatObj(float) => {
                if float.val.fract() == 0.0 {
                    self.value == (float.val as i64)
                } else {
                    false
                }
            }

            _ => false,
        };

        Ok(new_objectref(Object::get_native_boolean_object(
            !objects_matches,
        )))
    }

    pub fn lt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value < right_integer.value,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                (self.value as f64) < right_float.val,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
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
                self.value <= right_integer.value,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value as f64 <= right_float.val,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
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
                self.value > right_integer.value,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value as f64 > right_float.val,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
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
                self.value >= right_integer.value,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value as f64 >= right_float.val,
            ))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
                    ">=",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn lshift(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value << right_integer.value,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
                    "<<",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn rshift(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value >> right_integer.value,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
                    ">>",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn band(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value & right_integer.value,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
                    "&",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn bor(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value | right_integer.value,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
                    "|",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }

    pub fn bxor(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, PanicObj> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value ^ right_integer.value,
            }))),
            other_type => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected operand types: {} {} {}",
                    "int",
                    "^",
                    other_type.get_type()
                ),
                _state.clone(),
            )),
        }
    }
}
