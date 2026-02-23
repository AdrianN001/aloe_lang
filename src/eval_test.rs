use crate::{
    ast::{
        Parser, program,
        statement::{self, Statement},
    },
    lexer::Lexer,
    object::{Object, integer::Integer},
};

#[test]
fn test_eval_integer_object() {
    let testcases = [("5;", 5), ("10;", 10), ("67;", 67)];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        match program.evaluate().unwrap() {
            Object::Int(int) => assert_eq!(int.value, expected_value),
            _ => panic!(),
        }
    })
}

#[test]
fn test_eval_bool_object() {
    let testcases = [("true;", true), ("false;", false)];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        match program.evaluate().unwrap() {
            Object::Bool(bool) => assert_eq!(bool.value, expected_value),
            _ => panic!(),
        }
    })
}

#[test]
fn test_eval_bang_prefix() {
    let testcases = [
        ("!true;", false),
        ("!false;", true),
        ("!!true;", true),
        ("!!false;", false),
        ("!!!true;", false),
        ("!!!false;", true),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        match program.evaluate().unwrap() {
            Object::Bool(bool) => assert_eq!(bool.value, expected_value),
            _ => panic!(),
        }
    })
}

#[test]
fn test_eval_minus_prefix_operator() {
    let testcases = [("-5;", -5), ("--10;", 10), ("---67;", -67)];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        match program.evaluate().unwrap() {
            Object::Int(int) => assert_eq!(int.value, expected_value),
            _ => panic!(),
        }
    })
}

#[test]
fn test_eval_int_and_bool_operations() {
    let testcases = [
        ("5* (5+10);", 75),
        ("5*5+10;", 35),
        ("3 + 4 * 5 == 3 * 1 + 4 * 5;", 1),
        ("5 * 10 > 40 + 5;", 1),
        ("(10 + 2) * 30 == 300 + 20 * 3;", 1),
        ("(5 > 5 == true) != false;", 0),
        ("500 / 2 != 250;", 0),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        println!("{:?}", &input);

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        println!("{:?}", &program.to_string());

        match program.evaluate().unwrap() {
            Object::Int(int) => assert_eq!(int.value, expected_value),
            Object::Bool(bool) => assert_eq!(bool.value, expected_value == 1),
            _ => panic!(),
        }
    })
}

#[test]
fn test_if_statement_evalulation() {
    let testcases = [
        ("if (5 * 5 + 10 > 34) { 99 } else { 100 }", 99),
        ("if ((1000 / 2) + 250 * 2 == 1000) { 9999; }", 9999),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        match program.evaluate().unwrap() {
            Object::Int(int) => assert_eq!(int.value, expected_value),
            _ => panic!(),
        }
    })
}

#[test]
fn test_if_statement_null_eval() {
    let testcases = ["if (false){999}", "if (5 < 3){22}"];

    testcases.iter().for_each(|testcase| {
        let input = testcase.to_string();

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        assert!(matches!(program.evaluate().unwrap(), Object::Null(_)));
    })
}

#[test]
fn test_return_statement() {
    let testcases = [
        ("return 10;", 10),
        ("20; return 5; 50;", 5),
        ("return 2*10; 10;", 20),
        (
            "if (10 > 1) {
            if (10 > 1) {
                return 10;
            }
            return 1;
        }",
            10,
        ),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.to_string();
        let expected_ret_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        match program.evaluate().unwrap() {
            Object::Int(int_val) => assert_eq!(int_val.value, expected_ret_value),
            _ => panic!(),
        }
    });
}

#[test]
fn test_variable_evaluate() {
    let testcases = [
        ("let a = 10; let b = 5; let c = a + b; c;", 15),
        ("let a = true; if(a){5;}else{15;}", 5),
        ("let a = true; if(!a){5;}else{15;}", 15),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.to_string();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        match program.evaluate().unwrap() {
            Object::Int(int_val) => assert_eq!(int_val.value, expected_value),
            _ => panic!(),
        }
    });
}

#[test]
fn test_function_evaluation() {
    let testcases = [
        ("fn(x){x+2;}", ["x"].to_vec(), "(x + 2)"),
        ("fn(){3-4;}", Vec::new(), "(3 - 4)"),
        ("fn(y,z){y*z;}", ["y", "z"].to_vec(), "(y * z)"),
    ];

    testcases.iter().for_each(|test_case| {
        let input = test_case.0.to_string();
        let expected_parameters = &test_case.1;
        let expected_body = test_case.2.to_string();

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = program.evaluate().unwrap();

        let function_object = match last_object {
            Object::Func(func) => func,
            _ => panic!(""),
        };

        assert_eq!(function_object.body.to_string(), expected_body);

        assert_eq!(function_object.parameters.len(), expected_parameters.len());

        function_object
            .parameters
            .iter()
            .enumerate()
            .for_each(|(index, identifier)| {
                assert_eq!(identifier.value, expected_parameters[index]);
            })
    })
}

