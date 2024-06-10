use clap::Parser;
use colored::Colorize;
use nexus_rs::{filereader::FileReader, parser, scanner, source_line::SourceLine, token::Tokens};
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

    let file = FileReader::try_new(args.filename).unwrap_or_else(|e| {
        eprintln!("Failed to open file: {e}");
        exit(1);
    });

    let mut scanner = scanner::Scanner::new();
    let mut scan_error = false;

    let mut parser = parser::Parser::new(file.into_iter().enumerate().fold(
        Tokens::new(),
        |mut acc, line| {
            let (number, line) = line;
            match scanner.scan(SourceLine {
                line,
                number: Some(number + 1),
            }) {
                Ok(mut result) => acc.append(&mut result),
                Err(error) => {
                    scan_error = true;
                    eprintln!("line {}: {error:?}", number + 1)
                }
            }

            acc
        },
    ));

    if scan_error {
        eprintln!("scanning failed, aborting");
        return;
    }

    match parser.parse() {
        Ok(ast) => ast.iter().for_each(|n| {
            println!(
                "{} {}: {}",
                "==".yellow().bold(),
                "AST Node".bold(),
                n.to_string().bright_red().dimmed()
            )
        }),
        Err(e) => eprintln!("{e:?}"),
    }
}
