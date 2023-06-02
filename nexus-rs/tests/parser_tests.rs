use std::fs;

use nexus_rs::{parser::Parser, scanner::Scanner};

const CODE_PATH: &str = "tests/test_code/test.nxs";

#[test]
fn parser_test() {
    let code = fs::read_to_string(CODE_PATH)
        .unwrap_or_else(|e| panic!("{e}"))
        .trim()
        .to_owned();

    let mut scanner = Scanner::new();
    let mut parser = Parser::new(scanner.scan(code).unwrap());

    println!("{}", parser.parse());
}
