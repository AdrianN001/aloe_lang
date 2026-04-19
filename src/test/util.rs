use crate::{ast::Parser, lexer::Lexer, object::Object};

pub fn test_cases_for_input_output(testcases: &[(&str, &str)]) {
    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1.to_string();

        println!("{}", &input);

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(err) => {
                assert_eq!(err.value, expected_value);
                return;
            }
        };
        match &*last_object.borrow() {
            Object::Err(err) => assert_eq!(err.inspect_message(), expected_value),
            other_type => assert_eq!(other_type.inspect(), expected_value),
        }
    });
}
