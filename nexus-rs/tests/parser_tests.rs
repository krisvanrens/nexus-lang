use std::{ffi::OsStr, fs, path::Path};

use nexus_rs::{parser::Parser, scanner::Scanner};

const CODE_PATH: &str = "tests/test_code/";

/// Scan `CODE_PATH` for Nexus source files, and run scanner + parser for each of them.
#[test]
fn parser_test() {
    for entry in fs::read_dir(CODE_PATH).unwrap_or_else(|e| panic!("{e}")) {
        let filename = entry.expect("invalid directory entry").file_name();
        if Path::new(&filename).extension().and_then(OsStr::to_str) == Some("nxs") {
            let code = fs::read_to_string(CODE_PATH.to_owned() + filename.to_str().unwrap())
                .unwrap_or_else(|e| panic!("{e}"))
                .trim()
                .to_owned();

            println!("Parsing {filename:?}..");

            let mut scanner = Scanner::new();
            let mut parser = Parser::new(scanner.scan(code).unwrap());
            parser.parse();

            println!("..done");
        }
    }
}
