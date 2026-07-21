# Interpolation output: format-to-temp+copy vs format-in-place

2 variants, 6 samples per variant.
Baseline: **hx_output__inplace**

## Highlights

Baseline for all deltas below: **hx_output__inplace**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_output__temp shows alternating (throttle bounce) (autocorr -0.59)

hx_output__temp's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (38 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (hx_output__inplace)

The baseline hx_output__inplace is the fastest (485 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### hx_output__temp's edge over baseline is significant but tiny (-5 ns, 1.02%)

hx_output__temp differs from baseline hx_output__inplace by -5 ns (1.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (hx_output__inplace) is the fastest** at 484.8 ns median
- Spread: 1.01x (fastest 484.8 ns, slowest 488.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_output__inplace | 2987ns | 3059ns | 2677ns | 2944ns | 3206ns | base |
| hx_output__temp | 2946ns | 2982ns | 2574ns | 2939ns | 3144ns | -1.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_output__inplace | 474ns | 417ns | 511ns | base | 0.541 |
| hx_output__temp | 498ns | 409ns | 576ns | +5.27% | 0.514 |

## Performance model

- Peak throughput: **0.626 Gops/s** (hx_output__temp; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_output__inplace | 0.528 | 84.3% |
| hx_output__temp | 0.524 | 83.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_output__inplace | 2987ns | 2987ns | base |
| hx_output__temp | 2946ns | 2946ns | -1.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_output__inplace | 485ns | base | --- | [424, 511] | --- | --- | --- | --- |
| hx_output__temp | 489ns | no significant difference | [-35, +115]ns | [431, 576] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_output__inplace | hx_output__temp |
|---|---|---|
| 1 | 417ns | -2.0% |
| 2 | 432ns | +52.8% |
| 3 | 481ns | -5.9% |
| 4 | 494ns | -0.3% |
| 5 | 529ns | -8.0% |
| 6 | 489ns | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_output__inplace | 0.478 | moderate+ |
| hx_output__temp | -0.589 | HIGH- (thermal bounce) |

**Consistency summary:**

- **hx_output__temp**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_output__inplace | 4.6ns | 473.6ns | 1.0% |  |
| hx_output__temp | 3.3ns | 498.5ns | 0.7% |  |

## Distribution (algo ns)

```
hx_output__inplace (n=6, range 417.1-511.5 ns)
    417.1 |########################################
    421.8 |
    426.5 |
    431.3 |########################################
    436.0 |
    440.7 |
    445.4 |
    450.1 |
    454.8 |
    459.6 |
    464.3 |
    469.0 |
    473.7 |
    478.4 |########################################
    483.1 |
    487.9 |########################################
    492.6 |########################################
    497.3 |
    502.0 |
    506.7 |
  (0 below, 1 above range)

hx_output__temp (n=6, range 408.8-575.9 ns)
    408.8 |#############
    417.2 |
    425.5 |
    433.9 |
    442.2 |
    450.6 |#############
    458.9 |
    467.3 |
    475.6 |
    484.0 |########################################
    492.3 |
    500.7 |
    509.0 |
    517.4 |
    525.7 |
    534.1 |
    542.4 |
    550.8 |
    559.1 |
    567.5 |
  (0 below, 1 above range)

```
