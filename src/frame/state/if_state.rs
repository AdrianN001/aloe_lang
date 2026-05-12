use std::{cell::RefCell, rc::Rc};

use crate::object::stack_environment::{EnvRef, StackEnvironment};

#[derive(Debug, Clone)]
pub struct IfState {
    // environment
    pub environ: EnvRef,

    // path search
    pub current_path: usize,
    pub path_found: bool,
}

impl IfState {
    pub fn new(environ: EnvRef) -> Self {
        let new_environ = {
            let raw = StackEnvironment::new_enclosed(environ, "if expression".to_string());
            Rc::new(RefCell::new(raw))
        };

        Self {
            environ: new_environ,
            current_path: 0,
            path_found: false,
        }
    }
}
