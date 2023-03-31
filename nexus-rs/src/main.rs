use std::process::exit;

use clap::Parser;
use nexus_rs::{
    filereader::FileReader,
    scanner::{Scanner, Tokens},
};

/// Nexus programming language interpreter.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input source filename (omit for REPL).
    #[clap(short, long, default_value = "")]
    filename: String,
}

fn main() {
    let args = Args::parse();

    if !args.filename.is_empty() {
        let Ok(file) = FileReader::try_new("test.nxs") else {
            eprintln!("Failed to open file");
            exit(1);
        };

        let mut scanner = Scanner::new();

        let tokens = file.into_iter().fold(Tokens::new(), |mut acc, line| {
            acc.append(&mut scanner.scan(line));
            acc
        });

        // XXX
        tokens.into_iter().for_each(|t| print!("{:?} ", t));
        println!();
    } else {
        // TODO
        eprintln!("REPL not implemented yet!");
    }
}
