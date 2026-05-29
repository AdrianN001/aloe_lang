use crate::{
    ast::statement::break_statement::BreakStatement,
    object::{
        Object, ObjectRef, break_value::BreakValue, new_objectref, panic_obj::RuntimeSignal,
        stack_environment::EnvRef, state::StateRef,
    },
};

impl BreakStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let val = if let Some(break_expression_value) = &self.expression {
            break_expression_value
                .evaluate(environ.clone(), state)?
                .clone()
        } else {
            new_objectref(Object::NULL_OBJECT)
        };

        Ok(new_objectref(Object::BreakVal(BreakValue {
            value: Box::new(val),
        })))
    }
}
