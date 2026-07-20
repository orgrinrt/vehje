//! vehje-runtime-abi, the framework's tier-tagged C ABI.
//!
//! A `Checked` residual crosses tier-tagged: a flat serialized IR arena
//! (baseline), an optimized bytecode, or native code. The tier is an
//! optimization axis orthogonal to correctness; the effect proof is
//! discharged in Rust before any lowering. Pure type surface plus the
//! runtime-environment interface descriptor, no runtime logic.
//!
//! `#![no_std]`, no alloc. Bare primitives appear only at the
//! `#[repr(C)]` wire boundary (the documented FFI exception), which the
//! tier-0 serialization introduces when it lands.

#![no_std]
// const_trait_impl (test-only): WATCH-allowed (unstable-features.md); the
// blob round-trip test builds interned strings with hilavitkutin-str's
// `str_const!`, which needs it.
#![cfg_attr(test, feature(const_trait_impl))]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod encode;
pub mod wire;

/// Serialize a checked program into a tier-0 residual byte image.
pub use wire::serialize;

/// The format-agnostic residual encoder contract and node walk.
pub use encode::{encode, LitTag, NodeTag, ResidualEncoder};

/// The representation a residual crosses the ABI in.
///
/// The tier is chosen for execution speed, never semantics: the effect
/// proof holds across all tiers, so a residual is the same proven-safe
/// object whether it crosses as an arena, bytecode, or native code.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tier {
    /// The flat serialized IR arena: the reference semantics, and the
    /// tier the first working version ships.
    Arena,
    /// An optimized linear bytecode (encoder deferred).
    Bytecode,
    /// Native machine code, where the runtime is environment plumbing
    /// around LLVM/cranelift output (encoder deferred).
    Native,
}

/// A checked residual's crossing descriptor: the tier it was serialized
/// at.
///
/// The tier-0 byte image itself is produced by [`serialize`] (see the
/// [`wire`] module); this names which representation those bytes are in,
/// so the driver and runtime agree on how to read them.
// FIXME: carry the runtime-environment interface descriptor (the host
// capabilities the residual expects) and the `extern "C"` entry points
// alongside the tier tag once the driver bindings land (M3 step 3).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Residual {
    /// The representation the serialized residual crosses in.
    pub tier: Tier,
}
