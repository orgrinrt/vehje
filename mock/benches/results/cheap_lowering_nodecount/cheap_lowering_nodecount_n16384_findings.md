# Cheap lowering node-count reduction: fold+CSE vs fold-only (metric = node count, not time)

2 variants, 6 samples per variant.
Baseline: **cl_foldonly**

## Highlights

Baseline for all deltas below: **cl_foldonly**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cl_foldonly dominates: 67% faster than the next best (cl_foldcse)

cl_foldonly (91.71 us) leads cl_foldcse (153.39 us) by 67%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (cl_foldonly)

The baseline cl_foldonly is the fastest (91.71 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (cl_foldonly) is the fastest** at 91708.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.67x (fastest 91708.8 ns, slowest 153391.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cl_foldcse | 156796ns | 155688ns | 149868ns | 154852ns | 163177ns | +66.59% |
| cl_foldonly | 94122ns | 93982ns | 90808ns | 93034ns | 97413ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cl_foldcse | 154502ns | 147585ns | 160835ns | +68.25% | 0.106 |
| cl_foldonly | 91826ns | 88611ns | 94997ns | base | 0.178 |

## Performance model

- Peak throughput: **0.185 Gops/s** (cl_foldonly; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cl_foldcse | 0.107 | 57.8% |
| cl_foldonly | 0.179 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cl_foldcse | 156796ns | 156796ns | +66.59% |
| cl_foldonly | 94122ns | 94122ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cl_foldonly | 91709ns | base | --- | [88773, 94997] | --- | --- | --- | --- |
| cl_foldcse | 153392ns | +63041.8ns (+68.7%) | [+56683, +68303]ns | [149279, 160835] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | cl_foldonly | cl_foldcse |
|---|---|---|
| 1 | 88935ns | +71.8% |
| 2 | 88611ns | +73.8% |
| 3 | 89464ns | +68.8% |
| 4 | 94265ns | +66.0% |
| 5 | 93954ns | +75.8% |
| 6 | 95730ns | +54.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cl_foldcse | -0.314 | moderate- |
| cl_foldonly | 0.493 | moderate+ |

**Consistency summary:**

- **cl_foldcse**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cl_foldcse | 33776.3ns | 154502.0ns | 21.9% | HIGH |
| cl_foldonly | 23621.8ns | 91826.2ns | 25.7% | HIGH |

## Distribution (algo ns)

```
cl_foldcse (n=6, range 147584.6-160834.8 ns)
  147584.6 |########################################
  148247.1 |
  148909.6 |
  149572.1 |
  150234.6 |
  150897.1 |########################################
  151559.7 |
  152222.2 |########################################
  152884.7 |
  153547.2 |########################################
  154209.7 |
  154872.2 |
  155534.7 |
  156197.2 |########################################
  156859.7 |
  157522.2 |
  158184.8 |
  158847.3 |
  159509.8 |
  160172.3 |
  (0 below, 1 above range)

cl_foldonly (n=6, range 88610.8-94997.1 ns)
  88610.8 |########################################
  88930.1 |########################################
  89249.4 |########################################
  89568.7 |
  89888.1 |
  90207.4 |
  90526.7 |
  90846.0 |
  91165.3 |
  91484.6 |
  91804.0 |
  92123.3 |
  92442.6 |
  92761.9 |
  93081.2 |
  93400.5 |
  93719.8 |########################################
  94039.2 |########################################
  94358.5 |
  94677.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **cl_foldcse**: bridge=22.0% of algo (FFI overhead may distort results)
- **cl_foldonly**: bridge=25.8% of algo (FFI overhead may distort results)
