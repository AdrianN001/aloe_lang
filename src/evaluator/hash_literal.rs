use std::collections::BTreeMap;

use crate::{ast::expression::hash_map_literal::HashMapLiteral, object::{Object, hashable::Hashable, hashmap::{HashMap, HashPair}, stack_environment::StackEnvironment}};



impl HashMapLiteral{

    pub fn evaluate(&self, environ: &mut StackEnvironment) -> Result<Object, String>{
        let mut pairs = BTreeMap::new();

        for (k,v) in &self.pairs{
            let key = k.evaluate(environ)?;
            if !key.is_hashable(){
                return Err(format!("unhashable as hash key: {}", key.get_type()));
            }

            let value = v.evaluate(environ)?;

            let hashed_key = match &key{
                Object::Int(int) =>     int.hash(),
                Object::String(str) =>  str.hash(),
                Object::Bool(bool) =>   bool.hash(),
                _ => return Err(format!("unhashable as hash key: {}", key.get_type()))
            };
            pairs.insert(
                hashed_key,
                HashPair{ key, value }
            );
        }

        Ok(Object::HashMap(HashMap{
            pairs
        }))
    }
}
