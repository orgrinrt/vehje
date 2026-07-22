# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 140% faster than the next best (zig_tail)

zig_switch (54.50 us) leads zig_tail (130.60 us) by 140%, a clear separation rather than a photo finish. CV 5.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (54.50 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### zig_tail is inconsistent: worst-20% is 1.5x its best-20%

zig_tail's best 20% of batches run at 87.87 us but its worst 20% at 135.37 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (zig_switch) is the fastest** at 54500.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.40x (fastest 54500.8 ns, slowest 130604.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 55792ns | 56973ns | 49482ns | 56907ns | 57276ns | base |
| zig_tail | 126409ns | 133127ns | 90439ns | 130867ns | 137706ns | +126.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 53367ns | 47342ns | 54794ns | base | 0.038 |
| zig_tail | 123971ns | 87871ns | 135366ns | +132.30% | 0.017 |

## Performance model

- Peak throughput: **0.043 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.038 | 86.9% |
| zig_tail | 0.016 | 36.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 55792ns | 55792ns | base |
| zig_tail | 126409ns | 126409ns | +126.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 54501ns | base | --- | [50805, 54794] | --- | --- | --- | --- |
| zig_tail | 130605ns | +76268.0ns (+139.9%) | [+51149, +84397]ns | [105942, 135366] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 47342ns | +181.9% |
| 2 | 54269ns | +142.7% |
| 3 | 54598ns | +151.4% |
| 4 | 54661ns | +60.8% |
| 5 | 54404ns | +138.0% |
| 6 | 54926ns | +125.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | 0.005 | ok |
| zig_tail | -0.303 | moderate- |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 1.4ns | 53366.5ns | 0.0% |  |
| zig_tail | 2.2ns | 123971.0ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 47341.7-54793.5 ns)
  47341.7 |####################
  47714.3 |
  48086.9 |
  48459.5 |
  48832.1 |
  49204.6 |
  49577.2 |
  49949.8 |
  50322.4 |
  50695.0 |
  51067.6 |
  51440.2 |
  51812.8 |
  52185.4 |
  52558.0 |
  52930.6 |
  53303.1 |
  53675.7 |
  54048.3 |########################################
  54420.9 |########################################
  (0 below, 1 above range)

zig_tail (n=6, range 87871.2-135366.2 ns)
  87871.2 |########################################
  90246.0 |
  92620.7 |
  94995.5 |
  97370.2 |
  99745.0 |
  102119.7 |
  104494.5 |
  106869.2 |
  109244.0 |
  111618.7 |
  113993.5 |
  116368.2 |
  118743.0 |
  121117.7 |
  123492.5 |########################################
  125867.2 |
  128242.0 |########################################
  130616.7 |########################################
  132991.5 |########################################
  (0 below, 1 above range)

```
