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

## Result (fill after run, read node counts from output high 32 bits)

| n | N (input) | cl_foldonly count | cl_foldcse count | reduction |
|---|---|---|---|---|
| 64 | 64 | 64 | | |
| 256 | 256 | 256 | | |
| 1024 | 1024 | 1024 | | |
| 4096 | 4096 | 4096 | | |
| 16384 | 16384 | 16384 | | |

Decode: `count = output_u64 >> 32`. Reduction = `1 - foldcse_count / N`.

## Reading (fill after run)

State the node-count reduction on the redundancy-heavy IR (the fold+CSE benefit),
and note explicitly that this is a node-count metric, correcting the old latency
framing: the pass shrinks what must be emitted; its per-node time is dominated by
hash-cons probes and is not the figure of merit, because the recompute LLVM would
otherwise do is already CSE'd downstream.
