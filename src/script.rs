use std::fs;

use crate::{ast::Parser, lexer::Lexer};

fn read_source_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

pub fn run_script(file_path: &str) {
    let source_file_content = read_source_file(file_path);

    let lexer = Lexer::new(source_file_content);
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().unwrap();

    let _last_obj = program.evaluate().unwrap();
}
