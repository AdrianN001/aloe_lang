use crate::doc::export::html::project::file_system::prepare_html_path;
use std::fs;

#[test]
fn test_prepare_html_path_nested_directories() {
    let temp_dir = std::env::temp_dir().join("aloe_doc_test_nested");
    let base = temp_dir.clone();

    let result = prepare_html_path(&base, "./lib/module/file.aloe").unwrap();
    assert_eq!(result, base.join("lib/module/file.html"));
    assert!(base.join("lib/module").is_dir());

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_prepare_html_path_without_leading_dot_slash() {
    let temp_dir = std::env::temp_dir().join("aloe_doc_test_no_dot");
    let base = temp_dir.clone();

    let result = prepare_html_path(&base, "test/test_2.aloe").unwrap();
    assert_eq!(result, base.join("test/test_2.html"));
    assert!(base.join("test").is_dir());

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_prepare_html_path_file_in_base_directory() {
    let temp_dir = std::env::temp_dir().join("aloe_doc_test_root_file");
    let base = temp_dir.clone();

    let result = prepare_html_path(&base, "./main.aloe").unwrap();
    assert_eq!(result, base.join("main.html"));
    assert!(base.is_dir());

    let _ = fs::remove_dir_all(temp_dir);
}
