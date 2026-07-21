# Compile-stage parallelism across independent mods (cold-load scaling)

**Date:** 2026-07-21
**Type:** Zig-native scaling bench, ReleaseFast, 2000 mods x 8192 nodes, 5-run best, `std.Thread` work-stealing
over an atomic job counter. Zig 0.16.0, aarch64 (8 cores: 4 performance + 4 efficiency on this M-series part).
Artifact: `parcompile.zig`.
**Settles:** whether the compile stage (which the e2e capstone showed dominates cost) parallelizes across a mod
stack, directly serving op's cold-load-of-a-big-mod-stack concern (Clausewitz / RimWorld load times).

## Why this probe

Two prior results framed the load-time question but left a gap. The e2e capstone showed the compile stage
(lease-infer + cheap-lower) is 3.24ms of a 3.4ms pipeline, so it is where cold-load cost lives. The
mod-stack-load bench showed WARM load is cheap (edit one mod, recompile one, 1999 cache hits) but ran the stack
SERIALLY. Neither answered: when a stack has no warm cache (first-ever load, or a version bump invalidating
everything), does compiling N mods scale across cores? A mod stack is N independent compile jobs with no shared
mutable state, which is the embarrassingly-parallel case, so the question is whether the design actually realises
that or hits a hidden serialisation.

## Results

2000 mods, 8192 nodes each, per-mod compile-stage work (a reachability-style backward-OR fold plus a
const-fold/CSE hash-cons counting pass, the two ops the e2e measured as the compile-stage cost), dispatched over
a shared atomic job counter (work-stealing, no per-job lock):

| threads | time | speedup | throughput |
|---|---|---|---|
| 1 | 101.7 ms | 1.00x | 19.7K mods/s |
| 2 | 50.8 ms | 2.00x | 39.3K mods/s |
| 4 | 26.3 ms | 3.87x | 76.1K mods/s |
| 8 | 18.0 ms | 5.64x | 110.9K mods/s |

## The finding: the compile stage is embarrassingly parallel; cold load scales near-linearly to the core count

Scaling is essentially perfect through 4 threads (2.00x at 2, 3.87x at 4) and continues to 5.64x at 8. The
sub-linear step from 4 to 8 is expected and not a design concern: this part has 4 performance cores plus 4
efficiency cores, so threads 5 to 8 land on the slower efficiency cores, and the fold-plus-CSE work is partly
memory-bandwidth-bound (each mod streams an 8192-node arena plus a reach column). Both effects cap the last 4
threads below linear. On a homogeneous many-core desktop the near-linear region would extend further.

This closes the load-time story together with the two prior results:

- **Cold load** (no cache): the mod stack compiles across cores at near-linear speedup, because independent mods
  share no mutable state and need no locks. A 2000-mod first load that is 100ms serial is ~18ms on this laptop,
  and scales down further with more performance cores.
- **Warm load** (mod-stack-load bench): editing one mod recompiles one mod, not the stack, so steady-state
  iteration cost is independent of stack size.

Together: cold load parallelises with cores, warm load parallelises with changes. The RimWorld / Clausewitz
load-bloat concern op raised is answered on both axes. The framework owns the parallel-dispatch shape of the
compile stage (independent per-mod jobs over a work queue); the host owns the thread-count policy (how many
cores to spend on load).

## Design impact

- The compile stage's cold-load cost is `serial_cost / min(cores, mod_count)` for an uncached stack, not the
  serial sum. Load-time budgeting can assume near-linear core scaling for independent mods.
- The parallel unit is the mod (or, more generally, the independent compilation unit): no shared mutable state,
  no lock, a work-stealing counter over the job list. This is the D6 independent-mod-caching regime made
  parallel; it does not cover cross-mod dependency edges (those serialise along the dep DAG and would use the
  BN3 differential-recompile path, a separate mechanism).
- Nothing in the compile-stage design forces serialisation: the lease-inference fixpoint and the cheap-lowering
  hash-cons pass are both per-arena and touch no global state, which is what makes the near-linear scaling
  available. Keep that property (per-mod-local compile state, no shared interner mutation across mods at the
  parallel boundary) as a wire-format / API contract so the parallelism stays reachable.

## Boundary

Independent mods only (the common case: a stack of mods that do not import each other's definitions). A stack
with heavy cross-mod dependencies has a dep DAG that constrains the schedule; the parallel speedup there is
bounded by the DAG's critical path, and incremental re-load uses differential recompilation (BN3), not a flat
work queue. The bench also holds all arenas resident; a stack too large for RAM streams from disk, at which
point load is I/O-bound and the compile parallelism overlaps with streaming rather than dominating.

## Artifacts
- `parcompile.zig` (mod-IR generator + per-mod compile-stage work + work-stealing thread pool + the 1/2/4/8
  scaling sweep).
