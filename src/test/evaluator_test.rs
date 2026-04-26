use crate::{
    ast::{Parser, program},
    lexer::Lexer,
    object::{Object, integer::Integer, panic_obj::RuntimeSignal},
    test::util::test_cases_for_input_output,
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

        match &*program.evaluate_with_default().unwrap().borrow() {
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

        match &*program.evaluate_with_default().unwrap().borrow() {
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

        match &*program.evaluate_with_default().unwrap().borrow() {
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

        match &*program.evaluate_with_default().unwrap().borrow() {
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

        match &*program.evaluate_with_default().unwrap().borrow() {
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
            &*program.evaluate_with_default().unwrap().borrow(),
            Object::Null(_)
        ));
    })
}

#[test]
fn test_return_statement() {
    let testcases = [
        (
            "return 5;",
            "unexpected return keyword in non-function context",
        ),
        (
            "if (true) { return 5; }",
            "cannot return from a non-function context",
        ),
        (
            "
for i <- range(10){
    return 5;
}
",
            "return statement was used in a non-function context",
        ),
        (
            "
if (true){
    if (true){
        return 10;
    }
}
",
            "cannot return from a non-function context",
        ),
        (
            "
5;
return 10;
",
            "unexpected return keyword in non-function context",
        ),
        (
            "
let f = fn(){
    return 5;
};
f();
",
            "5",
        ),
        (
            "
let f = fn(){
    let g = fn(){
        return 99;
    };
    g();
};
f();
",
            "99",
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
fn test_variable_evaluate_with_default() {
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

        match &*program.evaluate_with_default().unwrap().borrow() {
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

        let last_object = program.evaluate_with_default().unwrap();

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

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => panic!("{}", err),
            _ => todo!(),
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

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => panic!("{}", err),
            _ => todo!(),
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

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => panic!("{}", err),
            _ => todo!(),
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
        (r#"[true, 3, "asd", false, true][1+1]"#, "asd"),
        ("let i = 0; [i][i];", "0"),
        (
            "[][1];",
            "indexing an array of size '0' with index '1' is illegal.",
        ),
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

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => panic!("{}", err),
            _ => todo!(),
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
        (
            "[1,2,3][100]",
            "indexing an array of size '3' with index '100' is illegal.",
        ),
        (
            "[1,2,3][-10]",
            "indexing an array of size '3' with index '-7' is illegal.",
        ),
        (
            "[][0]",
            "indexing an array of size '0' with index '0' is illegal.",
        ),
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

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => panic!("{}", err),
            _ => todo!(),
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
        (r#" rest("abcd");  "#, r#"bcd"#),
        (r#" rest([1]);"#, "[]"),
        (r#" rest("a");"#, r#""#),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => panic!("{}", err),
            _ => todo!(),
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
        (r#" first("abcd");  "#, "a"),
        (r#" first([1]);"#, "1"),
        (r#" first("a");"#, "a"),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn eval_push_builtin() {
    let testcases = [
        (r#" push([1,2,3], 4); "#, "[1, 2, 3, 4]"),
        (r#" push([1,2], [3,4]);  "#, "[1, 2, 3, 4]"),
        (r#" push([], [1,2,3,4]);"#, "[1, 2, 3, 4]"),
        (r#" push("a", "bc");"#, r#"abc"#),
        (r#" push("", "abc");"#, r#"abc"#),
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

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => panic!("{}", err),
            _ => todo!(),
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
        (
            r#"{}["asd"]"#,
            "Stack trace:\n\t at <global>\nItemNotFoundError: hashmap has no key: 'asd'",
        ),
        (r#"{false: fn(){return 5;}()}[false]"#, "5"),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.into();
        let expected_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => panic!("{}", err),
            _ => todo!(),
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
            "unknown method for string: 'random_method'",
        ),
        (r#" "abc".reversed(); "#, "cba"),
    ];

    test_cases_for_input_output(&testcases);
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

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => panic!("{}", err),
            _ => todo!(),
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

        let last_object = match program.evaluate_with_default() {
            Ok(x) => x,
            Err(RuntimeSignal::Panic(err)) => panic!("{}", err),
            _ => todo!(),
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
        (r#"["a", "b", "c"].join(",")"#, r#"a,b,c"#),
        // Empty separator
        (r#"["a", "b", "c"].join("")"#, r#"abc"#),
        // Single element
        (r#"["hello"].join(",")"#, r#"hello"#),
        // Empty array
        (r#"[].join(",")"#, r#""#),
        // Numbers (if auto string conversion allowed)
        (r#"[1,2,3].join("-")"#, r#"1-2-3"#),
        // Mixed types (if allowed)
        (r#"[1,true,"x"].join("|")"#, r#"1|true|x"#),
        // Multi-character separator
        (r#"["a","b","c"].join("--")"#, r#"a--b--c"#),
        // Separator not provided
        (r#"["a","b"].join()"#, r#"ab"#),
        // Non-array receiver
        (r#""hello".join(",")"#, "unknown method for string: 'join'"),
        (r#""a,b,c".split(",").join("-")"#, r#"a-b-c"#),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn eval_str_split() {
    let testcases = [
        // Normal case
        (r#""a,b,c".split(",")"#, r#"[a, b, c]"#),
        // Space split
        (
            r#""hello world test".split(" ")"#,
            r#"[hello, world, test]"#,
        ),
        // Multi-character separator
        (r#""a--b--c".split("--")"#, r#"[a, b, c]"#),
        // Separator not found
        (r#""abc".split(",")"#, r#"[abc]"#),
        // Split empty string
        (r#""".split(",")"#, r#"[]"#),
        // Empty separator (IMPORTANT EDGE CASE)
        (r#""abc".split("")"#, r#"[a, b, c]"#),
        // Trailing separator
        (r#""a,b,".split(",")"#, r#"[a, b, ]"#),
        // Leading separator
        (r#"",a,b".split(",")"#, r#"[, a, b]"#),
        // Only separator
        (r#""---".split("-")"#, r#"[, , , ]"#),
        // Non-string receiver
        (r#"123.split(",")"#, "unknown method for int: 'split'"),
        // Missing argument
        (r#""abc".split()"#, r#"[a, b, c]"#),
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
        ("for c <- \"hello\"{ if (c == \"e\"){ break c; } }", "e"),
        // Nicht gefunden
        ("for c <- \"abc\"{ if (c == \"z\"){ break c; } }", "null"),
        // Direkt break
        ("for c <- \"xyz\"{ break c; }", "x"),
        // Leerer String
        ("for c <- \"\"{ break 1; }", "null"),
        // Letztes Zeichen
        ("for c <- \"abc\"{ if (c == \"c\"){ break c; } }", "c"),
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
        (
            "let m = {\"a\":1}; m.get(\"b\");",
            "hashmap has no key: 'b'",
        ),
        // empty map
        ("let m = {}; m.get(\"x\");", "hashmap has no key: 'x'"),
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
        (
            "let a=[1,2,3]; a.remove(10);",
            "array.remove(), array with size: 3 was indexed with: 10",
        ),
        // Negative out of bounds
        (
            "let a=[1,2,3]; a.remove(-10);",
            "array.remove(), array with size: 3 was indexed with: -7",
        ),
        // Remove on empty array
        (
            "let a=[]; a.remove(0);",
            "array.remove(), array with size: 0 was indexed with: 0",
        ),
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
            "99",
        ),
        (
            "
let arr = [1,2,3,4];
let s = arr.slice(1,3);
s[0] = 99;
arr[1];
",
            "99",
        ),
        (
            "let arr = [1,2,3,4];
let s = arr.slice(1,4);
s[0] = 10;
s[1] = 20;
arr[1] + arr[2];
",
            "30",
        ),
        (
            "
let arr = [1,2,3,4];
let s = arr.slice(1,3);
arr[1] = 77;
s[0];
",
            "77",
        ),
        (
            "
let arr = [10,20,30,40];
let s = arr.slice(1,3);
s[1] = 999;
arr[2];
",
            "999",
        ),
        (
            "
let arr = [1,2,3,4,5];
let s1 = arr.slice(1,5);
let s2 = s1.slice(1,3);

s2[0] = 100;
arr[2];
",
            "100",
        ),
        (
            "
let arr = [1,2,3,4];
let s1 = arr.slice(1,3);
let s2 = arr.slice(1,3);

s1[0] = 500;
s2[0];
",
            "500",
        ),
        ("\"hello\".slice(1,4);", "ell"),
        ("\"hello\".slice(0,2);", "he"),
        ("\"hello\".slice(-3,5);", "llo"),
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

        assert!(program.evaluate_with_default().is_err());
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

#[test]
fn test_questionmak_operator() {
    let testcases = [
        ("{}.get();", "expected 1 argument for hashmap.get(), got: 0"),
        (
            "{}.remove();",
            "expected 1 argument for hashmap.remove(), got: 0",
        ),
        (
            "
let x = {}.get();
x;
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    {}.get()?; 
    10;
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    let m = {\"a\": 1};
    let v = m.get(\"a\")?;
    v;
};
f();
",
            "1",
        ),
        (
            "let g = fn(){
    {}.get()?;
    5;
};

let f = fn(){
    g()?;
    10;
};

f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    if (true){
        len()?;
    }
    10;
};
f();
",
            "expected 1 value, got 0 value.",
        ),
        (
            "
let f = fn(x){ x };
f({}.get()?);
",
            "tried to use ? on a function, without function-context",
        ),
        (
            "
let f = fn(){
    [1, {}.get()?, 3];
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let m = {\"a\": 10};
[1, m.get(\"a\")!, 3][1];
",
            "10",
        ),
        (
            "
let f = fn(x){ x };

let g = fn(){
    f({}.get()?);
};

g();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    {}.get()? + 5;
};
f();
",
            "unexpected operand types: return value + integer",
        ),
        (
            "
let f = fn(){
    for i <- range(5){
        {}.get()?; 
    }
    10;
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    let m = {};
    (m.get(\"a\")?) + 10;
};
f();
",
            "unexpected operand types: return value + integer",
        ),
        (
            "
let f = fn(){
    {\"x\": {}.get()?};
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let m = {\"a\": 5};
{\"x\": m.get(\"a\")!}[\"x\"];
",
            "5",
        ),
        (
            "
let f = fn(x){ x };

let g = fn(){
    f({}.get()?);
};

g();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    5 + {}.get()?;
};
f();
",
            "unexpected operand types: int + return value",
        ),
        (
            "
let m = {\"a\": 4};
2 * m.get(\"a\")!;
",
            "8",
        ),
        (
            "
let f = fn(){
    return {}.get()?;
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    let m = {};
    [1,2,3][m.get()?];
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let m = {\"a\": 3};
m.get(\"a\")?.as_str();
",
            "3",
        ),
        (
            "
fn(){
let m = {\"a\": 3};
m.get()?.as_str();
                }()",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let m = {\"a\": 3};
m.get(\"b\")?.as_str();
",
            "tried to use ? on a function, without function-context",
        ),
        (
            "
let f = fn(){
    [ {\"x\": {}.get()?} ];
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    1 + (2 * ({}.get()?));
};
f();
",
            "unexpected operand types: int * return value",
        ),
        (
            "
let f = fn(){
    let g = fn(){
        {}.get()?;
        10;
    };
    g();
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    for i <- range(10){
        {}.get()?;
    }
    99;
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    for i <- range(3){
        for j <- range(3){
            {}.get()?;
        }
    }
    42;
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    for i <- range(10){
        break {}.get()?;
    }
};
f();
",
            "expected 1 argument for hashmap.get(), got: 0",
        ),
        (
            "
let f = fn(){
    for i <- range(5){
        break 10;
        {}.get(\"a\")?;
    }
};
f();
",
            "10",
        ),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_bang_operator() {
    let testcases = [
        (
            "
let f = fn(){
    {}.get()!;
};
f();
",
            false,
        ),
        (
            "
let m = {\"a\": 1};
m.get(\"a\")!;
",
            true,
        ),
        (
            "
{}.get()!;
",
            false,
        ),
        (
            "
let m = {\"a\": 10};
[1, m.get(\"a\")!, 3][1];
",
            true,
        ),
        (
            "
let id = fn(x){ x };

let m = {\"a\": 7};

id(m.get(\"a\")!);
",
            true,
        ),
        (
            "
let m = {\"a\": 9};

let f = fn(){
    return m.get(\"a\")!;
};

f();
",
            true,
        ),
        (
            "
let m = {\"a\": 1};
[10,20,30][m.get(\"a\")!];
",
            true,
        ),
        (
            "
let m = {\"a\": 1};
[10,20,30][m.get()!];
",
            false,
        ),
        (
            "
let m = {\"a\": 3};
m.get(\"a\")!.as_str();
",
            true,
        ),
    ];

    testcases.iter().for_each(|testcase| {
        let input: String = testcase.0.into();
        let is_ok = testcase.1;

        let lexer = Lexer::new(input.clone());
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        println!("{}", input);

        assert_eq!(program.evaluate_with_default().is_ok(), is_ok);
    });
}

#[test]
fn test_coalescing() {
    let testcases = [
        ("1 ?? 2;", "1"),
        ("true ?? false;", "true"),
        ("\"hello\" ?? \"world\";", "hello"),
        ("false ?? true;", "true"),
        ("0 ?? 10;", "10"),
        ("\"\" ?? \"fallback\";", "fallback"),
        ("[] ?? [1,2];", "[1, 2]"),
        ("{} ?? {\"a\":1};", "{a:1}"),
        (
            "
let f = fn(){
    5 ?? {}.get(\"a\");
};
f();
",
            "5",
        ),
        ("(1 + 2) ?? 5;", "3"),
        ("(0 + 0) ?? 5;", "5"),
        ("1 ?? 2 ?? 3;", "1"),
        ("(2 ?? 5) + 3;", "5"),
        (
            "
let f = fn(){
    {}.get(\"a\") ?? 5;
};
f();
",
            "5",
        ),
        (
            "
let x = 0;
let f = fn(){
    x = 1;
    5 ?? (x = 2);
};
f();
x;
",
            "1",
        ),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
pub fn test_basic_struct() {
    let testcases = [
        (
            "
struct Person{
    first_name;
    last_name;
};
let p = Person();
p.first_name;
",
            "null",
        ),
        (
            "
struct Person{
    first_name;
    last_name;
};
let p = Person();
p.last_name;
",
            "null",
        ),
        (
            "
struct Person{
    first_name;
    last_name;
};
let p = Person(\"Max\", \"Mustermann\");
p.first_name;
",
            "Max",
        ),
        (
            "
struct Person{
    first_name;
    last_name;
};
let p = Person(\"Max\", \"Mustermann\");
p.last_name;
",
            "Mustermann",
        ),
        (
            "
struct Person{
    first_name;
    last_name;
};
Person(\"Max\");
",
            "expected 2 arguments for default constructor, got: 1.",
        ),
        (
            "
struct Person{
    first_name;
    last_name;
};
Person(\"A\", \"B\", \"C\");
",
            "expected 2 arguments for default constructor, got: 3.",
        ),
        (
            "
struct Person{
    first_name;
    last_name;
};
let p = Person(\"Max\", \"A\");
p.first_name = \"John\";
p.first_name;
",
            "John",
        ),
        (
            "
struct Person{
    first_name;
    last_name;
};
let p = Person(\"Max\", \"A\");
p.first_name = \"John\";
p.last_name;
",
            "A",
        ),
        (
            "
struct Person{
    name;
};
let p1 = Person(\"A\");
let p2 = p1;

p2.name = \"B\";
p1.name;
",
            "B",
        ),
        (
            "
struct Person{
    name;
};

struct Wrapper{
    p;
};

let w = Wrapper(Person(\"Max\"));
w.p.name;
",
            "Max",
        ),
        (
            "
struct Person{
    name;
};
let p = Person(\"Max\");
p.name = \"Anna\";
p.name;
",
            "Anna",
        ),
        (
            "
struct Data{
    a;
    b;
    c;
};
let d = Data(1, true, \"hi\");
d.a;
",
            "1",
        ),
        (
            "
struct Data{
    a;
    b;
    c;
};
let d = Data(1, true, \"hi\");
d.b;
",
            "true",
        ),
        (
            "
struct Data{
    a;
    b;
    c;
};
let d = Data(1, true, \"hi\");
d.c;
",
            "hi",
        ),
        (
            "
struct Person{
    name;
};
let p = Person(\"Max\");
p.age;
",
            "Person has no attribute: 'age'",
        ),
        (
            "
struct Person{
    name;
};

let f = fn(p){ p.name };

f(Person(\"Max\"));
",
            "Max",
        ),
        (
            "
struct Person{
    name;
};
let p = Person(\"A\");
p.name = \"B\";
p.name = \"C\";
p.name;
",
            "C",
        ),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_struct_methods() {
    let testcases = [
        (
            "
struct Car{
    color;

    fun get_color(this){
        return this.color;
    }
};

let c = Car(\"red\");
c.get_color();
",
            "red",
        ),
        (
            "
struct Car{
    color;
    plate;

    fun get_plate(this){
        return this.plate;
    }
};

let c = Car(\"red\", \"AB123\");
c.get_plate();
",
            "AB123",
        ),
        (
            "
struct Car{
    color;

    fun get_color(this){
        this.color;
    }
};

let c = Car(\"blue\");
c.get_color();
",
            "blue",
        ),
        (
            "
struct Car{
    color;

    fun set_color(this, new_color){
        this.color = new_color;
    }
};

let c = Car(\"red\");
c.set_color(\"green\");
c.color;
",
            "green",
        ),
        (
            "
struct Car{
    color;

    fun repaint(this, new_color){
        this.color = new_color;
        return this.color;
    }
};

let c = Car(\"red\");
c.repaint(\"black\");
",
            "black",
        ),
        (
            "
struct Car{
    color;

    fun get_color(this){
        return this.color;
    }

    fun set_color(this, c){
        this.color = c;
    }
};

let c = Car(\"red\");
c.set_color(\"yellow\");
c.get_color();
",
            "yellow",
        ),
        (
            "
struct Car{
    fun bad(){
        return 1;
    }
};
",
            "expected at least 1 parameter for method (to be used as 'this'), got: 0",
        ),
        (
            "
struct Car{
    color;

    fun is_color(this, c){
        return this.color == c;
    }
};

let c = Car(\"red\");
c.is_color(\"red\");
",
            "true",
        ),
        (
            "
struct Car{
    color;

    fun get_color(this){
        return this.color;
    }

    fun same_color(this, other){
        return this.get_color() == other;
    }
};

let c = Car(\"blue\");
c.same_color(\"blue\");
",
            "true",
        ),
        (
            "
struct Car{
    color;

    fun get_color(this){
        return this.color;
    }
};

let c = Car(\"red\");
c.get_color().length;
",
            "3",
        ),
        (
            "
struct Car{
    fun test(this, x){}
};

let c = Car();
c.test();
",
            "expected 2 arguments for function 'test()', got: 1",
        ),
        (
            "
struct Car{
    color;

    fun set(this, c){
        this.color = c;
    }
};

let c1 = Car(\"red\");
let c2 = c1;

c2.set(\"green\");
c1.color;
",
            "green",
        ),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_while_loop_eval() {
    let testcases = [
        ("let i = 0; while i < 5 { i = i + 1; }", "null"),
        (
            "let i = 0; while i < 10 { if (i == 3){ break i; } i = i + 1; }",
            "3",
        ),
        ("let i = 0; while i < 3 { i = i + 1; }", "null"),
        (
            "let i = 0; while { if (i == 4){ break i; } i = i + 1; }",
            "4",
        ),
        ("let cond = true; while cond { break 42; }", "42"),
        ("while false { break 1; }", "null"),
        ("while true { break; }", "null"),
        (
            "
    let i = 0;
    let sum = 0;
    while i < 5 {
        i = i + 1;
        if (i == 3){ continue; }
        sum = sum + i;
    }
    sum;
    ",
            "12",
        ),
        (
            "
    let i = 0;
    while i < 3 {
        let j = 0;
        while j < 3 {
            break 99;
        }
        i = i + 1;
    }
    ",
            "null",
        ),
        (
            "
    let i = 0;
    while i < 5 {
        if (i == 2){
            break 100;
        }
        i = i + 1;
    }
    ",
            "100",
        ),
        (
            "let i = 0;
    while (i < 3) {
        i = i + 1;
    }
    ",
            "null",
        ),
        (
            "
    let f = fn(){
        let i = 0;
        while i < 5 {
            if (i == 3){
                return i;
            }
            i = i + 1;
        }
        return 99;
    };
    f();
    ",
            "3",
        ),
        (
            "let i = 0;
    while true {
        break i + 10;
    }
    ",
            "10",
        ),
        (
            "
    let s = \"abc\";
    let i = 0;
    while i < s.length {
        if (s[i] == \"b\"){
            break s[i];
        }
        i = i + 1;
    }
    ",
            "b",
        ),
        (
            "
    let arr = [1,2,3];
    let i = 0;
    while i < 3 {
        arr[i] = arr[i] * 2;
        i = i + 1;
    }
    arr[2];
    ",
            "6",
        ),
        (
            "continue;",
            "unexpected continue keyword in non-loop context",
        ),
    ];

    test_cases_for_input_output(&testcases);
}

#[test]
fn test_compound_assignment() {
    let testcases = [
        ("let i = 0; i += 3; i;", "3"),
        ("let i = 6; i += (i + 3); i;", "15"),
        ("let i = 0; i -= 3; i;", "-3"),
        ("let i = 2; i *= 3; i;", "6"),
        ("let i = 3; i /= 2; i;", "1.5"),
        ("let i = 4; i **= 2; i;", "16"),
        ("let i = 10; i %= 2; i;", "0"),
        ("let i = 4; i <<= 3; i;", "32"),
        ("let i = 4; i >>= 2; i;", "1"),
        ("let i = 5; i &= 3; i;", "1"),
        ("let i = 5; i |= 2; i;", "7"),
        ("let i = 65565; i ^= i; i;", "0"),
    ];

    test_cases_for_input_output(&testcases);
}
