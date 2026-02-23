use std::env;

use crate::{ast::Parser, lexer::Lexer};

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

    let lexer = Lexer::new(args[1].clone());
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().unwrap();

    let _ = program.evaluate();
}

#[cfg(test)]
mod eval_test;
#[cfg(test)]
mod lexer_test;
#[cfg(test)]
mod parser_test;
