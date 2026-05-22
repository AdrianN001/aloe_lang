use crate::{
    ast::statement::launch_statement::LaunchStatement,
    object::{
        ObjectRef, built_in::async_await::spawn_builtin_function, panic_obj::RuntimeSignal,
        stack_environment::EnvRef, state::StateRef,
    },
};

impl LaunchStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let value = self.expr.evaluate(environ.clone(), state.clone())?;

        spawn_builtin_function(&[value], state)
    }
}
