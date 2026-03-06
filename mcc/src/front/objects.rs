//! The declaration of enums & structs
//! Also for stuff that is quite general, such as `::new` & `fmt::Display`

use std::fmt;

/// A wrapper for a program's original source code
#[derive(Debug)]
pub struct Source(pub String);

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
#[derive(Debug)]
pub struct Stream(pub Vec<Token>);

// Random impls, such as new & stuff

impl Stream {
    /// Appends an item to the back of the inner Vec<Token>
    pub fn push(&mut self, item: Token) {
        self.0.push(item);
    }
}

impl Source {
    /// Creates a new instance of a [`Source`] program
    #[must_use]
    pub fn new(str: String) -> Self {
        Self(str)
    }
}

// The fmt::Display area

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
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
