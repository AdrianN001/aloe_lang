use crate::object::{
    Object, ObjectRef,
    float_obj::FloatObj,
    integer::Integer,
    new_objectref,
    state::{self, StateRef},
};

impl Integer {
    pub fn add(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value + right_integer.value,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.value as f64 + right_float.val,
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                "+",
                other_type.get_type()
            )),
        }
    }

    pub fn sub(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value - right_integer.value,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.value as f64 - right_float.val,
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                "-",
                other_type.get_type()
            )),
        }
    }

    pub fn mul(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value * right_integer.value,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.value as f64 * right_float.val,
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                "*",
                other_type.get_type()
            )),
        }
    }

    pub fn div(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value / right_integer.value,
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: self.value as f64 / right_float.val,
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                "/",
                other_type.get_type()
            )),
        }
    }

    pub fn modulo(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value % right_integer.value,
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                "%",
                other_type.get_type()
            )),
        }
    }

    pub fn power(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::Int(Integer {
                value: self.value.pow(right_integer.value as u32),
            }))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::FloatObj(FloatObj {
                val: (self.value as f64).powf(right_float.val),
            }))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                "**",
                other_type.get_type()
            )),
        }
    }

    pub fn bool(&self) -> Result<ObjectRef, String> {
        Ok(new_objectref(Object::get_native_boolean_object(
            self.value.is_positive(),
        )))
    }

    pub fn eq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        if let Object::Int(integer) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                integer.value == self.value,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(false)))
    }

    pub fn neq(&self, right: ObjectRef) -> Result<ObjectRef, String> {
        if let Object::Int(integer) = &*right.borrow() {
            return Ok(new_objectref(Object::get_native_boolean_object(
                integer.value != self.value,
            )));
        }

        Ok(new_objectref(Object::get_native_boolean_object(true)))
    }

    pub fn lt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value < right_integer.value,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                (self.value as f64) < right_float.val,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                "<",
                other_type.get_type()
            )),
        }
    }

    pub fn le(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value <= right_integer.value,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value as f64 <= right_float.val,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                "<=",
                other_type.get_type()
            )),
        }
    }

    pub fn gt(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value > right_integer.value,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value as f64 > right_float.val,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                ">",
                other_type.get_type()
            )),
        }
    }

    pub fn ge(&self, right: ObjectRef, _state: StateRef) -> Result<ObjectRef, String> {
        match &*right.borrow() {
            Object::Int(right_integer) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value >= right_integer.value,
            ))),
            Object::FloatObj(right_float) => Ok(new_objectref(Object::get_native_boolean_object(
                self.value as f64 >= right_float.val,
            ))),
            other_type => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                ">=",
                other_type.get_type()
            )),
        }
    }
}
