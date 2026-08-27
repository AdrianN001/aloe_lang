use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{
    ast::Parser,
    lexer::Lexer,
    module::{Module, module_kind::ModuleKind, module_loader::ModuleLoader},
    object::{Object, ObjectRef, panic_obj::RuntimeSignal, stack_environment::StackEnvironment},
};

pub fn test_cases_for_input_output(testcases: &[(&str, &str)]) {
    testcases.iter().for_each(|testcase| {
        let input = testcase.0;
        let expected_value = testcase.1.to_string();

        //println!("{}", &input);

        let last_object = match run_testcase(&input) {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => {
                assert_eq!(err.value, expected_value);
                return;
            }
            _ => todo!(),
        };
        match &*last_object.borrow() {
            Object::Error(err) => assert_eq!(err.inspect_message(), expected_value),
            other_type => {
                println!("{}", other_type.inspect());
                assert_eq!(other_type.inspect(), expected_value)
            }
        }
    });
}

fn run_testcase(input: &str) -> Result<ObjectRef, RuntimeSignal> {
    let main_module = Module::new_dummy(ModuleKind::SourceFile).to_reference();

    let mut module_cache = ModuleLoader::new(&PathBuf::from("."));
    module_cache.set(main_module.clone());

    Module::execute_plain_script(main_module.clone(), &mut module_cache, input)
}
impl Module {
    fn execute_plain_script(
        self_ref: Rc<RefCell<Module>>,
        module_cache: &mut ModuleLoader,
        input: &str,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let program = {
            let lexer = Lexer::new(input.into());
            let parser = Parser::new(lexer);
            let program = parser.into_a_program().unwrap();
            program
        };

        let raw_environment = StackEnvironment::new();

        let environment = Rc::new(RefCell::new(raw_environment));

        {
            let mut borrow = self_ref.borrow_mut();
            borrow.environ = Some(environment.clone());
        }

        let _last_obj = program.evaluate(environment.clone(), module_cache)?;

        Ok(_last_obj)
    }
}
