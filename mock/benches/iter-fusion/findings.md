# Iterator fusion: the Iter core form's pipeline lowering

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 20M elements, filter->map(->map)->sum pipelines, opaque (non-vectorizable)
stages, 5-run best. Zig 0.16.0, aarch64. Artifacts: `iterfuse.zig`, `fusion.csv`.
**Settles:** how the `Iter` core form (iteration/comprehension) lowers: materialized intermediates versus fused,
and push-fused versus pull-fused. Sizes the fusion win and settles the push/pull choice for the pipeline hot path
that templating/config/script consumers run (loop over items, filter, transform, emit).

## Why this probe

`Iter` is a core form, and iteration pipelines (filter, map, take, emit-per-item) are pervasive in the consumers
op emphasised. The lowering fork is iterator fusion: a materialized pipeline builds an intermediate collection per
stage (filter -> temp, map -> temp2, ...), while a fused pipeline composes all stages into one loop with no
intermediates. Two fusion shapes exist: pull-based (the consumer calls `next()`, each pull walks the source
through the stages) and push-based (the source loop drives, each element flows through the stages inline). The
no-alloc constraint bears directly on this (materialized intermediates need allocation), so the bench both sizes
the win and confirms which shape to build.

To model the realistic runtime (where filter/map stages are interpreted closures, not inlinable native ops), the
stages are opaque scalar operations the vectoriser cannot collapse. A first run with plain inlinable stages is
reported below as a separate observation, because it revealed a second, larger fusion benefit.

## Results

filter(even) -> map(*3+1) [-> map] -> sum over 20M elements, opaque stages:

| pipeline | materialized | pull-fused | push-fused |
|---|---|---|---|
| 2-stage | 3.43 ns/elem | 3.21 | 3.20 |
| 3-stage | 3.61 ns/elem | (n/a) | 3.23 |

Plus the plain-inlinable-stages observation (before the stages were made opaque): the fused pipelines
auto-vectorised to effectively zero per element (a single fused reduction loop the vectoriser sees whole), while
the materialized version stayed at ~3.5 ns/elem because the store-then-load traffic through the temp arrays
blocks vectorisation.

## The finding: the Iter form must fuse; push and pull are throughput-equal; fusion also unlocks vectorisation

Three results, in order of design weight:

**1. Fusion is mandatory, not merely faster.** Materialization builds an intermediate collection per stage, which
requires allocation. The runtime is no-alloc, so materialized pipelines are not permitted at all. The `Iter` form
must lower to a fused pipeline. This is a constraint conclusion, not a performance one: the ~7 to 11% throughput
edge below is the secondary reason; the primary reason is that materialization cannot exist in the no-alloc
runtime.

**2. The fusion throughput edge grows with pipeline depth.** On opaque scalar stages, fusion is ~7% faster at
2 stages (3.20 vs 3.43 ns) and ~11% at 3 stages (3.23 vs 3.61 ns). Each additional materialized stage is a full
extra pass over the data plus a temp buffer, so the materialized cost compounds with depth, while the fused
pipeline stays flat (~3.2 ns) regardless of stage count because all stages run in one pass per element. Templating
pipelines are often deep (filter, map, map, format, emit), so the compounding matters.

**3. Push and pull fusion are throughput-equivalent, so choose by composability.** Push-fused (3.20) and
pull-fused (3.21) are within noise. Since throughput does not decide it, the design should pick the pull-based
(iterator/`next()`) shape, because it is strictly more composable: it supports lazy and early-terminating
operators (`take N`, `first matching`, `any`, `all`) naturally by simply stopping the pull, which the templating
consumers need (render the first N items, find the matching entry), whereas a pure push source runs to completion
unless it threads a stop signal back. Pull costs nothing extra here and buys lazy composition.

**4. Fusion additionally unlocks vectorisation in the native/specialised tier.** When the stages are native and
inlinable (the specialised/copy-and-patch tier, not the interpreter), the fused pipeline becomes one loop the
vectoriser can see and collapse, while the materialized version's intermediate memory traffic blocks it. So
fusion's payoff is workload-dependent: ~7 to 11% on interpreted scalar stages, and much larger (vectorisation) on
native stages. Either way fusion wins, and the native tier wins more from it.

## Design impact

- Lower the `Iter` core form to a fused pipeline, always. Materialized intermediates are forbidden (no-alloc) and
  slower (compounding with depth). This is the same "no intermediate collections" discipline the value model
  already carries.
- Use pull-based (iterator/`next()`) fusion. It is throughput-equal to push and strictly more composable,
  supporting the lazy and early-terminating operators the consumers need. The source is a pull iterator; filter
  and map are pull adapters; the sink drives by pulling to exhaustion or until a lazy operator stops it.
- The fused pipeline needs no per-stage allocation: filter and map are stateless adapters over the pull, the only
  state is the source cursor and the sink accumulator. This fits the no-alloc value model directly (the fusion is
  the mechanism that makes comprehensions no-alloc).
- In the native/specialised tier, the fused loop additionally vectorises where the stages are inlinable, a bonus
  the materialized shape structurally cannot get.

## Boundary

Stateless filter/map over a contiguous source; stateful adapters (scan, dedup, group) carry per-element state but
still fuse (the state lives in the adapter, not an intermediate collection). A pipeline whose source is itself
expensive to produce (a generator) shifts cost onto the source, where fusion's single-pass property matters more
(materialization would force the whole source before the first output). The opaque-stage model is the interpreted
case; the native case (larger fusion win via vectorisation) is the separate observation above. Early termination
(take/first) is where pull's composability advantage is decisive but was not separately timed here; it is a
correctness/expressiveness argument, not a throughput one.

## Artifacts
- `iterfuse.zig` (materialized vs pull-fused vs push-fused, 2-stage and 3-stage, opaque stages), `fusion.csv`.
