//! The front end of the compiler
pub mod lex;
pub mod objects;
pub use objects::{Keyword, Literal, Matched, Source, Stream, Token};
