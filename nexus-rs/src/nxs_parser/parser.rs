use crate::parse_error::*;
use crate::token::{Token, Tokens};
use crate::token_cursor::TokenCursor;
use crate::{ast, ptr::Ptr};
use lazy_static::lazy_static;

/// Parser for Nexus.
pub struct Parser {
    cursor: TokenCursor,
}

/// Preprocess token stream.
fn preprocess(tokens: Tokens) -> Tokens {
    let mut result = Tokens::new();

    // TODO: For now, ignore non-capturing closures and transform a '||' into 'Or':
    let mut found_pipe = false;
    tokens.into_iter().for_each(|t| {
        if found_pipe && t == Token::Pipe {
            result.push(Token::Or);
            found_pipe = false;
        } else {
            found_pipe = t == Token::Pipe;

            if !found_pipe {
                result.push(t);
            }
        }
    });

    result
}

impl Parser {
    /// Create a new parser from a collection of tokens.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::token::Tokens;
    /// use nexus_rs::parser::Parser;
    ///
    /// let t = Tokens::new();
    /// let p = Parser::new(t);
    /// ```
    pub fn new(tokens: Tokens) -> Self {
        Parser {
            cursor: TokenCursor::new(preprocess(tokens)),
        }
    }

    /// Parse tokens into AST.
    pub fn parse(&mut self) -> Result<ast::Stmts, ParseError> {
        let mut ast = ast::Stmts::new();

        while !self.cursor.eos() {
            ast.push(parse_decl(&mut self.cursor)?);
        }

        Ok(ast)
    }
}

fn parse_decl(c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    match c.peek() {
        Some(Token::Const) => parse_const_decl(c),
        Some(Token::Function) => parse_function_decl(c),
        Some(Token::Let) => parse_var_decl(c),
        Some(Token::Use) => parse_use_decl(c),
        _ => parse_stmt(c),
    }
}

fn parse_function_decl(c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    c.consume(Token::Function)?;

    let id = parse_identifier(c)?;

    c.consume_msg(Token::LeftParen, "expected '(' after function identifier")?;

    let args = if c.peek() != Some(Token::RightParen) {
        Some(parse_function_args(c)?)
    } else {
        None
    };

    c.consume_msg(
        Token::RightParen,
        "expected ')' after function argument list",
    )?;

    let ret_type = if c.peek() == Some(Token::Arrow) {
        c.consume_msg(Token::Arrow, "expected '->' in function declaration")?;
        Some(parse_type(c)?)
    } else {
        None
    };

    let body = parse_block_stmt(c)?;

    Ok(ast::Stmt {
        kind: ast::StmtKind::FunctionDecl(Ptr::new(ast::FunctionDecl {
            id,
            args,
            ret_type,
            body,
        })),
    })
}

fn parse_function_arg(c: &mut TokenCursor) -> ParseResult<ast::FunctionArg> {
    let id = parse_identifier(c)?;

    c.consume_msg(
        Token::Colon,
        "expected ':' after function argument identifier",
    )?;

    let typeid = parse_type(c)?;

    Ok(ast::FunctionArg { id, typeid })
}

fn parse_function_args(c: &mut TokenCursor) -> ParseResult<ast::FunctionArgs> {
    let mut result = ast::FunctionArgs::new();

    loop {
        result.push(parse_function_arg(c)?);

        if !c.advance_if(Token::Comma) {
            break;
        }
    }

    Ok(result)
}

fn parse_const_decl(c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    c.consume(Token::Const)?;

    let id = parse_identifier(c)?;

    // TODO: Enforce upper case style..

    c.consume_msg(
        Token::Colon,
        "expected ':' for type annotation of constant value",
    )?;

    let typeid = parse_type(c)?;

    c.consume_msg(
        Token::Is,
        "expected '=' for initialization of constant value",
    )?;

    let value = match typeid {
        ast::TypeKind::Bool => parse_bool_literal(c)?,
        ast::TypeKind::Group => {
            return Err(ParseError::new(ParseErrorKind::Custom(
                "cannot create a Group type literal".to_owned(),
            )));
        }
        ast::TypeKind::Node => {
            return Err(ParseError::new(ParseErrorKind::Custom(
                "cannot create a Node type literal".to_owned(),
            )));
        }
        ast::TypeKind::Number => parse_number_literal(c)?,
        ast::TypeKind::String => parse_string_literal(c)?,
    };

    c.consume(Token::SemiColon)?;

    Ok(ast::Stmt {
        kind: ast::StmtKind::ConstDecl(Ptr::new(ast::ConstDecl { id, typeid, value })),
    })
}

