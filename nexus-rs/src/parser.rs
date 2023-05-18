use crate::ast;
use crate::ptr::Ptr;
use crate::token::{Token, Tokens};
use crate::token_cursor::TokenCursor;
use lazy_static::lazy_static;

/// Parser for Nexus.
pub struct Parser {
    cursor: TokenCursor,
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
            cursor: TokenCursor::new(tokens),
        }
    }

    /// Parse tokens into AST.
    pub fn parse(&mut self) -> ast::Stmts {
        let mut ast = ast::Stmts::new();

        while !self.cursor.eos() {
            ast.push(parse_decl(&mut self.cursor));
        }

        ast
    }
}

fn parse_decl(c: &mut TokenCursor) -> ast::Stmt {
    match c.peek() {
        Some(Token::Const) => parse_const_decl(c),
        Some(Token::Function) => parse_function_decl(c),
        Some(Token::Let) => parse_var_decl(c),
        Some(Token::Use) => parse_use_decl(c),
        _ => parse_stmt(c),
    }
}

fn parse_function_decl(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::Function);

    let id = parse_identifier(c);

    c.consume_msg(Token::LeftParen, "expected '(' after function identifier"); // TODO: Proper error handling.

    let args = if c.peek() != Some(Token::RightParen) {
        Some(parse_function_args(c))
    } else {
        None
    };

    c.consume_msg(
        Token::RightParen,
        "expected ')' after function argument list",
    ); // TODO: Proper error handling.

    let ret_type = if c.peek() == Some(Token::Arrow) {
        c.consume_msg(Token::Arrow, "expected '->' in function declaration");
        Some(parse_type(c))
    } else {
        None
    };

    let body = parse_block_stmt(c);

    ast::Stmt {
        kind: ast::StmtKind::FunctionDecl(Ptr::new(ast::FunctionDecl {
            id,
            args,
            ret_type,
            body,
        })),
    }
}

fn parse_function_arg(c: &mut TokenCursor) -> ast::FunctionArg {
    let id = parse_identifier(c);

    c.consume_msg(
        Token::Colon,
        "expected ':' after function argument identifier",
    ); // TODO: Proper error handling.

    let typeid = parse_type(c);

    ast::FunctionArg { id, typeid }
}

fn parse_function_args(c: &mut TokenCursor) -> ast::FunctionArgs {
    let mut result = ast::FunctionArgs::new();

    loop {
        result.push(parse_function_arg(c));

        if !c.advance_if(Token::Comma) {
            break;
        }
    }

    result
}

fn parse_const_decl(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::Const);

    let id = parse_identifier(c);

    // TODO: Enforce upper case style..

    c.consume_msg(
        Token::Colon,
        "expected ':' for type annotation of constant value",
    );

    let typeid = parse_type(c);

    c.consume_msg(
        Token::Is,
        "expected '=' for initialization of constant value",
    );

    let value = match typeid {
        ast::TypeKind::Bool => parse_bool_literal(c),
        ast::TypeKind::Number => parse_number_literal(c),
        ast::TypeKind::String => parse_string_literal(c),
    };

    c.consume(Token::SemiColon);

    ast::Stmt {
        kind: ast::StmtKind::ConstDecl(Ptr::new(ast::ConstDecl { id, typeid, value })),
    }
}

fn parse_var_decl(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::Let);

    let mutable = c.advance_if(Token::Mut);

    let id = parse_identifier(c);

    let typeid = if c.advance_if(Token::Colon) {
        Some(parse_type(c))
    } else {
        None
    };

    let value = if c.advance_if(Token::Is) {
        Some(parse_expr(c))
    } else {
        None
    };

    c.consume(Token::SemiColon);

    ast::Stmt {
        kind: ast::StmtKind::VarDecl(Ptr::new(ast::VarDecl {
            id,
            mutable,
            typeid,
            value,
        })),
    } // TODO
}

fn parse_use_decl(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::Use);

    // TODO: Check for global scope?

    let filename = parse_string_literal(c);

    c.consume_msg(Token::SemiColon, "expected semicolon after statement");

    ast::Stmt {
        kind: ast::StmtKind::UseDecl(Ptr::new(ast::UseDecl { filename })),
    } // TODO
}

