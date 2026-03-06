//! The front end of the compiler
pub mod lex;
pub mod objects;
pub mod parse;
pub use objects::{
    AST, Expr, FnDecl, Keyword, Literal, Matched, Source, Statement, Stream, Token, Type,
};
