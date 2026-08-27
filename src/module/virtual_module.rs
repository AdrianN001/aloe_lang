use crate::module::{ModuleRef, module_error::ModuleError, module_loader::ModuleLoader};

pub mod _async;
pub mod _io;
pub mod _memory;
pub mod _ntv;
pub mod _os;
pub mod _sys;
pub mod _time;
pub mod math;
pub mod random;

impl ModuleLoader {
    pub const VIRTUAL_BUILTINS: [&'static str; 9] = [
        "@std::math",
        "@std::random",
        "@std::_os",
        "@std::_sys",
        "@std::_time",
        "@std::_async",
        "@std::_memory",
        "@std::_ntv",
        "@std::_io",
    ];

    pub fn try_load_math_module(&mut self) -> Result<ModuleRef, ModuleError> {
        // Check if the math module is already loaded in the virtual module cache
        if let Some(loaded_module_opt) = self.virtual_module_cache.get("@std::math") {
            if let Some(existing_module) = &loaded_module_opt {
                return Ok(existing_module.clone());
            }
        }
        let module = math::create_math_module();
        self.virtual_module_cache
            .insert("@std::math".into(), Some(module.clone()));
        Ok(module)
    }

    pub fn try_load_random_module(&mut self) -> Result<ModuleRef, ModuleError> {
        // Check if the random module is already loaded in the virtual module cache
        if let Some(loaded_module_opt) = self.virtual_module_cache.get("@std::random") {
            if let Some(existing_module) = &loaded_module_opt {
                return Ok(existing_module.clone());
            }
        }
        let module = random::create_random_module();
        self.virtual_module_cache
            .insert("@std::random".into(), Some(module.clone()));
        Ok(module)
    }

    pub fn try_load_underscore_os_module(&mut self) -> Result<ModuleRef, ModuleError> {
        // Check if the _os module is already loaded in the virtual module cache
        if let Some(loaded_module_opt) = self.virtual_module_cache.get("@std::_os") {
            if let Some(existing_module) = &loaded_module_opt {
                return Ok(existing_module.clone());
            }
        }
        let module = _os::create_underscore_os_module();
        self.virtual_module_cache
            .insert("@std::_os".into(), Some(module.clone()));
        Ok(module)
    }

    pub fn try_load_underscore_sys_module(&mut self) -> Result<ModuleRef, ModuleError> {
        // Check if the _sys module is already loaded in the virtual module cache
        if let Some(loaded_module_opt) = self.virtual_module_cache.get("@std::_sys") {
            if let Some(existing_module) = &loaded_module_opt {
                return Ok(existing_module.clone());
            }
        }
        let module = _sys::create_underscore_sys_module();
        self.virtual_module_cache
            .insert("@std::_sys".into(), Some(module.clone()));
        Ok(module)
    }

    pub fn try_load_underscore_time_module(&mut self) -> Result<ModuleRef, ModuleError> {
        // Check if the _time module is already loaded in the virtual module cache
        if let Some(loaded_module_opt) = self.virtual_module_cache.get("@std::_time") {
            if let Some(existing_module) = &loaded_module_opt {
                return Ok(existing_module.clone());
            }
        }
        let module = _time::create_underscore_time_module();
        self.virtual_module_cache
            .insert("@std::_time".into(), Some(module.clone()));
        Ok(module)
    }
    pub fn try_load_underscore_async_module(&mut self) -> Result<ModuleRef, ModuleError> {
        // Check if the _async module is already loaded in the virtual module cache
        if let Some(loaded_module_opt) = self.virtual_module_cache.get("@std::_async") {
            if let Some(existing_module) = &loaded_module_opt {
                return Ok(existing_module.clone());
            }
        }
        let module = _async::create_underscore_async_module();
        self.virtual_module_cache
            .insert("@std::_async".into(), Some(module.clone()));
        Ok(module)
    }

    pub fn try_load_underscore_memory_module(&mut self) -> Result<ModuleRef, ModuleError> {
        // Check if the _memory module is already loaded in the virtual module cache
        if let Some(loaded_module_opt) = self.virtual_module_cache.get("@std::_memory") {
            if let Some(existing_module) = &loaded_module_opt {
                return Ok(existing_module.clone());
            }
        }
        let module = _memory::create_underscore_memory_module();
        self.virtual_module_cache
            .insert("@std::_memory".into(), Some(module.clone()));
        Ok(module)
    }
    pub fn try_load_underscore_ntv_module(&mut self) -> Result<ModuleRef, ModuleError> {
        // Check if the _ntv module is already loaded in the virtual module cache
        if let Some(loaded_module_opt) = self.virtual_module_cache.get("@std::_ntv") {
            if let Some(existing_module) = &loaded_module_opt {
                return Ok(existing_module.clone());
            }
        }
        let module = _ntv::create_underscore_ntv_module();
        self.virtual_module_cache
            .insert("@std::_ntv".into(), Some(module.clone()));
        Ok(module)
    }
    pub fn try_load_underscore_io_module(&mut self) -> Result<ModuleRef, ModuleError> {
        // Check if the _io module is already loaded in the virtual module cache
        if let Some(loaded_module_opt) = self.virtual_module_cache.get("@std::_io") {
            if let Some(existing_module) = &loaded_module_opt {
                return Ok(existing_module.clone());
            }
        }
        let module = _io::create_underscore_io_module();
        self.virtual_module_cache
            .insert("@std::_io".into(), Some(module.clone()));
        Ok(module)
    }
}
