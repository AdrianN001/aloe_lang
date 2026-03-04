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

        match &*program.evaluate().unwrap().borrow() {
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

        match &*program.evaluate().unwrap().borrow() {
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

        match &*program.evaluate().unwrap().borrow() {
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

        match &*program.evaluate().unwrap().borrow() {
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

        match &*program.evaluate().unwrap().borrow() {
            Object::Int(int) => assert_eq!(int.value, expected_value),
            Object::Bool(bool) => assert_eq!(bool.value, expected_value == 1),
            _ => panic!(),
        }
    })
}

#[test]
fn test_if_statement_evaluation() {
    let testcases = [
        ("if (5 * 5 + 10 > 34) { 99 } else { 100 }", "99"),
        ("if ((1000 / 2) + 250 * 2 == 1000) { 9999; }", "9999"),
        ("if (true) { 10 }", "10"),
        ("if (false) { 10 }", "null"),
        ("if (1 < 2) { 99 }", "99"),
        ("if (1 > 2) { 99 }", "null"),
        ("if (true) { 10 } else { 20 }", "10"),
        ("if (false) { 10 } else { 20 }", "20"),
        ("if (1 > 2) { 1 } else { 2 }", "2"),
        ("if (false) { 1 } elif (true) { 2 }", "2"),
        ("if (false) { 1 } elif (false) { 2 }", "null"),
        ("if (1 > 2) { 1 } elif (2 > 1) { 2 }", "2"),
        (
            "
        if (false) { 1 }
        elif (false) { 2 }
        elif (true) { 3 }
        else { 4 }
    ",
            "3",
        ),
        (
            "
        if (false) { 1 }
        elif (false) { 2 }
        elif (false) { 3 }
        else { 4 }
    ",
            "4",
        ),
        ("if (false) { 1 } elif (false) { 2 } else { 3 }", "3"),
        ("if (false) { 1 } elif (true) { 2 } else { 3 }", "2"),
        ("if (true) { 1 } elif (true) { 2 } else { 3 }", "1"), // first match wins
    ];

    test_cases_for_input_output(&testcases);
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

        assert!(matches!(
            &*program.evaluate().unwrap().borrow(),
            Object::Null(_)
        ));
    })
}

