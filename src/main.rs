mod fatal;
mod lexer;

use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};
use fatal::Fatal;
use lexer::lex;

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
    let tokens = lex(file)
        .map_err(|error| {
            Fatal::new(error.to_string())
                .with_prefix_specifier(Some("lexer".to_string()))
                .exit();
        })
        .unwrap();

    println!("TOKENS: {:?}", tokens);
}

/// Validate that the given list of source files can be compiled and return them
/// as a list of [`PathBuf`]s.
fn validate_files(files: &Vec<String>) -> Vec<PathBuf> {
    files
        .iter()
        .map(|file| {
            let mut path = PathBuf::new();
            path.push(file);

            if !path.exists() {
                Fatal::new(format!("No file found with the name \"{}\"!", file)).exit();
            }
            if !file.ends_with(".c") {
                Fatal::new(format!(
                    "File with the name \"{}\" does not end with \".c\"!",
                    file
                ));
            }
            path
        })
        .collect()
}
