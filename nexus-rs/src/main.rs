use clap::Parser;
use nexus_rs::{filereader::FileReader, scanner::Scanner, token::Tokens};
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
        let Ok(file) = FileReader::try_new(filename) else {
            eprintln!("failed to open file");
            exit(1);
        };

        let mut scanner = Scanner::new();

        print_tokens(file.into_iter().fold(Tokens::new(), |mut acc, line| {
            acc.append(&mut scanner.scan(line).unwrap()); // TODO: Handle scanning error.
            acc
        }));
    } else {
        let Ok(mut rl) = DefaultEditor::new() else {
            eprintln!("failed to create REPL interface");
            exit(1);
        };

        loop {
            let line = rl.readline("> ");
            match line {
                Ok(line) => print_tokens(Scanner::new().scan(line).unwrap()), // TODO: Handle scanning error.
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
}

fn print_tokens(tokens: Tokens) {
    tokens.into_iter().for_each(|t| print!("{:?} ", t));
    println!();
}
