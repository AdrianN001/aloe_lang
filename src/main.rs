pub mod token;
pub mod ast;
pub mod lexer;



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod lexer_test;
mod parser_test;
