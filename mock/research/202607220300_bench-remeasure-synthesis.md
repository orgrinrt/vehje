# Bench re-measure synthesis: the honest numbers after the carrier rebuild

**Date:** 2026-07-22
**Scope:** consolidation of the re-measured bench evidence produced on the shared `vehje-bench-carrier`,
mapped to the audit's C1-C7 (`202607211539_bench-evidence-audit-panel/fabian_giesen_synthesis.md`) and to the
headlines of the synthesis topic (`design_rounds/202607211347_topic.bench-and-sketch-evidence-synthesis.md`)
that the audit found not dependable.
**Status:** evidence, not design. This doc supplies the corrected numbers so the design topic and any design
decision that rested on the old headlines can be updated. The design decisions (which record width, which
dispatch, whether to bound the e-graph) remain op's; this only says what the measurements now show.

All benches run on Apple M1, harness rev pinned to `70fb75b5`, wall-clock via CNTVCT_EL0 (24 MHz). There is no
userspace PMU cycle counter on M1, so each findings file carries a cost-model sanity line (ns -> cycles ->
implied IPC) instead, and mechanism attribution rests on designed sweeps. Every bench cross-validates its
variants byte-exact; every number here is in a committed CSV under `mock/benches/results/`.

## The five bad headlines, corrected

### 1. Native ceiling: "1.0-1.2x" was native-vs-native (audit C1)

The old ceiling measured 1.2x because its two-instruction program was a comptime constant the optimizer
partially evaluated the interpreter into. Re-measured with the program crossing FFI opaque
(`carrier_native_ceiling`): a switch interpreter is ~2.0x a shape-specialized native loop, a
function-pointer-table interpreter ~2.2x, stable across three orders of magnitude, cost-model ~1-1.5 IPC
(physically real). The native baseline was strengthened (it had allocated a per-call Vec, understating the
ratio at ~1.8x). Corrected claim: interpretation costs ~2x native on a program maximally friendly to it
(predictable dispatch); real programs are worse for the interpreter. The 1.2x headline is retracted.

### 2. Record width: "24 bytes precisely optimal" was mislabeled

`carrier_record_width` ties all layouts 12-32B at every harness-reachable size; the true 16-byte 3-operand
layout ties the wider ones. The wire format can go to 16 bytes with no measured penalty in the L1/L2-resident
regime. The bandwidth regime past L2 is owed to the scale-runner (not yet run for record width). Corrected
claim: 16B is a live contender; "24B precisely optimal" is not supported.

### 3. "Switch refutes Deegen" was a toolchain-ABI artifact (audit C3), now measured properly

`zig_dispatch`: a Zig 0.16 cdylib (first-class harness variant, same C ABI, byte-identical abi_hash) running
switch vs tail-threaded `@call(.always_tail)` over an identical opcode stream, cross-validated. Tail is 1.5x to
4x SLOWER than switch on M1 (a fine size sweep shows tail goes memory-bound earlier than switch, so the ratio
peaks ~3-4x in the mid working-set range and settles to ~1.5x once both saturate). The reason is the calling convention: Zig 0.16's `always_tail` uses the standard ABI
(callee-saved preservation per handler), while Deegen's advantage needs `preserve_none`, which Zig 0.16 cannot
express. Corrected claim: on this toolchain switch is the faster dispatch shape, and this neither confirms nor
refutes Deegen (whose mechanism is `preserve_none` tail dispatch); a fair Deegen test is blocked on
`preserve_none` support, not settled here.

### 4. Incremental warm "0.003ms / 1300x" was an `if(i==1234)` no-op (audit C4)

`incr_reload`: a real content-addressed cache (fast word-hash + multi-pass compile) that hashes all N modules,
hits the cache on the N-1 unchanged, misses on the single input-edited one, recompiles only it, folding to the
identical result as a cold rebuild (cross-validated). Warm reload is 2.5x-3.7x faster than cold, stabilizing
near 3.6x, with the load-bearing caveat that the multiple scales with the compile/hash cost ratio (a first cut
with a trivial compile and a slow serial FNV measured warm 18x SLOWER, because caching loses when compile is as
cheap as hashing). Corrected claim: content-addressed incremental caching wins in proportion to how much
compilation exceeds hashing; the 1300x figure is deleted; the honest mechanism and direction stand.

### 5. "Hardware-counter timing" was false; it is wall-clock (audit C7)

