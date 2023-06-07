use nexus_rs::{scanner::Scanner, token::Token};

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn token_test() {
    let test = |input: &str, expected: Token| {
        let mut s = Scanner::new();

        match s.scan(input.to_string()) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 1);
                assert_eq!(tokens.into_iter().next().unwrap(), expected);
            }
            Err(e) => assert!(false, "error: {}", e),
        }
    };

    test("(", Token::LeftParen);
    test(")", Token::RightParen);
    test("{", Token::LeftBrace);
    test("}", Token::RightBrace);
    test("[", Token::LeftBracket);
    test("]", Token::RightBracket);
    test(":", Token::Colon);
    test(";", Token::SemiColon);
    test("&", Token::Amp);
    test("+", Token::Plus);
    test("-", Token::Minus);
    test("->", Token::Arrow);
    test("*", Token::Star);
    test("/", Token::Slash);
    test("\\", Token::BackSlash);
    test("%", Token::Percent);
    test(",", Token::Comma);
    test(".", Token::Dot);
    test("..", Token::Range);
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
    //test("||", Token::Or);           // NOTE: Selected in token stream post-processing.
    //test("||", Token::EmptyClosure); // NOTE: Selected in token stream post-processing.
    test("|", Token::Pipe);
    test("true", Token::True);
    test("false", Token::False);
    test("const", Token::Const);
    test("let", Token::Let);
    test("mut", Token::Mut);
    test("fn", Token::Function);
    test("if", Token::If);
    test("else", Token::Else);
    test("for", Token::For);
    test("in", Token::In);
    test("while", Token::While);
    test("return", Token::Return);
    test("use", Token::Use);
    test("bool", Token::BoolId);
    test("Group", Token::GroupId);
    test("Node", Token::NodeId);
    test("Number", Token::NumberId);
    test("String", Token::StringId);
    test("print", Token::Print);
    test("node", Token::Node);
    test("group", Token::Group);

    test("2.8539", Token::Number(2.8539f64));
    test("top_id", Token::Identifier("top_id".to_string()));
    test("\"Hi\"", Token::String("Hi".to_string()));
}
