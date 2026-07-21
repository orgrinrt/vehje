# Interp output-building: format-to-temp+copy vs in-place vs span-list

3 variants, 6 samples per variant.
Baseline: **interp_out_inplace**

## Highlights

Baseline for all deltas below: **interp_out_inplace**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### interp_out_inplace dominates: 18% faster than the next best (interp_out_temp)

interp_out_inplace (221.06 us) leads interp_out_temp (261.29 us) by 18%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### interp_out_inplace shows alternating (throttle bounce) (autocorr -0.66)

interp_out_inplace's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (interp_out_inplace)

The baseline interp_out_inplace is the fastest (221.06 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (interp_out_inplace) is the fastest** at 221055.5 ns median
- 2 variants significantly slower than baseline
- Spread: 2.00x (fastest 221055.5 ns, slowest 441094.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| interp_out_inplace | 223429ns | 223519ns | 218670ns | 222863ns | 226657ns | base |
| interp_out_spanlist | 441845ns | 443564ns | 435318ns | 441484ns | 445650ns | +97.76% |
| interp_out_temp | 262992ns | 263974ns | 257878ns | 262590ns | 266151ns | +17.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| interp_out_inplace | 220925ns | 216238ns | 223954ns | base | 0.005 |
| interp_out_spanlist | 439258ns | 432880ns | 443038ns | +98.83% | 0.002 |
| interp_out_temp | 260522ns | 255599ns | 263709ns | +17.92% | 0.004 |

## Performance model

- Peak throughput: **0.005 Gops/s** (interp_out_inplace; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| interp_out_inplace | 0.005 | 97.8% |
| interp_out_spanlist | 0.002 | 49.0% |
| interp_out_temp | 0.004 | 82.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| interp_out_inplace | 223429ns | 223429ns | base |
| interp_out_spanlist | 441845ns | 441845ns | +97.76% |
| interp_out_temp | 262992ns | 262992ns | +17.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| interp_out_inplace | 221055ns | base | --- | [217766, 223954] | --- | --- | --- | --- |
| interp_out_spanlist | 441095ns | +217796.6ns (+98.5%) | [+212587, +224616]ns | [433642, 443038] | YES | 0.0313 | 0.0313 | 0 |
| interp_out_temp | 261290ns | +40327.5ns (+18.2%) | [+35782, +42682]ns | [256568, 263709] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | interp_out_inplace | interp_out_spanlist | interp_out_temp |
|---|---|---|---|
| 1 | 219352ns | +98.0% | +18.8% |
| 2 | 224607ns | +96.3% | +16.6% |
| 3 | 216238ns | +105.1% | +18.2% |
| 4 | 223301ns | +98.2% | +15.3% |
| 5 | 222759ns | +94.3% | +18.8% |
| 6 | 219295ns | +101.2% | +19.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| interp_out_inplace | -0.661 | HIGH- (thermal bounce) |
| interp_out_spanlist | -0.207 | moderate- |
| interp_out_temp | 0.080 | ok |

**Consistency summary:**

- **interp_out_spanlist**: won 0/6, lost 6/6
- **interp_out_temp**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| interp_out_inplace | 6.9ns | 220925.2ns | 0.0% |  |
| interp_out_spanlist | 6.8ns | 439258.4ns | 0.0% |  |
| interp_out_temp | 7.7ns | 260522.3ns | 0.0% |  |

## Distribution (algo ns)

```
interp_out_inplace (n=6, range 216238.3-223953.8 ns)
  216238.3 |########################################
  216624.1 |
  217009.8 |
  217395.6 |
  217781.4 |
  218167.2 |
  218552.9 |
  218938.7 |########################################
  219324.5 |########################################
  219710.3 |
  220096.0 |
  220481.8 |
  220867.6 |
  221253.3 |
  221639.1 |
  222024.9 |
  222410.7 |########################################
  222796.4 |
  223182.2 |########################################
  223568.0 |
  (0 below, 1 above range)

interp_out_spanlist (n=6, range 432880.0-443038.3 ns)
  432880.0 |########################################
  433387.9 |
  433895.8 |
  434403.7 |########################################
  434911.7 |
  435419.6 |
  435927.5 |
  436435.4 |
  436943.3 |
  437451.2 |
  437959.2 |
  438467.1 |
  438975.0 |
  439482.9 |
  439990.8 |
  440498.7 |########################################
  441006.6 |########################################
  441514.6 |
  442022.5 |
  442530.4 |########################################
  (0 below, 1 above range)

interp_out_temp (n=6, range 255598.7-263708.8 ns)
  255598.7 |########################################
  256004.2 |
  256409.7 |
  256815.2 |
  257220.7 |########################################
  257626.2 |
  258031.7 |
  258437.2 |
  258842.7 |
  259248.2 |
  259653.7 |
  260059.2 |
  260464.7 |########################################
  260870.2 |
  261275.7 |
  261681.2 |########################################
  262086.7 |
  262492.2 |########################################
  262897.7 |
  263303.2 |
  (0 below, 1 above range)

```
