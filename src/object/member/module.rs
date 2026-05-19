use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    module::ModuleObject,
    panic_obj::{PanicObj, RuntimeSignal},
    stack_environment::EnvRef,
    state::StateRef,
    struct_object::StructObject,
};

impl ModuleObject {
    pub fn search_variable(&self, name: &str, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let variable = match self.table.get(name) {
            Some(obj) => obj.clone(),
            None => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::IdentifierNotFoundInModule,
                    format!("module has no identifier: '{}'", name),
                    state.clone(),
                )));
            }
        };
        Ok(variable)
    }

    pub fn search_function(
        &self,
        name: &str,
        args: &[ObjectRef],
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let function = match self.table.get(name) {
            Some(obj) => obj.clone(),
            None => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::IdentifierNotFoundInModule,
                    format!("module has no identifier: '{}'", name),
                    state.clone(),
                )));
            }
        };

        let return_value = match &*function.borrow() {
            Object::Func(function) => function.apply(name.to_string(), &args, state.clone()),
            Object::AsyncFunc(async_function) => {
                async_function.apply(name.to_string(), &args, state.clone())
            }
            Object::BuiltIn(built_in_function) => {
                built_in_function.call(&args, environ.clone(), state.clone())
            }
            Object::StructModel(_) => {
                StructObject::create_new_object(function.clone(), &args, state.clone())
            }
            other_type => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::NonfunctionalObjectCalled,
                format!(
                    "'{}' is not callable. It cannot be called.",
                    other_type.inspect()
                ),
                state.clone(),
            ))),
        };

        return_value
    }
}
