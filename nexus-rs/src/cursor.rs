use std::{iter::Peekable, str::Chars};

/// Cursor to characters in a string, providing direct value access and advanced peeking.
#[derive(Debug)]
pub struct Cursor<'a> {
    chars: Peekable<Chars<'a>>,
    value: Option<char>,
}

impl<'a> Cursor<'a> {
    /// Create a new cursor given a string.
    ///
    /// The cursor is initialized with the first character of the string.
    ///
    /// Example:
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "Hello".to_string();
    /// let c = Cursor::new(&s);
    ///
    /// assert_eq!(c.value(), Some('H'));
    /// assert_eq!(c.peek(), Some('e'));
    /// ```
    pub fn new(line: &'a str) -> Self {
        let mut chars = line.chars();
        let value = chars.next();

        Cursor {
            chars: chars.peekable(),
            value,
        }
    }

    /// Get the value of the current character the cursor is pointing at (if any).
    ///
    /// Example:
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "ab".to_string();
    /// let mut c = Cursor::new(&s);
    ///
    /// assert_eq!(c.value(), Some('a'));
    /// c.advance();
    /// assert_eq!(c.value(), Some('b'));
    /// ```
    pub fn value(&self) -> Option<char> {
        self.value
    }

    /// Advance the cursor one position, replacing the inner value.
    ///
    /// Example:
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "abc".to_string();
    /// let mut c = Cursor::new(&s);
    ///
    /// assert_eq!(c.value(), Some('a'));
    /// c.advance();
    /// assert_eq!(c.value(), Some('b'));
    /// c.advance();
    /// assert_eq!(c.value(), Some('c'));
    /// c.advance();
    /// assert_eq!(c.value(), None);
    /// ```
    pub fn advance(&mut self) {
        self.value = self.chars.next();
    }

    /// Advance the cursor by N positions, consuming the value at each increment.
    ///
    /// If N is zero, `advance_by` is a no-op.
    /// It is a valid operation to advance the cursor beyond the end-of-line (EOL).
    ///
    /// Example:
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "abcdefg".to_string();
    /// let mut c = Cursor::new(&s);
    ///
    /// assert_eq!(c.value(), Some('a'));
    /// c.advance_by(3);
    /// assert_eq!(c.value(), Some('d'));
    /// c.advance_by(10);
    /// assert_eq!(c.value(), None);
    /// ```
    pub fn advance_by(&mut self, n: usize) {
        match n {
            0 => (),
            _ => {
                for _ in 0..(n - 1) {
                    self.chars.next();

                    if self.chars.peek().is_none() {
                        break;
                    }
                }

                self.value = self.chars.next();
            }
        }
    }

    /// Peek into the next character without consuming the current value.
    ///
    /// Example:
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "ab".to_string();
    /// let mut c = Cursor::new(&s);
    ///
    /// assert_eq!(c.value(), Some('a'));
    /// assert_eq!(c.peek(), Some('b'));
    /// c.advance();
    /// assert_eq!(c.value(), Some('b'));
    /// assert_eq!(c.peek(), None);
    /// ```
    pub fn peek(&self) -> Option<char> {
        self.chars.clone().peek().copied()
    }

    /// Peek into the next nth character without consuming the current value.
    ///
    /// Returns the current cursor value if 'n' is zero.
    ///
    /// Example:
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "abc".to_string();
    /// let c = Cursor::new(&s);
    ///
    /// assert_eq!(c.value(), Some('a'));
    /// assert_eq!(c.peek_nth(0), Some('a'));
    /// assert_eq!(c.peek_nth(1), Some('b'));
    /// assert_eq!(c.peek_nth(2), Some('c'));
    /// ```
    pub fn peek_nth(&self, n: usize) -> Option<char> {
        match n {
            0 => self.value(),
            1 => self.peek(),
            _ => {
                let mut iter_clone = self.chars.clone();
                for _ in 0..(n - 1) {
                    iter_clone.next();
                }

                iter_clone.peek().copied()
            }
        }
    }

    /// Peek while a predicated holds without consuming the current value.
    ///
    /// Example:
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "abc123 def".to_string();
    /// let c = Cursor::new(&s);
    ///
    /// assert_eq!(c.peek_while(|c| c.is_alphanumeric()), Some("abc123".to_string()));
    /// ```
    pub fn peek_while(&self, mut predicate: impl FnMut(char) -> bool) -> Option<String> {
        let mut result = self.value.unwrap().to_string();

        if !self.eol() {
            result += &self
                .chars
                .clone()
                .take_while(|c| predicate(*c))
                .collect::<String>();

            Some(result)
        } else {
            None
        }
    }

