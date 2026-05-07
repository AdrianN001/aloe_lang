use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::{Expression, await_expression::AwaitExpression},
    frame::state::{ExpressionState, await_state::AwaitState, call_state::CallState},
    object::{ObjectRef, panic_obj::RuntimeSignal, state::StateRef},
};

pub type ExprFrameRef = Rc<RefCell<ExpressionFrame>>;

#[derive(Debug)]
pub struct ExpressionFrame {
    pub expr: Expression,
    pub state: ExpressionState,
}

impl ExpressionFrame {
    pub fn new_await_frame(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::Await {
                future: None,
                state: AwaitState::Start,
            },
        }
    }

    pub fn new_primitive(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::Primitive,
        }
    }

    pub fn new_functioncall_frame(expr: Expression) -> Self {
        let params_n = {
            match &expr {
                Expression::Call(call_expr) => call_expr.arguments.len(),
                _ => todo!(),
            }
        };

        Self {
            expr,
            state: ExpressionState::Call {
                state: CallState {
                    parameters: vec![],
                    parameters_required_by_func: params_n,
                    current_argument: 0,
                },
                ready_to_evaluate: false,
            },
        }
    }

    pub fn to_ref(self) -> ExprFrameRef {
        Rc::new(RefCell::new(self))
    }
}

impl ExpressionFrame {
    pub fn resume_with(
        &mut self,
        object: ObjectRef,
        interpreter_state: StateRef,
    ) -> Result<(), RuntimeSignal> {
        match &mut self.state {
            ExpressionState::Call {
                ready_to_evaluate,
                state,
            } => {
                state.parameters.push(object);

                let params_needed = state.parameters_required_by_func;
                let params_already_resolved = &state.parameters;
                if params_needed == params_already_resolved.len() {
                    *ready_to_evaluate = true;
                }
                Ok(())
            }
            ExpressionState::Await { future, state } => {
                AwaitExpression::eval2(object.clone(), interpreter_state)?;
                *future = Some(object.clone());
                *state = AwaitState::Waiting;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

pub enum EvaluationResult {
    Done(ObjectRef),
    Pending,

    Push(Expression),
}
