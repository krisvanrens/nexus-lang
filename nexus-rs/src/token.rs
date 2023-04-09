/// Scanning/lexing token representation used in the Nexus grammar.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LeftParen,    // '('
    RightParen,   // ')'
    LeftBrace,    // '{'
    RightBrace,   // '}'
    LeftBracket,  // '['
    RightBracket, // ']'
    SemiColon,    // ';'
    Plus,         // '+'
    Minus,        // '-'
    Arrow,        // '->'
    Star,         // '*'
    Slash,        // '/'
    BackSlash,    // '\'
    Percent,      // '%'
    Comma,        // ','
    Dot,          // '.'
    Range,        // '..'
    Underscore,   // '_'
    Is,           // '='
    Eq,           // '=='
    Gt,           // '>'
    GtEq,         // '>='
    Lt,           // '<'
    LtEq,         // '<='
    Bang,         // '!'
    NotEq,        // '!='
    And,          // '&&'
    Or,           // '||'
    EmptyClosure, // '||'
    Pipe,         // '|'
    True,         // 'true'
    False,        // 'false'
    Let,          // 'let'
    Function,     // 'fn'
    If,           // 'if'
    For,          // 'for'
    While,        // 'while'
    Return,       // 'return'
    Print,        // 'print'
    Node,         // 'node'
    Group,        // 'group'
    Number(f64),
    Identifier(String),
    String(String),
}

/// Collection of tokens.
pub type Tokens = Vec<Token>;
