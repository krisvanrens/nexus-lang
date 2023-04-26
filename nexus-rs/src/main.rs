use clap::Parser;
use nexus_rs::{filereader::*, token::*, *};
use rustyline::{error::ReadlineError, DefaultEditor};
use std::process::exit;

/// Nexus programming language interpreter.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input source filename (omit for REPL).
    #[arg(short, long)]
    filename: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(filename) = args.filename {
        run_from_file(filename);
    } else {
        run_repl();
    }
}

fn run_from_file(filename: String) {
    let Ok(file) = FileReader::try_new(filename) else {
        eprintln!("failed to open file");
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

    println!("{:?}", parser.parse());
}

fn run_repl() {
    let Ok(mut rl) = DefaultEditor::new() else {
        eprintln!("failed to create REPL interface");
        exit(1);
    };

    loop {
        match rl.readline("> ") {
            Ok(line) => match scanner::Scanner::new().scan(line) {
                Ok(tokens) => {
                    println!("{:?}", parser::Parser::new(tokens).parse());
                }
                Err(error) => eprintln!("{error:?}"),
            },
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => {
                eprintln!("interrupted");
                break; // TODO: Do something else?
            }
            _ => {
                eprintln!("failed to parse input");
                exit(1);
            }
        }
    }
}
