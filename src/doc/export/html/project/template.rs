use std::path::PathBuf;

use askama::Template;

use crate::doc::{
    export::html::project::{file_system::prepare_html_path, project_map::DocProjectMap},
    symbol::{doc_module::DocModule, doc_symbol::DocSymbol, documentation::Documentation},
};

#[derive(Template)]
#[template(path = "project.html")]
pub struct DocsProjectTemplate<'a> {
    pub symbol_entries_parsed: &'a str,
    pub symbols: Vec<DocSymbol>,
}

impl Documentation {
    pub fn export_project_to_dir(
        &mut self,
        path: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let project_map = DocProjectMap::new_from_documentation(self);
        let symbol_entries = self.generate_symbol_entries(&project_map)?;

        let symbols_json = serde_json::to_string(&symbol_entries)?;

        for (module_path, module) in &project_map.module_map {
            Self::handle_module_doc_generation(module, module_path, &symbols_json, &path)?;
        }
        Ok(())
    }
    fn generate_symbol_entries(
        &self,
        project_map: &DocProjectMap,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let all_symbols = {
            let all_symbols = project_map.symbol_vec.iter().collect::<Vec<_>>();
            all_symbols
        };

        let symbols_json = serde_json::to_string(&all_symbols)?;
        Ok(symbols_json)
    }

    fn handle_module_doc_generation(
        module: &DocModule,
        module_path: &PathBuf,
        entries_parsed: &String,
        base_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let html = Self::generate_html_for_project(module, entries_parsed)?;

        let html_path = prepare_html_path(base_path, module_path)?;

        Self::write_html_to_file(&html, &html_path)?;
        Ok(())
    }

    fn generate_html_for_project(
        module: &DocModule,
        entries_parsed: &String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let template = DocsProjectTemplate {
            symbol_entries_parsed: entries_parsed,
            symbols: module.root_symbols.clone(),
        };
        let rendered = template.render()?;
        Ok(rendered)
    }

    fn write_html_to_file(
        html: &String,
        html_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(html_path.parent().unwrap())?;
        std::fs::write(html_path, html)?;
        Ok(())
    }
}
