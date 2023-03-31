use std::{iter::Peekable, str::Chars};

// TODO: Docs..
// TODO: Tests..

#[derive(Debug)]
pub struct Cursor<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Cursor<'a> {
    pub fn new(line: &'a str) -> Self {
        Cursor {
            iter: line.chars().peekable(),
        }
    }

    pub fn value(&self) -> Option<&char> {
        // TODO

        Some(&'x')
    }

    pub fn advance(&mut self) {
        self.iter.next();
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    pub fn peek_word(&mut self) -> Option<String> {
        // TODO

        Some("".to_string())
    }

    pub fn eol(&mut self) -> bool {
        self.iter.peek().is_none()
    }
}

// TODO: How to get current value???
