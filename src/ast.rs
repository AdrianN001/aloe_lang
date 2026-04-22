use crate::ast::expression::Expression;
use crate::ast::expression::identifier::Identifier;
use crate::ast::expression::value_assign_expression::ValueAssignExpression;
use crate::ast::precedence::OperationPrecedence;
use crate::ast::statement::async_function_statement::AsyncFunctionStatement;
use crate::ast::statement::break_statement::BreakStatement;
use crate::ast::statement::continue_statement::ContinueStatement;
use crate::ast::statement::function_statement::FunctionStatement;
use crate::ast::statement::import_statement::ImportStatement;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::struct_statement::StructStatement;
use crate::{
    ast::{
        program::Program,
        statement::{
            Statement, expression_statement::ExpressionStatement, return_statement::ReturnStatement,
        },
    },
    lexer::Lexer,
    token::{Token, token_type::TokenType},
};

pub mod error;
pub mod expression;
pub mod expression_parse;
pub mod precedence;
pub mod program;
pub mod statement;

pub struct Parser {
    lexer: Lexer,

    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut new_parser = Self {
            lexer,
            current_token: Token::simple(TokenType::Illegal, "\0"),
            peek_token: Token::simple(TokenType::Illegal, "\0"),
        };

        // both current_token und peek_token is set
        new_parser.next_token();
        new_parser.next_token();

        new_parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn into_a_program(mut self) -> Result<Program, String> {
        let mut program = Program::new();

        while self.current_token.token_type != TokenType::Eof {
            match self.parse_statement() {
                Ok(parsed_statement) => {
                    program.statements.push(parsed_statement);
                }
                Err(error_feedback) => {
                    println!("error: {}", error_feedback);
                    return Err(error_feedback);
                }
            }

            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token.token_type {
            TokenType::KwLet => self.parse_let(),
            TokenType::KwReturn => self.parse_return(),
            TokenType::KwBreak => self.parse_break(),
            TokenType::KwContinue => self.parse_continue(),
            TokenType::KwFunctionStatement => self.parse_function_statement(),
            TokenType::KwImport => self.parse_import(),
            TokenType::KwStruct => self.parse_struct_statement(),
            TokenType::KwAsync => self.parse_async_function_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let(&mut self) -> Result<Statement, String> {
        let mut statement = LetStatement {
            token: self.current_token.clone(),
            ..Default::default()
        };

        // if "let" is not followed by var name
        if self.peek_token.token_type != TokenType::Identifier {
            return Err(self.create_unexpected_error_feedback(
                &TokenType::Identifier,
                &(self.peek_token.token_type),
            ));
        } else {
            self.next_token();
        }

        statement.name = Identifier {
            value: self.current_token.literal.clone(),
            token: self.current_token.clone(),
        };

        if self.peek_token.token_type != TokenType::Assign {
            return Err(self.create_unexpected_error_feedback(
                &TokenType::Assign,
                &self.peek_token.token_type,
            ));
        } else {
            self.next_token();
        }

        self.next_token();
        statement.value = self.parse_expression(OperationPrecedence::Lowest)?;

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::Let(statement))
    }

    fn parse_continue(&mut self) -> Result<Statement, String> {
        Ok(Statement::Continue(ContinueStatement {
            token: {
                let token = self.current_token.clone();
                self.next_token();
                token
            },
        }))
    }

    fn parse_function_statement(&mut self) -> Result<Statement, String> {
        let mut function = FunctionStatement {
            token: self.current_token.clone(),
            ..Default::default()
        };

        if self.peek_token.token_type != TokenType::Identifier {
            return Err(format!(
                "expected identifier, got: '{}'",
                self.peek_token.token_type
            ));
        }
        self.next_token();

        function.name = self.current_token.literal.clone();

        if self.peek_token.token_type != TokenType::LParen {
            return Err("unexpected token. Expected 'LParen'".into());
        }
        self.next_token();

        function.parameters = self.parse_function_parameters()?;

        if self.peek_token.token_type != TokenType::LBrace {
            return Err("unexpected token. Expected 'LBrace'".into());
        }
        self.next_token();

        function.block = self.parse_block_statement()?;

        Ok(Statement::Function(function))
    }

    fn parse_import(&mut self) -> Result<Statement, String> {
        let token = self.current_token.clone();

        if self.peek_token.token_type != TokenType::LBrace {
            return Err("unexpected token. Expected 'LBrace'".into());
        }
        self.next_token();

        let identifiers = self.parse_expression_list(TokenType::RBrace)?;

        if self.peek_token.token_type != TokenType::KwFrom {
            return Err("unexpected token. Expected 'KwFrom'".into());
        }
        self.next_token();

        if self.peek_token.token_type != TokenType::String {
            return Err("unexpected token. Expected String".into());
        }
        self.next_token();

        let module_name = self.current_token.literal.clone();

        let mut custom_name: Option<String> = None;

        if self.peek_token.token_type == TokenType::KwInto {
            self.next_token();

            if self.peek_token.token_type != TokenType::Identifier {
                return Err("unexpected token. Expected Identifier".into());
            }
            self.next_token();

            custom_name = Some(self.current_token.literal.clone());
            self.next_token();

            if self.current_token.token_type == TokenType::Semicolon {
                self.next_token();
            }
        } else if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::Import(ImportStatement {
            token,
            identifiers,
            module_name,
            custom_name,
        }))
    }

    fn parse_return(&mut self) -> Result<Statement, String> {
        let statement = ReturnStatement {
            token: self.current_token.clone(),
            value: {
                self.next_token();
                if self.current_token.token_type == TokenType::Semicolon {
                    None
                } else {
                    Some(self.parse_expression(OperationPrecedence::Lowest)?)
                }
            },
        };

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::Return(statement))
    }

