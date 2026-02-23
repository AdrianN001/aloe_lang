use crate::ast::expression::Expression;
use crate::ast::expression::boolean::Boolean;
use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::string_expr::StringExpr;
use crate::ast::statement::Statement;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::{Parser, program};
use crate::lexer::Lexer;
use crate::object::Object;
use crate::object::integer::Integer;
use crate::token::token_type::TokenType;
use crate::token::{self, Token};

#[test]
fn test_basic_let_statement_parse() {
    let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";

    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    let program = match parser.into_a_program() {
        Ok(functional_program) => functional_program,
        Err(error_feedback) => panic!("{}", error_feedback),
    };

    assert_eq!(program.statements.len(), 3);

    let expected_identifiers = ["x", "y", "foobar"];

    for i in 0..program.statements.len() {
        match &program.statements[i] {
            Statement::Let(let_statement) => {
                assert_eq!(let_statement.name.value, expected_identifiers[i]);
            }
            _ => {
                panic!();
            }
        }
    }
}

#[test]
fn test_let_statement() {
    let testcases = [("let x = 5;", "x", 5), ("let y = 20;", "y", 20)];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.to_string();
        let expected_identifier_name = testcase.1.to_string();
        let expected_variable_value = testcase.2;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let let_statement = match &program.statements[0] {
            Statement::Let(expr) => expr,
            _ => panic!(),
        };

        let variable_name = &let_statement.name.value;

        assert_eq!(variable_name, &expected_identifier_name);

        let variable_value_expression = match &let_statement.value {
            Expression::IntegerLiteral(integer_expression) => integer_expression,
            _ => panic!(),
        };

        assert_eq!(variable_value_expression.value, expected_variable_value);
    });
}

#[test]
fn test_basic_error_recognision() {
    let input = "
let x = 5;
let y 10;
let foobar = 838383;
";
    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    let program = parser.into_a_program();

    assert!(program.is_err());
}

#[test]
fn test_basic_return_statements() {
    let input = "
return 10;
return 12;
return add(10,12);
";
    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().unwrap();

    assert_eq!(program.statements.len(), 3);
}

#[test]
fn test_return_statement() {
    let testcases = [("return 5;", 5), ("return 20;", 20)];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.to_string();
        let expected_variable_value = testcase.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let return_statement = match &program.statements[0] {
            Statement::Return(expr) => expr,
            _ => panic!(),
        };

        let variable_value_expression = match &return_statement.value {
            Expression::IntegerLiteral(integer_expression) => integer_expression,
            _ => panic!(),
        };

        assert_eq!(variable_value_expression.value, expected_variable_value);
    });
}

#[test]
fn test_identiefier_expression_parsing() {
    let input = "
    foobar;
";
    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().unwrap();

    assert_eq!(program.statements.len(), 1);

    assert!(matches!(program.statements[0], Statement::Expression(_)));

    if let Statement::Expression(expression) = &program.statements[0] {
        assert!(matches!(expression.expression, Expression::Identifier(_)))
    } else {
        panic!("unmöglich!");
    }
}

#[test]
fn test_basic_integer_parsing() {
    let input = "
    6;
";
    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().unwrap();

    assert_eq!(program.statements.len(), 1);

    assert!(matches!(program.statements[0], Statement::Expression(_)));

    if let Statement::Expression(expression) = &program.statements[0] {
        assert!(matches!(
            expression.expression,
            Expression::IntegerLiteral(_)
        ))
    } else {
        panic!("unmöglich!");
    }
}

#[test]
fn test_prefix_operator_parsing() {
    let testcases = [("!5;", "!", 5), ("-15;", "-", 15)];

    testcases.iter().for_each(|testcase| {
        let lexer = Lexer::new(testcase.0.to_string());
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        assert!(matches!(program.statements[0], Statement::Expression(_)));

        if let Statement::Expression(expression) = &program.statements[0] {
            assert!(matches!(expression.expression, Expression::Prefix(_)));
            if let Expression::Prefix(prefix_expression) = &expression.expression {
                assert_eq!(prefix_expression.operator, testcase.1);
            }
        } else {
            panic!("unmöglich!");
        }
    });
}

