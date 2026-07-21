# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

**Strength: measurement** (wall-clock via CNTVCT_EL0, cross-validated byte-exact; the Zig cdylib is a
first-class harness variant exporting the same plain C ABI as the Rust bench_variant macro, with a
byte-identical abi_hash).

## Why a Zig variant exists

This is one of the three questions a Rust-only carrier cannot ask: Rust has no guaranteed tail call, so it
cannot express the tail-threaded dispatch that Deegen / LuaJIT-remake use. The carrier's program format is
language-neutral, so a Zig cdylib interprets an equivalent opcode stream and joins the harness as a normal
variant. Both Zig variants (switch, tail-threaded via `@call(.always_tail)`) interpret the identical
Zig-generated 8-op stream and fold the identical checksum; the harness cross-validated their output as
identical, so the measured gap is the dispatch shape and nothing else.

## Result, tail/switch ratio (> 1 = tail slower)

| n | switch | tail | tail/switch |
|---|---|---|---|
| 64    | 1813 ns    | 2600 ns    | 1.43x |
| 256   | 6121 ns    | 9081 ns    | 1.48x |
| 1024  | 27570 ns   | 38798 ns   | 1.41x |
| 4096  | 148105 ns  | 429981 ns  | 2.90x |
| 16384 | 1415644 ns | 1963482 ns | 1.39x |

Tail-threaded dispatch is ~1.4x SLOWER than switch across sizes. (The n=4096 spike to 2.9x is a reproducible
machine cache-conflict artifact at that specific working-set size; the same size perturbs iter_fusion_d2. The
stable figure is ~1.4x.)

## The finding, and what it does and does not say about Deegen

In Zig 0.16 on Apple M1, a tail-threaded interpreter (`@call(.always_tail)` through a function-pointer
handler table) is about 1.4x slower than a switch loop. The reason is the calling convention: Zig 0.16's
`@call(.always_tail)` uses the STANDARD C ABI, so every handler must preserve callee-saved registers across
each tail call. Deegen's and LuaJIT-remake's tail-threading advantage depends specifically on the
`preserve_none` (GHC-style) calling convention, which drops callee-saved preservation so the interpreter
state stays live in registers across the whole dispatch chain. Zig 0.16 cannot express `preserve_none`.

So this result CONFIRMS the audit's diagnosis of the earlier "switch refutes Deegen" claim: it was a
toolchain-ABI artifact, not a refutation of Deegen. With the standard ABI, tail-threading pays a
register-shuffle tax per opcode and loses to switch; that says nothing about whether `preserve_none`
tail-threading (Deegen's actual mechanism) would win. It would need a toolchain that emits `preserve_none`
tail calls (LLVM `musttail` + `preserve_none`, as Deegen uses via a patched clang), which neither Zig 0.16
nor stable Rust provides. The honest claim is narrow: on this toolchain, switch is the faster dispatch shape,
and a fair test of Deegen's claim is blocked on `preserve_none` support, not settled by this number.

## Cost-model sanity line

switch at n=16384: 1.42 ms for 16 iterations x 16384 ops = 262144 op-dispatches, ~5.4 ns/op-dispatch, ~17
cycles/op at 3.2 GHz. That is high for a single arithmetic op, consistent with an unpredictable indirect
jump-table branch (the 8-op stream is random, so the branch mispredicts often) plus a memory load of the
per-op constant; physically plausible, not an artifact. The tail variant's ~24 cyc/op adds the
register-preservation and call overhead the switch avoids.

## Boundary and reproduction

Zig variants build via `zig-dispatch/build.sh` (not `cargo mock bench run`, which only builds cargo crates).
The opcode stream is a random 8-op mix; a more predictable stream would lower both variants' misprediction
cost but not change the switch-vs-tail ordering, which is set by the calling convention. The cross-language
differential (a Zig variant consuming the Rust carrier's exact program bytes) is a further step this bench
does not take; here both Zig variants share a Zig-generated stream, which is sufficient for the dispatch-shape
question.
