use std::collections::HashMap;

use crate::object::{
    ObjectRef,
    error::panic_type::PanicType,
    panic_obj::{PanicObj, RuntimeSignal},
    stack_environment::EnvRef,
    state::StateRef,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ModuleObject {
    pub table: HashMap<String, ObjectRef>,
    origin: String,
}

impl ModuleObject {
    pub fn get_type(&self) -> String {
        String::from("<type 'module'>")
    }

    pub fn inspect(&self) -> String {
        format!("[Module for file '{}']", self.origin)
    }
}

impl ModuleObject {
    pub fn new(
        identifiers: &[String],
        module_name: &str,
        module_environ: EnvRef,
        state: StateRef,
    ) -> Result<Self, RuntimeSignal> {
        let mut table = HashMap::new();
        let other_environ_borrow = module_environ.borrow();

        for identifier in identifiers {
            let exported_identifier = match other_environ_borrow.get(identifier) {
                Some(export) => export,
                None => {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::IdentifierNotFoundInModule,
                        format!("module has no identifier: '{}'", identifier),
                        state.clone(),
                    )));
                }
            };

            table.insert(identifier.clone(), exported_identifier.clone());
        }

        Ok(Self {
            table,
            origin: module_name.to_string(),
        })
    }
}
