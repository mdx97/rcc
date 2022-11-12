#![allow(dead_code)]

use std::fs::read_to_string;
use std::path::PathBuf;

use regex::Regex;

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
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Nullptr,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

/// A literal value (420, "hello world", etc)
#[derive(Debug, Eq, PartialEq)]
pub enum Literal {
    Char(char),
    Integer(i64),
    String(String),
}

/// A symbol (parentheses, brackets, etc)
#[derive(Debug, Eq, PartialEq)]
pub enum Symbol {
    Ampersand,
    Asterisk,
    BracketOpen,
    BracketClose,
    Comma,
    Colon,
    Equals,
    Hash,
    ParenOpen,
    ParenClose,
    Semicolon,
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
        Ok(match lexer.buffer.as_str() {
            "auto" => Token::Keyword(Keyword::Auto),
            "break" => Token::Keyword(Keyword::Break),
            "case" => Token::Keyword(Keyword::Case),
            "char" => Token::Keyword(Keyword::Char),
            "const" => Token::Keyword(Keyword::Const),
            "continue" => Token::Keyword(Keyword::Continue),
            "default" => Token::Keyword(Keyword::Default),
            "do" => Token::Keyword(Keyword::Do),
            "double" => Token::Keyword(Keyword::Double),
            "else" => Token::Keyword(Keyword::Else),
            "enum" => Token::Keyword(Keyword::Enum),
            "extern" => Token::Keyword(Keyword::Extern),
            "float" => Token::Keyword(Keyword::Float),
            "for" => Token::Keyword(Keyword::For),
            "goto" => Token::Keyword(Keyword::Goto),
            "if" => Token::Keyword(Keyword::If),
            "inline" => Token::Keyword(Keyword::Inline),
            "int" => Token::Keyword(Keyword::Int),
            "long" => Token::Keyword(Keyword::Long),
            "nullptr" => Token::Keyword(Keyword::Nullptr),
            "register" => Token::Keyword(Keyword::Register),
            "restrict" => Token::Keyword(Keyword::Restrict),
            "return" => Token::Keyword(Keyword::Return),
            "short" => Token::Keyword(Keyword::Short),
            "signed" => Token::Keyword(Keyword::Signed),
            "sizeof" => Token::Keyword(Keyword::Sizeof),
            "static" => Token::Keyword(Keyword::Static),
            "struct" => Token::Keyword(Keyword::Struct),
            "switch" => Token::Keyword(Keyword::Switch),
            "typedef" => Token::Keyword(Keyword::Typedef),
            "union" => Token::Keyword(Keyword::Union),
            "unsigned" => Token::Keyword(Keyword::Unsigned),
            "void" => Token::Keyword(Keyword::Void),
            "volatile" => Token::Keyword(Keyword::Volatile),
            "while" => Token::Keyword(Keyword::While),
            "&" => Token::Symbol(Symbol::Ampersand),
            "*" => Token::Symbol(Symbol::Asterisk),
            "{" => Token::Symbol(Symbol::BracketOpen),
            "}" => Token::Symbol(Symbol::BracketClose),
            "," => Token::Symbol(Symbol::Comma),
            ":" => Token::Symbol(Symbol::Colon),
            "=" => Token::Symbol(Symbol::Equals),
            "#" => Token::Symbol(Symbol::Hash),
            "(" => Token::Symbol(Symbol::ParenOpen),
            ")" => Token::Symbol(Symbol::ParenClose),
            ";" => Token::Symbol(Symbol::Semicolon),
            "[" => Token::Symbol(Symbol::SquareBracketOpen),
            "]" => Token::Symbol(Symbol::SquareBracketClose),
            _ => {
                if let Some(literal) = valid_literal(lexer.buffer.as_str()) {
                    return Ok(literal);
                }
                if let Some(ident) = valid_identifier(lexer.buffer.as_str()) {
                    return Ok(ident);
                }

                return Err(LexError::InvalidToken {
                    token: lexer.buffer.clone(),
                    column: lexer.column - lexer.buffer.len() as u64,
                    line: lexer.line,
                });
            }
        })
    }
}

fn if_valid(string: &str, pattern: &str, f: impl Fn() -> Token) -> Option<Token> {
    Regex::new(pattern).unwrap().is_match(string).then(f)
}

fn valid_literal(string: &str) -> Option<Token> {
    valid_integer_literal(string)
        .or(valid_string_literal(string))
        .or(valid_char_literal(string))
}