Confirmed and now stated everywhere: timing is CNTVCT_EL0 (a 24 MHz wall-clock timer), not a PMU cycle
counter, which is inaccessible in M1 userspace. Every findings file says so and carries a cost-model sanity
line as the substitute. The DAG-parallel "5.64x / 7.7x" model number is superseded by C4b below.

## New findings the original topic did not have

### C5. Whole-column vs real semi-naive reachability, at last measured fairly

The old "delta" scanned the full edge list every round (O(E)/round, not semi-naive). `carrier::reach`
implements real semi-naive (CSR in-edge index + frontier worklist). Over 5 shapes in-harness plus the
scale-runner at 1M-8M nodes, cross-validated. Semi wins 2x-5x on narrow/deep/random graphs (margin grows with
scale: random DAG 0.41x at 1M to 0.22x at 8M), but only ~1.2x on wide many-target graphs, because whole-column
amortizes all 64 tracked targets in one u64 OR per edge-op while semi-naive re-queues a node per bit gained.
Design implication (op's call): the shallow-regime whole-column choice is vindicated for wide shallow
many-target lease graphs; semi-naive is clearly better for deep narrow ones. The answer is shape-dependent,
which the old bench could not show.

### Sharded interner merge (audit-catalogue lever)

`scale-runner intern` (`carrier::sharded_intern`): real byte-hashing interner, single-threaded vs sharded
parallel intern + sequential merge tail, cross-validated on the canonical first-occurrence signature. Finding:
sharding pays but the merge tail caps it, and the dedup rate sets the tail size. High dedup (few distinct) ->
tiny constant tail -> 3.6x at S=8; med/low dedup -> tail GROWS with shard count (each shard rediscovers more
vocabulary) -> only 1.4-1.75x, cancelling the parallel gain at S=2 med. Implication (op's call): shard the
high-dedup identifier stream, keep low-dedup literals single-threaded or on a shared table; >4 threads earns
little on M1 (big.LITTLE + growing tail).

### Retraction. Recompute vs counted differential on delete (audit-catalogue item)

`scale-runner retract` (`carrier::retract`): the owed non-additive-edit bench. Single-target reach-to-sink;
delete propagates a counted (DBSP-style) retraction cascade vs a from-scratch backward BFS, cross-validated
identical. Clean crossover, governed by how many nodes actually stop reaching the sink (not the delete count):
counted wins 100-5000x for localised retractions, recompute wins for catastrophic ones (most nodes drop, so
the cascade visits everything while recompute rebuilds over a shrunken graph). Dense graphs barely retract, so
counted is nearly free. Implication (op's call): maintain counts and cascade on delete by default; fall back to
recompute when the expected drop count approaches the graph size.

### C4b. Threaded level-sync compile: real number replaces the frictionless model

`scale-runner threads`: persistent thread pool, per-level barriers, real compile per module, atomic
work-stealing, XOR cross-validation. On M1 (4P+4E cores) parallel compile peaks at 2.3x (4 threads) and
degrades at 8, far below the model's 5.6-7.7x, capped by big.LITTLE, the level-sync barrier, and memory
contention. Content-addressed caching (C4a) is the larger, more reliable incremental win; the two compose.
Machine-specific; re-run on target hardware before quoting a parallel multiple.

### Eqsat. The bounded window catches the associativity explosion, free

`scale-runner eqsat`: a real e-graph, bounded(512) vs unbounded over a reassociation chain. Bounded stays
~512 e-nodes and extracts the identical optimal cost as the unbounded run, which explodes 2.5k->399k e-nodes
(K=8..14) and 9.7ms->3004ms (K>=18 hits a 2M ceiling). Bounded is 17-17000x faster at zero extraction-quality
cost, cross-validated on value. Design implication (op's call): the bounded e-graph is vindicated on the case
meant to break it, for the assoc/comm rewrite set; re-run with the real rewrite set when it lands.

### Value representation (new axis, static vs tagged vs NaN-boxed)

`valrepr`: the carrier's fourth designed variant axis, previously unbenched. A mixed int/float program
interpreted with values raw (type static per node), runtime-tagged, or NaN-boxed, cross-validated. Honest
scope: monomorphic op sites, so it measures the interpreter-tier representation-CARRYING cost, not the
megamorphic misprediction cost (a compiled/JIT-tier concern). Finding: the cost is small and regime-dependent.
Compute-bound (n<=1024) static is marginally fastest (tagged +5-7%, nanbox ~tie); memory-bound (n>=4096) the
overhead inverts (nanbox 15-17%, tagged 9% FASTER), the tag/box work hiding under memory-load latency
(hypothesis; no M1 userspace PMU to confirm). Implication (op's call): dynamically-typed values in the
baseline interpreter are not the liability often assumed; the static-typing win lives in the compiled tier.

### The 23 port benches

Run + findings filled (`results/<name>/FINDINGS.md`), cross-validated, cost-model lines. Load-bearing ones:
thermometer effect encoding 24-69x over branch-max (vectorizes at scale); jump-table never beats if-chain for
match lowering on M1 (unpredictable indirect branch); flat shadow-stack resolve 5-20x over hashed-per-scope
(growing); record copy-on-write reaches full-copy cost by 60% sharing; monomorphic field access wants a direct
offset, megamorphic converges to a dead heat (the IC hazard). A fine size sweep corrected a coarse-sweep
reading: what looked like an isolated n=4096 spike in two benches (push-fusion, zig-tail) is a mid-size
working-set/cache transition spanning n~1024-6144 (peak ~3.8x at n=2048-3072), converging at large n as both
sides stream from DRAM. A machine effect, mapped, not a variant property. `cheap_lowering`'s
node-count metric (emitted in the variant output high-32-bits) is now extracted via a small dlopen probe and
committed: CSE reduces node count 59-75% (growing with N) at 1.5-2.7x lowering latency, so it is clearly worth it.

### The five standalone scaling benches, ported to the carrier (audit C7)

The audit named five standalone scaling benches with header-only or non-carrier data. All are now carrier-based
and cross-validated on the scale-runner: reach-8M (C5), record-width-past-L2 (both above), plus three ported
this pass. arena-locality: interp throughput is flat vs the operand backward-read window through an 8MB window
(whole L2), +16% only past L2 at 16MB, so operand-locality tuning buys <=16% and only at extreme windows real
programs never reach (the old standalone swept to 512 nodes and missed the cliff). value-arena: encode ~2GiB/s,
parse zero-copy (free), interpret latency-bound (same ~78ms for 128MB rec16 and 192MB rec24), confirming 16B is
as good as 24B for the interpreter from the throughput side. cfg-interp: a register-CFG interp over a nested
loop, 3.0 ns/step (9.6 cyc); the per-step cost is interpreter dispatch overhead (~5-9x a native loop). A
predictable-vs-unpredictable-branch variant isolates the misprediction tax: a 50%-mispredicting branch adds
+0.7 cyc/step (~+7%), which is the full ~13-cyc M1 penalty per data-dependent branch diluted across ~9
interpreted steps. Control-flow density alone does not slow the interp; unpredictable branches do, in
proportion to branch density.

## Headline status table

| topic headline | status | corrected number |
|---|---|---|
| native ceiling 1.0-1.2x | retracted | ~2.0x (switch), ~2.2x (fntable) |
| 24B precisely optimal | not supported | 12-32B tie; 16B a live contender |
| switch refutes Deegen | reframed | tail 1.5-4x slower on Zig 0.16 std ABI; Deegen blocked on preserve_none |
| incremental warm 1300x | deleted | warm 2.5-3.7x faster than cold (ratio scales with compile/hash) |
| hardware-counter timing | corrected | wall-clock CNTVCT + cost-model line |
| whole-column vs delta (unrun) | now measured | semi 2-5x deep/narrow, ~1.2x wide/many-target |
| DAG parallel 5.6-7.7x | superseded | 2.3x peak (4 threads) on M1, degrades at 8 |
| eqsat bound (unstated) | measured | bound catches explosion at zero quality cost |

## What is owed next

- Feed these into the design topic (op's edit; the topic is the design record).
- Record-width bandwidth past L2 on the scale-runner (the one regime not yet run).
- C6 sketch verification-strength labeling (the sketches, not the benches): gate `adversarial.zig` as test
  blocks, add a strength column to the topic's WORKS table, a 3x3 thermometer-lattice exhaustive check, a
  property test for the N*W let-nesting bound.
- Delete the superseded `hx_*` benches per no-legacy-shims (the carrier ports replace them; the
  arch_/branch_/multiway_/showdown_ branch-strategy benches are not superseded and stay).
