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
        let base_path = self.root_dir.clone().unwrap_or_else(|| PathBuf::from("."));
        let project_map = DocProjectMap::new_from_documentation(self, base_path.clone());

        for (module_path, module) in &project_map.module_map {
            Self::handle_module_doc_generation(module, module_path, &project_map, &path)?;
        }
        Ok(())
    }

    fn handle_module_doc_generation(
        module: &DocModule,
        module_path: &PathBuf,
        project_map: &DocProjectMap,
        output_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Berechne den relativen Pfad dieser Seite relativ zur base_path
        // z.B. wenn module_path = "./docs/types/int.aloe", current_page_doc_path = "types/int.html"
        let abs_module_path = if module_path.is_absolute() {
            module_path.clone()
        } else {
            std::env::current_dir()
                .unwrap_or_default()
                .join(module_path)
        };

        let abs_base_path = if project_map.base_path.is_absolute() {
            project_map.base_path.clone()
        } else {
            std::env::current_dir()
                .unwrap_or_default()
                .join(&project_map.base_path)
        };

        let current_page_path = abs_module_path
            .strip_prefix(&abs_base_path)
            .unwrap_or(&abs_module_path)
            .with_extension("html");

        let current_page_doc_path = current_page_path
            .to_string_lossy()
            .replace("\\", "/")
            .to_string();

        // Berechne die doc_path relativ zur aktuellen Seite
        let relative_entries = project_map.get_symbol_entries_relative_to(&current_page_doc_path);
        let symbols_json = serde_json::to_string(&relative_entries)?;

        let html = Self::generate_html_for_project(module, &symbols_json)?;
        let html_output_path = prepare_html_path(output_path, module_path)?;

        Self::write_html_to_file(&html, &html_output_path)?;
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
