//! The module for lexical analysis

use regex::Regex;
use std::fmt;

/// A wrapper for a program's original source code
#[derive(Debug)]
pub struct Source {
    src: String,
}

/// A matched character, such as:
/// - <
/// - >
/// - [
/// - ]
/// - {
/// - }
/// - (
/// - )
#[derive(Debug)]
pub enum Matched {
    /// A {
    OpenBrace,
    /// A }
    CloseBrace,
    /// A (
    OpenBracket,
    /// A )
    CloseBracket,
}

/// A literal, such as a an integer or string
#[derive(Debug)]
pub enum Literal {
    /// An integer literal
    Integer(usize),
}

/// A keyword, such as:
/// - `int`
/// - `return`
#[derive(Debug)]
pub enum Keyword {
    /// The `int` keyword
    Int,
    /// The `return` keyword
    Return,
}

/// A Token, such as a keyword, terminator or whatever
#[derive(Debug)]
pub enum Token {
    /// A keyword
    Keyword(Keyword),
    /// A terminator: `;`
    Terminator,
    /// A literal
    Literal(Literal),
    /// An identifier, such as:
    /// `char banana[] = "orange"`
    /// `      ^^^^^^`
    Identifier(String),
    /// A matched thing, such as ()
    Matched(Matched),
}

/// A wrapper for a stream of tokens
/// #[derive(Debug)]
pub struct Stream {
    stream: Vec<Token>,
}

impl Matched {
    fn extract(hay: &mut String) -> Option<Matched> {
        // Start bracket
        if hay.starts_with('(') {
            hay.drain(..1);
            return Some(Matched::OpenBracket);
        } else if hay.starts_with(')') {
            hay.drain(..1);
            return Some(Matched::CloseBracket);
        } else if hay.starts_with('{') {
            hay.drain(..1);
            return Some(Matched::OpenBrace);
        } else if hay.starts_with('}') {
            hay.drain(..1);
            return Some(Matched::CloseBrace);
        }
        None
    }
}

impl Literal {
    fn extract(hay: &mut String) -> Option<Literal> {
        if let Some(match_obj) = Regex::new(r"^[0-9]+").unwrap().find(hay) {
            return Some(Literal::Integer(
                hay.drain(match_obj.range())
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            ));
        }
        None
    }
}

impl Keyword {
    fn extract(hay: &mut String) -> Option<Keyword> {
        if hay.starts_with("int") {
            hay.drain(..3);
            return Some(Keyword::Int);
        } else if hay.starts_with("return") {
            hay.drain(..6);
            return Some(Keyword::Return);
        }
        None
    }
}

impl Token {
    fn extract(hay: &mut String) -> Option<Token> {
        if let Some(matched) = Matched::extract(hay) {
            return Some(Token::Matched(matched));
        } else if let Some(lit) = Literal::extract(hay) {
            return Some(Token::Literal(lit));
        } else if hay.starts_with(';') {
            hay.drain(..1);
            return Some(Token::Terminator);
        } else if let Some(key) = Keyword::extract(hay) {
            return Some(Token::Keyword(key));
        } else if let Some(ident) = Regex::new(r"^[A-Za-z_\-]+").unwrap().find(hay) {
            return Some(Token::Identifier(hay.drain(ident.range()).collect()));
        }
        None
    }
}

impl Stream {
    /// Appends an item to the back of the inner Vec<Token>
    pub fn push(&mut self, item: Token) {
        self.stream.push(item);
    }
}

impl Source {
    /// Creates a new instance of a Source program
    #[must_use]
    pub fn new(str: String) -> Self {
        Self { src: str }
    }
    /// Lexes the program, turning it into a stream of tokens
    #[must_use]
    pub fn lex(mut self) -> Stream {
        let mut out = Stream { stream: Vec::new() };
        loop {
            if let Some(token) = Token::extract(&mut self.src) {
                out.push(token);
            } else {
                self.src = String::from(self.src.trim_start());
            }
            if self.src.is_empty() {
                break;
            }
        }
        out
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Int => "int",
                Self::Return => "return",
            }
        )
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Integer(int) => int,
            }
        )
    }
}

impl fmt::Display for Matched {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OpenBrace => '{',
                Self::CloseBrace => '}',
                Self::OpenBracket => '(',
                Self::CloseBracket => ')',
            }
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Terminator => String::from(';'),
                Self::Identifier(ident) => String::from(ident),
                Self::Keyword(key) => format!("{key}"),
                Self::Literal(lit) => format!("{lit}"),
                Self::Matched(matched) => format!("{matched}"),
            }
        )
    }
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.stream)
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}
