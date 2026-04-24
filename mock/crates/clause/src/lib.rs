//! vehje — CLI orchestration for the Vehje compiler.
//!
//! Dispatch spine only. Each subcommand module runs one pipeline
//! chain and prints its result. Phase crates own the algorithmic
//! content; this crate only wires them and routes diagnostics
//! through caller-owned sinks.

#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod args;
pub mod build;
pub mod check;
pub mod diag;
pub mod lex;
pub mod parse;
pub mod run;

pub use args::single_file_arg;
