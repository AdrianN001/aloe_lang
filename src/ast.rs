use crate::ast::expression::Expression;
use crate::ast::expression::value_assign_expression::ValueAssignExpression;
use crate::ast::precedence::OperationPrecedence;
use crate::ast::statement::async_function_statement::AsyncFunctionStatement;
use crate::ast::statement::break_statement::BreakStatement;
use crate::ast::statement::continue_statement::ContinueStatement;
use crate::ast::statement::enum_statement::EnumStatement;
use crate::ast::statement::function_statement::FunctionStatement;
use crate::ast::statement::import_statement::ImportStatement;
use crate::ast::statement::launch_statement::LaunchStatement;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::struct_statement::StructStatement;
use crate::ast::statement::val_statement::ValStatement;
use crate::ast::syntax_error_report::SyntaxErrorReport;
use crate::ast::syntax_error_report::syntax_error::SyntaxError;
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
pub mod syntax_error_report;

pub struct Parser {
    lexer: Lexer,

    report: SyntaxErrorReport,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut new_parser = Self {
            lexer,
            report: SyntaxErrorReport::new(),
            current_token: Token::simple(TokenType::Illegal, "\0", 0),
            peek_token: Token::simple(TokenType::Illegal, "\0", 0),
        };

        // both current_token und peek_token is set
        new_parser.next_token();
        new_parser.next_token();

