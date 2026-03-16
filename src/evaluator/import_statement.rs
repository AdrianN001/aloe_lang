use crate::{
    ast::{expression::Expression, statement::import_statement::ImportStatement},
    module::{
        ModuleRef,
        module_loader::{self, ModuleLoader},
    },
    object::{
        Object, ObjectRef, new_objectref, panic_obj::PanicObj, stack_environment::EnvRef,
        state::StateRef,
    },
};

impl ImportStatement {
    pub fn evaluate(
        &self,
        _environ: EnvRef,
        _state: StateRef,
        module_loader: &mut ModuleLoader,
    ) -> Result<ObjectRef, PanicObj> {
        let _imported_identifiers =
            Self::get_identifier_expressions(&self.identifiers, _state.clone())?;
        let _path = self.module_name.clone();

        let resolved_module = module_loader.import_module(&_path);

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
