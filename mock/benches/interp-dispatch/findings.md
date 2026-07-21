# BN1 findings: interpreter dispatch shapes (switch wins for lean flat IR)

**Date:** 2026-07-21
**Type:** runtime micro-bench, Zig-native (dispatch shapes are Zig constructs; each variant an isolated
ReleaseFast binary, 4M-node program, warmup + 9-pass median, monotonic clock). Zig 0.16.0, aarch64.
**Settles:** the dispatch-shape choice and the magnitude of the win, and it removes `preserve_none` from the
critical path.
**Data:** `dispatch.csv`.

## Results (ns per op, median)

| variant | local reads | random reads | 11-op set |
|---|---|---|---|
| **switch** | **4.58** | **5.25** | **7.27** |
| labeled (direct-threaded, Zig labeled-switch `continue`) | 6.24 | 7.07 | -- |
| tail (tail-threaded, `@call(.always_tail)`) | 6.38 | 7.17 | 8.01 |
| fnptr (indirect-call table, non-tail) | 5.73 | 7.50 | -- |

## The finding, which is contrary to the dynamic-language literature

Plain `switch` dispatch is the FASTEST across every case, and the "sophisticated" shapes (direct-threaded,
tail-threaded) are consistently SLOWER, even when the op set grows to 11. This inverts the usual result (Deegen
tail-threading beats LuaJIT by 31%, Ertl-Gregg threading beats switch), and the reason is the regime:

- vehje's residual is a lean flat IR of tiny arithmetic ops (1-2 instructions each), statically checked, with no
  per-op type dispatch or inline-cache machinery. With few small ops, the switch's single indirect branch is
  extremely well predicted, and the arithmetic body plus the (cache-bound) operand reads dominate. The extra
  machinery of the fancier shapes (per-op argument passing, a table load per dispatch, and without a
  `preserve_none` convention a 16-byte frame prologue/epilogue per op, per SK3) costs more than the branch
  misprediction it saves.
- The literature's tail-threading win is in the dynamic-language regime: large op bodies, many ops, runtime type
  checks, where the giant switch function spills registers and the branch mispredicts. vehje is not in that
  regime, exactly the Carmack/Cluster-C observation ("the residual is already statically checked, so the
  interpreter overhead a baseline JIT removes is smaller here"), now measured for dispatch too.

Cache: backward-LOCAL operand reads (the design's L1-hot forward-scan property) are ~0.7-1.7 ns/op faster than
random reads across every variant, confirming the arena's backward-local discipline matters about as much as the
dispatch shape.

## Design impact

- **Ship the interpreter floor with plain `switch` dispatch.** It is the fastest for the hot arithmetic loop
  (the per-frame concern) and the simplest.
- **This removes `preserve_none` from the critical path entirely.** Switch needs no special calling convention,
  so the Cluster C `preserve_none` worry and the fix-the-stack-upstream ask are moot for the floor. SK3 confirmed
  tail-threading is buildable on stock Zig; BN1 shows it is not worth building for this workload.
- **Regime caveat (honest):** this is the lean-arithmetic-dispatch regime. If a consumer's op set grows heavy
  (host-calls, allocation, complex family ops with multi-instruction bodies), the crossover could shift toward
  tail-threading; a follow-up expanding this bench with heavy op bodies would locate it. For the hot inner loop
  that decides per-frame cost, switch is the answer.

## Artifacts
- `common.zig` / `v_{switch,labeled,tail,fnptr}.zig` (4 variants x local/random), `common11.zig` /
  `v11_{switch,tail}.zig` (op-count sensitivity), `dispatch.csv`.