#[test]
fn test_calling_expression() {
    let testcases = [
        ("let double = fn(x){2*x;}; double(16);", 32),
        ("let square = fn(y){y*y;}; square(5);", 25),
        ("let two = fn(){2;};two();", 2),
        //bools
        ("fn(x){x == 2;}(2);", 1),
        ("fn(y){y == 2;}(1);", 0),
        // closure
        (
            "let newAdder = fn(base){ 
            return fn(x){ 
                base + x; 
            }; 
        }; 
        let addTwo = newAdder(2);
        addTwo(5);",
            7,
        ),
    ];

    testcases.iter().for_each(|test_case| {
        let input = test_case.0.into();
        let expected_value = test_case.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        match last_object {
            Object::Int(integer_value) => assert_eq!(integer_value.value, expected_value),
            Object::Bool(bool_value) => assert_eq!(bool_value.value, expected_value == 1),
            _ => panic!(),
        };
    });
}

#[test]
fn test_basic_string_evaluation() {
    let testcases = [
        (r#"let hi = "hello, world"; hi;"#, "hello, world"),
        (r#"fn(){"random_string";}();"#, "random_string"),
        (r#"fn(){"";}();"#, ""),
    ];

    testcases.iter().for_each(|test_case| {
        let input = test_case.0.into();
        let expected_value = test_case.1.to_string();

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        match last_object {
            Object::String(string_value) => assert_eq!(string_value.value, expected_value),
            _ => panic!(),
        };
    });
}

#[test]
fn eval_string_concat() {
    let testcases = [
        (r#" "hello"+"world";  "#, "helloworld"),
        (r#" "hello"+" " + "world";  "#, "hello world"),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1.to_string();

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        match last_object {
            Object::String(string_value) => assert_eq!(string_value.value, expected_value),
            _ => panic!(),
        };
    })
}

#[test]
fn eval_index_operator() {
    let testcases = [
        ("[1,2,3][0]", "1"),
        (r#"[true, 3, "asd", false, true][1+1]"#, r#""asd""#),
        ("let i = 0; [i][i];", "0"),
        ("[][1];", "null"),
        ("[(fn(){return 15;})()][0];", "15"),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(last_object.inspect(), expected_value);
    })
}

#[test]
fn eval_len_for_strings() {
    let testcases = [
        (r#" len("hi") "#, 2),
        (r#" len("hello, world");  "#, 12),
        (r#" len("");"#, 0),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        match last_object {
            Object::Int(integer) => assert_eq!(integer.value, expected_value as i64),
            _ => panic!(),
        };
    })
}

#[test]
fn eval_len_for_arrays() {
    let testcases = [
        (r#" len([]) "#, 0),
        (r#" len([1]);  "#, 1),
        (r#" len([1,"a", true]);"#, 3),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        match last_object {
            Object::Int(integer) => assert_eq!(integer.value, expected_value as i64),
            _ => panic!(),
        };
    })
}

#[test]
fn eval_rest_builtin() {
    let testcases = [
        (r#" rest([1,2,3]) "#, "[2, 3]"),
        (r#" rest("abcd");  "#, r#""bcd""#),
        (r#" rest([1]);"#, "[]"),
        (r#" rest("a");"#, r#""""#),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        match last_object {
            Object::Array(arr) => assert_eq!(arr.inspect(), expected_value),
            Object::String(str) => assert_eq!(str.inspect(), expected_value),
            _ => panic!(),
        };
    })
}

#[test]
fn eval_first_builtin() {
    let testcases = [
        (r#" first([1,2,3]) "#, "1"),
        (r#" first("abcd");  "#, "\"a\""),
        (r#" first([1]);"#, "1"),
        (r#" first("a");"#, "\"a\""),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(last_object.inspect(), expected_value);
    })
}

#[test]
fn eval_push_builtin() {
    let testcases = [
        (r#" push([1,2,3], 4); "#, "[1, 2, 3, 4]"),
        (r#" push([1,2], [3,4]);  "#, "[1, 2, 3, 4]"),
        (r#" push([], [1,2,3,4]);"#, "[1, 2, 3, 4]"),
        (r#" push("a", "bc");"#, r#""abc""#),
        (r#" push("", "abc");"#, r#""abc""#),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(last_object.inspect(), expected_value);
    })
}

#[test]
fn eval_hashmap_pair_count() {
    let testcases = [
        (r#"{"asd": 123, true: "abc"}"#, 2),
        (r#"{}"#, 0),
        (r#"{false: fn(){return 5;}()}"#, 1),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        match last_object {
            Object::HashMap(hashmap) => assert_eq!(hashmap.pairs.len(), expected_value),
            _ => panic!("unexpected last object"),
        }
    })
}

#[test]
fn eval_hashmap_indexing() {
    let testcases = [
        (r#"{"asd": 123, true: "abc"}["asd"]"#, "123"),
        (r#"{}["asd"]"#, "null"),
        (r#"{false: fn(){return 5;}()}[false]"#, "5"),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(last_object.inspect(), expected_value)
    })
}
