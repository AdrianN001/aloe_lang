use crate::ast::{Parser};
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
    let program = parser.into_a_program();

    assert_eq!(program.statements.len(), 3);

    let expected_identifiers = ["x", "y", "foobar"];

    for i in 0..program.statements.len(){
        match &program.statements[i]{
            Statement::Let(let_statement) => {
                assert_eq!(let_statement.name.value, expected_identifiers[i]);
            }
            _ => {
                assert!(false);
            }
        }
    }
}
