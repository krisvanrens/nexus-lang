use super::cursor::Cursor;
use super::source_line::SourceLine;
use std::fmt::Display;
use thiserror::Error;

/// Scanning/lexing error representation.
#[derive(Error, Debug)]
pub enum ScanErrorKind {
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

#[derive(Error, Debug)]
pub struct ScanError {
    line: SourceLine,
    kind: ScanErrorKind,
    char_index: usize,
}

impl Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Simplify?
        let prefix_fill = " ".repeat(
            self.line
                .number
                .map_or("".to_owned(), |n| n.to_string())
                .len()
                + 2,
        ); // +2 for spaces.
        let char_fill = " ".repeat(self.char_index);
        f.write_fmt(format_args!(
            "{}|\n {} | {}\n{}| {}{}\n{}| error: {}\n{}|",
            prefix_fill,
            self.line.number.map_or("".to_owned(), |n| n.to_string()),
            self.line.line,
            prefix_fill,
            char_fill,
            "^",
            prefix_fill,
            self.kind,
            prefix_fill,
        ))
    }
}

impl ScanError {
    pub fn new(line: SourceLine, kind: ScanErrorKind, cursor: &Cursor) -> Self {
        ScanError {
            line,
            kind,
            char_index: cursor.index(),
        }
    }
}
