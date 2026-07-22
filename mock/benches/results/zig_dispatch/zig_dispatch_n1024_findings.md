# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 68% faster than the next best (zig_tail)

zig_switch (27.26 us) leads zig_tail (45.89 us) by 68%, a clear separation rather than a photo finish. CV 5.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### zig_switch is fastest but the noisiest (CV 5.3%)

zig_switch wins on median (27.26 us) yet has the highest variance (CV 5.3%), while zig_tail is the steadiest (CV 3.3%, 45.89 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (27.26 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (zig_switch) is the fastest** at 27260.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.68x (fastest 27260.6 ns, slowest 45889.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 29015ns | 29711ns | 26568ns | 28971ns | 30306ns | base |
| zig_tail | 47840ns | 48364ns | 44415ns | 48170ns | 49056ns | +64.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 26574ns | 24385ns | 27853ns | base | 0.039 |
| zig_tail | 45366ns | 42112ns | 46472ns | +70.72% | 0.023 |

## Performance model

- Peak throughput: **0.042 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.038 | 89.5% |
| zig_tail | 0.022 | 53.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 29015ns | 29015ns | base |
| zig_tail | 47840ns | 47840ns | +64.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 27261ns | base | --- | [24607, 27853] | --- | --- | --- | --- |
| zig_tail | 45890ns | +18523.2ns (+67.9%) | [+17199, +20655]ns | [43736, 46472] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 24829ns | +69.6% |
| 2 | 24385ns | +90.4% |
| 3 | 27248ns | +70.7% |
| 4 | 27273ns | +66.6% |
| 5 | 27460ns | +68.8% |
| 6 | 28246ns | +60.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | 0.398 | moderate+ |
| zig_tail | -0.151 | ok |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 2.1ns | 26573.6ns | 0.0% |  |
| zig_tail | 2.9ns | 45365.8ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 24385.4-27852.9 ns)
  24385.4 |####################
  24558.8 |
  24732.2 |####################
  24905.5 |
  25078.9 |
  25252.3 |
  25425.7 |
  25599.0 |
  25772.4 |
  25945.8 |
  26119.2 |
  26292.5 |
  26465.9 |
  26639.3 |
  26812.7 |
  26986.0 |
  27159.4 |########################################
  27332.8 |####################
  27506.2 |
  27679.5 |
  (0 below, 1 above range)

zig_tail (n=6, range 42112.5-46471.6 ns)
  42112.5 |####################
  42330.5 |
  42548.4 |
  42766.4 |
  42984.3 |
  43202.3 |
  43420.2 |
  43638.2 |
  43856.2 |
  44074.1 |
  44292.1 |
  44510.0 |
  44728.0 |
  44945.9 |
  45163.9 |####################
  45381.9 |####################
  45599.8 |
  45817.8 |
  46035.7 |
  46253.7 |########################################
  (0 below, 1 above range)

```
