use clap::Parser;
use nexus_rs::{filereader::*, *};
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
    let file = FileReader::try_new(&filename).unwrap_or_else(|e| {
        eprintln!("Failed to open file: {e}");
        exit(1);
    });

    let mut scanner = scanner::Scanner::new();
    let mut scan_error = false;

    let mut parser = parser::Parser::new(file.into_iter().enumerate().fold(
        token::Tokens::new(),
        |mut acc, line| {
            let (number, line) = line;
            match scanner.scan(source_line::SourceLine { line, number }) {
                Ok(mut result) => acc.append(&mut result),
                Err(error) => {
                    scan_error = true;

                    eprintln!("  ---> {filename}:{number}");
                    eprintln!("{error}");
                }
            }

            acc
        },
    ));

    if scan_error {
        eprintln!("scanning failed, aborting");
        return;
    }

    println!("{}", parser.parse()); // XXX
}

fn run_repl() {
    let Ok(mut rl) = DefaultEditor::new() else {
        eprintln!("failed to create REPL interface");
        exit(1);
    };

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                rl.add_history_entry(line.clone())
                    .expect("failed to store line to history");
                match scanner::Scanner::new().scan(source_line::SourceLine { line, number: 0 }) {
                    Ok(tokens) => {
                        println!("{}", parser::Parser::new(tokens).parse()); // XXX
                    }
                    Err(error) => eprintln!("{error}"),
                }
            }
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
