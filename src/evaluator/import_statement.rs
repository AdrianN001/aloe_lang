use crate::{
    ast::{expression::Expression, statement::import_statement::ImportStatement},
    module::{ModuleRef, module_loader::ModuleLoader},
    object::{
        Object, ObjectRef, error::panic_type::PanicType, new_objectref, panic_obj::PanicObj, stack_environment::EnvRef, state::StateRef
    },
};

impl ImportStatement {
    pub fn evaluate(
        &self,
        _environ: EnvRef,
        _state: StateRef,
        module_loader: &mut ModuleLoader,
    ) -> Result<ObjectRef, PanicObj> {
        let imported_identifiers =
            Self::get_identifier_expressions(&self.identifiers, _state.clone())?;
        let path = self.module_name.clone();

        let resolved_module = match module_loader.import_module(&path) {
            Ok(module) => module,
            Err(err_feedback) => return Err(PanicObj::new(PanicType::ModuleCouldNotBeLoaded, err_feedback.value, _state.clone())),
        };

        Self::load_exports_from_module(
            resolved_module,
            &imported_identifiers,
            _environ.clone(),
            _state.clone(),
        )?;

        Ok(new_objectref(Object::NULL_OBJECT))
    }

    fn get_identifier_expressions(
        expressions: &[Expression],
        state: StateRef,
    ) -> Result<Vec<String>, PanicObj> {
        let mut res = Vec::new();

        for expr in expressions {
            match expr {
                Expression::Identifier(identifier) => res.push(identifier.value.clone()),
                other_expr => {
                    return Err(PanicObj::new(
                        PanicType::MissingIdentifier,
                        format!(
                            "expected identifier in import, got: '{}'",
                            other_expr.to_string()
                        ),
                        state.clone(),
                    ));
                }
            }
        }

        Ok(res)
    }

    fn load_exports_from_module(
        module_ref: ModuleRef,
        identifiers: &[String],
        own_environ: EnvRef,
        state: StateRef,
    ) -> Result<(), PanicObj> {
        let module_env = {
            let module_borrow = module_ref.borrow();

            match &module_borrow.environ {
                Some(environ) => environ.clone(),
                None => {
                    return Err(PanicObj::new_simple(
                        PanicType::ModuleCouldNotBeExecuted,
                        "module was not executed",
                        state.clone(),
                    ));
                }
            }
        };

        let module_env_borrow = module_env.borrow();
        let mut own_environ_borrow = own_environ.borrow_mut();

        for identifier in identifiers {
            let exported_identifier = match module_env_borrow.get(identifier) {
                Some(export) => export,
                None => {
                    return Err(PanicObj::new(
                        PanicType::IdentifierNotFoundInModule,
                        format!("module has no identifier: '{}'", identifier),
                        state.clone(),
                    ));
                }
            };

            own_environ_borrow.set(identifier, exported_identifier.clone());
        }

        Ok(())
    }
}