fn parse_var_decl(c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    c.consume(Token::Let)?;

    let mutable = c.advance_if(Token::Mut);

    let id = parse_expr(c)?;

    let typeid = if c.advance_if(Token::Colon) {
        Some(parse_type(c)?)
    } else {
        None
    };

    let value = if c.advance_if(Token::Is) {
        Some(if c.advance_if(Token::Amp) {
            let expr = parse_expr(c)?;
            ast::Expr {
                kind: ast::ExprKind::Ref(Ptr::new(ast::Ref { expr })),
            }
        } else {
            parse_expr(c)?
        })
    } else {
        None
    };

    c.consume(Token::SemiColon)?;

    Ok(ast::Stmt {
        kind: ast::StmtKind::VarDecl(Ptr::new(ast::VarDecl {
            id,
            mutable,
            typeid,
            value,
        })),
    })
}

fn parse_use_decl(c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    c.consume(Token::Use)?;

    // TODO: Check for global scope?

    let filename = parse_expr(c)?;

    c.consume_msg(Token::SemiColon, "expected semicolon after statement")?;

    Ok(ast::Stmt {
        kind: ast::StmtKind::UseDecl(Ptr::new(ast::UseDecl { filename })),
    })
}

fn parse_stmt(c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    match c.peek() {
        Some(Token::LeftBrace) => parse_block_stmt(c),
        Some(Token::Print) => parse_print_stmt(c),
        Some(Token::Return) => parse_return_stmt(c),
        _ => parse_expr_stmt(c),
    }
}

fn parse_block_stmt(c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    c.consume(Token::LeftBrace)?;

    let mut body = ast::Stmts::new();
    loop {
        match c.peek() {
            Some(Token::RightBrace) => break,
            None => {
                return Err(ParseError::new(ParseErrorKind::UnexpectedEos(
                    "block statement".to_owned(),
                )))
            }
            _ => body.push(parse_decl(c)?),
        }
    }

    c.consume(Token::RightBrace)?;

    Ok(ast::Stmt {
        kind: ast::StmtKind::Block(body),
    })
}

fn parse_identifier(c: &mut TokenCursor) -> ParseResult<String> {
    lazy_static! {
        static ref KEYWORDS: Tokens = vec![
            Token::BoolId,
            Token::NodeId,
            Token::Const,
            Token::Else,
            Token::False,
            Token::For,
            Token::Function,
            Token::Group,
            Token::GroupId,
            Token::If,
            Token::Let,
            Token::Mut,
            Token::Node,
            Token::NumberId,
            Token::Print,
            Token::Return,
            Token::StringId,
            Token::True,
            Token::Use,
            Token::While,
        ];
    }

    match c.value() {
        Some(Token::Identifier(i)) => Ok(i),
        Some(t) if KEYWORDS.contains(&t) => {
            Err(ParseError::new(ParseErrorKind::KeywordAsIdentifier(t)))
        }
        Some(t) => Err(ParseError::new(ParseErrorKind::Unexpected(t))),
        None => Err(ParseError::new(ParseErrorKind::UnexpectedEos(
            "identifier".to_owned(),
        ))),
    }
}

fn parse_type(c: &mut TokenCursor) -> ParseResult<ast::TypeKind> {
    Ok(match c.value() {
        Some(Token::BoolId) => ast::TypeKind::Bool,
        Some(Token::NodeId) => ast::TypeKind::Node,
        Some(Token::GroupId) => ast::TypeKind::Group,
        Some(Token::NumberId) => ast::TypeKind::Number,
        Some(Token::StringId) => ast::TypeKind::String,
        Some(t) => {
            return Err(ParseError::new(ParseErrorKind::Custom(format!(
                "not a type ID '{:?}'",
                t
            ))));
        }
        None => {
            return Err(ParseError::new(ParseErrorKind::Custom(
                "empty type ID".to_owned(),
            )));
        }
    })
}

fn parse_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    // NOTE: The recursion depth encodes the operator precedence.
    parse_range_expr(c)
}

fn parse_range_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    let mut expr = parse_or_expr(c)?;

    if matches!(c.peek(), Some(Token::Range)) {
        c.consume(Token::Range)?;

        let kind = if c.peek() == Some(Token::Is) {
            c.consume(Token::Is)?;
            ast::RangeKind::Inclusive
        } else {
            ast::RangeKind::Exclusive
        };

        let start = expr;
        let end = parse_or_expr(c)?;

        let check_range_expr_type = |e: &ast::Expr| -> ParseResult<()> {
            if matches!(
                e.kind,
                ast::ExprKind::Literal(_) | ast::ExprKind::Var(_) | ast::ExprKind::Group(_)
            ) {
                Ok(())
            } else {
                Err(ParseError::new(ParseErrorKind::RangeDelimiter))
            }
        };

        check_range_expr_type(&start)?;
        check_range_expr_type(&end)?;

        expr = ast::Expr {
            kind: ast::ExprKind::Range(Ptr::new(ast::Range { kind, start, end })),
        };
    }

    Ok(expr)
}

