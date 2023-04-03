use clap::Parser;
use nexus_rs::{
    filereader::FileReader,
    scanner::{Scanner, Tokens},
};
use rustyline::DefaultEditor;
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
            eprintln!("Failed to open file");
            exit(1);
        };

        let mut scanner = Scanner::new();

        print_tokens(file.into_iter().fold(Tokens::new(), |mut acc, line| {
            acc.append(&mut scanner.scan(line));
            acc
        }));
    } else {
        let mut rl = DefaultEditor::new().expect("failed to create REPL");

        loop {
            let line = rl.readline("> ").expect("failed to parse input");
            print_tokens(Scanner::new().scan(line));
        }
    }
}

fn print_tokens(tokens: Tokens) {
    tokens.into_iter().for_each(|t| print!("{:?} ", t));
    println!();
}
