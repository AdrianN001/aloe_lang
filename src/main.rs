pub mod token;
pub mod ast;
pub mod lexer;
pub mod object;
pub mod evaluator;



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod lexer_test;
mod parser_test;
mod eval_test;
