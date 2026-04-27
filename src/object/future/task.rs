use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::statement::Statement,
    object::{
        Object, ObjectRef, future::task_kind::TaskKind, new_objectref, panic_obj::RuntimeSignal,
        stack_environment::EnvRef, state::StateRef,
    },
};

pub type TaskRef = Rc<RefCell<Task>>;

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct Task {
    pub statement_index: usize,
    pub statements: Vec<Statement>,

    pub name: String,
    pub last_object: Option<ObjectRef>,

    pub kind: Option<TaskKind>,

    pub environ: EnvRef,
    pub state: StateRef,
}
impl Task {
    pub fn run(self_as_ref: TaskRef) -> Result<ObjectRef, RuntimeSignal> {
        loop {
            // 1. Daten holen (kurz borrow)
            let (stmt, env, state) = {
                let task = self_as_ref.borrow();

                if task.statement_index >= task.statements.len() {
                    return Ok(task
                        .last_object
                        .clone()
                        .unwrap_or_else(|| new_objectref(Object::NULL_OBJECT)));
                }

                (
                    task.statements[task.statement_index].clone(),
                    task.environ.clone(),
                    task.state.clone(),
                )
            };
            println!("stmt: {}", stmt.to_string());

            // 2. evaluate ohne borrow
            let result = stmt.evaluate(env, state);

            match result {
                Ok(ok_value) => {
                    match &*ok_value.borrow() {
                        Object::ReturnVal(ret_val) => {
                            return Ok(*ret_val.value.clone());
                        }
                        _ => {
                            // 3. schreiben (kurz borrow_mut)
                            let mut task = self_as_ref.borrow_mut();
                            task.last_object = Some(ok_value.clone());
                            task.statement_index += 1;
                        }
                    }
                }

                Err(RuntimeSignal::Yield(y)) => {
                    let mut task = self_as_ref.borrow_mut();
                    task.statement_index += 1;
                    return Err(RuntimeSignal::Yield(y));
                }

                Err(RuntimeSignal::Panic(p)) => {
                    return Err(RuntimeSignal::Panic(p));
                }
            }
        }
    }
}
