use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::await_expression::AwaitExpression,
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        future::{future_kind::FutureKind, future_state::FutureState, task_kind::TaskKind},
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::{StateRef, scheduler::take_current_task},
    },
};

impl AwaitExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let awaitable_expression = self.expr.evaluate(environ, state.clone())?;
        let awaitable_expr_borrow = awaitable_expression.borrow();

        let future_obj = match &*awaitable_expr_borrow {
            Object::Future(future_obj) => future_obj,
            other_type => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::NonAwaitableObjectWasAwaited,
                    format!("{} is not awaitable.", other_type.get_type()),
                    state.clone(),
                )));
            }
        };

        match &future_obj.state {
            FutureState::Ready(value) => Ok(value.clone()),
            FutureState::Pending(future_kind) => {
                let current_task_rc = take_current_task().expect("no current task found");
                let mut curr_task = current_task_rc.borrow_mut();

                match future_kind {
                    FutureKind::Sleep(wait_until) => {
                        curr_task.kind = Some(TaskKind::Sleep(*wait_until))
                    }
                    _ => {}
                }
                Err(RuntimeSignal::Yield(Rc::new(RefCell::new(
                    (*curr_task).clone(),
                ))))
            }
        }
    }
}