#[test]
fn test_parsing_infix_expression() {
    let testcases = [
        ("5 + 5;", 5, "+", 5),
        ("5 - 5;", 5, "-", 5),
        ("5 * 5;", 5, "*", 5),
        ("5 / 5;", 5, "/", 5),
        ("5 > 5;", 5, ">", 5),
        ("5 < 5;", 5, "<", 5),
        ("5 == 5;", 5, "==", 5),
        ("5 != 5;", 5, "!=", 5),
    ];

    testcases.iter().for_each(|test_case| {
        let expression = test_case.0.to_string();
        let expected_left = test_case.1;
        let exprected_op = test_case.2.to_string();
        let exptected_right = test_case.3;

        let lexer = Lexer::new(expression);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let parsed_statement_expression = match &program.statements[0] {
            Statement::Expression(expr) => expr,
            _ => panic!(),
        };

        let parsed_expression = match &parsed_statement_expression.expression {
            Expression::Infix(infix_expr) => infix_expr,
            _ => panic!(),
        };

        match (
            *parsed_expression.left.clone(),
            *parsed_expression.right.clone(),
        ) {
            (
                Expression::IntegerLiteral(left_expression),
                Expression::IntegerLiteral(right_expression),
            ) => {
                assert_eq!(left_expression.value, expected_left);
                assert_eq!(right_expression.value, exptected_right);
            }
            _ => panic!(),
        }
        assert_eq!(parsed_expression.operator, exprected_op);
    })
}

#[test]
fn test_basic_boolean_parse() {
    let testcases = [("true;", true), ("false;", false)];

    testcases.iter().for_each(|test_case| {
        let input = test_case.0.to_string();
        let expected_token_value = test_case.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let parsed_expression_statement = match &program.statements[0] {
            Statement::Expression(expr) => expr,
            _ => panic!(),
        };

        let parsed_expression = match &parsed_expression_statement.expression {
            Expression::Bool(bool) => bool,
            _ => panic!(),
        };

        assert_eq!(parsed_expression.value, expected_token_value);
    })
}

#[test]
fn test_boolean_infix_expression() {
    let testcases = [
        ("true == false;", true, "==", false),
        ("true != true;", true, "!=", true),
    ];

    testcases.iter().for_each(|test_case| {
        let expression = test_case.0.to_string();
        let expected_left = test_case.1;
        let exprected_op = test_case.2.to_string();
        let exptected_right = test_case.3;

        let lexer = Lexer::new(expression);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let parsed_statement_expression = match &program.statements[0] {
            Statement::Expression(expr) => expr,
            _ => panic!(),
        };

        let parsed_expression = match &parsed_statement_expression.expression {
            Expression::Infix(infix_expr) => infix_expr,
            _ => panic!(),
        };

        match (
            *parsed_expression.left.clone(),
            *parsed_expression.right.clone(),
        ) {
            (Expression::Bool(left_expression), Expression::Bool(right_expression)) => {
                assert_eq!(left_expression.value, expected_left);
                assert_eq!(right_expression.value, exptected_right);
            }
            _ => panic!(),
        }
        assert_eq!(parsed_expression.operator, exprected_op);
    })
}

#[test]
fn test_prefix_bool_parsing() {
    let testcases = [("!true;", "!", true), ("!false;", "!", false)];

    testcases.iter().for_each(|test_case| {
        let input = test_case.0.to_string();
        let operator = test_case.1;
        let bool_val = test_case.2;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let parsed_expression_statement = match &program.statements[0] {
            Statement::Expression(expr) => expr,
            _ => panic!(""),
        };

        let parsed_expr = match &parsed_expression_statement.expression {
            Expression::Prefix(prefix_expr) => prefix_expr,
            _ => panic!(""),
        };

        assert_eq!(parsed_expr.operator, operator);

        match *parsed_expr.right.clone() {
            Expression::Bool(boolean_expr) => {
                assert_eq!(boolean_expr.value, bool_val);
            }
            _ => panic!(),
        };
    })
}

#[test]
fn test_operator_precedence() {
    let testcases = [
        ("-a*b;", "((-a) * b)"),
        ("a+b+c;", "((a + b) + c)"),
        ("a*b*c;", "((a * b) * c)"),
        ("a+b/c;", "(a + (b / c))"),
        ("5>4 == 3<4;", "((5 > 4) == (3 < 4))"),
        ("a + add(b * c) + d;", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
    ];

    testcases.iter().for_each(|test_case| {
        let input = test_case.0.to_string();
        let expected_parsed_output = test_case.1.to_string();

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);
        assert_eq!(expected_parsed_output, program.to_string());
    });
}

