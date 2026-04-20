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
//! `InvalidInput = -3`. Zig does not have a direct `extern
//! enum` equivalent, so we mirror the layout via a
//! one-field `extern struct { code: i32 }`. The field value
//! matches the Rust discriminant.

const std = @import("std");

pub const ClauseResult = extern struct {
    code: i32,
};

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
) ClauseResult {
    _ = rt;
    _ = input;
    _ = len;
    return .{ .code = -1 };
}
