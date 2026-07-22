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

## Result, ns/node across a fine size sweep (the working-set transition)

ns/node = algo_ns / (16 iters x n). A finer sweep than the original five sizes, to characterize the
non-monotonic ratio honestly:

| n | switch ns/node | tail ns/node | tail/switch |
|---|---|---|---|
| 64    | 1.77 | 2.91 | 1.64x |
| 256   | 1.45 | 2.76 | 1.90x |
| 1024  | 1.66 | 2.80 | 1.68x |
| 2048  | 1.66 | 3.99 | 2.40x |
| 3072  | 1.55 | 6.24 | 4.01x |
| 4096  | 1.99 | 6.81 | 3.43x |
| 6144  | 3.35 | 7.33 | 2.19x |
| 8192  | 3.91 | 7.35 | 1.88x |
| 16384 | 5.00 | 7.47 | 1.49x |

## The finding, and what it does and does not say about Deegen

Two things. First, the dispatch mechanism: in Zig 0.16 on Apple M1, tail-threaded dispatch
(`@call(.always_tail)` through a function-pointer handler table) is slower than a switch loop at every size,
by 1.5x to 4x. The reason is the calling convention. Zig 0.16's `always_tail` uses the STANDARD C ABI, so
every handler preserves callee-saved registers across each tail call, spilling the interpreter state to the
stack per opcode. Deegen's and LuaJIT-remake's tail-threading advantage depends specifically on
`preserve_none` (GHC-style), which drops that preservation so the state stays live in registers across the
whole dispatch chain. Zig 0.16 cannot express `preserve_none`.

Second, and this corrects the earlier coarse-sweep reading: the tail/switch ratio is NOT a flat 1.4x with an
isolated n=4096 cache spike. The fine sweep shows a smooth working-set transition. Both interpreters go from
compute-bound to memory-bound as their per-node arrays (the u64 constant array dominates, 8 bytes/node) grow
past cache, but tail transitions much earlier: its ns/node jumps at n=2048-3072 and plateaus by n=6144, while
switch stays flat until n=6144 and rises through n=16384. That is the standard-ABI cost made visible: the
per-node register spills add stack traffic that pushes tail memory-bound at a smaller working set than switch,
which keeps its accumulator in a register across the loop. So the ratio PEAKS at ~3-4x in the mid working-set
range (n=3072-4096, where tail is already memory-bound and switch is not) and settles to ~1.5x at n=16384 once
both saturate memory. The original "n=4096 cache-conflict valley" note was an artifact of sampling only five
sizes across a smooth crossover; the mechanism is earlier memory-boundedness from the ABI spills, not a
conflict miss at one size.

The corrected headline is stronger against tail-threading, not weaker: tail is 1.5x slower at best (both
memory-bound or both small) and up to 4x slower in the mid working-set range. This CONFIRMS the audit's
diagnosis that "switch refutes Deegen" was a toolchain-ABI artifact, and it still says nothing about whether
`preserve_none` tail-threading (Deegen's actual mechanism) would win; a fair Deegen test is blocked on
`preserve_none` support.

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
