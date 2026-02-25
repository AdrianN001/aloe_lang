use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::{
    ast::expression::hash_map_literal::HashMapLiteral,
    object::{
        Object, ObjectRef,
        hashable::Hashable,
        hashmap::{HashMap, HashPair},
        stack_environment::EnvRef,
    },
};

impl HashMapLiteral {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        let mut pairs = BTreeMap::new();

        for (k, v) in &self.pairs {
            let key = k.evaluate(environ.clone())?;
            if !key.borrow().is_hashable() {
                return Err(format!(
                    "unhashable as hash key: {}",
                    key.borrow().get_type()
                ));
            }

            let value = v.evaluate(environ.clone())?;

            let hashed_key = match &*key.borrow() {
                Object::Int(int) => int.hash(),
                Object::String(str) => str.hash(),
                Object::Bool(bool) => bool.hash(),
                _ => {
                    return Err(format!(
                        "unhashable as hash key: {}",
                        key.borrow().get_type()
                    ));
                }
            };
            pairs.insert(
                hashed_key,
                HashPair {
                    key: key.clone(),
                    value: value.clone(),
                },
            );
        }

        Ok(Rc::new(RefCell::new(Object::HashMap(HashMap { pairs }))))
    }
}
