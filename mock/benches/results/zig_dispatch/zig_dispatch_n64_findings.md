# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 64% faster than the next best (zig_tail)

zig_switch (1.82 us) leads zig_tail (2.98 us) by 64%, a clear separation rather than a photo finish. CV 8.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### zig_switch is fastest but the noisiest (CV 8.2%)

zig_switch wins on median (1.82 us) yet has the highest variance (CV 8.2%), while zig_tail is the steadiest (CV 2.5%, 2.98 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (1.82 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (zig_switch) is the fastest** at 1817.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.64x (fastest 1817.3 ns, slowest 2980.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 4332ns | 4409ns | 3545ns | 4401ns | 4621ns | base |
| zig_tail | 5553ns | 5599ns | 5222ns | 5592ns | 5660ns | +28.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 1785ns | 1468ns | 1898ns | base | 0.036 |
| zig_tail | 2967ns | 2811ns | 3029ns | +66.22% | 0.022 |

## Performance model

- Peak throughput: **0.044 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.035 | 80.8% |
| zig_tail | 0.021 | 49.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 4332ns | 4332ns | base |
| zig_tail | 5553ns | 5553ns | +28.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 1817ns | base | --- | [1640, 1898] | --- | --- | --- | --- |
| zig_tail | 2980ns | +1165.0ns (+64.1%) | [+1107, +1274]ns | [2892, 3029] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 1468ns | +91.6% |
| 2 | 1818ns | +63.8% |
| 3 | 1813ns | +64.6% |
| 4 | 1944ns | +54.4% |
| 5 | 1851ns | +65.1% |
| 6 | 1817ns | +63.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | 0.058 | ok |
| zig_tail | 0.084 | ok |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 2.2ns | 1785.1ns | 0.1% |  |
| zig_tail | 3.8ns | 2967.1ns | 0.1% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 1467.5-1897.7 ns)
   1467.5 |#############
   1489.0 |
   1510.5 |
   1532.0 |
   1553.5 |
   1575.0 |
   1596.6 |
   1618.1 |
   1639.6 |
   1661.1 |
   1682.6 |
   1704.1 |
   1725.6 |
   1747.1 |
   1768.6 |
   1790.2 |
   1811.7 |########################################
   1833.2 |#############
   1854.7 |
   1876.2 |
  (0 below, 1 above range)

zig_tail (n=6, range 2811.2-3028.7 ns)
   2811.2 |####################
   2822.1 |
   2832.9 |
   2843.8 |
   2854.7 |
   2865.6 |
   2876.4 |
   2887.3 |
   2898.2 |
   2909.1 |
   2919.9 |
   2930.8 |
   2941.7 |
   2952.6 |
   2963.4 |####################
   2974.3 |########################################
   2985.2 |
   2996.1 |####################
   3006.9 |
   3017.8 |
  (0 below, 1 above range)

```
