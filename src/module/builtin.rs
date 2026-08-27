use crate::module::{
    ModuleRef, builtin::math::create_math_module, module_error::ModuleError,
    module_loader::ModuleLoader,
};

pub mod math;

impl ModuleLoader {
    pub const PRELUDE_BUILTINS: [&'static str; 1] = ["@std::math"];

    pub fn try_load_math_module(&mut self) -> Result<ModuleRef, ModuleError> {
        // Check if the math module is already loaded in the prelude cache
        if let Some(loaded_module_opt) = self.prelude_cache.get("@std::math") {
            if let Some(existing_module) = &loaded_module_opt {
                return Ok(existing_module.clone());
            }
        }
        let module = create_math_module();
        self.prelude_cache
            .insert("@std::math".into(), Some(module.clone()));
        Ok(module)
    }
}
