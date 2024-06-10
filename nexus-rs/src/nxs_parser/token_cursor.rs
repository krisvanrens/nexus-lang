use crate::parse_error::*;
use crate::token::{Token, Tokens};
use std::{iter::Peekable, vec::IntoIter};

/// Cursor for tokens in a token collection.
#[derive(Debug)]
pub struct TokenCursor {
    iter: Peekable<IntoIter<Token>>,
    curr: Option<Token>,
}

impl TokenCursor {
    /// Create a new cursor from a collection of tokens.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::token_cursor::TokenCursor;
    /// use nexus_rs::token::Tokens;
    ///
    /// let t = Tokens::new();
    /// let c = TokenCursor::new(t);
    /// ```
    pub fn new(tokens: Tokens) -> Self {
        let mut iter = tokens.into_iter().peekable();
        let curr = iter.next();
        TokenCursor { iter, curr }
    }

    /// Take value and advance cursor.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::token_cursor::TokenCursor;
    /// use nexus_rs::token::Token;
    ///
    /// let t = vec![Token::Let];
    /// let mut c = TokenCursor::new(t);
    ///
    /// assert_eq!(c.value(), Some(Token::Let));
    /// assert_eq!(c.value(), None);
    /// ```
    pub fn value(&mut self) -> Option<Token> {
        let value = self.curr.take();
        self.advance();
        value
    }

    /// Peek upcoming value (without advancing).
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::token_cursor::TokenCursor;
    /// use nexus_rs::token::Token;
    ///
    /// let t = vec![Token::Let, Token::Arrow];
    /// let mut c = TokenCursor::new(t);
    ///
    /// assert_eq!(c.value(), Some(Token::Let));
    /// assert_eq!(c.peek(), Some(Token::Arrow));
    /// ```
    pub fn peek(&self) -> Option<Token> {
        self.curr.clone()
    }

    /// Peek one past the upcoming value (without advancing).
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::token_cursor::TokenCursor;
    /// use nexus_rs::token::Token;
    ///
    /// let t = vec![Token::Let, Token::Arrow, Token::For];
    /// let mut c = TokenCursor::new(t);
    ///
    /// assert_eq!(c.value(), Some(Token::Let));
    /// assert_eq!(c.peek(), Some(Token::Arrow));
    /// assert_eq!(c.peek_next(), Some(Token::For));
    /// ```
    pub fn peek_next(&self) -> Option<Token> {
        self.iter.clone().next()
    }

    /// Advance cursor.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::token_cursor::TokenCursor;
    /// use nexus_rs::token::Token;
    ///
    /// let t = vec![Token::Let];
    /// let mut c = TokenCursor::new(t);
    ///
    /// assert_eq!(c.peek(), Some(Token::Let));
    /// c.advance();
    /// assert_eq!(c.peek(), None);
    /// ```
    pub fn advance(&mut self) {
        self.curr = self.iter.next();
    }

    /// Conditionally advance cursor (and return match result).
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::token_cursor::TokenCursor;
    /// use nexus_rs::token::Token;
    ///
    /// let t = vec![Token::Let, Token::Arrow];
    /// let mut c = TokenCursor::new(t);
    ///
    /// assert_eq!(c.advance_if(Token::Let), true);
    /// assert_eq!(c.advance_if(Token::Let), false);
    /// ```
    pub fn advance_if(&mut self, match_token: Token) -> bool {
        if self.peek() == Some(match_token) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consume an expected token.
    /// Returns `Ok` if the consumed token matches the expected one, otherwise a parse error.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::token_cursor::TokenCursor;
    /// use nexus_rs::token::Token;
    ///
    /// let t = vec![Token::Let, Token::Arrow, Token::Colon];
    /// let mut c = TokenCursor::new(t);
    ///
    /// assert!(c.consume(Token::Let).is_ok());
    /// assert!(c.consume(Token::Arrow).is_ok());
    /// assert!(c.consume(Token::SemiColon).is_err());
    /// ```
    pub fn consume(&mut self, expected: Token) -> ParseResult<()> {
        if self.curr == Some(expected.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(ParseErrorKind::Expected(expected)))
        }
    }

