use crate::{ast::{expression::Expression, statement::struct_statement::StructStatement}, object::{Object, ObjectRef, new_objectref, panic_obj::PanicObj, stack_environment::EnvRef, state::StateRef, struct_model::StructModel}};



impl StructStatement{

    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, PanicObj>{
        let struct_name = match &self.name{
            Expression::Identifier(identifier) => identifier.value.clone(),
            other_expr => return Err(PanicObj::new(format!("expected an identifier expression for struct name, got: '{}'", other_expr.to_string()), state.clone()))
        };

        let attribute_name = self.get_attribute_names_from_expression(state.clone())?;

        let model = StructModel{
            name: struct_name.clone(),
            attributes: attribute_name,
            methods: Vec::new()
        };

        {
            let mut environ_borrow = environ.borrow_mut(); 

            environ_borrow.set(&struct_name, new_objectref(Object::StructModel(model)));
        }


        Ok(new_objectref(Object::NULL_OBJECT))
    }

    fn get_attribute_names_from_expression(&self, state: StateRef) -> Result<Vec<String>, PanicObj>{
        let mut attrs = Vec::new();

        for attribute in &self.attributes{
            match attribute{
                Expression::Identifier(identifier) => attrs.push(identifier.value.clone()),
                other_expr => return Err(
                    PanicObj::new(
                        format!("expected an identifier expression for struct attribute, got: '{}'", other_expr.to_string()), state.clone()
                    ))
            }
        }

        Ok(attrs)
    }
}
