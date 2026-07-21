# SK4 findings: the reachability binder rule + region-promotion bound (the N*W fix)

**Date:** 2026-07-21
**Outcome:** WORKS. The counter-audit's corrected fix is sound and bounded; the solution-set's InScope-only fix is
unsound in the opposite direction, confirmed executably.
**Settles:** the `N*W` reachability bug (Cluster C, counter-audit Finding 1). The complete fix is the
reachability-type binder rule (splice-and-drop at `Let`/`Lambda`, Bao-Wei OOPSLA 2021) plus a region-promotion
bound on the escape set, not a filter on the generic propagation.
**Toolchain:** Rust nightly-2026-05-28. `reach.rs`.

## Results

1. **Soundness: the binder rule is required.** On `let x = o in x` (a value that reaches an outer binder `o`
   *through* the let-bound `x`): the correct splice-and-drop rule computes `reach = {o}` (it splices `reach(e1) =
   {o}` where `e2` reaches `x`, then drops `x`); the broken InScope-filter-only rule drops `x` WITHOUT splicing,
   computing `reach = {}`. The broken rule loses `o`, judging the escape safe when it is not, unsound in the
   opposite direction from the original unbounded bug. Asserted: broken loses `o`, correct keeps it. This is
   exactly the counter-audit's catch, executable.

2. **The naming-set `N*W` bound holds and is `N`-independent.** On a width-bounded nest of N=2000 lets (each
   body referencing only the last W=4 in-scope binders, plus three free environment binders), the reach set at
   the root is 3, well under W=4, and does NOT grow with N. The splice-and-drop rule eliminates every out-of-scope
   let-bound binder as it leaves scope, so the live naming set stays bounded by the in-scope width W, giving the
   `N*W` (not `N*N`) tuple bound the no-alloc frontier depends on. Asserted `|reach| <= W`.

3. **The escape set is bounded by the live-region count.** On `\p. (b0 (b1 p))`, the closure's reach (its free
   captured binders, param dropped) is `{b0, b1}`, bounded by the number of enclosing live regions (2), the
   Tofte-Talpin region-promotion bound plus the shared-implies-promoted discipline. This is the second bound the
   counter-audit noted the InScope-only fix never established: the naming set bounds the in-scope reach to W, and
   region promotion bounds the escape set to the region count.

## Design impact

The corrected lease rule is the reachability-type binder rule (splice-and-drop at binder nodes) plus the
region-promotion bound, both prior art from the paradigm 2001 already picked, confirming the counter-audit over
the solution-set's InScope-only version. Both bounds together are what make the reachability lease no-alloc
tractable (`N*W` naming set, region-count escape set). This is the executable version of the fix that lands in the
design proper and in the relational-engine lease rules (SP5).

## Artifacts
- `reach.rs` (the correct vs broken rules + the three validated properties)
