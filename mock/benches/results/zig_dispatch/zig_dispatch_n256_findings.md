# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 48% faster than the next best (zig_tail)

zig_switch (6.12 us) leads zig_tail (9.08 us) by 48%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (6.12 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### zig_tail is inconsistent: worst-20% is 2.1x its best-20%

zig_tail's best 20% of batches run at 9.05 us but its worst 20% at 19.26 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (zig_switch) is the fastest** at 6121.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.48x (fastest 6121.2 ns, slowest 9081.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 8457ns | 8396ns | 8124ns | 8306ns | 8850ns | base |
| zig_tail | 15119ns | 11335ns | 11304ns | 11329ns | 22710ns | +78.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 6156ns | 5918ns | 6409ns | base | 0.042 |
| zig_tail | 12465ns | 9049ns | 19264ns | +102.48% | 0.021 |

## Performance model

- Peak throughput: **0.043 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.042 | 96.7% |
| zig_tail | 0.028 | 65.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 8457ns | 8457ns | base |
| zig_tail | 15119ns | 15119ns | +78.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 6121ns | base | --- | [5939, 6409] | --- | --- | --- | --- |
| zig_tail | 9081ns | +3046.5ns (+49.8%) | [+2853, +13026]ns | [9049, 19264] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 5959ns | +290.2% |
| 2 | 6517ns | +134.4% |
| 3 | 6300ns | +44.2% |
| 4 | 6127ns | +47.7% |
| 5 | 6115ns | +48.4% |
| 6 | 5918ns | +52.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | -0.049 | ok |
| zig_tail | 0.325 | moderate+ |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 2.3ns | 6156.1ns | 0.0% |  |
| zig_tail | 4.8ns | 12464.6ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 5917.9-6408.5 ns)
   5917.9 |####################
   5942.4 |####################
   5967.0 |
   5991.5 |
   6016.0 |
   6040.6 |
   6065.1 |
   6089.6 |
   6114.2 |########################################
   6138.7 |
   6163.2 |
   6187.8 |
   6212.3 |
   6236.8 |
   6261.4 |
   6285.9 |####################
   6310.4 |
   6335.0 |
   6359.5 |
   6384.0 |
  (0 below, 1 above range)

zig_tail (n=6, range 9048.7-19264.0 ns)
   9048.7 |########################################
   9559.5 |
  10070.2 |
  10581.0 |
  11091.8 |
  11602.5 |
  12113.3 |
  12624.0 |
  13134.8 |
  13645.6 |
  14156.3 |
  14667.1 |
  15177.9 |##########
  15688.6 |
  16199.4 |
  16710.1 |
  17220.9 |
  17731.7 |
  18242.4 |
  18753.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **zig_tail**: CV=42.8% (high variance, measurements may be unstable)
