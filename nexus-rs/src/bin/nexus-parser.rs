use clap::Parser;
use colored::Colorize;
use nexus_rs::{filereader::FileReader, parser, scanner, token::Tokens};
use std::process::exit;

/// Nexus programming language scanner/lexer tester.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input source filename.
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let Ok(file) = FileReader::try_new(args.filename) else {
        exit(1);
    };

    let mut scanner = scanner::Scanner::new();

    let mut parser = parser::Parser::new(file.into_iter().enumerate().fold(
        Tokens::new(),
        |mut acc, line| {
            let (index, line) = line;
            match scanner.scan(line) {
                Ok(mut result) => acc.append(&mut result),
                Err(error) => eprintln!("line {}: {error:?}", index + 1),
            }

            acc
        },
    ));

    // TODO: Chunk and print properly..
    parser.parse().into_iter().for_each(|n| println!("{n}"));
}