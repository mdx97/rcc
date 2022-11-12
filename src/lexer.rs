use std::fs::read_to_string;
use std::path::PathBuf;

/// A lexical token
#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
    Symbol(Symbol),
}

/// A keyword (`int`, `return`, `void`, etc)
#[derive(Debug)]
pub enum Keyword {
    Int,
}

/// A literal value (420, "hello world", etc)
#[derive(Debug)]
pub enum Literal {
    Integer(i64),
    String(String),
}

/// A symbol (parentheses, brackets, etc)
#[derive(Debug)]
pub enum Symbol {
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    SquareBracketOpen,
    SquareBracketClose,
}

/// The [`Result`] of calling the [`lex`] function.
pub type LexResult<T> = Result<T, LexError>;

/// An error that could be produced by the [`lex`] function.
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
    /// Parse the current contents of the buffer into a [`Token`].
    fn parse(lexer: &Lexer) -> LexResult<Token> {
        match lexer.buffer.as_str() {
            "int" => Ok(Token::Keyword(Keyword::Int)),
            _ => Err(LexError::InvalidToken {
                token: lexer.buffer.clone(),
                column: lexer.column - lexer.buffer.len() as u64,
                line: lexer.line,
            }),
        }
    }
}

/// Tracks state for a run of the [`lex`] function.
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

    /// Try to parse a new token with the contents of the buffer and then clear it.
    fn pop(&mut self) -> LexResult<()> {
        self.tokens.push(Token::parse(self)?);
        self.buffer.clear();
        Ok(())
    }

    /// Add a character to the buffer.
    fn push(&mut self, c: char) {
        self.buffer.push(c);
        self.column += 1;
        if c == '\n' {
            self.column = 0;
            self.line += 1;
        }
    }

    /// Try to parse what remains in the buffer before lexical analysis is done.
    fn finalize(&mut self) -> LexResult<()> {
        if !self.buffer.is_empty() {
            self.pop()?;
        }
        Ok(())
    }
}

/// Perform lexical analysis on the given file.
pub fn lex(file: PathBuf) -> LexResult<Vec<Token>> {
    let contents = read_to_string(file)?;
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
