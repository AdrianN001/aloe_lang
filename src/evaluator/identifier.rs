use crate::{
    ast::expression::identifier::Identifier,
    object::{Object, built_in::BuiltIn, stack_environment::StackEnvironment},
};

impl Identifier {
    fn get_builtin_from_identifier(&self) -> Option<BuiltIn> {
        match self.value.as_str() {
            "len" => Some(BuiltIn::Len),

            "rest" => Some(BuiltIn::Rest),
            "first" => Some(BuiltIn::First),
            "last" => Some(BuiltIn::Last),
            "push" => Some(BuiltIn::Push),

            "print" => Some(BuiltIn::Print),
            _ => None,
        }
    }

    pub fn evaluate(&self, environ: &StackEnvironment) -> Result<Object, String> {
        match environ.get_owned(&self.value) {
            Some(obj) => Ok(obj),
            None => {
                if let Some(built_in) = self.get_builtin_from_identifier() {
                    return Ok(Object::BuiltIn(built_in));
                }

                Err(format!("unknown identifier: {}", &self.value))
            }
        }
    }
}