fn parse_or_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    let mut expr = parse_and_expr(c)?;

    while matches!(c.peek(), Some(Token::Or)) {
        let op = parse_binary_op(c.value())?;
        let lhs = expr;
        let rhs = parse_and_expr(c)?;

        expr = ast::Expr {
            kind: ast::ExprKind::Binary(Ptr::new(ast::BinaryExpr { op, lhs, rhs })),
        };
    }

    Ok(expr)
}

fn parse_and_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    let mut expr = parse_equality_expr(c)?;

    while matches!(c.peek(), Some(Token::And)) {
        let op = parse_binary_op(c.value())?;
        let lhs = expr;
        let rhs = parse_equality_expr(c)?;

        expr = ast::Expr {
            kind: ast::ExprKind::Binary(Ptr::new(ast::BinaryExpr { op, lhs, rhs })),
        };
    }

    Ok(expr)
}

fn parse_equality_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    let mut expr = parse_relational_expr(c)?;

    while matches!(c.peek(), Some(Token::Eq) | Some(Token::NotEq)) {
        let op = parse_binary_op(c.value())?;
        let lhs = expr;
        let rhs = parse_relational_expr(c)?;

        expr = ast::Expr {
            kind: ast::ExprKind::Binary(Ptr::new(ast::BinaryExpr { op, lhs, rhs })),
        };
    }

    Ok(expr)
}

fn parse_relational_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    let mut expr = parse_expr_term(c)?;

    while matches!(
        c.peek(),
        Some(Token::Lt) | Some(Token::Gt) | Some(Token::LtEq) | Some(Token::GtEq)
    ) {
        let op = parse_binary_op(c.value())?;
        let lhs = expr;
        let rhs = parse_expr_term(c)?;

        expr = ast::Expr {
            kind: ast::ExprKind::Binary(Ptr::new(ast::BinaryExpr { op, lhs, rhs })),
        };
    }

    Ok(expr)
}

fn parse_expr_term(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    let mut expr = parse_factor_expr(c)?;

    while matches!(c.peek(), Some(Token::Plus) | Some(Token::Minus)) {
        let op = parse_binary_op(c.value())?;
        let lhs = expr;
        let rhs = parse_factor_expr(c)?;

        expr = ast::Expr {
            kind: ast::ExprKind::Binary(Ptr::new(ast::BinaryExpr { op, lhs, rhs })),
        };
    }

    Ok(expr)
}

fn parse_factor_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    let mut expr = parse_unary_expr(c)?;

    while matches!(
        c.peek(),
        Some(Token::Star) | Some(Token::Slash) | Some(Token::Percent)
    ) {
        let op = parse_binary_op(c.value())?;
        let lhs = expr;
        let rhs = parse_unary_expr(c)?;

        expr = ast::Expr {
            kind: ast::ExprKind::Binary(Ptr::new(ast::BinaryExpr { op, lhs, rhs })),
        };
    }

    Ok(expr)
}

fn parse_unary_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    if matches!(
        c.peek(),
        Some(Token::Bang)
            | Some(Token::Minus)
            | Some(Token::Group)
            | Some(Token::Node)
            | Some(Token::Plus)
    ) {
        let operator = parse_unary_op(c.value())?;
        let expr = parse_expr(c)?;

        Ok(ast::Expr {
            kind: ast::ExprKind::Unary(Ptr::new(ast::UnaryExpr { op: operator, expr })),
        })
    } else {
        parse_dot_expr(c)
    }
}

fn parse_dot_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    let mut expr = parse_call_expr(c)?;

    while matches!(c.peek(), Some(Token::Dot)) {
        let op = parse_binary_op(c.value())?;
        let lhs = expr;
        let rhs = parse_call_expr(c)?;

        expr = ast::Expr {
            kind: ast::ExprKind::Binary(Ptr::new(ast::BinaryExpr { op, lhs, rhs })),
        };
    }

    Ok(expr)
}

