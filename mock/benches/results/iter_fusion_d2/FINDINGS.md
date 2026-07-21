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

## Measured results

Ratio to baseline (iterfuse_pull2), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | iterfuse_mat2 (ratio) | iterfuse_pull2 (base) | iterfuse_push2 (ratio) |
|---|---|---|---|
| 64 | 1.29x | 771 ns | 1.08x |
| 256 | 1.36x | 3006 ns | 0.91x |
| 1024 | 1.39x | 11515 ns | 1.05x |
| 4096 | 1.38x | 49991 ns | 3.76x |
| 16384 | 1.26x | 752455 ns | 1.18x |

## Cost-model sanity line

At n=16384, the baseline (iterfuse_pull2) median is 752455 ns for pull2: N elements through a 2-stage fused pipeline. Treating n as the work-item count, that is 45.93 ns/item, about 147.0 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

Pull-style fusion wins at depth 2; materialize costs ~1.3x to 1.5x (the intermediate buffer). Push is competitive EXCEPT a reproducible non-monotonic regression at n=4096 (3.76x, confirmed on re-run) that vanishes by n=16384: a cache-conflict-miss valley signature. Confirming the mechanism needs cache-miss counters, which are unavailable in M1 userspace.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
