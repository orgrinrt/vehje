# Cross-mod dependency DAG: cold parallel scheduling + warm blast radius (the realistic mod case)

**Date:** 2026-07-21
**Type:** Zig-native scheduling-characteristic bench, ReleaseFast, 2000 mods, load-order DAG (each mod depends on
0 to 3 earlier mods), longest-path level assignment + transitive-dependents traversal. Zig 0.16.0, aarch64.
Artifact: `depdag.zig`.
**Settles:** the load-time story for the REALISTIC consumer case (mods that override/extend each other), which
the independent-mod benches (compile-stage-parallelism, mod-stack-load) explicitly deferred three times.

## Why this probe

op's biggest-consumer load-time concern is Clausewitz and RimWorld, where mods do not sit independent: they
override and extend one another, forming a cross-mod dependency DAG. Two questions the independent-mod benches
could not answer:

- **Q1, cold parallel:** with a dep DAG, parallel compile is bounded by the critical path (a level-sync
  schedule: a mod cannot start until every mod it depends on is compiled). Does the DAG structure destroy the
  near-linear speedup the independent case showed?
- **Q2, warm incremental:** when one mod changes, its transitive dependents must recompile. What is the blast
  radius for a realistic DAG?

The DAG is modelled as a load-order DAG: a mod may only depend on mods earlier in load order, which is exactly
how Paradox and RimWorld load orders work (a patch mod depends on the base it patches, which loads before it).

## Results

2000 mods, up to 3 deps each:

- The DAG is **20 levels** deep (critical-path depth) and therefore **wide**: ~100 mods per level on average.
- **Q1 cold:** serial 100ms; infinite-core level-sync 1.0ms (100x, the critical-path floor); **8-core level-sync
  12.95ms, 7.7x**.
- **Q2 warm:** editing one random mod recompiles on average **19.3 dependents (1.0%)**, but the maximum observed
  blast radius is **803 mods (40%)**.

## The finding: the realistic DAG parallelises as well as (here, better than) independent mods; warm recompile is usually tiny with a heavy tail

**Q1.** The dependency DAG does not hurt cold parallelism, because a load-order mod DAG is wide and shallow.
With ~100 mods per level and only 20 levels, an 8-core level-sync schedule keeps all cores busy within each level
and pays the sync barrier only 20 times. The 7.7x speedup actually exceeds the independent-mod bench's 5.64x,
because the level-sync schedule here is bottlenecked by having enough width to fill cores (which it does) rather
than by the efficiency-core / bandwidth ceiling the independent bench hit. The critical-path floor (infinite
cores) is 100x (1ms), so there is enormous headroom: cold load is core-bound, not critical-path-bound, for any
realistic core count. The DAG shape that matters is depth; load-order DAGs are shallow, so they parallelise well.

**Q2.** Warm incremental recompile is usually cheap: editing a typical mod recompiles ~19 transitive dependents,
1% of the stack. But the distribution has a heavy tail: editing a FOUNDATIONAL mod (a base that much of the stack
depends on, transitively) recompiles up to 803 mods, 40% of the stack. This is correct and unavoidable (a base
change genuinely invalidates everything downstream), but it means warm-load cost is bimodal: trivial for leaf and
mid-stack edits, substantial for base edits. The design should expose this (a mod's dependent count is knowable
statically from the DAG), so a host or author knows that editing a widely-depended-on mod is the expensive case.

## Design impact

- **Cold load, DAG case:** bounded by the level-sync critical path, but load-order DAGs are shallow (20 levels
  for 2000 mods) and wide, so cold load stays core-bound and parallelises well (7.7x on 8 cores here). The
  scheduler is a level-sync parallel walk of the dep DAG; arvo-graph already provides the topo/level machinery
  the framework reuses, so this is a scheduling characteristic, not new engine code. Combined with the
  independent-mod result: whether mods are independent or form a load-order DAG, cold load parallelises to the
  core count.
- **Warm load, DAG case:** incremental recompile propagates along transitive dependents (the BN3 differential
  path). Usually 1%, worst case ~40% for a base-mod edit. The blast radius is statically knowable from the
  dependent DAG, so it is a predictable cost, not a surprise. This refines the mod-stack-load bench (which
  measured the independent case where a warm edit recompiles exactly one mod): with real deps, a warm edit
  recompiles one mod PLUS its transitive dependents.
- The full load-time story across the three benches: independent cold parallelises to cores; DAG cold
  parallelises to cores (shallow-wide load orders); warm recompile is the transitive-dependent set (1% typical,
  bimodal with a base-edit tail); and the interner-merge tail (~34% of the parallel compile) applies to cold load
  in all cases.

## Boundary

Models a load-order DAG with uniform per-mod compile cost. Real mods vary in size, so a heavy base mod on the
critical path would lengthen the effective critical time; but base mods are few and the DAG is shallow, so the
core-bound conclusion holds. Cyclic dependencies are not modelled (load orders are acyclic by construction). The
blast-radius traversal counts mods, not compile cost; a base edit whose 803 dependents are mostly tiny mods is
cheaper than the count suggests.

## Artifacts
- `depdag.zig` (load-order DAG generator + level assignment for the cold schedule + transitive-dependents
  traversal for the warm blast radius).
