use std::{fmt, process};

use colored::Colorize;

/// Options to customize the behavior of [`fatal`].
pub struct FatalOptions {
    prefix: String,
    specifier: Option<String>,
    message: String,
}

impl FatalOptions {
    /// Create a new instance of [`Fatal`] with the given `message`.
    pub fn new(message: impl Into<String>) -> Self {
        let mut options = Self::default();
        options.message = message.into();
        options
    }

    /// Modifies the prefix to the error message.
    ///
    /// Example - PREFIX: <message>
    #[allow(dead_code)]
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Adds a specifier to the error message.
    ///
    /// Example - error(SPECIFIER): <message>
    pub fn specifier(mut self, specifier: impl Into<String>) -> Self {
        self.specifier = Some(specifier.into());
        self
    }
}

impl Default for FatalOptions {
    fn default() -> Self {
        Self {
            prefix: "error".to_string(),
            specifier: Default::default(),
            message: Default::default(),
        }
    }
}

impl<S> From<S> for FatalOptions
where
    S: Into<String>,
{
    fn from(other: S) -> Self {
        Self::new(other)
    }
}

pub trait Fatal {
    type Output;

    fn fatal(self, options: FatalOptions) -> Self::Output;
}

impl<T, E: fmt::Display> Fatal for Result<T, E> {
    type Output = T;

    fn fatal(self, mut options: FatalOptions) -> Self::Output {
        match self {
            Ok(value) => value,
            Err(error) => {
                options.message = error.to_string();
                fatal(options);
            }
        }
    }
}

/// Print an error message and exit the program.
pub fn fatal(options: FatalOptions) -> ! {
    let prefix = format!(
        "{}{}:",
        options.prefix,
        options
            .specifier
            .map(|ps| format!("({})", ps))
            .unwrap_or("".into()),
    );
    eprintln!("{} {}", prefix.bright_red().bold(), options.message);
    process::exit(1);
}
