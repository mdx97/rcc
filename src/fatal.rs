use std::process;

use colored::Colorize;

/// Used to print an error message and exit the program.
pub struct Fatal {
    prefix: String,
    prefix_specifier: Option<String>,
    message: String,
}

impl Fatal {
    /// Create a new instance of [`Fatal`].
    pub fn new(message: String) -> Self {
        Self {
            prefix: "error".to_string(),
            prefix_specifier: None,
            message,
        }
    }

    /// Modifies the prefix to the error message.
    ///
    /// Example - PREFIX: <message>
    #[allow(dead_code)]
    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }

    /// Adds a prefix specifier to the error message.
    ///
    /// Example - error(SPECIFIER): <message>
    pub fn with_prefix_specifier(mut self, prefix_specifier: Option<String>) -> Self {
        self.prefix_specifier = prefix_specifier;
        self
    }

    /// Print the message and exit the program.
    pub fn exit(self) -> ! {
        let prefix = format!(
            "{}{}:",
            self.prefix,
            self.prefix_specifier
                .map(|ps| format!("({})", ps))
                .unwrap_or("".into()),
        );
        eprintln!("{} {}", prefix.bright_red().bold(), self.message);
        process::exit(1);
    }
}
