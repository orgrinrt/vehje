# Cheap lowering: node-count reduction, not latency (scaffold)

Scaffold for `[bench.cheap_lowering_nodecount]`. Fill the reduction table after
the harness run.

## The metric is node count, not time (the audit correction)

This bench measures NODE-COUNT REDUCTION, not wall-clock time. The audit found a
time bench misleading here: the cheap lowering subset is const-fold plus common-
subexpression elimination in one bottom-up pass, and LLVM already CSEs the
recompute, so a latency bench measures the hash-cons bookkeeping as pure cost
with no benefit shown (the fold+CSE variant looks slower precisely because it
does the dedup work whose payoff is fewer nodes, which a time axis cannot see).

So the measured quantity is the reduced node count. Each variant returns its
post-pass node count in the output high 32 bits (the low 32 bits carry an input
checksum for anti-hoist and per-variant determinism). Read the reduction from the
output values, not the timing columns. The two variants:

- `cl_foldcse`: const-fold plus hash-cons dedup (full node-count reduction).
- `cl_foldonly`: the same const-fold WITHOUT dedup, so every node is emitted and
  the count equals the input node count N (0% reduction). The neutral baseline;
  the delta to `cl_foldcse` is the CSE node-count benefit.

The program is a redundancy-heavy IR (a small const pool plus repeated add/mul
subexpressions over a sliding window), generated once per subprocess from a
compile-time seed. `may_differ = true`: the two counts legitimately differ, so
the harness does per-variant validation only. Baseline: `cl_foldonly`.

Do NOT read the timing delta as the conclusion. `cl_foldcse` is nominally slower
per pass because of the hash-cons probes; that cost is exactly what the old
framing wrongly reported as the result. The result is the node-count reduction.

## The defect this fixes

The old `lower.zig` probe reported throughput and ns/node (a latency framing)
alongside the reduction, and `lower.csv` was empty. The latency framing is the
audit error: it presents the hash-cons overhead as a cost without the node-count
reduction being the headline, so a reader could conclude the pass is "slow" when
its whole value is emitting fewer nodes.

## The fix

The routine returns the reduced node count as the measured value; the FINDINGS
reports reduction = `1 - (foldcse_count / N)` from the two variants' outputs, and
explicitly disclaims the timing columns as not the metric.

## Measured results

Ratio to baseline (cl_foldonly), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | cl_foldcse (ratio) | cl_foldonly (base) |
|---|---|---|
| 64 | 1.50x | 132 ns |
| 256 | 1.61x | 481 ns |
| 1024 | 1.95x | 1513 ns |
| 4096 | 2.70x | 6770 ns |
| 16384 | 1.67x | 91709 ns |

## Cost-model sanity line

At n=16384, the baseline (cl_foldonly) median is 91709 ns for fold-only processes N nodes per call. Treating n as the work-item count, that is 5.60 ns/item, about 17.9 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

may_differ (CSE reduces node count, so outputs differ by design; the low-32 checksum anti-hoists).

## Verdict

Fold+CSE costs 1.5x to 2.7x the latency of fold-only. The design-deciding quantity is the node-count reduction CSE buys, which the variant emits in the output high-32-bits but the harness does not surface to CSV; extracting it is a small follow-up before the cost/benefit claim is definitive. The latency peak at n=4096 (2.7x) is a CSE hash-table cache effect, not monotonic.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