fn valid_integer_literal(string: &str) -> Option<Token> {
    const PATTERN: &str = "[0-9]+";
    if_valid(string, PATTERN, || {
        Token::Literal(Literal::Integer(string.parse().unwrap()))
    })
}

fn valid_string_literal(string: &str) -> Option<Token> {
    const PATTERN: &str = "\".*\"";
    if_valid(string, PATTERN, || {
        Token::Literal(Literal::String(string.into()))
    })
}

fn valid_char_literal(string: &str) -> Option<Token> {
    const PATTERN: &str = "'.*'";
    if_valid(string, PATTERN, || {
        Token::Literal(Literal::Char(string.parse().unwrap()))
    })
}

fn valid_identifier(string: &str) -> Option<Token> {
    const PATTERN: &str = "[a-zA-Z][a-zA-Z0-9]*";
    if_valid(string, PATTERN, || Token::Identifier(string.into()))
}

/// Tracks state for a lexical analysis run.
struct Lexer {
    buffer: String,
    column: u64,
    line: u64,
    tokens: Vec<Token>,
    context: LexerContext,
}

enum LexerContext {
    Normal,
    InString,
}

impl Lexer {
    /// Create a new instance of [`Lexer`].
    fn new() -> Self {
        Self {
            buffer: String::new(),
            column: 0,
            line: 1,
            tokens: Vec::new(),
            context: LexerContext::Normal,
        }
    }

    /// Process a character.
    fn process(&mut self, c: char) -> LexResult<()> {
        match self.context {
            LexerContext::Normal => {
                if c == ' ' || c == '\n' {
                    self.pop()?;
                } else if c == '(' || c == ')' || c == '{' || c == '}' || c == ';' || c == '*' {
                    self.pop()?;
                    self.push(c);
                    self.pop()?;
                } else if c == '"' {
                    self.context = LexerContext::InString;
                } else {
                    self.push(c);
                }
            }
            LexerContext::InString => {
                if c == '"' {
                    if self.buffer.len() > 0 {
                        self.tokens
                            .push(Token::Literal(Literal::String(self.buffer.clone())));
                        self.buffer.clear();
                    }
                    self.context = LexerContext::Normal;
                } else {
                    self.push(c);
                }
            }
        }
        Ok(())
    }

    /// Try to parse a new [`Token`] with the contents of the `buffer` and then clear it.
    fn pop(&mut self) -> LexResult<()> {
        if self.buffer.len() > 0 {
            self.tokens.push(Token::parse(self)?);
            self.buffer.clear();
        }
        Ok(())
    }

    /// Add the `character` to the buffer.
    fn push(&mut self, c: char) {
        self.buffer.push(c);
        self.column += 1;
        if c == '\n' {
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
    let mut lexer = Lexer::new();
    for c in contents.chars() {
        lexer.process(c)?;
    }

    lexer.finalize()?;
    Ok(lexer.tokens)
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
    fn lexical_analysis_works_on_assignment_to_integer_literal() {
        test_lex(
            "int foo = 5;",
            vec![
                Token::Keyword(Keyword::Int),
                Token::Identifier("foo".into()),
                Token::Symbol(Symbol::Equals),
                Token::Literal(Literal::Integer(5)),
                Token::Symbol(Symbol::Semicolon),
            ],
        );
    }

    #[test]
    fn lexical_analysis_works_on_assignemnt_to_string_literal() {
        test_lex(
            r#"const char *string = "Hello world and all who inhabit it!";"#,
            vec![
                Token::Keyword(Keyword::Const),
                Token::Keyword(Keyword::Char),
                Token::Symbol(Symbol::Asterisk),
                Token::Identifier("string".into()),
                Token::Symbol(Symbol::Equals),
                Token::Literal(Literal::String(
                    "Hello world and all who inhabit it!".into(),
                )),
                Token::Symbol(Symbol::Semicolon),
            ],
        );
    }

    #[test]
    fn lexical_analysis_works_on_function_declaration() {
        test_lex(
            r#"
            int main() {
                return 0;
            }
            "#,
            vec![
                Token::Keyword(Keyword::Int),
                Token::Identifier("main".into()),
                Token::Symbol(Symbol::ParenOpen),
                Token::Symbol(Symbol::ParenClose),
                Token::Symbol(Symbol::BracketOpen),
                Token::Keyword(Keyword::Return),
                Token::Literal(Literal::Integer(0)),
                Token::Symbol(Symbol::Semicolon),
                Token::Symbol(Symbol::BracketClose),
            ],
        )
    }
}
