
use crate::{
    ast::{expression::Expression, statement::import_statement::ImportStatement},
    object::{Object, ObjectRef, new_objectref, stack_environment::EnvRef, state::StateRef},
};

impl ImportStatement {
    pub fn evaluate(&self, _environ: EnvRef, _state: StateRef) -> Result<ObjectRef, String> {
        let _imported_identifiers = Self::get_identifier_expressions(&self.identifiers)?;
        let _path = self.module_name.clone();

        Ok(new_objectref(Object::NULL_OBJECT))
    }

    fn get_identifier_expressions(expressions: &[Expression]) -> Result<Vec<String>, String> {
        let mut res = Vec::new();

        for expr in expressions {
            match expr {
                Expression::Identifier(identifier) => res.push(identifier.value.clone()),
                _ => return Err("expected identifier".into()),
            }
        }

        Ok(res)
    }
}
