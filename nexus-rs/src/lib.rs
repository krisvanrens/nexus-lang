/// File reader based on a buffered, line-by-line file reader.
pub mod filereader;

pub mod nxs_token {
    /// Scanning/lexing token representations.
    pub mod token;
}

pub mod nxs_ast {
    /// AST definitions for Nexus.
    pub mod ast;

    /// Pointer-wrapper used in the AST.
    pub mod ptr;
}

pub mod nxs_scanner {
    /// Scanner/lexer for Nexus.
    pub mod scanner;

    /// Character-based cursor used for scanning/lexing.
    pub mod cursor;
}

pub mod nxs_parser {
    /// Parser for Nexus.
    pub mod parser;

    /// Token cursor used for parsing.
    pub mod token_cursor;
}

pub use nxs_ast::*;
pub use nxs_parser::*;
pub use nxs_scanner::*;
pub use nxs_token::*;
