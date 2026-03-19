use std::collections::HashMap;

use crate::object::{Object, ObjectRef, new_objectref, panic_obj::PanicObj, state::StateRef, struct_model::MethodTableRef};


#[derive(PartialEq, Eq, Clone)]
pub struct StructObject{
    pub model: ObjectRef,
    pub model_name: String, 

    pub attribute_table: HashMap<String, ObjectRef>,
    pub method_table: MethodTableRef,
}

impl StructObject{

    pub fn init_new(model: ObjectRef, args: &[ObjectRef] ,state: StateRef) -> Result<ObjectRef, PanicObj>{
        let (attribute_table, method_table, name)  = {
            let borrow = model.borrow();
            let model_raw = match &*borrow{
                Object::StructModel(struct_model) => struct_model,
                other_type => return Err(PanicObj::new(format!("expected to be the model a 'Struct Model', got: {}", other_type.inspect()), state))
            };

            let mut attribute_map = HashMap::new();

            model_raw.attributes.iter().for_each(|attribute_name|{
                attribute_map.insert(attribute_name.clone(), new_objectref(Object::NULL_OBJECT));
            });

            (attribute_map, model_raw.methods.clone(), model_raw.name.clone())
        };

        Ok(new_objectref(Object::StructObject(Self{
            model: model.clone(),
            model_name: name, 

            attribute_table,
            method_table
        })))
    }

    pub fn inspect(&self) -> String{
        panic!()
    }

    pub fn get_type(&self) -> String{
        format!("<struct '{}'>", self.model_name)
    }

}