    /// Peek the next word without consuming the current value.
    ///
    /// A "word" is defined as a consecutive sequence of alphanumeric characters or '_' (underscore).
    /// The current value of the cursor is taken as the first character of the word.
    ///
    /// Example:
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "abc_12 def".to_string();
    /// let mut c = Cursor::new(&s);
    ///
    /// assert_eq!(c.peek_word(), Some("abc_12".to_string()));
    /// ```
    pub fn peek_word(&self) -> Option<String> {
        if !self.eol() && self.value.unwrap().is_alphanumeric() {
            let mut result = self.value.unwrap().to_string();

            result += &self
                .chars
                .clone()
                .take_while(|x| x.is_alphanumeric() || x == &'_')
                .collect::<String>();

            Some(result)
        } else {
            None
        }
    }

    /// Check if the cursor is at end-of-line (EOL).
    ///
    /// Example:
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "x".to_string();
    /// let mut c = Cursor::new(&s);
    ///
    /// assert!(!c.eol());
    /// c.advance();
    /// assert!(c.eol());
    /// ```
    pub fn eol(&self) -> bool {
        self.value.is_none()
    }
}

#[test]
fn cursor_new_test() {
    let empty = "".to_string();
    let line = "Test123".to_string();

    let c1 = Cursor::new(&empty);
    let c2 = Cursor::new(&line);

    assert_eq!(c1.value(), None);
    assert_eq!(c2.value(), Some('T'));
}

#[test]
fn cursor_eol_test() {
    let empty = "".to_string();
    let line = "abc".to_string();

    let c1 = Cursor::new(&empty);
    let mut c2 = Cursor::new(&line);

    assert!(c1.eol());
    assert!(!c2.eol());

    c2.advance();
    c2.advance();
    c2.advance();

    assert!(c2.eol());
}

#[test]
fn cursor_advance_test() {
    let line = "abcdefg".to_string();

    let mut c = Cursor::new(&line);

    for i in "abcdefg".chars() {
        assert_eq!(c.value().unwrap(), i);
        c.advance();
    }

    assert!(c.eol());
}

#[test]
fn cursor_advance_by_test() {
    let line = "ab_cd_ɘƒ_gh".to_string();

    let mut c = Cursor::new(&line);

    assert_eq!(c.value(), Some('a'));
    c.advance_by(3);
    assert_eq!(c.value(), Some('c'));
    c.advance_by(3);
    assert_eq!(c.value(), Some('ɘ'));
    c.advance_by(3);
    assert_eq!(c.value(), Some('g'));

    c.advance_by(3);
    assert!(c.eol());
}

#[test]
fn cursor_peek_test() {
    let line = "abcdefg".to_string();

    let mut c = Cursor::new(&line);

    for i in "bcdefg".chars() {
        assert_eq!(c.peek().unwrap(), i);
        c.advance();
    }

    assert!(!c.eol());

    c.advance();

    assert!(c.eol());
}

#[test]
fn cursor_peek_nth_test() {
    let line = "abcdefg".to_string();

    let c = Cursor::new(&line);

    for (i, x) in line.char_indices() {
        assert_eq!(c.peek_nth(i), Some(x));
    }
}

#[test]
fn cursor_peek_while_test() {
    let line = "abc def".to_string();

    let mut c = Cursor::new(&line);

    let is_word_char = |c: char| -> bool { c.is_alphanumeric() || c == '_' };

    assert!(!c.eol());
    assert_eq!(c.value(), Some('a'));

    assert_eq!(c.peek_while(is_word_char), Some("abc".to_string()));

    assert!(!c.eol());
    assert_eq!(c.value(), Some('a'));

    c.advance();
    c.advance();
    c.advance();

    assert!(!c.eol());
    assert_eq!(c.value(), Some(' '));

    c.advance();

    assert!(!c.eol());
    assert_eq!(c.value(), Some('d'));

    assert_eq!(c.peek_while(is_word_char), Some("def".to_string()));

    assert!(!c.eol());
    assert_eq!(c.value(), Some('d'));
}

#[test]
fn peek_while_word_test() {
    let line = "abc def".to_string();

    let mut c = Cursor::new(&line);

    assert!(!c.eol());
    assert_eq!(c.value(), Some('a'));

    assert_eq!(c.peek_word(), Some("abc".to_string()));

    assert!(!c.eol());
    assert_eq!(c.value(), Some('a'));

    c.advance();
    c.advance();
    c.advance();

    assert!(!c.eol());
    assert_eq!(c.value(), Some(' '));

    c.advance();

    assert!(!c.eol());
    assert_eq!(c.value(), Some('d'));

    assert_eq!(c.peek_word(), Some("def".to_string()));

    assert!(!c.eol());
    assert_eq!(c.value(), Some('d'));
}

#[test]
fn parse_word_test() {
    let test = |word: &str| {
        let cursor = Cursor::new(word);
        assert_eq!(
            cursor
                .peek_while(|c| { c.is_alphanumeric() || c == '_' })
                .unwrap(),
            word.to_string()
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
