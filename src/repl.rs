use std::{cell::RefCell, io::Write, rc::Rc};

use crate::{
    ast::Parser,
    lexer::Lexer,
    module::Module,
    object::{panic_obj::RuntimeSignal, stack_environment::StackEnvironment},
    terminal::handler::TerminalHandler,
};

pub fn start_repl() {
    TerminalHandler::writeln("🌿 Aloe REPL 🌿");
    TerminalHandler::writeln("Type exit to quit.\n");

    let environ = Rc::new(RefCell::new(StackEnvironment::new()));

    {
        let mut environ_borrow = environ.borrow_mut();
        Module::load_prelude(&mut environ_borrow);
    }

    loop {
        TerminalHandler::write(">> ");
        TerminalHandler::flush();

        let input = TerminalHandler::read();

        if input.trim() == "exit" {
            break;
        }

        let lexer = Lexer::new(input);

        let parser = Parser::new(lexer);
        let program = match parser.into_a_program() {
            Ok(program) => program,
            Err(err) => {
                TerminalHandler::writeln_error(&err.to_string());
                break;
            }
        };

        match program.evaluate_as_repl(environ.clone()) {
            Ok(last_object) => {
                TerminalHandler::writeln_italic(&last_object.borrow().inspect());
            }
            Err(RuntimeSignal::Panic(panic_reason)) => {
                TerminalHandler::writeln_error(&panic_reason.inspect());
                break;
            }
            _ => todo!(),
        };
    }
}
