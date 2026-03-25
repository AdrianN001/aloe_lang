use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        expression::Expression,
        statement::{Statement, struct_statement::StructStatement},
    },
    object::{
        Object, ObjectRef, error::panic_type::PanicType, new_objectref, panic_obj::PanicObj, stack_environment::EnvRef, state::StateRef, struct_model::StructModel
    },
};

impl StructStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, PanicObj> {
        let struct_name = match &self.name {
            Expression::Identifier(identifier) => identifier.value.clone(),
            other_expr => {
                return Err(PanicObj::new(
                    PanicType::IllegalExpression,
                    format!(
                        "expected an identifier expression for struct name, got: '{}'",
                        other_expr.to_string()
                    ),
                    state.clone(),
                ));
            }
        };

        let attribute_name = self.get_attribute_names_from_expression(state.clone())?;
        let methods = {
            let mut map = HashMap::new();

            self.methods
                .iter()
                .try_for_each(|stmt|{
                    match stmt{
                        Statement::Function(func_stmt) => {
                            if func_stmt.parameters.is_empty(){
                                return Err(PanicObj::new_simple(PanicType::MethodMissingThis, "expected at least 1 parameter for method (to be used as 'this'), got: 0", state.clone()))
                            }
                            let method_obj = func_stmt.evauluate_without_registering(environ.clone());
                            let name = func_stmt.name.clone();

                            map.insert(name, method_obj);
                            Ok(())
                        },
                        other_stmt => Err(PanicObj::new(PanicType::IllegalExpression, format!("expected the method the be function statement, got: '{}'", other_stmt.to_string() ), state.clone()))
                    }
                })?;
            map
        };

        let model = StructModel {
            name: struct_name.clone(),
            attributes: attribute_name,
            methods: Rc::new(RefCell::new(methods)),
        };

        let model = new_objectref(Object::StructModel(model));
        {
            let mut environ_borrow = environ.borrow_mut();

            environ_borrow.set(&struct_name, model.clone());
        }

        Ok(model)
    }

    fn get_attribute_names_from_expression(
        &self,
        state: StateRef,
    ) -> Result<Vec<String>, PanicObj> {
        let mut attrs = Vec::new();

        for attribute in &self.attributes {
            match attribute {
                Expression::Identifier(identifier) => attrs.push(identifier.value.clone()),
                other_expr => {
                    return Err(PanicObj::new(
                        PanicType::IllegalExpression,
                        format!(
                            "expected an identifier expression for struct attribute, got: '{}'",
                            other_expr.to_string()
                        ),
                        state.clone(),
                    ));
                }
            }
        }

        Ok(attrs)
    }
}
