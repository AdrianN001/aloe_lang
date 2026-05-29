use crate::{
    ast::expression::{Expression, await_expression::AwaitExpression},
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        future::{future_kind::FutureKind, future_state::FutureState},
        panic_obj::{PanicObj, RuntimeSignal},
        state::StateRef,
    },
    scheduler::{send_task_to_scheduler, send_task_to_sleeper_scheduler, take_current_task},
};

impl AwaitExpression {
    pub fn eval2(
        expr: &Expression,
        future_ref: ObjectRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let mut future_ref_borrow = future_ref.borrow_mut();

        let await_expr = {
            match &expr {
                Expression::AwaitExpr(await_expr) => await_expr,
                _ => unreachable!(),
            }
        };

        let future_obj = match &mut *future_ref_borrow {
            Object::Future(future_obj) => future_obj,
            Object::ReturnVal(_) => return Ok(future_ref.clone()), //progagation
            other_type => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::NonAwaitableObjectWasAwaited,
                    format!("{} is not awaitable.", other_type.get_type()),
                    state.clone(),
                )));
            }
        };

        let current_task_rc = take_current_task().expect("no current task found");

        match &future_obj.state {
            FutureState::Ready(finished_object) => await_expr
                .handle_return_value_according_the_expression(finished_object.clone(), state),
            FutureState::Pending(type_of_future) => {
                match type_of_future {
                    FutureKind::Value(task) => {
                        future_obj.waiters.push(current_task_rc.clone());

                        send_task_to_scheduler(task.clone())
                    }
                    FutureKind::IO => {
                        future_obj.waiters.push(current_task_rc.clone());
                    }
                    FutureKind::Sleep(instant) => {
                        send_task_to_sleeper_scheduler(current_task_rc.clone(), *instant);
                    }
                }
                Ok(future_ref.clone())
            }
            _ => unreachable!(),
        }
    }

    pub fn handle_return_value_according_the_expression(
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
                        return Err(RuntimeSignal::Propagation(return_value.clone()));
                    }
                }
                Ok(return_value)
            }
            _ => Ok(return_value),
        }
    }
}
