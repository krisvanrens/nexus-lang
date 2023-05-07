use crate::ast;
use crate::ptr::Ptr;
use crate::token::{Token, Tokens};
use crate::token_cursor::TokenCursor;

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
        Some(&Token::Function) => parse_function_decl(c),
        Some(&Token::Const) => parse_const_decl(c),
        Some(&Token::Let) => parse_var_decl(c),
        _ => parse_stmt(c),
    }
}

fn parse_function_decl(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::Function);

    let id = parse_identifier(c);

    c.consume_msg(Token::LeftParen, "expected '(' after function identifier"); // TODO: Proper error handling.

    let args = if c.peek() != Some(&Token::RightParen) {
        Some(parse_function_args(c))
    } else {
        None
    };

    c.consume_msg(
        Token::RightParen,
        "expected ')' after function argument list",
    ); // TODO: Proper error handling.

    let ret_type = if c.peek() == Some(&Token::Arrow) {
        c.consume_msg(Token::Arrow, "expected '->' in function declaration");
        Some(parse_type(c))
    } else {
        None
    };

    let _body = parse_block_stmt(c); // TODO

    ast::Stmt {
        kind: ast::StmtKind::FunctionDecl(Ptr::new(ast::FunctionDecl { id, args, ret_type })),
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

        if !c.advance_if(&Token::Comma) {
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
    c.fast_forward();
    ast::Stmt {
        kind: ast::StmtKind::Unsupported,
    } // TODO
}

fn parse_stmt(c: &mut TokenCursor) -> ast::Stmt {
    match c.peek() {
        Some(&Token::LeftBrace) => parse_block_stmt(c),
        Some(&Token::Node) => parse_node_stmt(c),
        Some(&Token::Print) => parse_print_stmt(c),
        _ => parse_expr_stmt(c),
    }
}

fn parse_block_stmt(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::LeftBrace);

    let mut body = ast::Stmts::new();
    loop {
        match c.peek() {
            Some(&Token::RightBrace) => break,
            None => panic!("unexpected EOS while parsing block statement"), // TODO: Proper error handling..
            _ => body.push(parse_stmt(c)),
        }
    }

    c.consume(Token::RightBrace);

    ast::Stmt {
        kind: ast::StmtKind::Block(body),
    }
}

fn parse_identifier(c: &mut TokenCursor) -> String {
    match c.value() {
        Some(Token::Identifier(i)) => i,
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
    c.fast_forward();
    ast::Expr {
        kind: ast::ExprKind::Empty,
    } // TODO
}

fn parse_expr_stmt(c: &mut TokenCursor) -> ast::Stmt {
    c.fast_forward_while(|t| t != &Token::SemiColon);
    c.consume(Token::SemiColon);

    ast::Stmt {
        kind: ast::StmtKind::Unsupported,
    } // TODO
}

fn parse_node_stmt(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::Print);

    let expr = parse_expr(c);

    c.consume_msg(Token::SemiColon, "expected semicolon after statement");

    ast::Stmt {
        kind: ast::StmtKind::Node(Ptr::new(ast::Node { expr })),
    }
}

fn parse_print_stmt(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::Node);

    let expr = parse_expr(c);

    c.consume_msg(Token::SemiColon, "expected semicolon after statement");

    ast::Stmt {
        kind: ast::StmtKind::Print(Ptr::new(ast::Print { expr })),
    }
}

fn parse_bool_literal(c: &mut TokenCursor) -> ast::Literal {
    ast::Literal {
        kind: ast::LiteralKind::Boolean(match c.value() {
            Some(Token::True) => true,
            Some(Token::False) => false,
            _ => panic!("not a boolean literal"), // TODO: Proper error handling..
        }),
    }
}

fn parse_number_literal(c: &mut TokenCursor) -> ast::Literal {
    ast::Literal {
        kind: ast::LiteralKind::Number(match c.value() {
            Some(Token::Number(n)) => n,
            _ => panic!("not a number literal"), // TODO: Proper error handling..
        }),
    }
}

fn parse_string_literal(c: &mut TokenCursor) -> ast::Literal {
    ast::Literal {
        kind: ast::LiteralKind::String(match c.value() {
            Some(Token::String(s)) => s,
            _ => panic!("not a string literal"), // TODO: Proper error handling..
        }),
    }
}
