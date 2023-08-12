use std::{iter::Peekable, str::Chars};

#[cfg(test)]
use pretty_assertions::assert_eq;

/// Cursor for characters in a string, providing direct value access and advanced peeking.
#[derive(Debug)]
pub struct Cursor<'a> {
    chars: Peekable<Chars<'a>>,
    index: usize,
    value: Option<char>,
}

impl<'a> Cursor<'a> {
    /// Create a new cursor given a string.
    ///
    /// The cursor is initialized with the first character of the string.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "Hello".to_string();
    /// let c = Cursor::new(&s);
    ///
    /// assert_eq!(c.value(), Some('H'));
    /// assert_eq!(c.index(), 0);
    /// assert_eq!(c.peek(), Some('e'));
    /// ```
    pub fn new(line: &'a str) -> Self {
        let mut chars = line.chars();
        let value = chars.next();

        Cursor {
            chars: chars.peekable(),
            index: 0,
            value,
        }
    }

    /// Get the value of the current character the cursor is pointing at (if any).
    ///
    /// # Example
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
    /// c.advance();
    /// assert_eq!(c.value(), None);
    /// ```
    pub fn value(&self) -> Option<char> {
        self.value
    }

    /// Advance the cursor one position, replacing the inner value.
    ///
    /// # Example
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
        self.index_inc();
    }

    /// Advance the cursor by N positions, consuming the value at each increment.
    ///
    /// If N is zero, `advance_by` is a no-op.
    /// It is a valid operation to advance the cursor beyond the end-of-line (EOL).
    ///
    /// # Example
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
                    self.index_inc();

                    if self.chars.peek().is_none() {
                        break;
                    }
                }

                self.value = self.chars.next();
                self.index_inc();
            }
        }
    }

    /// Peek into the next character without consuming the current value.
    ///
    /// # Example
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
    /// # Example
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
                let mut chars = self.chars.clone();
                for _ in 0..(n - 1) {
                    chars.next();
                }

                chars.peek().copied()
            }
        }
    }

    /// Peek while a predicated holds without consuming the current value.
    ///
    /// # Example
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

    /// Get the current character index of the cursor.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::cursor::Cursor;
    ///
    /// let s = "abcdefg".to_string();
    /// let mut c = Cursor::new(&s);
    ///
    /// assert_eq!(c.index(), 0);
    /// c.advance();
    /// assert_eq!(c.index(), 1);
    /// c.advance_by(3);
    /// assert_eq!(c.index(), 4);
    /// ```
    pub fn index(&self) -> usize {
        self.index
    }

    /// Check if the cursor is at end-of-line (EOL).
    ///
    /// # Example
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

    /// Increment index.
    fn index_inc(&mut self) {
        if !self.eol() {
            self.index += 1;
        }
    }
}

#[test]
fn new_test() {
    let empty = "".to_string();
    let line = "Test123".to_string();

    let c1 = Cursor::new(&empty);
    let c2 = Cursor::new(&line);

    assert_eq!(c1.value(), None);
    assert_eq!(c2.value(), Some('T'));
}

#[test]
fn eol_test() {
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
fn advance_test() {
    let line = "abcdefg".to_string();

    let mut c = Cursor::new(&line);

    for i in "abcdefg".chars() {
        assert_eq!(c.value().unwrap(), i);
        c.advance();
    }

    assert!(c.value().is_none());
    assert!(c.eol());
}

#[test]
fn advance_by_test() {
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

    assert!(c.value().is_none());
    assert!(c.eol());
}

#[test]
fn peek_test() {
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
fn peek_nth_test() {
    let line = "abcdefg".to_string();

    let c = Cursor::new(&line);

    for (i, x) in line.char_indices() {
        assert_eq!(c.peek_nth(i), Some(x));
    }
}

#[test]
fn peek_while_test() {
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

#[test]
fn index_advance_test() {
    let mut cursor = Cursor::new("0123456789");

    assert_eq!(cursor.index(), 0);

    while !cursor.eol() {
        assert_eq!(
            cursor.index(),
            cursor.value().unwrap().to_digit(10).unwrap() as usize
        );
        cursor.advance();
    }

    assert!(cursor.eol());
    assert_eq!(cursor.index(), 9);

    cursor.advance();
    cursor.advance();

    assert!(cursor.eol());
    assert_eq!(cursor.index(), 9);
}

#[test]
fn index_advance_by_test() {
    let mut cursor = Cursor::new("This is a test string");
    //                            0    5  7 9    14    EOL

    let test = |cursor: &Cursor, i: usize, c: char| {
        assert_eq!(cursor.index(), i);
        assert_eq!(cursor.value(), Some(c));
    };

    test(&cursor, 0, 'T');
    cursor.advance_by(5);
    test(&cursor, 5, 'i');
    cursor.advance_by(3);
    test(&cursor, 8, 'a');
    cursor.advance_by(2);
    test(&cursor, 10, 't');
    cursor.advance_by(5);
    test(&cursor, 15, 's');
    cursor.advance_by(6);

    assert!(cursor.eol());
    assert_eq!(cursor.index(), 20);

    cursor.advance_by(10);

    assert!(cursor.eol());
    assert_eq!(cursor.index(), 20);
}
