use nexus_rs::{
    scanner::{Scanner, Tokens},
    token::Token,
};
use std::iter::zip;

#[test]
fn primitive_test() {
    let test = |input: &str, expected: Token| {
        let mut s = Scanner::new();

        let tokens = s.scan(input.to_string());

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.into_iter().next().unwrap(), expected);
    };

    let test_set = |input: &str, expected: Tokens| {
        let mut s = Scanner::new();

        let tokens = s.scan(input.to_string());

        assert_eq!(tokens.len(), expected.len());
        assert_eq!(
            expected.len(),
            zip(tokens, expected).filter(|(a, b)| { a == b }).count()
        );
    };

    test("(", Token::LeftParen);
    test(")", Token::RightParen);
    test("{", Token::LeftBrace);
    test("}", Token::RightBrace);
    test("[", Token::LeftBracket);
    test("]", Token::RightBracket);
    test(";", Token::SemiColon);
    test("+", Token::Plus);
    test("-", Token::Minus);
    test("->", Token::Arrow);
    test("*", Token::Star);
    test("/", Token::Slash);
    test("\\", Token::BackSlash);
    test("%", Token::Percent);
    test(",", Token::Comma);
    test(".", Token::Dot);
    test("_", Token::Underscore);
    test("=", Token::Is);
    test("==", Token::Eq);
    test(">", Token::Gt);
    test(">=", Token::GtEq);
    test("<", Token::Lt);
    test("<=", Token::LtEq);
    test("!", Token::Bang);
    test("!=", Token::NotEq);
    test("&&", Token::And);
    test("||", Token::Or);
    test("|", Token::Pipe);
    test("true", Token::True);
    test("false", Token::False);
    test("let", Token::Let);
    test("fn", Token::Function);
    test("if", Token::If);
    test("for", Token::For);
    test("while", Token::While);
    test("return", Token::Return);
    test("print", Token::Print);
    test("node", Token::Node);

    test_set("||{", vec![Token::EmptyClosure, Token::LeftBrace]);

    test("2.8539", Token::Number(2.8539f64));
    test("top_id", Token::Identifier("top_id".to_string()));
    test("\"Hi\"", Token::String("Hi".to_string()));
}
