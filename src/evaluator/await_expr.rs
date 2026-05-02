use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::{Expression, await_expression::AwaitExpression},
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        future::{future_kind::FutureKind, future_state::FutureState, task_kind::TaskKind},
        new_objectref,
        panic_obj::{PanicObj, RuntimeSignal},
        return_value::ReturnValue,
        stack_environment::EnvRef,
        state::{StateRef, scheduler::take_current_task},
    },
};

impl AwaitExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let future_to_await = {
            let current_task_rc = take_current_task().expect("no current task found");
            let curr_task = current_task_rc.borrow();

            if let Some(pending_future) = &curr_task.pending_future {
                pending_future.clone()
            } else {
                self.expr.evaluate(environ, state.clone())?
            }
        };

        let mut awaitable_expr_borrow = future_to_await.borrow_mut();

        let future_obj = match &mut *awaitable_expr_borrow {
            Object::Future(future_obj) => future_obj,
            Object::ReturnVal(_) => return Ok(future_to_await.clone()), //progagation
            other_type => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::NonAwaitableObjectWasAwaited,
                    format!("{} is not awaitable.", other_type.get_type()),
                    state.clone(),
                )));
            }
        };

        match &future_obj.state {
            FutureState::Ready(value) => {
                {
                    take_current_task()
                        .expect("no current task found")
                        .borrow_mut()
                        .pending_future = None;
                }
                self.handle_return_value_according_the_expression(value.clone(), state)
            }
            FutureState::Pending(future_kind) => {
                let current_task_rc = take_current_task().expect("no current task found");
                let mut curr_task = current_task_rc.borrow_mut();

                match future_kind {
                    FutureKind::Sleep(wait_until) => {
                        if curr_task.pending_future.is_none() {
                            curr_task.kind = Some(TaskKind::Sleep(*wait_until));
                            curr_task.pending_future = Some(future_to_await.clone());
                        }
                    }
                    FutureKind::Value(task) => {
                        curr_task.pending_future = Some(future_to_await.clone());
                        future_obj.waiters.push(current_task_rc.clone());
                        curr_task.kind = Some(TaskKind::ValueJoin(task.clone()))
                    }
                    FutureKind::FileIO => {
                        curr_task.pending_future = Some(future_to_await.clone());
                        future_obj.waiters.push(current_task_rc.clone());
                        curr_task.kind = Some(TaskKind::FileIOJoin);
                    }
                }
                Err(RuntimeSignal::Yield(Rc::new(RefCell::new(
                    (*curr_task).clone(),
                ))))
            }
            _ => panic!(),
        }
    }

    fn handle_return_value_according_the_expression(
        &self,
        return_value: ObjectRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        match &*self.expr {
            Expression::Call(call_expr) => {
                if let Object::Err(err) = &*return_value.borrow() {
                    if call_expr.bang_set {
                        return Err(RuntimeSignal::Panic(PanicObj::from_error(
                            err,
                            state.clone(),
                        )));
                    } else if call_expr.question_mark_set {
                        return Ok(new_objectref(Object::ReturnVal(ReturnValue {
                            value: Box::new(return_value.clone()),
                        })));
                    }
                }
                Ok(return_value)
            }
            _ => Ok(return_value),
        }
    }
}
