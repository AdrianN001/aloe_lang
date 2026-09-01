use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use crate::{
    ast::syntax_error_report::syntax_error::SyntaxError, doc::symbol::doc_module::DocModule,
    module::Module,
};

#[derive(Serialize, Deserialize)]
pub struct Documentation {
    pub modules: Vec<DocModule>,
    pub root_dir: Option<PathBuf>,
}

impl Documentation {
    pub const SINGLE_INPUT_MODULE_NAME: &str = "single unit";
    pub fn from_single_input(input: &str) -> Result<Self, SyntaxError> {
        let doc_module = DocModule::from_single_input(Self::SINGLE_INPUT_MODULE_NAME, input)?;

        Ok(Self {
            modules: vec![doc_module],
            root_dir: None,
        })
    }

    pub fn from_single_file(path: PathBuf) -> Result<Self, SyntaxError> {
        let input = Module::read_source_file(&path).unwrap();
        Self::from_single_input(&input)
    }

    pub fn from_project(root_path: PathBuf) -> Result<Self, SyntaxError> {
        let input = Module::read_source_file(&root_path).unwrap();

        let mut root_module =
            DocModule::from_single_input(root_path.display().to_string().as_str(), &input)?;
        root_module.set_path(root_path.clone());

        let mut modules = vec![];
        Self::load_modules(&root_module, &mut modules, &root_path).unwrap();

        modules.push(root_module);

        let root_dir = root_path.parent().map(|p| p.to_path_buf());
        Ok(Self { modules, root_dir })
    }

    fn load_modules(
        root_module: &DocModule,
        modules: &mut Vec<DocModule>,
        root_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for imports_from in &root_module.imports {
            let formatted_imports_from = {
                let base = if let Some(parent) = root_path.parent().clone() {
                    parent
                } else {
                    &PathBuf::from(".")
                };
                base.join(imports_from)
            };

            //println!("Loading module from: {:?}", formatted_imports_from);

            let input = Module::read_source_file(&formatted_imports_from)?;
            let mut module = DocModule::from_single_input(imports_from, &input).unwrap();
            module.set_path(formatted_imports_from);

            if module.imports.len() > 0 {
                Self::load_modules(&module, modules, root_path)?;
            }

            modules.push(module);
        }

        Ok(())
    }
}
