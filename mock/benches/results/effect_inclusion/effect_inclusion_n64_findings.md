# Effect inclusion gate: thermometer subset test vs naive per-family compare

2 variants, 6 samples per variant.
Baseline: **eg_thermo**

## Highlights

Baseline for all deltas below: **eg_thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### eg_thermo dominates: 2363% faster than the next best (eg_branchmax)

eg_thermo (175 ns) leads eg_branchmax (4.32 us) by 2363%, a clear separation rather than a photo finish. CV 29.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### eg_thermo is fastest but the noisiest (CV 29.5%)

eg_thermo wins on median (175 ns) yet has the highest variance (CV 29.5%), while eg_branchmax is the steadiest (CV 25.9%, 4.32 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (eg_thermo)

The baseline eg_thermo is the fastest (175 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 24.6x the fastest

Fastest eg_thermo (175 ns) to slowest eg_branchmax (4.32 us): 24.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### eg_thermo is inconsistent: worst-20% is 1.8x its best-20%

eg_thermo's best 20% of batches run at 138 ns but its worst 20% at 242 ns (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (eg_thermo) is the fastest** at 175.4 ns median
- 1 variant significantly slower than baseline
- Spread: 24.63x (fastest 175.4 ns, slowest 4320.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| eg_branchmax | 7402ns | 6945ns | 5935ns | 6612ns | 9321ns | +139.84% |
| eg_thermo | 3086ns | 2902ns | 2290ns | 2726ns | 4025ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| eg_branchmax | 4603ns | 3686ns | 5797ns | +2376.05% | 0.014 |
| eg_thermo | 186ns | 138ns | 242ns | base | 0.344 |

## Performance model

- Peak throughput: **0.464 Gops/s** (eg_thermo; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| eg_branchmax | 0.015 | 3.2% |
| eg_thermo | 0.365 | 78.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| eg_branchmax | 7402ns | 7402ns | +139.84% |
| eg_thermo | 3086ns | 3086ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| eg_thermo | 175ns | base | --- | [140, 242] | --- | --- | --- | --- |
| eg_branchmax | 4320ns | +4144.6ns (+2363.0%) | [+3552, +5555]ns | [3692, 5797] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | eg_thermo | eg_branchmax |
|---|---|---|
| 1 | 289ns | +2315.7% |
| 2 | 196ns | +2059.8% |
| 3 | 195ns | +2258.2% |
| 4 | 155ns | +2738.9% |
| 5 | 142ns | +2501.1% |
| 6 | 138ns | +2581.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| eg_branchmax | 0.015 | ok |
| eg_thermo | 0.268 | moderate+ |

**Consistency summary:**

- **eg_branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| eg_branchmax | 3.8ns | 4603.0ns | 0.1% |  |
| eg_thermo | 4.9ns | 185.9ns | 2.6% |  |

## Distribution (algo ns)

```
eg_branchmax (n=6, range 3685.8-5797.0 ns)
   3685.8 |########################################
   3791.4 |
   3896.9 |
   4002.5 |
   4108.1 |
   4213.6 |####################
   4319.2 |####################
   4424.7 |
   4530.3 |####################
   4635.9 |
   4741.4 |
   4847.0 |
   4952.5 |
   5058.1 |
   5163.7 |
   5269.2 |
   5374.8 |
   5480.4 |
   5585.9 |
   5691.5 |
  (0 below, 1 above range)

eg_thermo (n=6, range 137.9-242.5 ns)
    137.9 |########################################
    143.1 |
    148.4 |
    153.6 |####################
    158.8 |
    164.1 |
    169.3 |
    174.5 |
    179.7 |
    185.0 |
    190.2 |####################
    195.4 |####################
    200.7 |
    205.9 |
    211.1 |
    216.3 |
    221.6 |
    226.8 |
    232.0 |
    237.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **eg_branchmax**: CV=24.3% (high variance, measurements may be unstable)
- **eg_thermo**: CV=27.8% (high variance, measurements may be unstable)
