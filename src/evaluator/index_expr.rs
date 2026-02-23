use crate::{ast::expression::{Expression, index_expression::IndexExpression}, object::{Object, hashable::Hashable, stack_environment::StackEnvironment}};



impl IndexExpression{

    pub fn evaluate(&self, environ: &mut StackEnvironment) -> Result<Object, String>{

        let left_expr = self.left.evaluate(environ)?;
        let index = self.right.evaluate(environ)?;

        match (&left_expr, &index){
            (Object::Array(arr), Object::Int(index)) => {
                let arr_interior_value = &arr.items;
                let mut index_interior_value = index.value;

                if index_interior_value < 0{
                    index_interior_value = arr.items.len() as i64  - index_interior_value;  
                }

                if index_interior_value > (arr_interior_value.len() as i64)-1{
                    return Ok(Object::NULL_OBJECT);
                }

                Ok(
                    arr_interior_value[index_interior_value as usize].clone()
                )
            },
            (Object::HashMap(map), key_object) => {
                if !key_object.is_hashable(){
                    return Err(format!("key object is not hashable: {}", index.get_type()));
                }

                let hashed_object = match key_object{
                    Object::String(str) => str.hash(),
                    Object::Int(int) => int.hash(),
                    Object::Bool(bool) => bool.hash(),
                    _ => panic!()
                };

                if !map.pairs.contains_key(&hashed_object){
                    return Ok(Object::NULL_OBJECT);
                }

                Ok(
                    map.pairs[&hashed_object].value.clone()
                )
            }
            _ => Err(format!("index operator not supported: {}", index.get_type()))
        }
        
    }

}
