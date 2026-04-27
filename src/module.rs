pub mod module_error;
pub mod module_loader;

use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

use crate::{
    ast::Parser,
    lexer::Lexer,
    module::{module_error::ModuleError, module_loader::ModuleLoader},
    object::{
        Object, new_objectref,
        panic_obj::RuntimeSignal,
        stack_environment::{EnvRef, StackEnvironment},
        string_obj::StringObj,
    },
};

pub type ModuleRef = Rc<RefCell<Module>>;

#[derive(Default)]
pub struct Module {
    name: String,
    rel_path: PathBuf,
    abs_path: PathBuf,
    pub environ: Option<EnvRef>,
}

impl Module {
    pub fn new(name: String) -> Result<Self, ModuleError> {
        let rel_path = PathBuf::from(name.clone());

        if !rel_path.is_file() {
            return Err(ModuleError::new(&name, "module is not a file"));
        }

        Ok(Self {
            abs_path: match fs::canonicalize(&rel_path) {
                Ok(ok_value) => ok_value,
                Err(err_feedback) => {
                    return Err(ModuleError::new(&name, &err_feedback.to_string()));
                }
            },
            rel_path,
            name,
            ..Default::default()
        })
    }

    fn read_source_file(file_path: &str) -> String {
        fs::read_to_string(file_path).unwrap()
    }

    pub fn as_abs_path(&self) -> String {
        self.abs_path.display().to_string()
    }

    pub fn mod_name_as_abs_path(module_name: &str) -> Result<String, ModuleError> {
        let rel_path = PathBuf::from(module_name);

        if !rel_path.is_file() {
            return Err(ModuleError::new(module_name, "module is not a file"));
        }

        if rel_path.is_absolute() {
            return Ok(rel_path.display().to_string());
        }

        match fs::canonicalize(rel_path) {
            Ok(ok_value) => Ok(ok_value.display().to_string()),
            Err(err) => Err(ModuleError::new(module_name, &err.to_string())),
        }
    }

    pub fn execute(&mut self, module_loader: &mut ModuleLoader) -> Result<(), RuntimeSignal> {
        let source_file_content = Self::read_source_file(&self.abs_path.display().to_string());

        let lexer = Lexer::new(source_file_content);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let mut raw_environment = StackEnvironment::new();
        self.load_dunder_into_env(&mut raw_environment, module_loader);

        let environment = Rc::new(RefCell::new(raw_environment));

        let _last_obj = program.evaluate(environment.clone(), module_loader)?;

        self.environ = Some(environment);

        Ok(())
    }

    fn load_dunder_into_env(
        &self,
        environment: &mut StackEnvironment,
        module_loader: &ModuleLoader,
    ) {
        let name = new_objectref(Object::String(StringObj {
            value: self.rel_path.display().to_string(),
        }));
        environment.set("__name__", name);

        let module_path = new_objectref(Object::String(StringObj {
            value: self.abs_path.display().to_string(),
        }));
        environment.set("__module__", module_path);

        let root_file = new_objectref(Object::String(StringObj {
            value: module_loader.root_file.display().to_string(),
        }));
        environment.set("__main__", root_file);
    }

    pub fn to_reference(self) -> ModuleRef {
        Rc::new(RefCell::new(self))
    }
}
