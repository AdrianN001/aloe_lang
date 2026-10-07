use std::{cell::RefCell, io::Write, rc::Rc};

use tokio::io::{self, AsyncBufReadExt};

use crate::{
    object::{
        Object, ObjectRef,
        error::error_type::ErrorType,
        future::{FutureObj, future_kind::FutureKind, future_state::FutureState},
        new_objectref,
        stack_environment::EnvRef,
        string_obj::StringObj,
    },
    scheduler::{SCHEDULER_CHANNEL, TOKIO_RUNTIME, add_io_future, message_output::MessageOutput},
    terminal::handler::TerminalHandler,
};

// print(object, ends)
pub fn console_write_builtin_function(args: &[ObjectRef], _environ: EnvRef) -> ObjectRef {
    if args.is_empty() {
        return Rc::new(RefCell::new(Object::NULL_OBJECT));
    }

    TerminalHandler::write(&args[0].borrow().inspect());

    if args.len() > 1
        && let Object::String(end_str) = &*args[1].borrow()
    {
        TerminalHandler::write(&end_str.value);
    } else {
        TerminalHandler::write("\n");
    }

    TerminalHandler::flush();

    new_objectref(Object::NULL_OBJECT)
}

// println(object_0, ...object_n)
pub fn console_writeln_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    args.iter().for_each(|arg| {
        let borrow = arg.borrow();
        TerminalHandler::write(&borrow.inspect());
    });
    TerminalHandler::write("\n");
    TerminalHandler::flush();

    new_objectref(Object::NULL_OBJECT)
}

// input()
pub fn console_read_builtin_function() -> ObjectRef {
    std::io::stdout().flush().unwrap();

    let mut buffer = TerminalHandler::read();
    buffer.pop(); // Remove the trailing newline

    new_objectref(Object::String(Box::new(StringObj { value: buffer })))
}

//input_async()
pub fn console_read_async_builtin_function() -> ObjectRef {
    let future = new_objectref(Object::Future(Box::new(FutureObj::new(
        FutureState::Pending(FutureKind::IO),
    ))));

    let future_id = {
        let future_borrow = future.borrow();
        if let Object::Future(future_obj) = &*future_borrow {
            future_obj.get_id()
        } else {
            panic!("Expected a Future object");
        }
    };

    add_io_future(future_id, future.clone());

    let tx = SCHEDULER_CHANNEL.with(|slot| slot.borrow().0.clone());

    TOKIO_RUNTIME.with(|slot| {
        let runtime = slot.borrow();

        runtime.spawn(async move {
            let stdin = io::stdin();
            let mut reader = io::BufReader::new(stdin);

            let mut buffer = String::new();

            match reader.read_line(&mut buffer).await {
                Ok(_) => {
                    buffer.pop(); // Remove the trailing newline
                    let _ = tx.send((future_id, MessageOutput::PlainText(buffer)));
                }
                Err(e) => {
                    let _ = tx.send((
                        future_id,
                        MessageOutput::Error((
                            ErrorType::IO,
                            format!("Failed to read from stdin: {}", e),
                            String::from("input_async"),
                        )),
                    ));
                }
            }
        })
    });

    future
}