fn parse_stmt(c: &mut TokenCursor) -> ast::Stmt {
    match c.peek() {
        Some(Token::LeftBrace) => parse_block_stmt(c),
        Some(Token::Print) => parse_print_stmt(c),
        Some(Token::Return) => parse_return_stmt(c),
        _ => parse_expr_stmt(c),
    }
}

fn parse_block_stmt(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::LeftBrace);

    let mut body = ast::Stmts::new();
    loop {
        match c.peek() {
            Some(Token::RightBrace) => break,
            None => panic!("unexpected EOS while parsing block statement"), // TODO: Proper error handling..
            _ => body.push(parse_decl(c)),
        }
    }

    c.consume(Token::RightBrace);

    ast::Stmt {
        kind: ast::StmtKind::Block(body),
    }
}

fn parse_identifier(c: &mut TokenCursor) -> String {
    lazy_static! {
        static ref KEYWORDS: Tokens = vec![
            Token::NumberId,
            Token::StringId,
            Token::BoolId,
            Token::Const,
            Token::Else,
            Token::False,
            Token::Function,
            Token::For,
            Token::Group,
            Token::If,
            Token::Let,
            Token::Mut,
            Token::Node,
            Token::Print,
            Token::Return,
            Token::True,
            Token::Use,
            Token::While,
        ];
    }

    match c.value() {
        Some(Token::Identifier(i)) => i,
        Some(t) if KEYWORDS.contains(&t) => {
            panic!("cannot use a reserved language keyword as an identifier")
        } // TODO: Proper error handling..
        _ => panic!("unexpected token"), // TODO: Proper error handling..
    }
}

fn parse_type(c: &mut TokenCursor) -> ast::TypeKind {
    match c.value() {
        Some(Token::BoolId) => ast::TypeKind::Bool,
        Some(Token::NumberId) => ast::TypeKind::Number,
        Some(Token::StringId) => ast::TypeKind::String,
        _ => panic!("unexpected type ID"), // TODO: Proper error handling..
    }
}

fn parse_expr(c: &mut TokenCursor) -> ast::Expr {
    let cur_token = c.peek().expect("unexpected end of token stream"); // TODO: Proper error handling..
    let next_token = c.peek_next().expect("unexpected end of token stream"); // TODO: Proper error handling..

    // Match expressions in descending precedence order:
    match (dbg!(cur_token), dbg!(next_token)) {
        // Function call expression:
        (Token::Identifier(_), Token::LeftParen) => parse_func_call_expr(c),
        // Unary expressions:
        (Token::Bang, _)
        | (Token::Plus, _)
        | (Token::Minus, _)
        | (Token::Group, _)
        | (Token::Node, _) => parse_unary_expr(c),
        // Binary expressions:
        (_, Token::Star)
        | (_, Token::Slash)
        | (_, Token::Percent)
        | (_, Token::Plus)
        | (_, Token::Minus)
        | (_, Token::Lt)
        | (_, Token::Gt)
        | (_, Token::LtEq)
        | (_, Token::GtEq)
        | (_, Token::Eq)
        | (_, Token::NotEq)
        | (_, Token::And)
        | (_, Token::Or) => parse_binary_expr(c),
        // Literal expressions:
        (Token::Number(_), _) => parse_number_literal(c),
        (Token::String(_), _) => parse_string_literal(c),
        (Token::True, _) | (Token::False, _) => parse_bool_literal(c),
        // Group expressions:
        (Token::LeftParen, _) => parse_group_expr(c),
        _ => {
            c.fast_forward_while(|t| t != &Token::SemiColon);

            ast::Expr {
                kind: ast::ExprKind::Unsupported("...".to_string()),
            }
        }
    }
}

fn parse_expr_stmt(c: &mut TokenCursor) -> ast::Stmt {
    let expr = parse_expr(c);

    if !c.advance_if(Token::SemiColon) {
        // TODO: Mark block result value..
    }

    ast::Stmt {
        kind: ast::StmtKind::Expr(Ptr::new(expr)),
    }
}

fn parse_func_call_expr(c: &mut TokenCursor) -> ast::Expr {
    c.fast_forward_while(|t| t != &Token::SemiColon); // TODO
    ast::Expr {
        kind: ast::ExprKind::Unsupported("func_call".to_string()),
    }
}

