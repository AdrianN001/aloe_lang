use crate::doc::symbol::documentation::Documentation;

pub mod doc_comment;
pub mod export;
pub mod symbol;
pub mod traits;

pub fn json_document_a_single_file(file_path: &str) {
    let documentation = Documentation::from_single_file(file_path).unwrap();

    let json_doc = documentation.export_to_json_str().unwrap();

    println!("{}", json_doc);
}

pub fn html_document_a_single_file(file_path: &str) {
    let documentation = Documentation::from_single_file(file_path).unwrap();

    let html_doc = documentation.export_to_single_html_str();

    println!("{}", html_doc);
}
