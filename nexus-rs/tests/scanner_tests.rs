use nexus_rs::{
    scanner::{Scanner, Tokens},
    token::Token,
};

#[test]
fn primitive_test() {
    let test_single = |input: &str, expected: Token| {
        let mut s = Scanner::new();

        let tokens = s.scan(input.to_string());

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.into_iter().next().unwrap(), expected);
    };

    let test_multiple = |input: &str, expected: Tokens| {
        let mut s = Scanner::new();

        let tokens = s.scan(input.to_string());

        assert_eq!(tokens.len(), expected.len());

        // TODO: Compare collections!
    };

    test_single("(", Token::LeftParen);
    test_single(")", Token::RightParen);
    test_single("{", Token::LeftBrace);
    test_single("}", Token::RightBrace);
    test_single("[", Token::LeftBracket);
    test_single("]", Token::RightBracket);
    test_single(";", Token::SemiColon);
    test_single("+", Token::Plus);
    test_single("-", Token::Minus);
    test_single("->", Token::Arrow);
    test_single("*", Token::Star);
    test_single("/", Token::Slash);
    test_single("\\", Token::BackSlash);
    test_single("%", Token::Percent);
    test_single(",", Token::Comma);
    test_single(".", Token::Dot);
    test_single("_", Token::Underscore);
    test_single("=", Token::Is);
    test_single("==", Token::Eq);
    test_single(">", Token::Gt);
    test_single(">=", Token::GtEq);
    test_single("<", Token::Lt);
    test_single("<=", Token::LtEq);
    test_single("!", Token::Bang);
    test_single("!=", Token::NotEq);
    test_single("&&", Token::And);
    test_single("||", Token::Or);
    test_single("|", Token::Pipe);
    test_single("true", Token::True);
    test_single("false", Token::False);
    test_single("let", Token::Let);
    test_single("fn", Token::Function);
    test_single("if", Token::If);
    test_single("for", Token::For);
    test_single("while", Token::While);
    test_single("return", Token::Return);
    test_single("print", Token::Print);
    test_single("node", Token::Node);

    test_multiple("||{", vec![Token::EmptyClosure, Token::LeftBrace]);

    // TODO: Test data-bearing types.
    //Number(f64),
    //Identifier(String),
    //String(String),
}
