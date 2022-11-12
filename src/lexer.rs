#![allow(dead_code)]

use std::fs::read_to_string;
use std::path::PathBuf;

/// A lexical token
#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
    Symbol(Symbol),
}

/// A keyword (`int`, `return`, `void`, etc)
#[derive(Debug, Eq, PartialEq)]
pub enum Keyword {
    Char,
    Int,
    Void,
}

/// A literal value (420, "hello world", etc)
#[derive(Debug, Eq, PartialEq)]
pub enum Literal {
    Integer(i64),
    String(String),
}

/// A symbol (parentheses, brackets, etc)
#[derive(Debug, Eq, PartialEq)]
pub enum Symbol {
    Equals,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    SemiColon,
    SquareBracketOpen,
    SquareBracketClose,
}

/// Type alias for a [`Result`] with an error of type [`LexError`].
pub type LexResult<T> = Result<T, LexError>;

/// An error that could be produced during lexical analysis.
#[derive(thiserror::Error, Debug)]
pub enum LexError {
    #[error("invalid token encountered at line {line}, column {column}: {token}")]
    InvalidToken {
        token: String,
        column: u64,
        line: u64,
    },

    #[error("could not read or write a file, see logs for more details")]
    Io(#[from] std::io::Error),
}

impl Token {
    /// Try to parse the current contents of the `lexer`s buffer into a [`Token`].
    fn parse(lexer: &Lexer) -> LexResult<Token> {
        match lexer.buffer.as_str() {
            "char" => Ok(Token::Keyword(Keyword::Char)),
            "int" => Ok(Token::Keyword(Keyword::Int)),
            "void" => Ok(Token::Keyword(Keyword::Void)),
            _ => Err(LexError::InvalidToken {
                token: lexer.buffer.clone(),
                column: lexer.column - lexer.buffer.len() as u64,
                line: lexer.line,
            }),
        }
    }
}

/// Tracks state for a lexical analysis run.
struct Lexer {
    buffer: String,
    column: u64,
    line: u64,
    tokens: Vec<Token>,
}

impl Lexer {
    /// Create a new instance of [`Lexer`].
    fn new() -> Self {
        Self {
            buffer: String::new(),
            column: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    /// Try to parse a new [`Token`] with the contents of the `buffer` and then clear it.
    fn pop(&mut self) -> LexResult<()> {
        self.tokens.push(Token::parse(self)?);
        self.buffer.clear();
        Ok(())
    }

    /// Add the `character` to the buffer.
    fn push(&mut self, character: char) {
        self.buffer.push(character);
        self.column += 1;
        if character == '\n' {
            self.column = 0;
            self.line += 1;
        }
    }

    /// Try to parse what remains in the buffer so lexical analysis can finish.
    fn finalize(&mut self) -> LexResult<()> {
        if !self.buffer.is_empty() {
            self.pop()?;
        }
        Ok(())
    }
}

/// Perform lexical analysis on the given file.
pub fn lex(file: PathBuf) -> LexResult<Vec<Token>> {
    lex_contents(read_to_string(file)?)
}

/// Perform lexical analysis on the given file contents.
fn lex_contents(contents: String) -> LexResult<Vec<Token>> {
    let mut state = Lexer::new();

    for c in contents.chars() {
        if c == ' ' || c == '\n' {
            state.pop()?;
        } else if c == '(' {
            state.pop()?;
            state.push(c);
        } else {
            state.push(c);
        }
    }

    state.finalize()?;
    Ok(state.tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lex_single(string: &str, token: Token) {
        let result = lex_contents(string.to_string());
        assert!(
            result.is_ok(),
            "lexical analysis returned non-OK result for '{}' and should have returned a single token {:?} ({:?})",
            string,
            token,
            result.unwrap_err(),
        );
        assert_eq!(result.unwrap(), vec![token]);
    }

    fn test_lex(string: &str, tokens: Vec<Token>) {
        let result = lex_contents(string.to_string());
        assert!(
            result.is_ok(),
            "lexical analysis returned non-OK result for '{}' and should have returned a list of tokens {:?} ({:?})",
            string,
            tokens,
            result.unwrap_err(),
        );
        assert_eq!(result.unwrap(), tokens);
    }

    #[test]
    fn lexical_analysis_works_on_single_tokens() {
        test_lex_single("char", Token::Keyword(Keyword::Char));
        test_lex_single("int", Token::Keyword(Keyword::Int));
        test_lex_single("void", Token::Keyword(Keyword::Void));
    }

    #[test]
    fn lexical_analysis_works_on_assignment_to_literal() {
        test_lex(
            "int foo = 5;",
            vec![
                Token::Keyword(Keyword::Int),
                Token::Identifier("foo".to_string()),
                Token::Symbol(Symbol::Equals),
                Token::Literal(Literal::Integer(5)),
                Token::Symbol(Symbol::SemiColon),
            ],
        )
    }
}