        new_parser
    }

    fn get_current_line(&self) -> usize {
        self.lexer.current_line
    }

    fn next_token(&mut self) {
        self.report.push_token(&self.peek_token);

        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn into_a_program(mut self) -> Result<Program, SyntaxErrorReport> {
        let mut program = Program::new();

        while self.current_token.token_type != TokenType::Eof {
            match self.parse_statement() {
                Ok(parsed_statement) => {
                    program.statements.push(parsed_statement);
                }
                Err(error_feedback) => {
                    return Err(self.handle_syntax_error(error_feedback));
                }
            }

            self.next_token();
        }

        Ok(program)
    }

    pub fn handle_syntax_error(&mut self, error_feedback: SyntaxError) -> SyntaxErrorReport {
        self.report.set_error(error_feedback);

        self.report.clone()
    }

    fn parse_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.report.clear();
        self.report.push_token(&self.current_token);

        match self.current_token.token_type {
            TokenType::KwLet => self.parse_let(),
            TokenType::KwVal => self.parse_val(),
            TokenType::KwReturn => self.parse_return(),
            TokenType::KwBreak => self.parse_break(),
            TokenType::KwContinue => self.parse_continue(),
            TokenType::KwFunctionStatement => self.parse_function_statement(),
            TokenType::KwImport => self.parse_import(),
            TokenType::KwStruct => self.parse_struct_statement(),
            TokenType::KwAsync => self.parse_async_function_statement(),
            TokenType::KwLaunch => self.parse_launch_statement(),
            TokenType::KwEnum => self.parse_enum_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let(&mut self) -> Result<Statement, SyntaxError> {
        let curr_token = self.current_token.clone();
        self.next_token();

        let assignment = self.parse_expression(OperationPrecedence::Lowest)?;

        match assignment {
            Expression::ValueAssign(_) => {}
            other_type => {
                return Err(SyntaxError::UnexpectedExpression(
                    vec!["value assignment"],
                    other_type,
                    self.get_current_line(),
                ));
            }
        };

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::Let(LetStatement {
            token: curr_token,
            assignment,
        }))
    }

    fn parse_val(&mut self) -> Result<Statement, SyntaxError> {
        let curr_token = self.current_token.clone();
        self.next_token();

        let assignment = self.parse_expression(OperationPrecedence::Lowest)?;

        match assignment {
            Expression::ValueAssign(_) => {}
            other_type => {
                return Err(SyntaxError::UnexpectedExpression(
                    vec!["value assignment"],
                    other_type,
                    self.get_current_line(),
                ));
            }
        };

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::Val(ValStatement {
            token: curr_token,
            assignment,
        }))
    }

    fn parse_continue(&mut self) -> Result<Statement, SyntaxError> {
        Ok(Statement::Continue(ContinueStatement {
            token: {
                let token = self.current_token.clone();
                self.next_token();
                token
            },
        }))
    }

    fn parse_function_statement(&mut self) -> Result<Statement, SyntaxError> {
        let mut function = FunctionStatement {
            token: self.current_token.clone(),
            ..Default::default()
        };

        if self.peek_token.token_type != TokenType::Identifier {
            return Err(SyntaxError::UnexpectedToken(
                TokenType::Identifier,
                self.peek_token.token_type.clone(),
                self.get_current_line(),
            ));
        }
        self.next_token();

        function.name = self.current_token.literal.clone();

        if self.peek_token.token_type != TokenType::LParen {
            return Err(SyntaxError::UnexpectedToken(
                TokenType::LParen,
                self.peek_token.token_type.clone(),
                self.get_current_line(),
            ));
        }
        self.next_token();

        function.parameters = self.parse_function_parameters()?;

        if self.peek_token.token_type != TokenType::LBrace {
            return Err(SyntaxError::UnexpectedToken(
                TokenType::LBrace,
                self.peek_token.token_type.clone(),
                self.get_current_line(),
            ));
        }
        self.next_token();

        function.block = self.parse_block_statement()?;

        Ok(Statement::Function(function))
    }

    fn parse_import(&mut self) -> Result<Statement, SyntaxError> {
        let token = self.current_token.clone();

        if self.peek_token.token_type != TokenType::LBrace {
            return Err(SyntaxError::UnexpectedKeyword(
                TokenType::LBrace,
                self.peek_token.token_type.clone(),
                self.get_current_line(),
            ));
        }
        self.next_token();

        let identifiers = self.parse_expression_list(TokenType::RBrace)?;

        if self.peek_token.token_type != TokenType::KwFrom {
            return Err(SyntaxError::UnexpectedKeyword(
                TokenType::KwFrom,
                self.peek_token.token_type.clone(),
                self.get_current_line(),
            ));
        }
        self.next_token();

        if self.peek_token.token_type != TokenType::String {
            return Err(SyntaxError::UnexpectedToken(
                TokenType::String,
                self.peek_token.token_type.clone(),
                self.get_current_line(),
            ));
        }
        self.next_token();

        let module_name = self.current_token.literal.clone();

        let mut custom_name: Option<String> = None;

        if self.peek_token.token_type == TokenType::KwInto {
            self.next_token();

            if self.peek_token.token_type != TokenType::Identifier {
                return Err(SyntaxError::UnexpectedToken(
                    TokenType::Identifier,
                    self.peek_token.token_type.clone(),
                    self.get_current_line(),
                ));
            }
            self.next_token();

            custom_name = Some(self.current_token.literal.clone());

            if self.peek_token.token_type == TokenType::Semicolon {
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

    fn parse_return(&mut self) -> Result<Statement, SyntaxError> {
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

    fn parse_launch_statement(&mut self) -> Result<Statement, SyntaxError> {
        let token = self.current_token.clone();

        let expr = {
            self.next_token();
            self.parse_expression(OperationPrecedence::Lowest)?
        };

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::Launch(LaunchStatement { token, expr }))
    }

    fn parse_value_assign(&mut self, left: &Expression) -> Result<Expression, SyntaxError> {
        let expr = ValueAssignExpression {
            token: self.current_token.clone(),
            left: {
                match left {
                    Expression::Index(_)
                    | Expression::Identifier(_)
                    | Expression::Member(_)
                    | Expression::Array(_) => Box::new(left.clone()),
                    other_expression_type => {
                        return Err(SyntaxError::UnexpectedExpression(
                            vec![
                                "Index Expression".into(),
                                "Identifier".into(),
                                "Member Expression".into(),
                            ],
                            other_expression_type.clone(),
                            self.get_current_line(),
                        ));
                    }
                }
            },
            right: {
                if self.peek_token.token_type == TokenType::Semicolon {
                    return Err(SyntaxError::UnexpectedSemicolon(self.get_current_line()));
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

    fn parse_break(&mut self) -> Result<Statement, SyntaxError> {
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

    fn parse_struct_statement(&mut self) -> Result<Statement, SyntaxError> {
        let token = self.current_token.clone();

        if self.peek_token.token_type != TokenType::Identifier {
            return Err(SyntaxError::UnexpectedToken(
                TokenType::Identifier,
                self.peek_token.token_type.clone(),
                self.get_current_line(),
            ));
        }
        self.next_token();

        let name = self.parse_expression(OperationPrecedence::Lowest)?;

        let name_of_the_struct_raw = {
            match &name {
                Expression::Identifier(identifier) => identifier.value.clone(),
                other_expr => {
                    return Err(SyntaxError::UnexpectedExpression(
                        vec!["Identifer"],
                        other_expr.clone(),
                        self.get_current_line(),
                    ));
                }
            }
        };

        if self.peek_token.token_type != TokenType::LBrace {
            return Err(SyntaxError::UnexpectedToken(
                TokenType::LBrace,
                self.peek_token.token_type.clone(),
                self.get_current_line(),
            ));
        }
        self.next_token();

        let (attributes, methods) = self.parse_struct_body(&name_of_the_struct_raw)?;

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

    fn parse_struct_body(
        &mut self,
        struct_name: &str,
    ) -> Result<(Vec<Expression>, Vec<Statement>), SyntaxError> {
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
                        return Err(SyntaxError::UnexpectedTokenInStruct(
                            vec![TokenType::Semicolon],
                            self.peek_token.token_type.clone(),
                            struct_name.to_string(),
                            self.get_current_line(),
                        ));
                    }
                    self.next_token();

                    attributes.push(attribute);
                }
                TokenType::KwFunctionStatement => {
                    let method = self.parse_function_statement()?;
                    methods.push(method);
                }
                TokenType::KwAsync => {
                    if self.peek_token.token_type != TokenType::KwFunctionStatement {
                        return Err(SyntaxError::UnexpectedTokenInStruct(
                            vec![TokenType::KwFunctionStatement],
                            self.peek_token.token_type.clone(),
                            struct_name.to_string(),
                            self.get_current_line(),
                        ));
                    }
                    let method = self.parse_async_function_statement()?;
                    methods.push(method);
                }
                _ => {
                    return Err(SyntaxError::UnexpectedTokenInStruct(
                        vec![TokenType::Identifier, TokenType::KwFunction],
                        self.current_token.token_type.clone(),
                        struct_name.to_string(),
                        self.get_current_line(),
                    ));
                }
            }
        }
        self.next_token();

        Ok((attributes, methods))
    }

    fn parse_enum_statement(&mut self) -> Result<Statement, SyntaxError> {
        let token = self.current_token.clone();

        if self.peek_token.token_type != TokenType::Identifier {
            return Err(SyntaxError::UnexpectedToken(
                TokenType::Identifier,
                self.peek_token.token_type.clone(),
                self.get_current_line(),
            ));
        }
        self.next_token();

        let name = self.parse_expression(OperationPrecedence::Lowest)?;

        let name_of_the_enum_raw = {
            match &name {
                Expression::Identifier(identifier) => identifier.value.clone(),
                other_expr => {
                    return Err(SyntaxError::UnexpectedExpression(
                        vec!["Identifer"],
                        other_expr.clone(),
                        self.get_current_line(),
                    ));
                }
            }
        };

        if self.peek_token.token_type != TokenType::LBrace {
            return Err(SyntaxError::UnexpectedToken(
                TokenType::LBrace,
                self.peek_token.token_type.clone(),
                self.get_current_line(),
            ));
        }
        self.next_token();

        let values = self.parse_enum_body(&name_of_the_enum_raw)?;

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Ok(Statement::Enum(EnumStatement {
            token,
            name,
            values,
        }))
    }

    fn parse_enum_body(&mut self, enum_name: &str) -> Result<Vec<Expression>, SyntaxError> {
        let mut values = Vec::new();

        if self.peek_token.token_type == TokenType::RBrace {
            self.next_token();
            return Ok(values);
        }

        while self.peek_token.token_type != TokenType::RBrace {
            self.next_token();
            match self.current_token.token_type {
                TokenType::Identifier => {
                    let identifier = self.parse_identifier();

                    if self.peek_token.token_type != TokenType::Semicolon {
                        return Err(SyntaxError::UnexpectedTokenInEnum(
                            vec![TokenType::Semicolon],
                            self.peek_token.token_type.clone(),
                            enum_name.to_string(),
                            self.get_current_line(),
                        ));
                    }
                    self.next_token();

                    values.push(identifier);
                }
                _ => {
                    return Err(SyntaxError::UnexpectedTokenInEnum(
                        vec![TokenType::Identifier, TokenType::KwFunction],
                        self.current_token.token_type.clone(),
                        enum_name.to_string(),
                        self.get_current_line(),
                    ));
                }
            }
        }
        self.next_token();

        Ok(values)
    }

    fn parse_async_function_statement(&mut self) -> Result<Statement, SyntaxError> {
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

            other_token => Err(SyntaxError::UnexpectedTokenAfterAsync(
                vec![TokenType::KwFunction, TokenType::KwFunctionStatement],
                other_token.clone(),
                self.get_current_line(),
            )),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, SyntaxError> {
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
