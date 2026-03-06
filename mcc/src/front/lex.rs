//! The module for lexical analysis

use crate::front::{Keyword, Literal, Matched, Source, Stream, Token};
use regex::Regex;

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

impl Source {
    /// Lexes the program, turning it into a stream of tokens
    #[must_use]
    pub fn lex(mut self) -> Stream {
        let mut out = Stream::new();
        loop {
            if let Some(token) = Token::extract(&mut self.0) {
                out.push(token);
            } else {
                self.0 = String::from(self.0.trim_start());
            }
            if self.0.is_empty() {
                break;
            }
        }
        out
    }
}
