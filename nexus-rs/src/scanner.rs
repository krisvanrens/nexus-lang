use crate::cursor::Cursor;
use crate::token::Token;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[cfg(test)]
use std::f64::consts::PI;

pub type Tokens = Vec<Token>;

pub struct Scanner {
    comment_: bool, //<! Indicates multiline comment state.
}

type TokenMap = HashMap<&'static str, Token>;

impl Scanner {
    /// Construct a new scanner.
    pub fn new() -> Self {
        Scanner { comment_: false }
    }

    /// Scan a line of text and output the tokens found.
    ///
    /// Example:
    ///
    /// ```
    /// use nexus_rs::{scanner::Scanner, token::Token};
    ///
    /// let mut s = Scanner::new();
    /// assert_eq!(s.scan("let x;".to_string()),
    ///            vec![Token::Let,
    ///                 Token::Identifier("x".to_string()),
    ///                 Token::SemiColon]);
    /// ```
    pub fn scan(&mut self, line: String) -> Tokens {
        lazy_static! {
            static ref KEYWORDS: TokenMap = {
                let mut map: TokenMap = HashMap::new();
                map.insert("false", Token::False);
                map.insert("fn", Token::Function);
                map.insert("for", Token::For);
                map.insert("if", Token::If);
                map.insert("let", Token::Let);
                map.insert("node", Token::Node);
                map.insert("print", Token::Print);
                map.insert("return", Token::Return);
                map.insert("true", Token::True);
                map.insert("while", Token::While);
                map
            };
        }

        let mut tokens = Vec::new();

        let mut cursor = Cursor::new(&line);
        while let Some(c) = cursor.value() {
            if !self.comment_ {
                match c {
                    ' ' | '\r' | '\t' => (),
                    '(' => tokens.push(Token::LeftParen),
                    ')' => tokens.push(Token::RightParen),
                    '{' => tokens.push(Token::LeftBrace),
                    '}' => tokens.push(Token::RightBrace),
                    '[' => tokens.push(Token::LeftBracket),
                    ']' => tokens.push(Token::RightBracket),
                    ';' => tokens.push(Token::SemiColon),
                    '+' => tokens.push(Token::Plus),
                    '-' => match cursor.peek() {
                        Some('>') => {
                            cursor.advance();
                            tokens.push(Token::Arrow);
                        }
                        _ => tokens.push(Token::Minus),
                    },
                    '*' => tokens.push(Token::Star),
                    '\\' => tokens.push(Token::BackSlash),
                    '%' => tokens.push(Token::Percent),
                    ',' => tokens.push(Token::Comma),
                    '.' => match cursor.peek() {
                        Some('.') => {
                            cursor.advance();
                            tokens.push(Token::Range);
                        }
                        _ => tokens.push(Token::Dot),
                    },
                    '_' => tokens.push(Token::Underscore),
                    '=' => match cursor.peek() {
                        Some('=') => {
                            cursor.advance();
                            tokens.push(Token::Eq);
                        }
                        _ => tokens.push(Token::Is),
                    },
                    '|' => match cursor.peek() {
                        Some('|') => {
                            cursor.advance();

                            if cursor.peek_nonwhitespace() == Some('{') {
                                tokens.push(Token::EmptyClosure);
                            } else {
                                tokens.push(Token::Or);
                            }
                        }
                        _ => tokens.push(Token::Pipe),
                    },
                    '>' => match cursor.peek() {
                        Some('=') => {
                            cursor.advance();
                            tokens.push(Token::GtEq);
                        }
                        _ => tokens.push(Token::Gt),
                    },
                    '<' => match cursor.peek() {
                        Some('=') => {
                            cursor.advance();
                            tokens.push(Token::LtEq);
                        }
                        _ => tokens.push(Token::Lt),
                    },
                    '!' => match cursor.peek() {
                        Some('=') => {
                            cursor.advance();
                            tokens.push(Token::NotEq);
                        }
                        _ => tokens.push(Token::Bang),
                    },
                    '&' => {
                        if cursor.peek() == Some('&') {
                            cursor.advance();
                            tokens.push(Token::And);
                        }
                    }
                    '/' => match cursor.peek() {
                        Some('/') => break,
                        Some('*') => {
                            cursor.advance();
                            self.comment_ = true;
                        }
                        _ => tokens.push(Token::Slash),
                    },
                    '"' => {
                        tokens.push(Token::String(parse_string(&mut cursor)));
                    }
                    x if x.is_alphanumeric() => {
                        if c.is_ascii_digit() {
                            tokens.push(Token::Number(parse_number(&mut cursor)));
                        } else {
                            let word = cursor.peek_word().unwrap();
                            cursor.advance_by(word.chars().count() - 1);

                            tokens.push(if let Some(token) = KEYWORDS.get(&word.as_str()) {
                                token.clone()
                            } else {
                                Token::Identifier(word)
                            });
                        }
                    }
                    _ => panic!("unexpected character: '{c}'"), // TODO: Handle lexing error.
                }
            } else if (c == '*') && (cursor.peek() == Some('/')) {
                cursor.advance();
                self.comment_ = false;
            }

            cursor.advance();
        }

        tokens
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_string(cursor: &mut Cursor) -> String {
    let mut result = String::new();
    let mut escaped = false;
    let mut ended = false;

    cursor.advance(); // Consume opening quotes.

    while let Some(c) = cursor.value() {
        match c {
            '"' => {
                if !escaped {
                    ended = true;
                    break;
                } else {
                    result.push(c);
                }
            }
            '\\' => {
                if escaped {
                    result.push(c);
                }

                escaped = !escaped;
            }
            _ => result.push(c),
        }

        escaped = escaped && (c == '\\');

        cursor.advance();
    }

    if !ended {
        panic!("unterminated string"); // TODO: Handle lexing error.
    }

    if escaped {
        panic!("malformed string with escaping"); // TODO: Handle lexing error.
    }

    result
}

#[test]
fn parse_string_test() {
    let test = |input: &str| {
        let s = "\"".to_string() + input + "\"";
        let mut cursor = Cursor::new(&s);
        assert_eq!(
            parse_string(&mut cursor),
            input
                .to_string()
                .replace("\\\"", "\"")
                .replace("\\\\", "\\")
        );
    };

    test("");
    test("x");
    test("abc");
    test("With spaces");
    test("With multiple spaces");
    test("W1th num63r5");
    test("W|]h $pec!@l ch@r@ct#r5!");
    test("With ŮñĭçøƋɇ characters");
    test("With \newli\nes and \tab");

    test(r#"With escaped \"quotes\""#);
    test(r#"With backslashes \\\\"#);
    test(r#"\"quotes at the sides\""#);
}

fn parse_number(cursor: &mut Cursor) -> f64 {
    let mut result = cursor.value().unwrap().to_string();

    while let Some(c) = cursor.peek() {
        match c {
            '0'..='9' => result.push(c),
            '.' => match cursor.peek_nth(2) {
                Some('.') => break,
                Some(x) if x.is_ascii_digit() => {
                    result.push(c);
                }
                _ => panic!("unexpected token"), // TODO: Handle lexing error.
            },
            _ => break,
        }

        cursor.advance();
    }

    result.parse::<f64>().expect("failed to parse number") // TODO: Handle lexing error.
}

#[test]
fn parse_number_test() {
    let test = |input: &str, expected: f64| {
        let mut cursor = Cursor::new(input);
        assert!(parse_number(&mut cursor) - expected < 0.001);
    };

    test("0", 0.0);
    test("1", 1.0);
    test("0.0", 0.0);
    test("1.0", 1.0);
    test("0.0000", 0.0);
    test("1.0000", 1.0);
    test("1000", 1000.0);
    test("123456", 123456.0);
    test("123.456", 123.456);
    test("123.456", 123.456);
    test("3.1415926535", PI);
}
