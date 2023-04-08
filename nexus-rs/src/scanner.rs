use crate::cursor::Cursor;
use crate::token::{Token, Tokens};
use lazy_static::lazy_static;
use std::collections::HashMap;
use thiserror::Error;

#[cfg(test)]
use std::f64::consts::PI;

/// Scanner for Nexus.
///
/// **Note**: at this moment, the scanner is *not* suitable for out-of-order parallel operation.
/// Due to support for multiline comments in Nexus, line scans are noncommutative.
pub struct Scanner {
    comment_: bool, //<! Indicates multiline comment state.
}

/// Scanning/lexing error representation.
#[derive(Error, Debug)]
pub enum ScanError {
    #[error("malformed string literal")]
    MalformedString,

    #[error("failed to parse number '{0}'")]
    NumberParseError(String),

    #[error("failed to parse word")]
    WordParseError,

    #[error("unexpected character")]
    UnexpectedCharacter,

    #[error("unterminated string")]
    UnterminatedString,
}

impl Scanner {
    /// Construct a new scanner.
    pub fn new() -> Self {
        Scanner { comment_: false }
    }

    /// Scan a line of text and output the tokens found, or a scanning error.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::{scanner::Scanner, token::Token};
    ///
    /// let mut s = Scanner::new();
    /// if let Ok(tokens) = s.scan("let x;".to_string()) {
    ///     assert_eq!(tokens,
    ///                vec![Token::Let,
    ///                     Token::Identifier("x".to_string()),
    ///                     Token::SemiColon]);
    /// }
    /// ```
    pub fn scan(&mut self, line: String) -> Result<Tokens, ScanError> {
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
                    '|' => tokens.push(Token::Pipe),
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
                        tokens.push(Token::String(parse_string(&mut cursor)?));
                    }
                    '0'..='9' => tokens.push(Token::Number(parse_number(&mut cursor)?)),
                    x if x.is_alphabetic() => tokens.push(parse_word(&mut cursor)?),
                    _ => return Err(ScanError::UnexpectedCharacter),
                }
            } else if (c == '*') && (cursor.peek() == Some('/')) {
                cursor.advance();
                self.comment_ = false;
            }

            cursor.advance();
        }

        Ok(tokens)
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_string(cursor: &mut Cursor) -> Result<String, ScanError> {
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
        return Err(ScanError::UnterminatedString);
    }

    if escaped {
        return Err(ScanError::MalformedString);
    }

    Ok(result)
}

#[test]
fn parse_string_test() {
    let test = |input: &str| {
        let s = "\"".to_string() + input + "\"";
        let mut cursor = Cursor::new(&s);
        assert_eq!(
            parse_string(&mut cursor).unwrap(),
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

fn parse_number(cursor: &mut Cursor) -> Result<f64, ScanError> {
    let mut result = cursor.value().unwrap().to_string();

    while let Some(c) = cursor.peek() {
        match c {
            '0'..='9' => result.push(c),
            '.' => match cursor.peek_nth(2) {
                Some('.') => break,
                Some(x) if x.is_ascii_digit() => {
                    result.push(c);
                }
                _ => return Err(ScanError::UnexpectedCharacter),
            },
            _ => break,
        }

        cursor.advance();
    }

    result
        .parse::<f64>()
        .map_err(|e| ScanError::NumberParseError(e.to_string()))
}

#[test]
fn parse_number_test() {
    let test = |input: &str, expected: f64| {
        let mut cursor = Cursor::new(input);
        assert!(parse_number(&mut cursor).unwrap() - expected < 0.001);
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

type TokenMap = HashMap<&'static str, Token>;

/// Initialize a token map using 'key => value' notation.
macro_rules! token_map {
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map: TokenMap = HashMap::new();
            $(map.insert($key, $value);)+
            map
        }
    }
}

fn parse_word(cursor: &mut Cursor) -> Result<Token, ScanError> {
    lazy_static! {
        static ref KEYWORDS: TokenMap = token_map! {
            "false"  => Token::False,
            "fn"     => Token::Function,
            "for"    => Token::For,
            "if"     => Token::If,
            "let"    => Token::Let,
            "node"   => Token::Node,
            "print"  => Token::Print,
            "return" => Token::Return,
            "true"   => Token::True,
            "while"  => Token::While,
        };
    }

    match cursor.peek_while(|c| c.is_alphanumeric() || c == '_') {
        Some(word) => {
            cursor.advance_by(word.chars().count() - 1);
            if let Some(token) = KEYWORDS.get(&word.as_str()) {
                Ok(token.clone())
            } else {
                Ok(Token::Identifier(word))
            }
        }
        _ => Err(ScanError::WordParseError),
    }
}

#[test]
fn parse_word_identifier_test() {
    let test = |word: &str| {
        let mut cursor = Cursor::new(word);
        assert_eq!(
            parse_word(&mut cursor).unwrap(),
            Token::Identifier(word.to_string())
        );
    };

    test("x");
    test("ah");
    test("word");
    test("CamelCase");
    test("snake_case");
    test("ALLUPPER");
    test("ŮñĭçøƋɇ");
    test("trailing_numbers012");
    test("numbers1n8etw33n");
    test("veeeeeeeerylooooooongwooooooord");
}

#[test]
fn parse_word_keyword_test() {
    let test = |word: &str, expected: Token| {
        let mut cursor = Cursor::new(word);
        assert_eq!(parse_word(&mut cursor).unwrap(), expected);
    };

    test("false", Token::False);
    test("fn", Token::Function);
    test("for", Token::For);
    test("if", Token::If);
    test("let", Token::Let);
    test("node", Token::Node);
    test("print", Token::Print);
    test("return", Token::Return);
    test("true", Token::True);
    test("while", Token::While);
}
