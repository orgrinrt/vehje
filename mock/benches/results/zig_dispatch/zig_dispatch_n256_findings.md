# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 90% faster than the next best (zig_tail)

zig_switch (5.95 us) leads zig_tail (11.31 us) by 90%, a clear separation rather than a photo finish. CV 9.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### zig_switch is fastest but the noisiest (CV 9.4%)

zig_switch wins on median (5.95 us) yet has the highest variance (CV 9.4%), while zig_tail is the steadiest (CV 8.6%, 11.31 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (5.95 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (zig_switch) is the fastest** at 5948.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.90x (fastest 5948.9 ns, slowest 11313.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 8567ns | 8124ns | 7798ns | 8107ns | 9642ns | base |
| zig_tail | 13680ns | 13696ns | 12460ns | 13293ns | 14871ns | +59.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 6266ns | 5696ns | 7048ns | base | 0.041 |
| zig_tail | 11290ns | 10284ns | 12269ns | +80.19% | 0.023 |

## Performance model

- Peak throughput: **0.045 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.043 | 95.7% |
| zig_tail | 0.023 | 50.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 8567ns | 8567ns | base |
| zig_tail | 13680ns | 13680ns | +59.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 5949ns | base | --- | [5800, 7048] | --- | --- | --- | --- |
| zig_tail | 11313ns | +4839.4ns (+81.3%) | [+4340, +5895]ns | [10289, 12269] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 5696ns | +115.5% |
| 2 | 7040ns | +74.0% |
| 3 | 7057ns | +73.8% |
| 4 | 5964ns | +72.6% |
| 5 | 5934ns | +73.3% |
| 6 | 5903ns | +75.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | 0.081 | ok |
| zig_tail | 0.499 | moderate+ |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 1.7ns | 6265.6ns | 0.0% |  |
| zig_tail | 1.8ns | 11290.3ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 5695.8-7048.4 ns)
   5695.8 |#############
   5763.4 |
   5831.1 |
   5898.7 |########################################
   5966.3 |
   6033.9 |
   6101.6 |
   6169.2 |
   6236.8 |
   6304.4 |
   6372.1 |
   6439.7 |
   6507.3 |
   6575.0 |
   6642.6 |
   6710.2 |
   6777.8 |
   6845.5 |
   6913.1 |
   6980.7 |#############
  (0 below, 1 above range)

zig_tail (n=6, range 10284.2-12268.8 ns)
  10284.2 |########################################
  10383.4 |
  10482.7 |
  10581.9 |
  10681.1 |
  10780.3 |
  10879.6 |
  10978.8 |
  11078.0 |
  11177.2 |
  11276.5 |
  11375.7 |
  11474.9 |
  11574.2 |
  11673.4 |
  11772.6 |
  11871.8 |
  11971.1 |
  12070.3 |
  12169.5 |##########################
  (0 below, 1 above range)

```
