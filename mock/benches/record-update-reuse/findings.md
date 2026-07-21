# Record-update reuse throughput: sizing the exact-meet in-place-mutation payoff

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 2M records x 16 fields, 8 field-updates each, three sharing regimes,
5-run best. Zig 0.16.0, aarch64. Artifacts: `reuse.zig`, `update.csv`.
**Settles:** the THROUGHPUT payoff of the exact-meet in-place-when-unique mutation model, which SK16 left
unmeasured (SK16 sized the PEAK-MEMORY payoff: emit-time referrer count-down, peak = live frontier, no runtime
refcounts). Sizes how much the reuse analysis is worth on the record-update hot path.

## Why this probe

vehje's biggest consumers (doc DSLs, config, game scripts) mutate records constantly: set a field, extend a
record, update a nested value. With functional (immutable) semantics, each update is a copy-on-write: allocate a
fresh record, copy every field, change one. The exact-meet model (SK16) instead mutates in place whenever the
emit-time analysis proves the record is unique (referrer count 1), copying only when it is genuinely shared. SK16
proved this keeps peak memory at the live frontier; it did not measure the throughput difference between
always-copy and in-place-when-unique on the update hot path. That difference is what decides whether the reuse
analysis is worth building for these consumers.

Because the uniqueness verdict is known at emit time (SK16's key result: the counting is compile-time, there are
no runtime reference counts), the runtime pays no analysis cost. It reads the emitted uniqueness bit and either
mutates in place or copies. This bench models that: a per-update `forced_shared` flag stands in for the emitter's
verdict, and a tunable fraction of updates are forced-shared.

## Results

2M records, 16 fields, 8 updates each (16M updates), across sharing regimes:

| shared | always-copy | in-place-when-unique | copies made | mutable ceiling | reuse speedup |
|---|---|---|---|---|---|
| 0% | 5.19 ns/upd | **0.53 ns/upd** | 0% | 0.53 ns/upd | **9.72x** |
| 20% | 5.05 ns/upd | 1.52 ns/upd | 21% | 0.60 ns/upd | 3.32x |
| 60% | 5.05 ns/upd | 3.37 ns/upd | 59% | 0.58 ns/upd | 1.50x |

## The finding: in-place-when-unique is 1.5x to 9.7x faster than functional copy, at the mutable ceiling when records are unique, and it never loses

In-place-when-unique reuse is dramatically faster than copy-on-write on the update hot path, and the payoff scales
with uniqueness:

- **Mostly-unique records (0 to 20% shared), the common case for locally-built templating/config/script records:
  3.3x to 9.7x faster.** At 0% sharing the reuse path hits the always-mutable ceiling exactly (0.53 ns/upd, zero
  copies): a functional update becomes a plain in-place field write, with no copy and no cost beyond the write.
- **Even heavily-shared records (60% shared): still 1.5x faster.** Reuse copies only when forced (exactly when
  copy-on-write would), and skips the copy the rest of the time, so it can never be slower than always-copy. The
  60% case still avoids 41% of the copies.
- **Reuse never loses to copy.** The worst case (100% shared) degenerates to always-copy; every unique record is
  a strict win. So the analysis has no downside regime.

Combined with SK16, the exact-meet reuse analysis is pure win on both axes: peak memory bounded to the live
frontier (SK16), and 1.5x to 9.7x faster record mutation (this bench), with the entire cost paid at compile/emit
time (no runtime refcount traffic). For the authoring/templating consumers op emphasised, this is the
"functional semantics, imperative speed" property that makes immutable-value scripting viable at scale: the
consumer writes pure functional updates, the emitter's uniqueness analysis turns them into in-place mutations
wherever it can prove safety, and the runtime just follows the emitted plan.

## Design impact

- Build the exact-meet reuse analysis: it is worth it. Functional record updates lower to in-place mutation
  wherever emit-time uniqueness holds, giving near-ceiling throughput for the mostly-unique common case and a
  strict win otherwise. This is a load-bearing enabler for the templating/config/scripting consumers, not an
  optional optimization.
- The runtime value-mutation path reads the emitted uniqueness bit and branches (in-place vs copy); it does no
  reference counting (SK16). The bit is one flag per update site in the residual, negligible wire cost.
- This composes with the value-arena (the copies and in-place writes are arena-slot operations) and with the
  frontier-bounded peak memory (SK16): reuse both bounds peak memory and accelerates mutation, the same mechanism
  serving both.

## Boundary

Fixed-width 16-field records; variable-width or deeply-nested records shift the copy cost (a wider record makes
copy-on-write more expensive, widening reuse's lead, so 16 fields is a conservative midpoint). The sharing
fraction is the load-bearing variable; real consumer sharing rates are workload-dependent, but locally-built
records (the templating norm: build a record, fill it, emit it) are overwhelmingly unique, landing in the 0 to
20% regime where reuse is 3x to 10x. The emit-time uniqueness analysis itself is not benched here (it is a
compile-side cost SK16 established is emit-time-computable); this sizes the runtime payoff of its verdict.

## Artifacts
- `reuse.zig` (always-copy vs in-place-when-unique vs always-mutable, across three sharing regimes),
  `update.csv`.
