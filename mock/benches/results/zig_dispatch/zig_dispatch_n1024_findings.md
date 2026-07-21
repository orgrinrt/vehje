# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 41% faster than the next best (zig_tail)

zig_switch (27.57 us) leads zig_tail (38.80 us) by 41%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (27.57 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (zig_switch) is the fastest** at 27569.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.41x (fastest 27569.8 ns, slowest 38797.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 29589ns | 30028ns | 26574ns | 29753ns | 30852ns | base |
| zig_tail | 41193ns | 41285ns | 40231ns | 41202ns | 41661ns | +39.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 27163ns | 24402ns | 28292ns | base | 0.038 |
| zig_tail | 38704ns | 37864ns | 39088ns | +42.49% | 0.026 |

## Performance model

- Peak throughput: **0.042 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.037 | 88.5% |
| zig_tail | 0.026 | 62.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 29589ns | 29589ns | base |
| zig_tail | 41193ns | 41193ns | +39.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 27570ns | base | --- | [25627, 28292] | --- | --- | --- | --- |
| zig_tail | 38798ns | +11378.3ns (+41.3%) | [+10528, +12716]ns | [38225, 39088] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 24402ns | +55.2% |
| 2 | 26852ns | +44.6% |
| 3 | 28657ns | +35.3% |
| 4 | 27288ns | +41.4% |
| 5 | 27852ns | +41.1% |
| 6 | 27927ns | +39.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | 0.108 | ok |
| zig_tail | -0.063 | ok |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 2.7ns | 27162.9ns | 0.0% |  |
| zig_tail | 2.6ns | 38703.6ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 24402.1-28292.1 ns)
  24402.1 |########################################
  24596.6 |
  24791.1 |
  24985.6 |
  25180.1 |
  25374.6 |
  25569.1 |
  25763.6 |
  25958.1 |
  26152.6 |
  26347.1 |
  26541.6 |
  26736.1 |########################################
  26930.6 |
  27125.1 |########################################
  27319.6 |
  27514.1 |
  27708.6 |########################################
  27903.1 |########################################
  28097.6 |
  (0 below, 1 above range)

zig_tail (n=6, range 37864.2-39087.9 ns)
  37864.2 |########################################
  37925.4 |
  37986.6 |
  38047.8 |
  38108.9 |
  38170.1 |
  38231.3 |
  38292.5 |
  38353.7 |
  38414.9 |
  38476.0 |
  38537.2 |########################################
  38598.4 |
  38659.6 |
  38720.8 |########################################
  38782.0 |########################################
  38843.2 |########################################
  38904.3 |
  38965.5 |
  39026.7 |
  (0 below, 1 above range)

```
