use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::Expression,
    frame::state::{ExpressionState, await_state::AwaitState},
    object::ObjectRef,
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

    pub fn to_ref(self) -> ExprFrameRef {
        Rc::new(RefCell::new(self))
    }
}

pub enum EvaluationResult {
    Done(ObjectRef),
    Pending,
}
