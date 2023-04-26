use crate::ast;
use crate::ptr::Ptr;
use crate::token::{Token, Tokens};
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
struct TokenCursor {
    iter: Peekable<IntoIter<Token>>,
    prev: Option<Token>,
    curr: Option<Token>,
}

impl TokenCursor {
    fn new(tokens: Tokens) -> Self {
        let mut iter = tokens.into_iter().peekable();
        let curr = iter.next();
        TokenCursor {
            iter,
            prev: None,
            curr,
        }
    }

    fn value(&mut self) -> Option<Token> {
        self.curr.take()
    }

    fn advance(&mut self) {
        self.prev = self.curr.take();
        self.curr = self.iter.next();
    }

    // TODO: Extend with parsing of value for a content-bearing tokens like Identifier/String/etc..
    fn consume(&mut self, token: Token, msg: &str) {
        assert_eq!(self.curr, Some(token), "{}", msg); // TODO: Proper error handling..
        self.advance();
    }

    fn eos(&self) -> bool {
        self.curr.is_none()
    }
}

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
            ast.push(parse_print(&mut self.cursor));
        }

        ast
    }
}

fn parse_print(c: &mut TokenCursor) -> ast::Stmt {
    c.consume(Token::Print, "expected print statement"); // TODO: Auto-consume this one during match..
    c.consume(Token::LeftParen, "expected '(' after print statement");

    // XXX: Strings only for now..
    let value = parse_string_literal(c);

    c.consume(Token::RightParen, "expected ')' after expression");
    c.consume(Token::SemiColon, "expected semicolon after statement");

    ast::Stmt {
        kind: ast::StmtKind::Print(Ptr::new(ast::Print { value })),
    }
}

fn parse_string_literal(c: &mut TokenCursor) -> ast::StringLiteral {
    ast::StringLiteral {
        value: match c.value() {
            Some(Token::String(s)) => {
                c.advance();
                s
            }
            _ => panic!("not a string literal"), // TODO: Proper error handling..
        },
    }
}
