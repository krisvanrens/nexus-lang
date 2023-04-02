use clap::Parser;
use colored::Colorize;
use nexus_rs::{filereader::FileReader, scanner::Scanner};
use std::process::exit;

/// Nexus programming language scanner/lexer tester.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input source filename.
    #[clap(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let Ok(file) = FileReader::try_new(args.filename) else {
        exit(1);
    };

    let mut s = Scanner::new();

    for line in file {
        println!("{} {}: '{}'", "==".yellow().bold(), "Scan line".bold(), format!("{line}").bright_red().dimmed());
        s.scan(line).into_iter().for_each(|t| print!("{:?} ", t));
        println!();
    }
}
