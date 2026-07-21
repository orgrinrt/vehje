# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 43% faster than the next best (zig_tail)

zig_switch (1.81 us) leads zig_tail (2.60 us) by 43%, a clear separation rather than a photo finish. CV 5.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### zig_switch is fastest but the noisiest (CV 5.0%)

zig_switch wins on median (1.81 us) yet has the highest variance (CV 5.0%), while zig_tail is the steadiest (CV 4.8%, 2.60 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (1.81 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (zig_switch) is the fastest** at 1813.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.43x (fastest 1813.1 ns, slowest 2600.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 4312ns | 4401ns | 3821ns | 4399ns | 4429ns | base |
| zig_tail | 5244ns | 5195ns | 4828ns | 5185ns | 5540ns | +21.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 1778ns | 1576ns | 1829ns | base | 0.036 |
| zig_tail | 2616ns | 2400ns | 2760ns | +47.17% | 0.024 |

## Performance model

- Peak throughput: **0.041 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.035 | 86.9% |
| zig_tail | 0.025 | 60.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 4312ns | 4312ns | base |
| zig_tail | 5244ns | 5244ns | +21.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 1813ns | base | --- | [1690, 1829] | --- | --- | --- | --- |
| zig_tail | 2600ns | +809.0ns (+44.6%) | [+756, +950]ns | [2488, 2760] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 1576ns | +52.2% |
| 2 | 1840ns | +40.0% |
| 3 | 1815ns | +50.5% |
| 4 | 1818ns | +43.7% |
| 5 | 1811ns | +42.9% |
| 6 | 1804ns | +54.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | -0.132 | ok |
| zig_tail | -0.012 | ok |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 2.6ns | 1777.5ns | 0.1% |  |
| zig_tail | 3.1ns | 2616.0ns | 0.1% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 1576.2-1829.3 ns)
   1576.2 |#############
   1588.9 |
   1601.5 |
   1614.2 |
   1626.8 |
   1639.5 |
   1652.1 |
   1664.8 |
   1677.5 |
   1690.1 |
   1702.8 |
   1715.4 |
   1728.1 |
   1740.7 |
   1753.4 |
   1766.1 |
   1778.7 |
   1791.4 |
   1804.0 |########################################
   1816.7 |#############
  (0 below, 1 above range)

zig_tail (n=6, range 2399.6-2759.8 ns)
   2399.6 |########################################
   2417.6 |
   2435.6 |
   2453.6 |
   2471.6 |
   2489.7 |
   2507.7 |
   2525.7 |
   2543.7 |
   2561.7 |########################################
   2579.7 |########################################
   2597.7 |########################################
   2615.7 |
   2633.7 |
   2651.7 |
   2669.8 |
   2687.8 |
   2705.8 |
   2723.8 |########################################
   2741.8 |
  (0 below, 1 above range)

```