fn parse_call_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    match (c.peek(), c.peek_next()) {
        (Some(Token::Identifier(_)), Some(Token::LeftParen)) => {
            let id = parse_identifier(c)?;

            c.consume(Token::LeftParen)?;

            let mut args = Vec::new();
            while c.peek() != Some(Token::RightParen) {
                args.push(parse_expr(c)?);

                if !c.advance_if(Token::Comma) {
                    break;
                }
            }

            c.consume(Token::RightParen)?;

            Ok(ast::Expr {
                kind: ast::ExprKind::FuncCall(Ptr::new(ast::FuncCall { id, args })),
            })
        }
        _ => parse_primary_expr(c),
    }
}

fn parse_primary_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    match c.peek() {
        Some(Token::Number(_)) => parse_number_literal(c),
        Some(Token::String(_)) => parse_string_literal(c),
        Some(Token::True | Token::False) => parse_bool_literal(c),
        Some(Token::Identifier(_)) => parse_var_expr(c),
        Some(Token::If) => parse_if_expr(c),
        Some(Token::While) => parse_while_expr(c),
        Some(Token::For) => parse_for_expr(c),
        Some(Token::LeftParen) => parse_group_expr(c),
        Some(Token::LeftBrace) => parse_block_expr(c),
        Some(Token::SemiColon) => Ok(ast::Expr {
            kind: ast::ExprKind::Empty(),
        }),
        Some(t) => Err(ParseError::new(ParseErrorKind::Unexpected(t))),
        None => Err(ParseError::new(ParseErrorKind::UnexpectedEos(
            "primary expression".to_owned(),
        ))),
    }
}

fn parse_expr_stmt(c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    let expr = parse_expr(c)?;

    match c.peek() {
        Some(Token::Arrow) => parse_connect_stmt(expr, c),
        Some(Token::Is) => parse_assignment_stmt(expr, c),
        None => Err(ParseError::new(ParseErrorKind::UnexpectedEos(
            "expression statement".to_owned(),
        ))),
        _ => {
            if !c.advance_if(Token::SemiColon) {
                // TODO: Mark block result value?
            }

            Ok(ast::Stmt {
                kind: ast::StmtKind::Expr(Ptr::new(expr)),
            })
        }
    }
}

fn parse_if_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    c.consume(Token::If)?;

    let expr = parse_expr(c)?;
    let body_then = parse_block_expr(c)?;

    let body_else = if c.advance_if(Token::Else) {
        if c.peek() == Some(Token::If) {
            Some(parse_if_expr(c)?)
        } else {
            Some(parse_block_expr(c)?)
        }
    } else {
        None
    };

    Ok(ast::Expr {
        kind: ast::ExprKind::If(Ptr::new(ast::If {
            expr,
            body_then,
            body_else,
        })),
    })
}

fn parse_while_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    c.consume(Token::While)?;

    let expr = parse_expr(c)?;
    let body = parse_block_expr(c)?;

    Ok(ast::Expr {
        kind: ast::ExprKind::While(Ptr::new(ast::While { expr, body })),
    })
}

fn parse_for_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    c.consume(Token::For)?;

    let id = parse_identifier(c)?;

    c.consume(Token::In)?;

    let expr = parse_expr(c)?;
    let body = parse_block_expr(c)?;

    Ok(ast::Expr {
        kind: ast::ExprKind::For(Ptr::new(ast::For { id, expr, body })),
    })
}

fn parse_var_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    let id = parse_identifier(c)?;

    Ok(ast::Expr {
        kind: ast::ExprKind::Var(Ptr::new(ast::Var { id })),
    })
}

fn parse_bool_literal(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    Ok(ast::Expr {
        kind: ast::ExprKind::Literal(Ptr::new(ast::Literal {
            kind: ast::LiteralKind::Bool(match c.value() {
                Some(Token::True) => true,
                Some(Token::False) => false,
                Some(t) => {
                    return Err(ParseError::new(ParseErrorKind::Custom(format!(
                        "not a boolean literal: '{t:?}'"
                    ))));
                }
                None => {
                    return Err(ParseError::new(ParseErrorKind::UnexpectedEos(
                        "boolean literal".to_owned(),
                    )));
                }
            }),
        })),
    })
}

fn parse_number_literal(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    Ok(ast::Expr {
        kind: ast::ExprKind::Literal(Ptr::new(ast::Literal {
            kind: ast::LiteralKind::Number(match c.value() {
                Some(Token::Number(n)) => n,
                Some(n) => {
                    return Err(ParseError::new(ParseErrorKind::Custom(format!(
                        "not a number literal: '{n:?}'"
                    ))));
                }
                None => {
                    return Err(ParseError::new(ParseErrorKind::UnexpectedEos(
                        "number literal".to_owned(),
                    )));
                }
            }),
        })),
    })
}