fn parse_binary_expr(c: &mut TokenCursor) -> ast::Expr {
    let lhs = parse_expr(c);

    let operator = match c.value() {
        Some(Token::And) => ast::BinaryOp::And,
        Some(Token::Slash) => ast::BinaryOp::Divide,
        Some(Token::Eq) => ast::BinaryOp::Eq,
        Some(Token::Gt) => ast::BinaryOp::Gt,
        Some(Token::GtEq) => ast::BinaryOp::GtEq,
        Some(Token::Lt) => ast::BinaryOp::Lt,
        Some(Token::LtEq) => ast::BinaryOp::LtEq,
        Some(Token::Star) => ast::BinaryOp::Multiply,
        Some(Token::NotEq) => ast::BinaryOp::NotEq,
        Some(Token::Or) => ast::BinaryOp::Or,
        Some(Token::Plus) => ast::BinaryOp::Plus,
        Some(Token::Percent) => ast::BinaryOp::Remainder,
        Some(Token::Minus) => ast::BinaryOp::Subtract,
        Some(_) => panic!("not a binary expression token"), // TODO: Proper error handling..
        None => panic!("unexpected end of token stream"),   // TODO: Proper error handling..
    };

    let rhs = parse_expr(c);

    ast::Expr {
        kind: ast::ExprKind::Binary(Ptr::new(ast::BinaryExpr { operator, lhs, rhs })),
    }
}

fn parse_unary_expr(c: &mut TokenCursor) -> ast::Expr {
    let operator = match c.value() {
        Some(Token::Bang) => ast::UnaryOp::Bang,
        Some(Token::Minus) => ast::UnaryOp::Minus,
        Some(Token::Group) => ast::UnaryOp::Group,
        Some(Token::Node) => ast::UnaryOp::Node,
        Some(Token::Plus) => ast::UnaryOp::Plus,
        Some(_) => panic!("not a unary expression token"), // TODO: Proper error handling..
        None => panic!("unexpected end of token stream"),  // TODO: Proper error handling..
    };

    let expr = parse_expr(c);

    ast::Expr {
        kind: ast::ExprKind::Unary(Ptr::new(ast::UnaryExpr { operator, expr })),
    }
}

fn parse_group_expr(c: &mut TokenCursor) -> ast::Expr {
    c.consume(Token::LeftParen);

    let expr = parse_expr(c);

    c.consume(Token::RightParen);

    ast::Expr {
        kind: ast::ExprKind::Group(Ptr::new(expr)),
    }
}

fn parse_print_stmt(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::Print);

    let expr = parse_expr(c);

    c.consume_msg(Token::SemiColon, "expected semicolon after statement");

    ast::Stmt {
        kind: ast::StmtKind::Print(Ptr::new(ast::Print { expr })),
    }
}

fn parse_return_stmt(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::Return);

    let expr = parse_expr(c);

    c.consume_msg(Token::SemiColon, "expected semicolon after statement");

    ast::Stmt {
        kind: ast::StmtKind::Return(Ptr::new(ast::Return { expr })),
    }
}

fn parse_bool_literal(c: &mut TokenCursor) -> ast::Expr {
    ast::Expr {
        kind: ast::ExprKind::Literal(Ptr::new(ast::Literal {
            kind: ast::LiteralKind::Bool(match c.value() {
                Some(Token::True) => true,
                Some(Token::False) => false,
                _ => panic!("not a boolean literal"), // TODO: Proper error handling..
            }),
        })),
    }
}

fn parse_number_literal(c: &mut TokenCursor) -> ast::Expr {
    ast::Expr {
        kind: ast::ExprKind::Literal(Ptr::new(ast::Literal {
            kind: ast::LiteralKind::Number(match c.value() {
                Some(Token::Number(n)) => n,
                _ => panic!("not a number literal"), // TODO: Proper error handling..
            }),
        })),
    }
}

fn parse_string_literal(c: &mut TokenCursor) -> ast::Expr {
    ast::Expr {
        kind: ast::ExprKind::Literal(Ptr::new(ast::Literal {
            kind: ast::LiteralKind::String(match c.value() {
                Some(Token::String(s)) => s,
                _ => panic!("not a string literal"), // TODO: Proper error handling..
            }),
        })),
    }
}
