/// Module group for utilities.
pub mod utils {
    /// File reader based on a buffered, line-by-line file reader.
    pub mod filereader;
}

/// Module group for lexing token-related items.
pub mod nxs_token {
    /// Scanning/lexing token representations.
    pub mod token;
}

/// Module group for AST (Abstract Syntax Tree)-related items.
pub mod nxs_ast {
    /// AST definitions for Nexus.
    pub mod ast;

    /// Pointer-wrapper used in the AST.
    pub mod ptr;
}

/// Module group for lexing/scanner-related items.
pub mod nxs_scanner {
    /// Scanner/lexer for Nexus.
    pub mod scanner;

    /// Line of source code.
    pub mod source_line;

    /// Character-based cursor used for scanning/lexing.
    pub mod cursor;
}

/// Module group for parsing-related items.
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
pub use utils::*;
