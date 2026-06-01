use crate::{
    ast::{Parser, expression::string_expr::StringExpr, precedence::OperationPrecedence},
    lexer::Lexer,
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        new_objectref,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
        string_obj::StringObj,
    },
};

impl StringExpr {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }
        let raw_str = &self.value;
        if raw_str.is_empty() {
            return Ok(new_objectref(Object::String(Box::new(StringObj {
                value: String::new(),
            }))));
        }

        let result = Self::process_formatted_string(raw_str, environ.clone(), state.clone())?;

        Ok(new_objectref(Object::String(Box::new(StringObj {
            value: result,
        }))))
    }

    fn process_formatted_string(
        text: &str,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<String, RuntimeSignal> {
        let mut result = String::new();
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' {
                if let Some(&'{') = chars.peek() {
                    chars.next(); // consume '{'

                    let mut expr_content = String::new();
                    let mut depth = 1;
                    let mut in_string: Option<char> = None;
                    let mut escaped = false;

                    while let Some(inner_ch) = chars.next() {
                        if escaped {
                            expr_content.push(inner_ch);
                            escaped = false;
                            continue;
                        }

                        if let Some(quote_char) = in_string {
                            expr_content.push(inner_ch);
                            if inner_ch == '\\' {
                                escaped = true;
                            } else if inner_ch == quote_char {
                                in_string = None;
                            }
                            continue;
                        }

                        match inner_ch {
                            '\\' => {
                                expr_content.push(inner_ch);
                                escaped = true;
                            }
                            '"' | '\'' => {
                                in_string = Some(inner_ch);
                                expr_content.push(inner_ch);
                            }
                            '{' => {
                                depth += 1;
                                expr_content.push(inner_ch);
                            }
                            '}' => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                                expr_content.push(inner_ch);
                            }
                            _ => {
                                expr_content.push(inner_ch);
                            }
                        }
                    }

                    //parse and evaluate the expression
                    if !expr_content.is_empty() {
                        match Self::evaluate_expression(
                            &expr_content,
                            environ.clone(),
                            state.clone(),
                        ) {
                            Ok(obj_ref) => {
                                let obj = obj_ref.borrow();
                                result.push_str(&obj.inspect());
                            }
                            Err(e) => return Err(e),
                        }
                    }
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }

    fn evaluate_expression(
        expr_str: &str,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let lexer = Lexer::new(expr_str.to_string());
        let mut parser = Parser::new(lexer);

        match parser.parse_expression(OperationPrecedence::Lowest) {
            Ok(expr) => expr.evaluate(environ, state),
            Err(e) => Err(RuntimeSignal::Panic(PanicObj::new_simple(
                PanicType::IllegalExpression,
                &format!("Failed to parse formatted string expression: {}", e),
                state,
            ))),
        }
    }
}
