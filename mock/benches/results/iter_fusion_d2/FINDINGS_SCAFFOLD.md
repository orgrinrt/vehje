# Iterator fusion (carrier-harness port): the Iter form's pipeline lowering

Scaffold for `iter_fusion_d2` and `iter_fusion_d3`. Numbers are filled in after
the harness run; this states what is measured, why the earlier shape was
defective, and the direction the finding is expected to confirm.

## What this replaces

The standalone Zig bench (`iter-fusion/iterfuse.zig`) ran outside the harness
(hand-timed 5-run-best loop, its own CSV) and the first-pass harness port
(`hx_iter_fusion`) measured only one materialized-vs-fused pair with an inline,
input-independent body. This port fixes both: two depth sections (2-stage and
3-stage) each carry three variants (materialized, fused-push, fused-pull), the
pipeline source is the FFI `input` folded into the accumulator so nothing
hoists, and the three variants produce byte-identical `output` that the harness
cross-validates (verified: all three match at every checked size, and output
varies with the input).

## What it measures

`source |> filter(even) |> map(*3+1) [|> map(^0x5a5a)] |> sum` over the N input
bytes, ITERS=16 passes, each pass re-walking the array with a different
FFI-derived perturbation. The map stages are opaque (black_box), so the
vectoriser cannot collapse either side: the only difference measured is the
fused single-pass shape versus the materialized store-then-reload traffic
through the temp arrays. Temp arrays are allocated outside the timed block, so
the bench measures the intermediate memory traffic, not the allocation the
no-alloc runtime forbids outright. Fused-pull is expressed with Rust iterator
adapters (the consumer drives next()); fused-push is the explicit source-driven
loop.

## Result (medians, normalised against the fused-pull baseline)

| n | mat2 | push2 | pull2 (base) | mat3 | push3 | pull3 (base) |
|---|---|---|---|---|---|---|
| 64 | TBD | TBD | TBD | TBD | TBD | TBD |
| 256 | TBD | TBD | TBD | TBD | TBD | TBD |
| 1024 | TBD | TBD | TBD | TBD | TBD | TBD |
| 4096 | TBD | TBD | TBD | TBD | TBD | TBD |
| 16384 | TBD | TBD | TBD | TBD | TBD | TBD |

Expected direction (from the prior Zig run, to confirm or refute): materialized
trails fused; the fused edge grows from roughly 7% at depth 2 to roughly 11% at
depth 3 as each extra materialized stage adds a full pass plus a temp buffer
while the fused pipeline stays one pass per element; push and pull sit within
noise of each other.

## What it says for the design

- Fusion is mandatory, not merely faster: materialized intermediates need
  allocation, which the no-alloc runtime does not permit, so the Iter form must
  lower to a fused pipeline. The measured edge is the secondary reason.
- The fused edge grows with pipeline depth (the d2-vs-d3 comparison), because
  materialized cost compounds per stage while fused stays flat.
- Push and pull are throughput-equal, so the choice is composability: pull
  (iterator next()) supports lazy and early-terminating operators (take N,
  first-match, any, all) by stopping the pull, which the templating consumers
  need. Hence the pull baseline.

## Cost-model sanity line (to complete at run)

At n=16384, ITERS=16, about 8192 even elements per pass times 16 passes is
roughly 131072 map-plus-add ops per call. Divide the measured per-call time by
that to get ns/element and confirm it lands in the low-single-digit-ns range a
scalar opaque map-plus-add should cost; confirm near-linear scaling with N so
the timed work is the pipeline and not a hoisted constant.

## Caveats

The opaque-stage model is the interpreted-tier case. The native tier (inlinable
stages) gets a larger fusion payoff via vectorisation, which this bench
deliberately blocks with black_box to isolate the memory-traffic edge; that
larger win is a separate observation, not measured here. Early termination
(take/first), where pull's composability is decisive, is an expressiveness
argument, not timed here.
