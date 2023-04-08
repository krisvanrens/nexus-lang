use clap::Parser;
use colored::Colorize;
use nexus_rs::{filereader::FileReader, scanner::Scanner};
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

    let mut s = Scanner::new();

    for line in file {
        println!(
            "{} {}: '{}'",
            "==".yellow().bold(),
            "Scan line".bold(),
            line.to_string().bright_red().dimmed()
        );
        match s.scan(line) {
            Ok(tokens) => tokens.into_iter().for_each(|t| print!("{t:?}")),
            Err(error) => eprint!("Error({error:?})"),
        }
        println!();
    }
}
