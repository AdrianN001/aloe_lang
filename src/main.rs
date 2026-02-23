use std::env;

use crate::{ast::Parser, lexer::Lexer, src_file_read::read_source_file};

pub mod src_file_read;
pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod token;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    }


    let source_file_content = read_source_file(&args[1]);

    let lexer = Lexer::new(source_file_content);
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().unwrap();

    let _last_obj = program.evaluate().unwrap();

}

#[cfg(test)]
mod eval_test;
#[cfg(test)]
mod lexer_test;
#[cfg(test)]
mod parser_test;
