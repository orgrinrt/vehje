# SK3 findings: Zig dispatch foundation + preserve_none probe

**Date:** 2026-07-21
**Outcome:** WORKS (the interpreter dispatch foundation holds on stock Zig 0.16.0).
**Settles:** the Cluster C open item ("register-pinned tail-call dispatch does not hold on stock Zig") against
the PINNED toolchain, and the interpreter floor for BN1.
**Toolchain:** Zig 0.16.0, aarch64 (Apple Silicon), ReleaseFast.

## (a) preserve_none / ghccc convention: ABSENT in 0.16.0

`std.builtin.CallingConvention` in 0.16.0 is a `union(enum)` with 102 fields (auto, async, naked, inline, plus
every arch-ABI variant: x86_64_sysv, aarch64_aapcs/_darwin, riscv, powerpc, wasm_mvp, bpf_std, ...). There is NO
`preserve_none`, `ghc`, `ghccc`, or `hot` convention. So Cluster C's concern is confirmed against the pinned
toolchain, not just against 0.14: the register-preserving convention that CPython's copy-and-patch JIT and the
protobuf-parser use is not exposed by Zig 0.16. (probe: `probe_callconv.zig`.)

## (b) The core register-pinning does NOT need preserve_none

`@call(.always_tail, table[next_op], .{pc+1, nodes, res})` COMPILES on stock Zig 0.16 (always_tail errors if
TCO cannot be guaranteed, so a clean compile is the guarantee). The emitted aarch64 asm (`threaded.s`,
`dispatch_threaded.zig`) confirms, per handler:

- The walk state stays register-resident: pc in x0, nodes ptr in x1, res ptr in x2 (the aarch64 C-convention
  argument registers), never spilled across the handler. `res[pc]` is `str x9, [x2, x0, lsl #3]`, direct.
- Dispatch is an indirect tail JUMP, not a call: 4 handlers, 4 `br x3` (the next handler loaded into x3, then
  `br x3`), and ZERO `bl` to any `h_*` handler. Verified by grep count.
- No C-stack growth: each handler uses only the 16-byte `stp x29, x30, [sp, #-16]!` frame-pointer frame, and
  tears it down (`ldp x29, x30, [sp], #16`) BEFORE the `br`. So an arbitrarily long dispatch chain runs in
  constant stack. (The larger `sub sp, sp, #N` allocations in `threaded.s` are in `main` and std formatting, not
  the handlers.)
- Correctness: the threaded interpreter computes `[2,3,5,15]` for the test program (lit 2; lit 3; add; mul).

## Conclusion

The interpreter dispatch foundation is sound on stock Zig 0.16 today. `preserve_none` is an optimization tier,
not a correctness dependency: it would remove the 16-byte frame-pointer prologue/epilogue and free the
callee-saved registers for heavier handlers, but the load-bearing property (the three walk pointers stay in
argument registers, dispatch is a jump, no stack growth) is already achieved by `@call(.always_tail)` plus
explicit-argument passing. This matches Haberman's musttail result (which predates preserve_none). The
fix-the-stack-upstream ask (a preserve_none-equivalent convention in a later Zig) stands as an optimization, not
a blocker.

Design impact: BN1 can bench tail-call threading as a real, buildable variant. The interpreter (the floor on
every platform, including the iOS/console targets that forbid the native tier) is not tool-gated.

## Artifacts
- `probe_callconv.zig` (enumerates the 102 conventions)
- `dispatch_threaded.zig` (the threaded dispatch skeleton)
- `threaded.s` (emitted asm; grep `br x3` for the tail dispatches)
