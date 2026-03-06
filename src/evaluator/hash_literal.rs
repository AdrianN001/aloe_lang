use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::{
    ast::expression::hash_map_literal::HashMapLiteral,
    object::{
        Object, ObjectRef,
        hashmap::{HashMap, HashPair},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

impl HashMapLiteral {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, String> {
        let mut pairs = BTreeMap::new();

        for (k, v) in &self.pairs {
            let key = k.evaluate(environ.clone(), state.clone())?;
            if !key.borrow().is_hashable() {
                return Err(format!(
                    "unhashable as hash key: {}",
                    key.borrow().get_type()
                ));
            }

            let value = v.evaluate(environ.clone(), state.clone())?;

            let hashed_key = key.borrow().hash()?;

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