    /// Consume an expected token (with a custom message explaining the specific reason).
    /// Returns `Ok` if the consumed token matches the expected one, otherwise a parse error.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::token_cursor::TokenCursor;
    /// use nexus_rs::token::Token;
    ///
    /// let t = vec![Token::Let, Token::Arrow, Token::Colon];
    /// let mut c = TokenCursor::new(t);
    ///
    /// assert!(c.consume_msg(Token::Let, "for funzies").is_ok());
    /// assert!(c.consume_msg(Token::Arrow, "just because").is_ok());
    /// assert!(c.consume_msg(Token::SemiColon, "I like it").is_err());
    /// ```
    pub fn consume_msg(&mut self, expected: Token, reason: &str) -> ParseResult<()> {
        if self.curr == Some(expected.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(ParseErrorKind::ExpectedReason(
                expected,
                reason.to_owned(),
            )))
        }
    }

    /// Check if token stream is end-of-stream (EOS).
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::token_cursor::TokenCursor;
    /// use nexus_rs::token::Token;
    ///
    /// let t = vec![Token::Let];
    /// let mut c = TokenCursor::new(t);
    ///
    /// assert!(!c.eos());
    /// c.advance();
    /// assert!(c.eos());
    /// ```
    pub fn eos(&self) -> bool {
        self.curr.is_none()
    }
}

#[test]
fn new_test() {
    let t = vec![Token::Let];
    let mut c = TokenCursor::new(t);

    assert_eq!(c.value(), Some(Token::Let));
    assert_eq!(c.value(), None);
}

#[test]
fn value_test() {
    let t = vec![Token::Let, Token::Arrow];
    let mut c = TokenCursor::new(t);

    assert_eq!(c.value(), Some(Token::Let));
    assert_eq!(c.value(), Some(Token::Arrow));
    assert_eq!(c.value(), None);
}

#[test]
fn peek_test() {
    let t = vec![Token::Let, Token::Arrow];
    let mut c = TokenCursor::new(t);

    assert_eq!(c.value(), Some(Token::Let));
    assert_eq!(c.peek(), Some(Token::Arrow));
    assert_eq!(c.value(), Some(Token::Arrow));
    assert_eq!(c.peek(), None);
    assert_eq!(c.value(), None);
}

#[test]
fn peek_next_test() {
    let t = vec![Token::Let, Token::Arrow, Token::For];
    let mut c = TokenCursor::new(t);

    assert_eq!(c.value(), Some(Token::Let));
    assert_eq!(c.peek_next(), Some(Token::For));
    assert_eq!(c.value(), Some(Token::Arrow));
    assert_eq!(c.peek_next(), None);
    assert_eq!(c.value(), Some(Token::For));
}

#[test]
fn advance_test() {
    let t = vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::SemiColon,
    ];
    let mut c = TokenCursor::new(t);

    assert_eq!(c.peek(), Some(Token::Let));

    c.advance();

    assert_eq!(c.peek(), Some(Token::Identifier("x".to_string())));

    c.advance();

    assert_eq!(c.peek(), Some(Token::SemiColon));

    c.advance();

    assert_eq!(c.peek(), None);
}

#[test]
fn advance_if_test() {
    let t = vec![Token::Let, Token::Arrow];
    let mut c = TokenCursor::new(t);

    assert_eq!(c.advance_if(Token::Let), true);
    assert_eq!(c.advance_if(Token::Let), false);
}

#[test]
fn consume_test() {
    let t = vec![Token::Let, Token::Arrow, Token::Colon];
    let mut c = TokenCursor::new(t);

    assert!(c.consume(Token::Let).is_ok());
    assert!(c.consume(Token::Arrow).is_ok());
    assert!(c.consume(Token::SemiColon).is_err());
}

#[test]
fn consume_msg_test() {
    let t = vec![Token::Let, Token::Arrow, Token::Colon];
    let mut c = TokenCursor::new(t);

    assert!(c.consume_msg(Token::Let, "expected 'let'").is_ok());
    assert!(c.consume_msg(Token::Arrow, "expected '->'").is_ok());
    assert!(c.consume_msg(Token::SemiColon, "expected ';'").is_err());
}

#[test]
fn eos_test() {
    let t = vec![Token::Let, Token::Arrow];
    let mut c = TokenCursor::new(t);

    assert!(!c.eos());
    c.advance();
    assert!(!c.eos());
    c.advance();
    assert!(c.eos());
}
