mod fatal;
mod lexer;

use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};
use fatal::{fatal, Fatal};
use lexer::lex;

use crate::fatal::FatalOptions;

fn main() {
    let matches = Command::new("rcc")
        .version("1.0")
        .author("Mathew H. <mathewhorner456@gmail.com>")
        .about("A C compiler written in Rust.")
        .arg(Arg::new("FILES").required(true).action(ArgAction::Append))
        .get_matches();

    let files: Vec<String> = matches
        .get_many::<String>("FILES")
        .unwrap()
        .map(ToString::to_string)
        .collect();

    let files = validate_files(&files);

    // TEMP: Just lex the first file for now.
    let file = files.into_iter().next().unwrap();
    let tokens = lex(file).fatal(FatalOptions::default().with_specifier("lexer"));

    println!("TOKENS: {:?}", tokens);
}

/// Validate that the given list of source file names can be compiled and return
/// them as a list of [`PathBuf`]s.
fn validate_files(files: &Vec<String>) -> Vec<PathBuf> {
    files
        .iter()
        .map(|file| {
            let mut path = PathBuf::new();
            path.push(file);

            if !path.exists() {
                fatal(format!("no file found with the name {}", file).into());
            }
            if !file.ends_with(".c") {
                fatal(format!("file with the name {} does not end with .c", file).into());
            }
            path
        })
        .collect()
}