fn parse_string_literal(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    Ok(ast::Expr {
        kind: ast::ExprKind::Literal(Ptr::new(ast::Literal {
            kind: ast::LiteralKind::String(match c.value() {
                Some(Token::String(s)) => s,
                Some(s) => {
                    return Err(ParseError::new(ParseErrorKind::Custom(format!(
                        "not a string literal: '{s:?}'"
                    ))));
                }
                None => {
                    return Err(ParseError::new(ParseErrorKind::UnexpectedEos(
                        "string literal".to_owned(),
                    )));
                }
            }),
        })),
    })
}

fn parse_group_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    c.consume(Token::LeftParen)?;

    let expr = parse_expr(c)?;

    c.consume(Token::RightParen)?;

    Ok(ast::Expr {
        kind: ast::ExprKind::Group(Ptr::new(expr)),
    })
}

fn parse_block_expr(c: &mut TokenCursor) -> ParseResult<ast::Expr> {
    let body = parse_block_stmt(c)?;

    Ok(ast::Expr {
        kind: ast::ExprKind::Block(Ptr::new(ast::BlockExpr { body })),
    })
}

fn parse_unary_op(t: Option<Token>) -> ParseResult<ast::UnaryOp> {
    Ok(match t {
        Some(Token::Bang) => ast::UnaryOp::Bang,
        Some(Token::Minus) => ast::UnaryOp::Minus,
        Some(Token::Group) => ast::UnaryOp::Group,
        Some(Token::Node) => ast::UnaryOp::Node,
        Some(Token::Plus) => ast::UnaryOp::Plus,
        Some(_) => {
            return Err(ParseError::new(ParseErrorKind::Custom(
                "not a unary expression token".to_owned(),
            )));
        }
        None => {
            return Err(ParseError::new(ParseErrorKind::UnexpectedEos(
                "unary expression".to_owned(),
            )));
        }
    })
}

fn parse_binary_op(t: Option<Token>) -> ParseResult<ast::BinaryOp> {
    Ok(match t {
        Some(Token::And) => ast::BinaryOp::And,
        Some(Token::Dot) => ast::BinaryOp::Dot,
        Some(Token::Eq) => ast::BinaryOp::Eq,
        Some(Token::Gt) => ast::BinaryOp::Gt,
        Some(Token::GtEq) => ast::BinaryOp::GtEq,
        Some(Token::Lt) => ast::BinaryOp::Lt,
        Some(Token::LtEq) => ast::BinaryOp::LtEq,
        Some(Token::Minus) => ast::BinaryOp::Subtract,
        Some(Token::NotEq) => ast::BinaryOp::NotEq,
        Some(Token::Or) => ast::BinaryOp::Or,
        Some(Token::Percent) => ast::BinaryOp::Remainder,
        Some(Token::Plus) => ast::BinaryOp::Plus,
        Some(Token::Slash) => ast::BinaryOp::Divide,
        Some(Token::Star) => ast::BinaryOp::Multiply,
        Some(_) => {
            return Err(ParseError::new(ParseErrorKind::Custom(
                "not a binary expression token".to_owned(),
            )));
        }
        None => {
            return Err(ParseError::new(ParseErrorKind::UnexpectedEos(
                "binary expression".to_owned(),
            )));
        }
    })
}

fn parse_print_stmt(c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    c.consume(Token::Print)?;

    let expr = parse_expr(c)?;

    c.consume_msg(Token::SemiColon, "after statement")?;

    Ok(ast::Stmt {
        kind: ast::StmtKind::Print(Ptr::new(ast::Print { expr })),
    })
}

fn parse_return_stmt(c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    c.consume(Token::Return)?;

    let expr = parse_expr(c)?;

    c.consume_msg(Token::SemiColon, "after statement")?;

    Ok(ast::Stmt {
        kind: ast::StmtKind::Return(Ptr::new(ast::Return { expr })),
    })
}

fn parse_assignment_stmt(lhs: ast::Expr, c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    c.consume(Token::Is)?;

    let rhs = parse_expr(c)?;

    c.consume_msg(Token::SemiColon, "expected semicolon after statement")?;

    Ok(ast::Stmt {
        kind: ast::StmtKind::Assignment(Ptr::new(ast::Assignment { lhs, rhs })),
    })
}

fn parse_connect_stmt(source: ast::Expr, c: &mut TokenCursor) -> ParseResult<ast::Stmt> {
    c.consume(Token::Arrow)?;

    let sink = parse_expr(c)?;

    c.consume_msg(Token::SemiColon, "after statement")?;

    Ok(ast::Stmt {
        kind: ast::StmtKind::Connect(Ptr::new(ast::Connect { source, sink })),
    })
}
