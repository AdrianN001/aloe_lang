use crate::object::{
    Object, ObjectRef, error::panic_type::PanicType, function::Function, integer::Integer,
    new_objectref, panic_obj::PanicObj, stack_environment::EnvRef, state::StateRef,
};

impl Function {
    pub fn apply_attribute(
        &self,
        name: &str,
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "arity" => self.get_arity(),
            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for function: '{}'", name),
                state,
            )),
        }
    }
    pub fn apply_method(
        &mut self,
        name: &str,
        _args: &[ObjectRef],
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for function: '{}'", name),
                state,
            )),
        }
    }

    // Attributes
    fn get_arity(&self) -> Result<ObjectRef, PanicObj> {
        let arity = self.parameters.len();

        Ok(new_objectref(Object::Int(Integer {
            value: arity as i64,
        })))
    }
}
