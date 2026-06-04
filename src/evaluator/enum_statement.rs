use std::collections::HashMap;

use crate::{
    ast::{expression::Expression, statement::enum_statement::EnumStatement},
    object::{
        Object, ObjectRef,
        enum_model::EnumModel,
        error::panic_type::PanicType,
        new_objectref,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

impl EnumStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }
        let name = match &self.name {
            Expression::Identifier(identifier) => identifier.value.clone(),
            other_expr => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::WrongSyntax,
                    format!("expected identifier, got: {}", other_expr.to_string()),
                    state,
                )));
            }
        };

        let values = self
            .values
            .iter()
            .enumerate()
            .map(|(indx, expr)| match expr {
                Expression::Identifier(identifier) => Ok((identifier.value.clone(), indx)),
                other_expr => {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::WrongSyntax,
                        format!("expected identifier, got: {}", other_expr.to_string()),
                        state.clone(),
                    )));
                }
            })
            .collect::<Result<Vec<(String, usize)>, RuntimeSignal>>()?;

        let map = {
            let mut map = HashMap::new();

            values.iter().for_each(|(value, indx)| {
                let _ = map.insert(value.clone(), *indx);
            });

            map
        };

        let new_enum_object = new_objectref(Object::EnumModel(Box::new(EnumModel {
            name: name.clone(),
            values: map,
        })));

        {
            environ
                .borrow_mut()
                .set_to_lowest_level(&name, new_enum_object.clone());
        }

        Ok(new_enum_object)
    }
}
