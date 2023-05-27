/// File reader based on a buffered, line-by-line file reader.
pub mod filereader;

/// Scanning/lexing token representations.
pub mod token;

pub mod ast {
    /// AST definitions for Nexus.
    pub mod ast;

    /// Pointer-wrapper used in the AST.
    pub mod ptr;
}

pub mod scanner {
    /// Scanner/lexer for Nexus.
    pub mod scanner;

    /// Character-based cursor used for scanning/lexing.
    pub mod cursor;
}

pub mod parser {
    /// Parser for Nexus.
    pub mod parser;

    /// Token cursor used for parsing.
    pub mod token_cursor;
}
