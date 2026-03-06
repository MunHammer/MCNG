//! The parser module that turns the token [`Stream`] into an [`AST`]
use crate::front::{AST, Expr, FnDecl, Keyword, Literal, Matched, Statement, Stream, Token, Type};
impl Stream {
    /// Parses a token [`Stream`] & expects a functions' parameters to be there & parses it
    /// # Panics
    /// If there is an unexpected token
    pub fn parse_params(&mut self) -> Vec<(Type, String)> {
        let /*mut*/ out = Vec::new();
        let token = self.0.drain(..1).next().unwrap();
        if let Token::Matched(Matched::OpenBracket) = token {
            {
                eprintln!("Placeholder, please replace this sometime");
                // TODO: Replace this with actuall parameter parsing
                let token = self.0.drain(..1).next().unwrap();
                if let Token::Matched(Matched::CloseBracket) = token {
                } else {
                    panic!("Unexpected token: {token}\nExpected: )")
                }
            }
        } else {
            panic!("Unexpected token: {token}\nExpected: (");
        }
        out
    }
    /// Parses a single statement from a [`Stream`] & returns it
    /// # Panics
    /// if there is an unexpected token
    pub fn parse_statement(&mut self) -> Statement {
        let token = self.0.drain(..1).next().unwrap();
        if let Token::Keyword(Keyword::Return) = token {
            let token = self.0.drain(..1).next().unwrap();
            if let Token::Literal(Literal::Integer(int)) = token {
                let token = self.0.drain(..1).next().unwrap();
                if let Token::Terminator = token {
                    return Statement::Return(Expr::Const(int));
                }
                panic!("Unexpected token:{token}\nExpected: SEMICOLON");
            }
        }
        panic!("Unexpected token {token}");
    }
    /// Parse non function code in a [`Stream`] form & returns a `Vec<Statement>`
    pub fn parse_code(&mut self) -> Vec<Statement> {
        let mut out = Vec::new();
        loop {
            out.push(self.parse_statement());
            if let Token::Matched(Matched::CloseBrace) = self.0[0] {
                self.0.drain(..1);
                break;
            }
        }
        out
    }

    /// Parses a token [`Stream`], & expects a function to be there & parses it
    /// # Panics
    /// If the function does not exist
    pub fn parse_fn(&mut self) -> FnDecl {
        let mut out = FnDecl::new();
        let token = self.0.drain(..1).next().unwrap();
        if let Token::Keyword(Keyword::Int) = token {
            out.r#type = Type::Int;
            let token = self.0.drain(..1).next().unwrap();
            if let Token::Identifier(ident) = token {
                out.name = ident;
                out.args = self.parse_params();
                let token = self.0.drain(..1).next().unwrap();
                if let Token::Matched(Matched::OpenBrace) = token {
                    out.code = self.parse_code();
                } else {
                    panic!("Unexpected token: {token}\nExpected {{");
                }
            } else {
                panic!("Unexpected token: {token}\nexpected an identifier");
            }
        } else {
            panic!("Unexpected token: {token}\nexpected a type");
        }
        out
    }
    /// A parser that turns a [`Stream`] of tokens into an [`AST`]
    /// # Panics
    ///
    #[must_use]
    pub fn parse(mut self) -> AST {
        let mut out = AST::new();
        loop {
            out.0.push(self.parse_fn());
            if self.0.is_empty() {
                break;
            }
        }
        out
    }
}
