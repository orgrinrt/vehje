# Record update: always-copy vs in-place-when-unique (exact-meet)

2 variants, 6 samples per variant.
Baseline: **hx_reuse__reuse**

## Highlights

Baseline for all deltas below: **hx_reuse__reuse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (40 ns) is smaller than the fastest variant's own run-to-run std-dev (704 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (hx_reuse__reuse)

The baseline hx_reuse__reuse is the fastest (7.63 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### hx_reuse__copy's edge over baseline is significant but tiny (12 ns, 0.16%)

hx_reuse__copy differs from baseline hx_reuse__reuse by 12 ns (0.16%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (hx_reuse__reuse) is the fastest** at 7626.4 ns median
- Spread: 1.01x (fastest 7626.4 ns, slowest 7666.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reuse__copy | 10530ns | 9840ns | 9751ns | 9821ns | 11982ns | +1.04% |
| hx_reuse__reuse | 10421ns | 9797ns | 9771ns | 9789ns | 11695ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reuse__copy | 8197ns | 7581ns | 9329ns | +1.04% | 1.999 |
| hx_reuse__reuse | 8113ns | 7603ns | 9109ns | base | 2.020 |

## Performance model

- Peak throughput: **2.161 Gops/s** (hx_reuse__copy; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reuse__copy | 2.137 | 98.9% |
| hx_reuse__reuse | 2.148 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reuse__copy | 10530ns | 10530ns | +1.04% |
| hx_reuse__reuse | 10421ns | 10421ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reuse__reuse | 7626ns | base | --- | [7603, 9109] | --- | --- | --- | --- |
| hx_reuse__copy | 7667ns | no significant difference | [-16, +255]ns | [7594, 9329] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reuse__reuse | hx_reuse__copy |
|---|---|---|
| 1 | 7603ns | +0.0% |
| 2 | 7603ns | +0.3% |
| 3 | 7648ns | +0.8% |
| 4 | 7605ns | -0.3% |
| 5 | 9102ns | -0.1% |
| 6 | 9115ns | +4.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reuse__copy | 0.401 | moderate+ |
| hx_reuse__reuse | 0.410 | moderate+ |

**Consistency summary:**

- **hx_reuse__copy**: won 1/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reuse__copy | 3.6ns | 8196.8ns | 0.0% |  |
| hx_reuse__reuse | 3.5ns | 8112.8ns | 0.0% |  |

## Distribution (algo ns)

```
hx_reuse__copy (n=6, range 7581.2-9329.4 ns)
   7581.2 |########################################
   7668.6 |#############
   7756.0 |
   7843.4 |
   7930.8 |
   8018.2 |
   8105.7 |
   8193.1 |
   8280.5 |
   8367.9 |
   8455.3 |
   8542.7 |
   8630.1 |
   8717.5 |
   8804.9 |
   8892.4 |
   8979.8 |
   9067.2 |#############
   9154.6 |
   9242.0 |
  (0 below, 1 above range)

hx_reuse__reuse (n=6, range 7603.3-9108.5 ns)
   7603.3 |########################################
   7678.6 |
   7753.8 |
   7829.1 |
   7904.4 |
   7979.6 |
   8054.9 |
   8130.1 |
   8205.4 |
   8280.7 |
   8355.9 |
   8431.2 |
   8506.4 |
   8581.7 |
   8657.0 |
   8732.2 |
   8807.5 |
   8882.8 |
   8958.0 |
   9033.3 |##########
  (0 below, 1 above range)

```
