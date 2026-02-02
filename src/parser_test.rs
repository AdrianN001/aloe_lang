use crate::ast::expression::Expression;
use crate::ast::{Parser, program};
use crate::ast::statement::Statement;
use crate::ast::statement::let_statement::LetStatement;
use crate::lexer::Lexer;

#[test]
fn test_basic_let_statement_parse(){
    let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";


    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    let program = match parser.into_a_program(){
        Ok(functional_program) => functional_program,
        Err(error_feedback) => panic!("{}", error_feedback)
    };

    assert_eq!(program.statements.len(), 3);

    let expected_identifiers = ["x", "y", "foobar"];

    for i in 0..program.statements.len(){
        match &program.statements[i]{
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
fn test_basic_error_recognision(){
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
fn test_basic_return_statements(){
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
fn test_identiefier_expression_parsing(){
    let input = "
    foobar;
";
    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().unwrap();

    assert_eq!(program.statements.len(), 1);
    
    assert!(
        matches!(
            program.statements[0], 
            Statement::Expression(_)
        )
    );

    if let Statement::Expression(expression) = &program.statements[0]{
        
        assert!(
            matches!(
                expression.expression, 
                Expression::Identifier(_)
            )
        )

    }else{
        panic!("unmöglich!");
    }
}


#[test]
fn test_basic_integer_parsing(){
    let input = "
    6;
";
    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().unwrap();

    assert_eq!(program.statements.len(), 1);
    
    assert!(
        matches!(
            program.statements[0], 
            Statement::Expression(_)
        )
    );

    if let Statement::Expression(expression) = &program.statements[0]{
        assert!(
            matches!(
                expression.expression, 
                Expression::IntegerLiteral(_)
            )
        )

    }else{
        panic!("unmöglich!");
    }
}

#[test]
fn test_prefix_operator_parsing(){
    let testcases = [
        ("!5;", "!", 5),
        ("-15;", "-", 15)
    ];

    testcases.iter().for_each(|testcase|{
        
        let lexer = Lexer::new(testcase.0.to_string());
        let parser = Parser::new(lexer);
        
        let program = parser.into_a_program().unwrap();
        
        assert_eq!(program.statements.len(),1);

        assert!(
            matches!(
                program.statements[0],
                Statement::Expression(_)
            )
        );


        if let Statement::Expression(expression) = &program.statements[0]{
            assert!(
                matches!(
                    expression.expression, 
                    Expression::Prefix(_)
                )
            );
            if let Expression::Prefix(prefix_expression) = &expression.expression{
                assert_eq!(prefix_expression.operator, testcase.1); 
            }
        }else{
            panic!("unmöglich!");
        }



    });
}

#[test]
fn test_parsing_infix_expression(){
    let testcases = [
        ("5 + 5;", 5, "+", 5),
        ("5 - 5;", 5, "-", 5),
        ("5 * 5;", 5, "*", 5),
        ("5 / 5;", 5, "/", 5),
        ("5 > 5;", 5, ">", 5),
        ("5 < 5;", 5, "<", 5),
        ("5 == 5;", 5, "==", 5),
        ("5 != 5;", 5, "!=", 5)
    ];

    testcases.iter().for_each(|test_case|{
        let expression = test_case.0.to_string();
        let expected_left = test_case.1;
        let exprected_op  = test_case.2.to_string();
        let exptected_right = test_case.3;

        let lexer = Lexer::new(expression);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let parsed_statement_expression = match &program.statements[0]{
            Statement::Expression(expr) => expr, 
            _ => panic!()
        };

        let parsed_expression = match &parsed_statement_expression.expression { 
                Expression::Infix(infix_expr) => infix_expr, 
                _ => panic!()
        };

        match (*parsed_expression.left.clone(), *parsed_expression.right.clone()){ 
            (Expression::IntegerLiteral(left_expression), Expression::IntegerLiteral(right_expression)) => {
                assert_eq!(left_expression.value, expected_left);
                assert_eq!(right_expression.value, exptected_right);
            },
            _ => panic!()
        }
        assert_eq!(parsed_expression.operator, exprected_op);


    })
}

#[test]
fn test_basic_boolean_parse(){
    let testcases = [
        ("true;", true),
        ("false;",false)
    ];

    testcases.iter().for_each(|test_case|{
        let input = test_case.0.to_string();
        let expected_token_value = test_case.1;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let parsed_expression_statement = match &program.statements[0]{
            Statement::Expression(expr) => expr,
            _ => panic!()
        };

        let parsed_expression = match &parsed_expression_statement.expression{
            Expression::Bool(bool) => bool,
            _ => panic!()
        };

        assert_eq!(parsed_expression.value, expected_token_value);

    })
}

#[test]
fn test_boolean_infix_expression(){
    let testcases = [
        ("true == false;", true, "==", false),
        ("true != true;", true, "!=", true)
    ];

    testcases.iter().for_each(|test_case|{
        let expression = test_case.0.to_string();
        let expected_left = test_case.1;
        let exprected_op  = test_case.2.to_string();
        let exptected_right = test_case.3;

        let lexer = Lexer::new(expression);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let parsed_statement_expression = match &program.statements[0]{
            Statement::Expression(expr) => expr, 
            _ => panic!()
        };

        let parsed_expression = match &parsed_statement_expression.expression { 
                Expression::Infix(infix_expr) => infix_expr, 
                _ => panic!()
        };

        match (*parsed_expression.left.clone(), *parsed_expression.right.clone()){ 
            (Expression::Bool(left_expression), Expression::Bool(right_expression)) => {
                assert_eq!(left_expression.value, expected_left);
                assert_eq!(right_expression.value, exptected_right);
            },
            _ => panic!()
        }
        assert_eq!(parsed_expression.operator, exprected_op);
    })
}

#[test]
fn test_prefix_bool_parsing(){
    let testcases = [
        ("!true;", "!", true),
        ("!false;","!", false)
    ];

    testcases.iter().for_each(|test_case|{
        let input = test_case.0.to_string();
        let operator = test_case.1;
        let bool_val = test_case.2;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);

        let parsed_expression_statement = match &program.statements[0]{
            Statement::Expression(expr) => expr,
            _ => panic!("")
        };

        let parsed_expr = match &parsed_expression_statement.expression{
            Expression::Prefix(prefix_expr) => prefix_expr, 
            _ => panic!("")
        };

        assert_eq!(parsed_expr.operator, operator);
        
        match *parsed_expr.right.clone(){
            Expression::Bool(boolean_expr) => {
                assert_eq!(boolean_expr.value, bool_val);
            },
            _ => panic!()
        };
    })
}

#[test]
fn test_operator_precedence(){
    let testcases = [
        ("-a*b;", "((-a) * b)"),
        ("a+b+c;","((a + b) + c)"),
        ("a*b*c;", "((a * b) * c)"),
        ("a+b/c;", "(a + (b / c))"),
        ("5>4 == 3<4;", "((5 > 4) == (3 < 4))")
    ];

    testcases.iter().for_each(|test_case|{
        let input = test_case.0.to_string();
        let expected_parsed_output = test_case.1.to_string();

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        assert_eq!(program.statements.len(), 1);
        assert_eq!(expected_parsed_output,program.to_string());
        
    });
}
