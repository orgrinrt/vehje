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
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

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

/// The wire form of a checked residual at a given tier.
///
/// M0 names the tier; the tier-0 wire struct (a `#[repr(C)]` node buffer,
/// the child-index pool, a string blob, and a root id) and the
/// runtime-environment interface descriptor land with the serialization.
// FIXME: define the tier-0 `#[repr(C)]` wire struct mirroring the vehje-ir
// arena layout, plus the runtime-environment interface descriptor and the
// `extern "C"` entry points. Bare primitives at that boundary carry the
// documented FFI `lint:allow`. M0 ships the tier tag only.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Residual {
    /// The representation this residual crosses in.
    pub tier: Tier,
}
