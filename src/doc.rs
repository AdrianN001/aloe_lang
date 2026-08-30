use std::path::PathBuf;

use crate::doc::symbol::documentation::Documentation;

pub mod doc_comment;
pub mod export;
pub mod symbol;
pub mod traits;

pub fn json_document_a_single_file(file_path: PathBuf) {
    let documentation = Documentation::from_single_file(file_path).unwrap();

    let json_doc = documentation.export_to_json_str().unwrap();

    println!("{}", json_doc);
}

pub fn html_document_a_single_file(file_path: PathBuf) {
    let documentation = Documentation::from_single_file(file_path).unwrap();

    let html_doc = documentation.export_to_single_html_str();

    println!("{}", html_doc);
}

pub fn html_document_a_project(root_file_path: PathBuf, output_dir: PathBuf) {
    let mut documentation = Documentation::from_project(root_file_path).unwrap();
    documentation.export_project_to_dir(output_dir).unwrap();
}
