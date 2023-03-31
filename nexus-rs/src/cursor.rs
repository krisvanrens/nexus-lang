use std::{iter::Peekable, str::Chars};

// TODO: Docs..

#[derive(Debug)]
pub struct Cursor<'a> {
    iter: Peekable<Chars<'a>>,
    value: Option<char>,
}

impl<'a> Cursor<'a> {
    pub fn new(line: &'a str) -> Self {
        let mut iter = line.chars();
        let value = iter.next();

        Cursor {
            iter: iter.peekable(),
            value: value,
        }
    }

    pub fn value(&self) -> Option<char> {
        self.value
    }

    pub fn advance(&mut self) {
        self.value = self.iter.next();
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    pub fn peek_word(&mut self) -> Option<String> {
        if !self.eol() && self.value.unwrap().is_alphanumeric() {
            let mut result = self.value.unwrap().to_string();

            result += &self
                .iter
                .clone()
                .take_while(|x| x.is_alphanumeric())
                .collect::<String>();

            Some(result)
        } else {
            None
        }
    }

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

    assert!(c1.value() == None);
    assert!(c2.value() == Some('T'));
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
        assert!(c.value().unwrap() == i);
        c.advance();
    }

    assert!(c.eol());
}

#[test]
fn cursor_peek_test() {
    let line = "abcdefg".to_string();

    let mut c = Cursor::new(&line);

    for i in "bcdefg".chars() {
        assert!(c.peek().unwrap() == &i);
        c.advance();
    }

    assert!(!c.eol());

    c.advance();

    assert!(c.eol());
}

#[test]
fn cursor_peek_word_test() {
    let line = "abc def".to_string();

    let mut c = Cursor::new(&line);

    assert!(!c.eol());
    assert!(c.value() == Some('a'));

    assert!(c.peek_word() == Some("abc".to_string()));

    assert!(!c.eol());
    assert!(c.value() == Some('a'));

    c.advance();
    c.advance();
    c.advance();

    assert!(!c.eol());
    assert!(c.value() == Some(' '));

    c.advance();

    assert!(!c.eol());
    assert!(c.value() == Some('d'));

    assert!(c.peek_word() == Some("def".to_string()));

    assert!(!c.eol());
    assert!(c.value() == Some('d'));
}
