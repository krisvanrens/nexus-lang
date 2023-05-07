use crate::token::{Token, Tokens};
use std::iter::Peekable;
use std::vec::IntoIter;

/// Cursor for tokens in a token collection.
#[derive(Debug)]
pub struct TokenCursor {
    iter: Peekable<IntoIter<Token>>,
    prev: Option<Token>,
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
        TokenCursor {
            iter,
            prev: None,
            curr,
        }
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

    /// Peek value only (without advancing).
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
    /// assert_eq!(c.peek(), Some(&Token::Arrow));
    /// ```
    pub fn peek(&mut self) -> Option<&Token> {
        self.curr.as_ref()
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
    /// assert_eq!(c.peek(), Some(&Token::Let));
    /// c.advance();
    /// assert_eq!(c.peek(), None);
    /// ```
    pub fn advance(&mut self) {
        self.prev = self.curr.take();
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
    /// assert_eq!(c.advance_if(&Token::Let), true);
    /// assert_eq!(c.advance_if(&Token::Let), false);
    /// ```
    pub fn advance_if(&mut self, match_token: &Token) -> bool {
        if self.peek() == Some(match_token) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consume an expected token.
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
    /// c.consume(Token::Let);
    /// c.consume(Token::Arrow);
    /// ```
    pub fn consume(&mut self, expected: Token) {
        assert_eq!(self.curr, Some(expected));
        self.advance();
    }

    /// Consume an expected token (with fail error message).
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
    /// c.consume_msg(Token::Let, "Expected 'let' token");
    /// c.consume_msg(Token::Arrow, "Expected 'arrow' token");
    /// ```
    pub fn consume_msg(&mut self, expected: Token, msg: &str) {
        assert_eq!(self.curr, Some(expected), "{}", msg); // TODO: Proper error handling..
        self.advance();
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

// Temporary helpers.
impl TokenCursor {
    /// Temporary helper function to fast-forward unsupported tokens (until EOS).
    pub fn fast_forward(&mut self) {
        while !self.eos() {
            self.advance();
        }
    }

    /// Temporary helper function to fast-forward unsupported tokens (while the predicate holds).
    pub fn fast_forward_while(&mut self, mut pred: impl FnMut(&Token) -> bool) {
        while !self.eos() && pred(self.curr.as_ref().unwrap()) {
            self.advance();
        }
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
fn advance_test() {
    let t = vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::SemiColon,
    ];
    let mut c = TokenCursor::new(t);

    assert_eq!(c.peek(), Some(&Token::Let));

    c.advance();

    assert_eq!(c.peek(), Some(&Token::Identifier("x".to_string())));

    c.advance();

    assert_eq!(c.peek(), Some(&Token::SemiColon));

    c.advance();

    assert_eq!(c.peek(), None);
}

#[test]
fn advance_if_test() {
    let t = vec![Token::Let, Token::Arrow];
    let mut c = TokenCursor::new(t);

    assert_eq!(c.advance_if(&Token::Let), true);
    assert_eq!(c.advance_if(&Token::Let), false);
}

#[test]
fn consume_test() {
    let t = vec![Token::Let, Token::Arrow];
    let mut c = TokenCursor::new(t);

    c.consume(Token::Let);
    c.consume(Token::Arrow);
}

#[test]
fn consume_msg_test() {
    let t = vec![Token::Let, Token::Arrow];
    let mut c = TokenCursor::new(t);

    c.consume_msg(Token::Let, "Expected 'let' token");
    c.consume_msg(Token::Arrow, "Expected 'arrow' token");
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
