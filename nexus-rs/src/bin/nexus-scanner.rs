use clap::Parser;
use colored::Colorize;
use nexus_rs::{filereader::FileReader, scanner::Scanner, source_line::SourceLine};
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

    let mut s = Scanner::new();

    for (number, line) in file.into_iter().enumerate() {
        println!(
            "{} {}: '{}'",
            "==".yellow().bold(),
            "Scan line".bold(),
            line.to_string().bright_red().dimmed()
        );
        match s.scan(SourceLine { line, number }) {
            Ok(tokens) => tokens.into_iter().for_each(|t| print!("{t:?} ")),
            Err(error) => eprint!("{error}"),
        }
        println!();
    }
}
