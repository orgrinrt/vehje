# SK5 findings: the comptime kernel must be iterative (recursive segfaults the compiler)

**Date:** 2026-07-21
**Outcome:** WORKS (the iterative defunctionalised work-stack comptime kernel is viable and mandatory).
**Settles:** the 1845 constraint ("comptime recursion segfaults the compiler, the kernel must be iterative") and
the 2001 defunctionalization + functional-correspondence owed piece, against the pinned Zig 0.16.0.
**Toolchain:** Zig 0.16.0, aarch64, ReleaseSafe.

## The recursive comptime kernel SIGSEGVs the compiler at ~2500 depth

A comptime recursive fold over a left-spine IR tree of height D (`recursive_comptime.zig`, worst case: tree
height == the compiler's own comptime call-stack depth). With `@setEvalBranchQuota` raised to a billion so the
branch quota is not the limit:

| D | result |
|---|---|
| 500 | OK (124750) |
| 2000 | OK (1999000) |
| 3000 | SIGSEGV (rc 139) |
| 4000 | SIGSEGV |
| 5000 | SIGSEGV |
| 6000 | SIGSEGV |

The break is between 2000 and 3000. The failure is a hard SIGSEGV of the Zig compiler process (a stack overflow
of the compiler's own native recursion during comptime evaluation), NOT a graceful `error: evaluation exceeded N
backwards branches`. So a recursive comptime kernel is not merely slow at depth, it crashes the toolchain, and it
cannot be rescued by raising the eval branch quota (the quota was already at 1e9). This is exactly 1845's warning,
confirmed on the pinned toolchain.

## The iterative explicit-stack kernel removes the limit

`iterative_comptime.zig` folds the same tree with an explicit comptime work-stack (a fixed-capacity `[CAP]usize`
array, push/pop, a `while (sp > 0)` loop), the defunctionalised shape (Reynolds; Danvy-Nielsen), so the
compiler's own call stack stays flat. It folds a D=100000 spine (fifty times past the recursive segfault point)
cleanly, correct result 4999950000, with CAP=8 (a spine's live frontier is 1; CAP is the depth-cap bound for a
branching tree). No branch-quota or stack issue.

## Conclusion and design impact

The certified-generation kernel that runs at comptime to specialise the engine to the language data MUST be
iterative (an explicit fixed-capacity work-stack machine), not natively recursive. This is not a preference: a
recursive kernel SIGSEGVs the toolchain on any language whose IR or table exceeds ~2500 comptime-recursion depth,
which real programs will. The explicit stack is bounded by the depth cap (the same finite quantity already used
for the no-alloc frontier, the untrusted traversal, and the load verifier), so it fits the existing budget. SK1
(cert-gen core) and every comptime specialisation path is written iteratively from the start. The functional
correspondence (Ager-Biernacki-Danvy-Midtgaard) is the argument that the iterative machine computes the recursive
specification; here it is confirmed empirically (identical results where the recursive version survives).

## Artifacts
- `recursive_comptime.zig` (segfaults at depth; sed `__D__` for the depth)
- `iterative_comptime.zig` (the defunctionalised work-stack kernel; clean at D=100000)
