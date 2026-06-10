use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, boolean::Boolean, error::panic_type::PanicType, float_obj::FloatObj,
    integer::Integer, new_objectref, panic_obj::PanicObj, stack_environment::EnvRef,
    state::StateRef, string_obj::StringObj,
};

impl FloatObj {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "is_negative" => Ok(self.is_negative()),
            "is_positive" => Ok(self.is_positive()),

            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for float: '{}'", name),
                state,
            )),
        }
    }
    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "as_str" => self.as_str(args, state),
            "as_int" => self.as_int(args, state),
            "as_float" => self.as_float(args, state),
            "clone" => self.deep_copy(args, state),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for float: '{}'", name),
                state,
            )),
        }
    }

    // Attributes

    pub fn is_negative(&self) -> ObjectRef {
        new_objectref(Object::Bool(Boolean {
            value: self.val.is_sign_negative(),
        }))
    }

    pub fn is_positive(&self) -> ObjectRef {
        new_objectref(Object::Bool(Boolean {
            value: self.val.is_sign_positive(),
        }))
    }

    // Methods

    pub fn as_str(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "float.as_str() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(Rc::new(RefCell::new(Object::String(Box::new(StringObj {
            value: self.val.to_string(),
        })))))
    }

    pub fn as_int(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "float.as_int() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(Rc::new(RefCell::new(Object::Int(Integer {
            value: self.val as i64,
        }))))
    }

    pub fn as_float(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "float.as_float() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(new_objectref(Object::FloatObj(FloatObj { val: self.val })))
    }

    pub fn deep_copy(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "float.clone() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
            val: self.val,
        }))))
    }
}
