//! Zig runtime skeleton.
//!
//! Phase 6 skeleton round (2026-04-20). Three extern "C"
//! stubs matching `clause-runtime-abi`'s exports. Real
//! interpreter body — arena allocator, IR interpreter,
//! scratch-var evaluator, TokenStream emitter, diagnostic
//! emission — is BACKLOG.
//!
//! The Rust-side `ClauseResult` is `#[repr(i32)]` with
//! discriminants `Ok = 0`, `Err = -1`, `NullHandle = -2`,
//! `InvalidInput = -3`. We return the layout as a plain `i32`
//! on both sides to preserve scalar-in-register calling
//! convention symmetry across every target: MSVC Windows x64
//! would lower a one-field extern struct via hidden-pointer
//! sret, breaking the match against a Rust `#[repr(i32)] enum`
//! return (which lowers to `eax`). Plain `i32` is the
//! trivially portable form.
//!
//! Named constants (`CLAUSE_RESULT_*`) mirror the Rust
//! discriminants for readable call sites.

const std = @import("std");

pub const CLAUSE_RESULT_OK: i32 = 0;
pub const CLAUSE_RESULT_ERR: i32 = -1;
pub const CLAUSE_RESULT_NULL_HANDLE: i32 = -2;
pub const CLAUSE_RESULT_INVALID_INPUT: i32 = -3;

export fn clause_runtime_new() ?*anyopaque {
    return null;
}

export fn clause_runtime_free(rt: ?*anyopaque) void {
    _ = rt;
}

export fn clause_runtime_execute(
    rt: ?*anyopaque,
    input: [*]const u8,
    len: usize,
) i32 {
    _ = rt;
    _ = input;
    _ = len;
    return CLAUSE_RESULT_ERR;
}