    fn parse_value_assign(&mut self, left: &Expression) -> Result<Expression, String> {
        let expr = ValueAssignExpression {
            token: self.current_token.clone(),
            left: {
                match left {
                    Expression::Index(_) | Expression::Identifier(_) | Expression::Member(_) => {
                        Box::new(left.clone())
                    }
                    other_expression_type => {
                        return Err(format!(
                            "expected 'LValue', got: {}",
                            other_expression_type.to_string()
                        ));
                    }
                }
            },
            right: {
                if self.peek_token.token_type == TokenType::Semicolon {
                    return Err("expected expression, got semicolon.".into());
                }
                self.next_token();
                let right_expr = self.parse_expression(OperationPrecedence::Lowest)?;

                if self.current_token.token_type == TokenType::Semicolon {
                    self.next_token();
                }
                Box::new(right_expr)
            },
        };

        Ok(Expression::ValueAssign(expr))
    }

    fn parse_break(&mut self) -> Result<Statement, String> {
        let mut statement = BreakStatement {
            token: self.current_token.clone(),
            expression: None,
        };

        self.next_token();

        if self.current_token.token_type != TokenType::Semicolon {
            match self.parse_expression(OperationPrecedence::Lowest) {
                Ok(ok_val) => statement.expression = Some(ok_val),
                Err(error_val) => return Err(error_val),
            };
        };

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::Break(statement))
    }

    fn parse_struct_statement(&mut self) -> Result<Statement, String> {
        let token = self.current_token.clone();

        if self.peek_token.token_type != TokenType::Identifier {
            return Err(format!(
                "expected Identifier, got: '{}'",
                self.peek_token.token_type
            ));
        }
        self.next_token();

        let name = self.parse_expression(OperationPrecedence::Lowest)?;

        if self.peek_token.token_type != TokenType::LBrace {
            return Err(format!(
                "expected LBrace, got: '{}'",
                self.peek_token.token_type
            ));
        }
        self.next_token();

        let (attributes, methods) = self.parse_struct_body()?;

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::Struct(StructStatement {
            token,
            name,
            attributes,
            methods,
        }))
    }

    fn parse_struct_body(&mut self) -> Result<(Vec<Expression>, Vec<Statement>), String> {
        let mut attributes = Vec::new();
        let mut methods = Vec::new();

        if self.peek_token.token_type == TokenType::RBrace {
            self.next_token();
            return Ok((attributes, methods));
        }

        while self.peek_token.token_type != TokenType::RBrace {
            self.next_token();
            match self.current_token.token_type {
                TokenType::Identifier => {
                    let attribute = self.parse_identifier();

                    if self.peek_token.token_type != TokenType::Semicolon {
                        return Err(format!(
                            "illegal token found in struct body. expected: Semicolon, got: '{:?}'",
                            self.peek_token
                        ));
                    }
                    self.next_token();

                    attributes.push(attribute);
                }
                TokenType::KwFunctionStatement => {
                    let method = self.parse_function_statement()?;
                    methods.push(method);
                }
                _ => {
                    return Err(format!(
                        "illegal token found in struct body. expected: Identifier or KwFun, got: '{:?}'",
                        self.current_token
                    ));
                }
            }
        }
        self.next_token();

        Ok((attributes, methods))
    }

    fn parse_async_function_statement(&mut self) -> Result<Statement, String> {
        let token = self.current_token.clone();

        match &self.peek_token.token_type {
            TokenType::KwFunctionStatement => {
                self.next_token();

                let async_fun_stmt = self.parse_function_statement()?;

                Ok(Statement::AsyncFunction(AsyncFunctionStatement {
                    token,
                    function: Box::new(async_fun_stmt),
                }))
            }

            other_token => Err(format!(
                "expected 'fn' or 'fun' after 'async', got: '{}",
                other_token
            )),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        let statement = ExpressionStatement {
            token: self.current_token.clone(),
            expression: self.parse_expression(OperationPrecedence::Lowest)?,
        };

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::Expression(statement))
    }
}
