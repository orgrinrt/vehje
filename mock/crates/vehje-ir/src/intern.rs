//! Interner integration.
//!
//! Name identity in the IR is `hilavitkutin_str::Str`, the 4-byte
//! `#![no_std]` interned handle. `StringInterner<A: ArenaInterner>`
//! interns; the `ArenaInterner` impl (runtime string storage) is supplied
//! by the host. Const strings resolve through the linker-section table
//! with zero runtime storage; runtime strings go through the provided
//! arena.
//!
//! This module re-exports the interner surface so the IR's name-identity
//! contract has one home.

pub use hilavitkutin_str::{ArenaInterner, Str, StringInterner};
