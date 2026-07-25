//! The Clause front end: surface syntax to the vehje Core.
//!
//! The bar is the COMPLETE grammar rather than a runnable subset, so the shape
//! here is driven by `CLAUSE_EBNF.md` production by production rather than by
//! what happens to run today.
//!
//! `no_std`, no allocator, caller-lent arenas. A front end is a dev-time tool
//! and could have reached for a heap; matching the framework's discipline
//! instead means the parser's arena and the IR arena it lowers into read alike,
//! and one host sizes one budget for the whole pipeline.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod ast;
pub mod lex;
pub mod parse;
pub mod token;

pub use ast::{Arena, AstList, AstRef, Node};
pub use lex::{lex, LexError};
pub use parse::{ParseError, Parser};
pub use token::{Keyword, Span, Token, TokenKind};
