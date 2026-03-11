use crate::object::{
    Object, ObjectRef,
    float_obj::FloatObj,
    new_objectref,
    state::StateRef,
};

impl FloatObj {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val + right_integer.value as f64,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val + right_float.val,
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                "+",
                other_type.get_type()
            )),
        }
    }

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val - right_integer.value as f64,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val as f64 - right_float.val,
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                "-",
                other_type.get_type()
            )),
        }
    }

    pub fn mul(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val * right_integer.value as f64,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val as f64 * right_float.val,
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                "*",
                other_type.get_type()
            )),
        }
    }

    pub fn div(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val as f64 * right_integer.value as f64,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val as f64 * right_float.val,
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                "/",
                other_type.get_type()
            )),
        }
    }

    pub fn modulo(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                "%",
                other_type.get_type()
            )),
        }
    }

    pub fn power(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val.powf(right_integer.value as f64),
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.val.powf(right_float.val),
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                "**",
                other_type.get_type()
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, String> {
        Ok(new_objectref(Object::get_native_boolean_object(
            self.val.is_sign_positive(),
        )))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        if let Object::FloatObj(float) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                float.val.to_bits() == self.val.to_bits(),
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        if let Object::FloatObj(float) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                float.val.to_bits() != self.val.to_bits(),
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(true)))
    }

    pub fn lt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val < right_integer.value as f64,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val < right_float.val,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                "<",
                other_type.get_type()
            )),
        }
    }

    pub fn le(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val <= right_integer.value as f64,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val <= right_float.val,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                "<=",
                other_type.get_type()
            )),
        }
    }

    pub fn gt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val > right_integer.value as f64,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val > right_float.val,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                ">",
                other_type.get_type()
            )),
        }
    }

    pub fn ge(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val >= right_integer.value as f64,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.val >= right_float.val,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                ">=",
                other_type.get_type()
            )),
        }
    }
}
