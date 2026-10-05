use std::path::PathBuf;

use crate::{doc::symbol::documentation::Documentation, object::panic_obj::RuntimeSignal};

pub mod doc_comment;
pub mod export;
pub mod symbol;
pub mod traits;

pub fn json_document_a_single_file(file_path: PathBuf) -> Result<(), RuntimeSignal> {
    let documentation = Documentation::from_single_file(file_path)?;
    let json_doc = match documentation.export_to_json_str() {
        Ok(json) => json,
        Err(err) => return Err(RuntimeSignal::GenericError(Box::new(err))),
    };

    println!("{}", json_doc);
    Ok(())
}

pub fn html_document_a_single_file(file_path: PathBuf) -> Result<(), RuntimeSignal> {
    let documentation = Documentation::from_single_file(file_path)?;

    let html_doc = documentation.export_to_single_html_str();

    println!("{}", html_doc);
    Ok(())
}

pub fn html_document_a_project(
    root_file_path: PathBuf,
    output_dir: PathBuf,
) -> Result<(), RuntimeSignal> {
    let mut documentation = Documentation::from_project(root_file_path)?;
    match documentation.export_project_to_dir(output_dir) {
        Ok(_) => Ok(()),
        Err(err) => Err(RuntimeSignal::GenericError(err)),
    }
}
