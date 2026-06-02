use crate::object::{
    Object, ObjectRef,
    error::{Error, panic_type::PanicType},
    new_objectref,
    panic_obj::PanicObj,
    stack_environment::EnvRef,
    state::StateRef,
    string_obj::StringObj,
};

impl Error {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "message" => Ok(self.message()),

            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for error: '{}'", name),
                state,
            )),
        }
    }
    pub fn apply_method(
        &self,
        name: &str,
        args: &[ObjectRef],
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "as_str" => self.to_str(args, state),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for error: '{}'", name),
                state,
            )),
        }
    }

    // attributes
    pub fn message(&self) -> ObjectRef {
        new_objectref(Object::String(Box::new(StringObj {
            value: self.value.clone(),
        })))
    }

    // methods

    pub fn to_str(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "error.as_str() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }

        Ok(self.message())
    }
}