#[test]
fn test_if_parsing() {
    let input = "if (x<y) { x }".to_string();

    let lexer = Lexer::new(input);
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().unwrap();
    assert_eq!(program.statements.len(), 1);

    let expression = match &program.statements[0] {
        Statement::Expression(expr) => expr,
        _ => panic!(),
    };

    let if_expr = match &expression.expression {
        Expression::If(if_expression) => if_expression,
        _ => panic!(),
    };

    let condition = match *if_expr.condition.clone() {
        Expression::Infix(infix_expr) => infix_expr,
        _ => panic!(),
    };

    match (*condition.left.clone(), *condition.right.clone()) {
        (Expression::Identifier(left_identifier), Expression::Identifier(right_identifier)) => {
            assert_eq!(left_identifier.token.literal, "x".to_string());
            assert_eq!(right_identifier.token.literal, "y".to_string());
        }
        _ => panic!(""),
    }

    assert_eq!(condition.operator, "<".to_string());

    assert!(if_expr.alternative.is_none());
}

#[test]
fn test_function_parameter_parsing() {
    let testcases = [
        ("fn(){};", Vec::new()),
        ("fn(a){};", ["a"].to_vec()),
        ("fn(a,b){};", ["a", "b"].to_vec()),
        ("fn(a, b, c){};", ["a", "b", "c"].to_vec()),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0.to_string();
        let expected_parameters: Vec<String> = testcase
            .1
            .iter()
            .map(|str_ref| str_ref.to_string())
            .collect();

        println!("{}", &input);

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let expr_statement = match &program.statements[0] {
            Statement::Expression(expr) => expr,
            _ => panic!(),
        };

        let func_expr = match &expr_statement.expression {
            Expression::Function(func_expression) => func_expression,
            _ => panic!(),
        };

        assert_eq!(func_expr.parameters.len(), expected_parameters.len());

        func_expr
            .parameters
            .iter()
            .zip(expected_parameters.iter())
            .for_each(|(func_parameter, expected_parameter)| {
                assert_eq!(&func_parameter.token.literal, expected_parameter);
            });
    });
}

#[test]
fn test_array_liter_parsing() {
    let testcases = [
        (
            "[1, 2];",
            [
                Expression::IntegerLiteral(IntegerLiteral {
                    token: Token::simple(TokenType::Integer, "1"),
                    value: 1,
                }),
                Expression::IntegerLiteral(IntegerLiteral {
                    token: Token::simple(TokenType::Integer, "2"),
                    value: 2,
                }),
            ]
            .to_vec(),
        ),
        (
            r#"[true, "hello, world", 1];"#,
            [
                Expression::Bool(Boolean {
                    token: Token::simple(TokenType::KwTrue, "true"),
                    value: true,
                }),
                Expression::String(StringExpr {
                    token: Token::simple(TokenType::String, "hello, world"),
                    value: "hello, world".into(),
                }),
                Expression::IntegerLiteral(IntegerLiteral {
                    token: Token::simple(TokenType::Integer, "1"),
                    value: 1,
                }),
            ]
            .to_vec(),
        ),
        ("[]", Vec::new()),
    ];

    testcases.iter().for_each(|test_case| {
        let input = test_case.0.into();
        let expected_expressions = &test_case.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let expr_statement = match &program.statements[0] {
            Statement::Expression(expr) => expr,
            _ => panic!(),
        };

        let array_expr = match &expr_statement.expression {
            Expression::Array(arr) => arr,
            _ => panic!(),
        };

        assert_eq!(array_expr.elements.len(), expected_expressions.len());

        array_expr
            .elements
            .iter()
            .zip(expected_expressions)
            .for_each(|(expression, expected_expr)| {
                assert_eq!(expression.to_string(), expected_expr.to_string());
            });
    });
}

#[test]
fn test_index_parsing() {
    let input = "myArray[1+1];";

    let lexer = Lexer::new(input.into());
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().unwrap();

    assert_eq!(program.statements.len(), 1);

    let expr_statement = match &program.statements[0] {
        Statement::Expression(expr) => expr,
        _ => panic!(),
    };

    let index_expr = match &expr_statement.expression {
        Expression::Index(indx) => indx,
        _ => panic!(),
    };

    match &*index_expr.left {
        Expression::Identifier(identifier) => {
            assert_eq!(identifier.value, "myArray");
        }
        _ => panic!(),
    }

    match &*index_expr.right {
        Expression::Infix(infix_expr) => {
            match &*infix_expr.right {
                Expression::IntegerLiteral(left_integer) => assert_eq!(left_integer.value, 1),
                _ => panic!(),
            };

            match &*infix_expr.left {
                Expression::IntegerLiteral(right_integer) => assert_eq!(right_integer.value, 1),
                _ => panic!(),
            };
            assert_eq!(infix_expr.operator, "+");
        }
        _ => panic!(),
    }
}