#[test]
fn test_return_statement() {
    let testcases = [
        ("return 10;", "10"),
        ("20; return 5; 50;", "5"),
        ("return 2*10; 10;", "20"),
        (
            "if (10 > 1) {
            if (10 > 1) {
                return 10;
            }
            return 1;
        }",
            "10",
        ),
        (
            "let f = fn(){ if (true){ return 1; } return 2; }; f();",
            "1",
        ),
        (
            "let f = fn(){ if (false){ return 1; } return 2; }; f();",
            "2",
        ),
        ("let f = fn(){ if (true){ 5; } else { 10; } }; f();", "5"),
        ("let f = fn(){ if (false){ 5; } else { 10; } }; f();", "10"),
        (
            "
        let f = fn(){
            if (true){
                if (true){
                    return 42;
                }
            }
            return 0;
        };
        f();
    ",
            "42",
        ),
        (
            "
        let f = fn(){
            for i <- range(10){
                if (i == 3){
                    return i;
                }
            }
            return 99;
        };
        f();
    ",
            "3",
        ),
        (
            "
        let f = fn(){
            for i <- range(5){
                for j <- range(5){
                    return 123;
                }
            }
            return 0;
        };
        f();
    ",
            "123",
        ),
        (
            "
        let f = fn(){
            for i <- range(5){
                break 10;
            }
        };
        f();
    ",
            "10",
        ),
        (
            "
        let f = fn(){
            return 5;
            10;
        };
        f();
    ",
            "5",
        ),
        (
            "
        let fact = fn(n){
            if (n == 0){
                return 1;
            }
            return n * fact(n - 1);
        };
        fact(5);
    ",
            "120",
        ),
    ];

    test_cases_for_input_output(&testcases);
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

        match &*program.evaluate().unwrap().borrow() {
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

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.to_string();
        let expected_parameters = &testcase.1;
        let expected_body = testcase.2.to_string();

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = program.evaluate().unwrap();

        let binding = last_object.borrow();
        let function_object = match &*binding {
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

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        println!("{}", &input);

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        match &*last_object.borrow() {
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

        match &*last_object.borrow() {
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

        match &*last_object.borrow() {
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

    test_cases_for_input_output(&testcases);
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

        match &*last_object.borrow() {
            Object::Int(integer) => assert_eq!(integer.value, expected_value as i64),
            _ => panic!(),
        };
    })
}

#[test]
fn test_array_indexing_edgecases() {
    let testcases = [
        ("[1,2,3][100]", "null"),
        ("[1,2,3][-10]", "null"),
        ("[][0]", "null"),
    ];

    test_cases_for_input_output(&testcases);
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

        match &*last_object.borrow() {
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

        match &*last_object.borrow() {
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

    test_cases_for_input_output(&testcases);
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

    test_cases_for_input_output(&testcases);
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

        match &*last_object.borrow() {
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

        assert_eq!(last_object.borrow().inspect(), expected_value)
    })
}

#[test]
fn eval_member_operator() {
    let testcases = [
        (r#"let str = "abc"; str.length;"#, "3"),
        (
            r#" "valami".random_method(3); "#,
            r#"unknown method for string: 'random_method'"#,
        ),
        (r#" "abc".reversed(); "#, r#""cba""#),
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

        assert_eq!(last_object.borrow().inspect(), expected_value)
    })
}

#[test]
fn eval_floats() {
    let testcases = [("3.4;", "3.4"), ("-12.54;", "-12.54")];

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

        assert!(matches!(&*last_object.borrow(), Object::FloatObj(_),));

        assert_eq!(last_object.borrow().inspect(), expected_value)
    })
}

#[test]
fn eval_iterator_collect() {
    let testcases = [
        ("range(5).collect()", "[0, 1, 2, 3, 4]"),
        ("range(1,5).collect()", "[1, 2, 3, 4]"),
        ("range(5,1).collect()", "[5, 4, 3, 2]"),
        ("range(1,5,2).collect()", "[1, 3]"),
        ("range(5,1, -2).collect()", "[5, 3]"),
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

        assert!(matches!(&*last_object.borrow(), Object::Array(_),));

        assert_eq!(last_object.borrow().inspect(), expected_value)
    })
}

#[test]
fn eval_for_loop() {
    let testcases = [
        ("for i <- range(10){if (i == 3){break true;}}", "true"),
        ("for i <- range(10){if (i == 20){ break true;}}", "null"),
        ("for i <- range(10){ break 23;}", "23"),
        ("for i <- range(100){}", "null"),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn eval_array_join() {
    let testcases = [
        // Normal case
        (r#"["a", "b", "c"].join(",")"#, r#""a,b,c""#),
        // Empty separator
        (r#"["a", "b", "c"].join("")"#, r#""abc""#),
        // Single element
        (r#"["hello"].join(",")"#, r#""hello""#),
        // Empty array
        (r#"[].join(",")"#, r#""""#),
        // Numbers (if auto string conversion allowed)
        (r#"[1,2,3].join("-")"#, r#""1-2-3""#),
        // Mixed types (if allowed)
        (r#"[1,true,"x"].join("|")"#, r#""1|true|x""#),
        // Multi-character separator
        (r#"["a","b","c"].join("--")"#, r#""a--b--c""#),
        // Separator not provided
        (r#"["a","b"].join()"#, r#""ab""#),
        // Non-array receiver
        (r#""hello".join(",")"#, "unknown method for string: 'join'"),
        (r#""a,b,c".split(",").join("-")"#, r#""a-b-c""#),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn eval_str_split() {
    let testcases = [
        // Normal case
        (r#""a,b,c".split(",")"#, r#"["a", "b", "c"]"#),
        // Space split
        (
            r#""hello world test".split(" ")"#,
            r#"["hello", "world", "test"]"#,
        ),
        // Multi-character separator
        (r#""a--b--c".split("--")"#, r#"["a", "b", "c"]"#),
        // Separator not found
        (r#""abc".split(",")"#, r#"["abc"]"#),
        // Split empty string
        (r#""".split(",")"#, r#"[""]"#),
        // Empty separator (IMPORTANT EDGE CASE)
        (r#""abc".split("")"#, r#"["a", "b", "c"]"#),
        // Trailing separator
        (r#""a,b,".split(",")"#, r#"["a", "b", ""]"#),
        // Leading separator
        (r#"",a,b".split(",")"#, r#"["", "a", "b"]"#),
        // Only separator
        (r#""---".split("-")"#, r#"["", "", "", ""]"#),
        // Non-string receiver
        (r#"123.split(",")"#, "unknown method for int: 'split'"),
        // Missing argument
        (r#""abc".split()"#, r#"["a", "b", "c"]"#),
    ];
    test_cases_for_input_output(&testcases);
}

#[test]
fn test_range_based_for_loop_evaluation() {
    let testcases = [
        // Break trifft
        ("for i <- range(10){ if (i == 3){ break true; } }", "true"),
        // Break trifft nicht
        ("for i <- range(10){ if (i == 20){ break true; } }", "null"),
        // Direktes break
        ("for i <- range(10){ break 23; }", "23"),
        // Kein break
        ("for i <- range(5){}", "null"),
        // Break mit letztem Wert
        ("for i <- range(5){ if (i == 4){ break i; } }", "4"),
        // Break erstes Element
        ("for i <- range(5){ break i; }", "0"),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_list_based_for_loop_evaluation() {
    let testcases = [
        // Element gefunden
        ("for x <- [1,2,3,4]{ if (x == 3){ break x; } }", "3"),
        // Element nicht gefunden
        ("for x <- [1,2,3]{ if (x == 10){ break x; } }", "null"),
        // Direkt break
        ("for x <- [7,8,9]{ break x; }", "7"),
        // Leere Liste
        ("for x <- []{ break 1; }", "null"),
        // Boolean break
        ("for x <- [1,2,3]{ if (x == 2){ break true; } }", "true"),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_string_based_for_loop_evaluation() {
    let testcases = [
        // Zeichen gefunden
        ("for c <- \"hello\"{ if (c == \"e\"){ break c; } }", "\"e\""),
        // Nicht gefunden
        ("for c <- \"abc\"{ if (c == \"z\"){ break c; } }", "null"),
        // Direkt break
        ("for c <- \"xyz\"{ break c; }", "\"x\""),
        // Leerer String
        ("for c <- \"\"{ break 1; }", "null"),
        // Letztes Zeichen
        ("for c <- \"abc\"{ if (c == \"c\"){ break c; } }", "\"c\""),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_break_without_value_for_loop_evaluation() {
    let testcases = [
        ("for i <- range(5){ break; }", "null"),
        ("for x <- [1,2,3]{ if (x == 2){ break; } }", "null"),
        ("for i <- range(5){ continue; break 10; }", "null"),
        (
            "
let arr = [1,2,3];
for x <- arr{
    arr[0] = 99;
}
arr[0];
",
            "99",
        ),
    ];
    test_cases_for_input_output(&testcases);
}

#[test]
fn test_nested_for_loop_evaluation() {
    let testcases = [
        // Inner break darf nur inner loop beenden
        (
            "
        for i <- range(3){
            for j <- range(3){
                break 99;
            }
        }
    ",
            "null",
        ),
        // Outer break
        (
            "
        for i <- range(3){
            if (i == 2){
                break 42;
            }
        }
    ",
            "42",
        ),
        (
            "
        let f = fn(){
            for i <- range(5){
                break 5;
            }
            99;
        };
        f();
    ",
            "99",
        ),
        (
            "
        let f = fn(){
            for i <- range(5){
                return 10;
            }
            99;
        };
        f();
    ",
            "10",
        ),
    ];
    test_cases_for_input_output(&testcases);
}

#[test]
fn test_multiple_break_for_loop_evaluation() {
    let testcases = [(
        "
        for i <- range(10){
            if (i == 2){ break 2; }
            if (i == 5){ break 5; }
        }
    ",
        "2",
    )];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_complex_break_for_loop_evaluation() {
    let testcases = [(
        "
        for x <- \"abc\"{
            if (x == \"b\"){
                break for i <- range(5){
                    if (i == 3){ break i; }
                };
            }
        }
    ",
        "3",
    )];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_variable_reassignment() {
    let testcases = [
        // Simple reassignment
        ("let x = 5; x = 10; x;", "10"),
        // Reassign using expression
        ("let x = 5; x = x + 5; x;", "10"),
        // Reassign boolean
        ("let b = true; b = false; b;", "false"),
        // Reassign multiple times
        ("let x = 1; x = 2; x = 3; x;", "3"),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_index_assignment() {
    let testcases = [
        // Basic index assignment
        ("let arr = [1,2,3]; arr[0] = 10; arr[0];", "10"),
        // Middle element
        ("let arr = [1,2,3]; arr[1] = 99; arr[1];", "99"),
        // Last element
        ("let arr = [1,2,3]; arr[2] = 42; arr[2];", "42"),
        // Negative index (if supported)
        ("let arr = [1,2,3]; arr[-1] = 7; arr[2];", "7"),
        // Index assignment using expression
        ("let arr = [1,2,3]; let i = 1; arr[i] = 50; arr[1];", "50"),
        // Chain read after write
        ("let arr = [1,2]; arr[0] = arr[1]; arr[0];", "2"),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_array_mutation() {
    let testcases = [
        // Mutation must persist
        ("let arr = [1,2,3]; arr[1] = 100; arr;", "[1, 100, 3]"),
        // Shared reference behavior (if assignment copies reference)
        ("let arr = [1,2,3]; let b = arr; b[0] = 9; arr[0];", "9"),
    ];
    test_cases_for_input_output(&testcases);
}

#[test]
fn test_hashmap_index_assignment() {
    let testcases = [
        // Basic hash assignment
        ("let m = {\"a\":1}; m[\"a\"] = 5; m[\"a\"];", "5"),
        // Insert new key (if allowed)
        ("let m = {\"a\":1}; m[\"b\"] = 2; m[\"b\"];", "2"),
        // Overwrite existing
        ("let m = {\"a\":1}; m[\"a\"] = 99; m[\"a\"];", "99"),
        // Int key
        ("let m = {1:10}; m[1] = 20; m[1];", "20"),
        // Bool key
        ("let m = {true:1}; m[true] = 2; m[true];", "2"),
    ];
    test_cases_for_input_output(&testcases);
}

#[test]
fn test_nested_structures() {
    let testcases = [
        // Nested array
        ("let arr = [[1,2],[3,4]]; arr[0][1] = 99; arr[0][1];", "99"),
        // Nested hash
        (
            "let m = {\"a\": {\"b\": 1}}; m[\"a\"][\"b\"] = 42; m[\"a\"][\"b\"];",
            "42",
        ),
    ];
    test_cases_for_input_output(&testcases);
}

#[test]
fn test_assignment_operator_as_value() {
    let testcases = [
        ("let x = 0; x = 5;", "5"),
        ("let arr = [1,2]; arr[0] = 10;", "10"),
    ];
    test_cases_for_input_output(&testcases);
}

#[test]
fn test_for_loop_and_index_assign() {
    let testcases = [(
        "
        let arr = [1,2,3];
        for i <- range(3){
            arr[i] = arr[i] * 2;
        }
        arr[2];
    ",
        "6",
    )];
    test_cases_for_input_output(&testcases);
}

#[test]
fn test_hashmap_methods_and_attributes() {
    let testcases = [
        ("let m = {\"a\":1, \"b\":2}; m.length;", "2"),
        ("let m = {}; m.length;", "0"),
        ("let m = {\"a\":1, \"b\":2}; m.keys.length;", "2"),
        ("let m = {}; m.keys.length;", "0"),
        ("let m = {\"a\":1, \"b\":2}; m.values.length;", "2"),
        ("let m = {\"a\":1}; m.values[0];", "1"),
        ("let m = {}; m.set(\"a\", 10); m.get(\"a\");", "10"),
        // overwrite existing
        ("let m = {\"a\":1}; m.set(\"a\", 5); m.get(\"a\");", "5"),
        // return value of set
        ("let m = {}; m.set(\"x\", 99);", "99"),
        ("let m = {\"a\":1}; m.get(\"a\");", "1"),
        // non existing → null
        ("let m = {\"a\":1}; m.get(\"b\");", "null"),
        // empty map
        ("let m = {}; m.get(\"x\");", "null"),
        ("let m = {\"a\":1}; m.has_key(\"a\");", "true"),
        ("let m = {\"a\":1}; m.has_key(\"b\");", "false"),
        ("let m = {}; m.has_key(\"x\");", "false"),
        ("let m = {\"a\":1}; m.remove(\"a\");", "true"),
        (
            "let m = {\"a\":1}; m.remove(\"a\"); m.has_key(\"a\");",
            "false",
        ),
        ("let m = {\"a\":1}; m.remove(\"b\");", "false"),
        ("let m = {\"a\":1, \"b\":2}; m.clear(); m.length;", "0"),
        ("let m = {}; m.clear(); m.length;", "0"),
        (
            "
        let m = {\"a\":1};
        let c = m.clone();
        c.set(\"a\", 99);
        m.get(\"a\");
    ",
            "1",
        ),
        (
            "
        let m = {\"a\":1};
        let c = m.clone();
        c.set(\"a\", 99);
        m.get(\"a\");
    ",
            "1",
        ),
        (
            "
        let m = {\"a\":1};
        m[\"a\"] = 42;
        m.get(\"a\");
    ",
            "42",
        ),
        (
            "
        let m = {\"a\":1, \"b\":2};
        let count = 0;
        for k <- m.keys{
            count = count + 1;
        }
        count;
    ",
            "2",
        ),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_array_remove() {
    let testcases = [
        // Basic remove
        ("let a=[1,2,3]; a.remove(1); a;", "[1, 3]"),
        // Remove first
        ("let a=[1,2,3]; a.remove(0); a;", "[2, 3]"),
        // Remove last
        ("let a=[1,2,3]; a.remove(2); a;", "[1, 2]"),
        // Negative index remove
        ("let a=[1,2,3]; a.remove(-1); a;", "[1, 2]"),
        // Negative middle
        ("let a=[1,2,3,4]; a.remove(-2); a;", "[1, 2, 4]"),
        // Remove from single element
        ("let a=[5]; a.remove(0); a;", "[]"),
        // Remove using variable
        ("let a=[1,2,3]; let i=1; a.remove(i); a;", "[1, 3]"),
        (
            "
let a=[1,2,3];
a.remove(-1);
a.remove(-1);
a.remove(-1);
a;
",
            "[]",
        ),
        // Remove out of bounds
        ("let a=[1,2,3]; a.remove(10);", "null"),
        // Negative out of bounds
        ("let a=[1,2,3]; a.remove(-10);", "null"),
        // Remove on empty array
        ("let a=[]; a.remove(0);", "null"),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_closure() {
    let testcases = [
        // Outer reassignment
        (
            "
        let x = 10;
        let f = fn() {
            x = 20;
        };
        f();
        x;
    ",
            "20",
        ),
        // Local shadowing
        (
            "
        let x = 10;
        let f = fn() {
            let x = 99;
            x = 50;
        };
        f();
        x;
    ",
            "10",
        ),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_array_and_string_slice() {
    let testcases = [
        ("[1,2,3,4,5].slice(1,4);", "[2, 3, 4]"),
        ("[10,20,30].slice(0,2);", "[10, 20]"),
        ("[1,2,3].slice(0,3);", "[1, 2, 3]"),
        // empty slice
        ("[1,2,3].slice(1,1);", "[]"),
        // start > end
        ("[1,2,3].slice(2,1);", "[]"),
        // end out of bounds
        ("[1,2,3].slice(1,10);", "[2, 3]"),
        // start out of bounds
        ("[1,2,3].slice(10,20);", "[]"),
        ("[1,2,3,4].slice(-3,4);", "[2, 3, 4]"),
        ("[1,2,3,4].slice(0,-1);", "[1, 2, 3]"),
        ("[1,2,3,4].slice(-3,-1);", "[2, 3]"),
        (
            "
let a=[[1],[2],[3]];
let b=a.slice(1,3);
b[0][0];
",
            "2",
        ),
        (
            "
let a=[1,2,3];
let b=a.slice(0,2);
b[0]=99;
a[0];
",
            "1",
        ),
        ("\"hello\".slice(1,4);", "\"ell\""),
        ("\"hello\".slice(0,2);", "\"he\""),
        ("\"hello\".slice(-3,5);", "\"llo\""),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_break_and_continue() {
    let testcases = [
        ("break;", "unexpected break keyword in non-loop context"),
        ("break 5;", "unexpected break keyword in non-loop context"),
        (
            "if (true){ break; }",
            "unexpected break keyword in non-loop context",
        ),
        (
            "if (true){ break 10; }",
            "unexpected break keyword in non-loop context",
        ),
        (
            "continue;",
            "unexpected continue keyword in non-loop context",
        ),
        (
            "if (true){ continue; }",
            "unexpected continue keyword in non-loop context",
        ),
        (
            "
        let f = fn(){
            break;
        };
        f();
    ",
            "error",
        ),
        (
            "
        let f = fn(){
            break 10;
        };
        f();
    ",
            "error",
        ),
        (
            "
        let f = fn(){
            continue;
        };
        f();
    ",
            "error",
        ),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let _expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert!(program.evaluate().is_err());
    })
}

#[test]
fn test_array_deepcopy() {
    let testcases = [
        (
            "
let arr = [[1]];
let copy = arr.clone();
copy[0][0] = 42;
arr[0][0];
",
            "1",
        ),
        (
            "
let a = [[1,2]];
let b = a.clone();
b[0][0] = 99;
a[0][0];
",
            "1",
        ),
        (
            "
let a = [1,2];
let b = a.clone();
b.push(3);
a.length;
",
            "2",
        ),
        (
            "
let a = {\"x\": {\"y\": 1}};
let b = a.clone();
b[\"x\"][\"y\"] = 99;
a[\"x\"][\"y\"];
",
            "1",
        ),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_clone_cyclus() {
    let testcases = [(
        "
let a = [1,2];
a.push(a);
let b = a.clone();
b.length;
",
        "3",
    )];

    test_cases_for_input_output(&testcases);
}

// util

fn test_cases_for_input_output(testcases: &[(&str, &str)]) {
    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1.to_string();

        println!("{}", &input);

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(last_object.borrow().inspect(), expected_value)
    });
}
