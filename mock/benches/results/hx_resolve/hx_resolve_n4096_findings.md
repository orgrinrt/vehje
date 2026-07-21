# Name resolution: linear scope-chain vs flat shadow-stack

2 variants, 6 samples per variant.
Baseline: **hx_resolve__flat**

## Highlights

Baseline for all deltas below: **hx_resolve__flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_resolve__flat dominates: 1201% faster than the next best (hx_resolve__linear)

hx_resolve__flat (4.23 us) leads hx_resolve__linear (54.99 us) by 1201%, a clear separation rather than a photo finish. CV 7.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_resolve__flat)

The baseline hx_resolve__flat is the fastest (4.23 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 13.0x the fastest

Fastest hx_resolve__flat (4.23 us) to slowest hx_resolve__linear (54.99 us): 13.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_resolve__flat) is the fastest** at 4227.5 ns median
- 1 variant significantly slower than baseline
- Spread: 13.01x (fastest 4227.5 ns, slowest 54990.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_resolve__flat | 6530ns | 6664ns | 5852ns | 6398ns | 7067ns | base |
| hx_resolve__linear | 55866ns | 57388ns | 44040ns | 55056ns | 62994ns | +755.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_resolve__flat | 4134ns | 3689ns | 4477ns | base | 0.991 |
| hx_resolve__linear | 53394ns | 41837ns | 60438ns | +1191.54% | 0.077 |

## Performance model

- Peak throughput: **1.110 Gops/s** (hx_resolve__flat; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_resolve__flat | 0.969 | 87.3% |
| hx_resolve__linear | 0.074 | 6.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_resolve__flat | 6530ns | 6530ns | base |
| hx_resolve__linear | 55866ns | 55866ns | +755.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_resolve__flat | 4228ns | base | --- | [3698, 4477] | --- | --- | --- | --- |
| hx_resolve__linear | 54990ns | +50951.5ns (+1205.2%) | [+40654, +56173]ns | [44752, 60438] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_resolve__flat | hx_resolve__linear |
|---|---|---|
| 1 | 3689ns | +1034.0% |
| 2 | 3706ns | +1275.1% |
| 3 | 4447ns | +1228.9% |
| 4 | 4508ns | +957.5% |
| 5 | 4144ns | +1391.0% |
| 6 | 4311ns | +1268.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_resolve__flat | 0.275 | moderate+ |
| hx_resolve__linear | -0.063 | ok |

**Consistency summary:**

- **hx_resolve__linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_resolve__flat | 3.1ns | 4134.1ns | 0.1% |  |
| hx_resolve__linear | 2.8ns | 53393.6ns | 0.0% |  |

## Distribution (algo ns)

```
hx_resolve__flat (n=6, range 3689.2-4477.1 ns)
   3689.2 |########################################
   3728.6 |
   3768.0 |
   3807.4 |
   3846.8 |
   3886.2 |
   3925.6 |
   3965.0 |
   4004.4 |
   4043.8 |
   4083.2 |
   4122.5 |####################
   4161.9 |
   4201.3 |
   4240.7 |
   4280.1 |####################
   4319.5 |
   4358.9 |
   4398.3 |
   4437.7 |####################
  (0 below, 1 above range)

hx_resolve__linear (n=6, range 41836.7-60438.3 ns)
  41836.7 |####################
  42766.8 |
  43696.9 |
  44626.9 |
  45557.0 |
  46487.1 |
  47417.2 |####################
  48347.3 |
  49277.4 |
  50207.4 |####################
  51137.5 |
  52067.6 |
  52997.7 |
  53927.8 |
  54857.9 |
  55787.9 |
  56718.0 |
  57648.1 |
  58578.2 |########################################
  59508.3 |
  (0 below, 1 above range)

```
