use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

use crate::{ast::Parser, lexer::Lexer, object::stack_environment::StackEnvironment};

pub fn start_repl() {
    println!("🌿 Aloe REPL");
    println!("Type exit to quit.\n");

    let environ = Rc::new(RefCell::new(StackEnvironment::new()));

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let lexer = Lexer::new(input);

        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        if let Ok(last_object) = program.evaluate_with_other_environment(environ.clone()) {
            println!("{}", last_object.borrow().inspect());
        };
    }
}
