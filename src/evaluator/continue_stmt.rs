use crate::{
    ast::statement::continue_statement::ContinueStatement,
    object::{
        ObjectRef, error::panic_type::PanicType, panic_obj::RuntimeSignal,
        stack_environment::EnvRef, state::StateRef,
    },
};

impl ContinueStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }

        let is_loop_context = { environ.borrow().is_loop_context() };

        if !is_loop_context {
            return Err(RuntimeSignal::Panic(
                crate::object::panic_obj::PanicObj::new_simple(
                    PanicType::UnexpectedKeyword,
                    "unexpected continue keyword in non-loop context".into(),
                    state.clone(),
                ),
            ));
        }

        Err(RuntimeSignal::Continue)
    }
}
