use crate::{ast::{Parser, program, statement::{self, Statement}}, lexer::Lexer, object::{Object, integer::Integer}};



#[test]
fn test_eval_integer_object(){
    let testcases = [
        ("5;", 5),
        ("10;", 10),
        ("67;", 67)
    ];

    testcases.iter().for_each(|testcase|{
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        match program.statements[0].evaluate().unwrap(){
            Object::Int(int) => assert_eq!(int.value, expected_value),
            _ => panic!()
        }
    })
}

#[test]
fn test_eval_bool_object(){
    let testcases = [
        ("true;", true),
        ("false;", false),
    ];

    testcases.iter().for_each(|testcase|{
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        match program.statements[0].evaluate().unwrap(){
            Object::Bool(bool) => assert_eq!(bool.value, expected_value),
            _ => panic!()
        }
    })
}

#[test]
fn test_eval_bang_prefix(){
    let testcases = [
        ("!true;", false),
        ("!false;", true),

        ("!!true;", true),
        ("!!false;", false),

        ("!!!true;", false),
        ("!!!false;", true)
    ];

    testcases.iter().for_each(|testcase|{
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        match program.statements[0].evaluate().unwrap(){
            Object::Bool(bool) => assert_eq!(bool.value, expected_value),
            _ => panic!()
        }
    })
}

#[test]
fn test_eval_minus_prefix_operator(){
    let testcases = [
        ("-5;", -5),
        ("--10;", 10),
        ("---67;", -67)
    ];

    testcases.iter().for_each(|testcase|{
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        match program.statements[0].evaluate().unwrap(){
            Object::Int(int) => assert_eq!(int.value, expected_value),
            _ => panic!()
        }
    })
}

#[test]
fn test_eval_int_and_bool_operations(){
    let testcases = [
        ("5* (5+10);",                      75),
        ("5*5+10;",                         35),
        ("3 + 4 * 5 == 3 * 1 + 4 * 5;",     1),
        ("5 * 10 > 40 + 5;",                1),
        ("(10 + 2) * 30 == 300 + 20 * 3;",  1),
        ("(5 > 5 == true) != false;",       0),
        ("500 / 2 != 250;",                 0)
    ];

    testcases.iter().for_each(|testcase|{
        let input = testcase.0.into();
        let expected_value = testcase.1;

        println!("{:?}", &input);
        
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        println!("{:?}", &program.statements[0].to_string());


        match program.statements[0].evaluate().unwrap(){
            Object::Int(int) => assert_eq!(int.value, expected_value),
            Object::Bool(bool) => assert_eq!(bool.value, expected_value == 1),
            _ => panic!()
        }
    })
}

