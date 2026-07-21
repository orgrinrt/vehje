# Interp output-building: format-to-temp+copy vs in-place vs span-list

3 variants, 6 samples per variant.
Baseline: **interp_out_inplace**

## Highlights

Baseline for all deltas below: **interp_out_inplace**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### interp_out_inplace shows alternating (throttle bounce) (autocorr -0.57)

interp_out_inplace's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (interp_out_inplace)

The baseline interp_out_inplace is the fastest (3.27 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (interp_out_inplace) is the fastest** at 3269516.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.25x (fastest 3269516.0 ns, slowest 4070888.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| interp_out_inplace | 3273493ns | 3272666ns | 3248077ns | 3265431ns | 3298295ns | base |
| interp_out_spanlist | 3405560ns | 3407144ns | 3383682ns | 3400573ns | 3423978ns | +4.03% |
| interp_out_temp | 4086479ns | 4074537ns | 4071698ns | 4073662ns | 4113095ns | +24.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| interp_out_inplace | 3270175ns | 3244248ns | 3295074ns | base | 0.005 |
| interp_out_spanlist | 3401970ns | 3379902ns | 3420532ns | +4.03% | 0.005 |
| interp_out_temp | 4082738ns | 4067565ns | 4109655ns | +24.85% | 0.004 |

## Performance model

- Peak throughput: **0.005 Gops/s** (interp_out_inplace; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| interp_out_inplace | 0.005 | 99.2% |
| interp_out_spanlist | 0.005 | 95.3% |
| interp_out_temp | 0.004 | 79.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| interp_out_inplace | 3273493ns | 3273493ns | base |
| interp_out_spanlist | 3405560ns | 3405560ns | +4.03% |
| interp_out_temp | 4086479ns | 4086479ns | +24.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| interp_out_inplace | 3269516ns | base | --- | [3245934, 3295074] | --- | --- | --- | --- |
| interp_out_spanlist | 3403515ns | +143239.4ns (+4.4%) | [+86789, +165357]ns | [3381863, 3420532] | YES | 0.0313 | 0.0313 | 0 |
| interp_out_temp | 4070889ns | +812069.8ns (+24.8%) | [+790615, +835005]ns | [4067670, 4109655] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | interp_out_inplace | interp_out_spanlist | interp_out_temp |
|---|---|---|---|
| 1 | 3269015ns | +4.6% | +24.4% |
| 2 | 3270018ns | +4.2% | +24.5% |
| 3 | 3244248ns | +4.8% | +25.7% |
| 4 | 3285096ns | +3.0% | +23.8% |
| 5 | 3247620ns | +5.4% | +25.3% |
| 6 | 3305052ns | +2.3% | +25.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| interp_out_inplace | -0.574 | HIGH- (thermal bounce) |
| interp_out_spanlist | -0.465 | moderate- |
| interp_out_temp | -0.058 | ok |

**Consistency summary:**

- **interp_out_spanlist**: won 0/6, lost 6/6
- **interp_out_temp**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| interp_out_inplace | 39.6ns | 3270174.8ns | 0.0% |  |
| interp_out_spanlist | 58.0ns | 3401970.1ns | 0.0% |  |
| interp_out_temp | 52.3ns | 4082738.0ns | 0.0% |  |

## Distribution (algo ns)

```
interp_out_inplace (n=6, range 3244248.3-3295074.1 ns)
  3244248.3 |########################################
  3246789.6 |########################################
  3249330.9 |
  3251872.2 |
  3254413.5 |
  3256954.8 |
  3259496.1 |
  3262037.3 |
  3264578.6 |
  3267119.9 |########################################
  3269661.2 |########################################
  3272202.5 |
  3274743.8 |
  3277285.1 |
  3279826.4 |
  3282367.7 |
  3284909.0 |########################################
  3287450.3 |
  3289991.6 |
  3292532.9 |
  (0 below, 1 above range)

interp_out_spanlist (n=6, range 3379901.7-3420532.1 ns)
  3379901.7 |########################################
  3381933.2 |########################################
  3383964.7 |
  3385996.3 |
  3388027.8 |
  3390059.3 |
  3392090.8 |
  3394122.3 |
  3396153.9 |
  3398185.4 |
  3400216.9 |########################################
  3402248.4 |
  3404279.9 |
  3406311.5 |########################################
  3408343.0 |
  3410374.5 |
  3412406.0 |
  3414437.5 |
  3416469.1 |
  3418500.6 |########################################
  (0 below, 1 above range)

interp_out_temp (n=6, range 4067565.0-4109655.2 ns)
  4067565.0 |########################################
  4069669.5 |########################################
  4071774.0 |
  4073878.5 |
  4075983.0 |####################
  4078087.5 |
  4080192.1 |
  4082296.6 |
  4084401.1 |
  4086505.6 |
  4088610.1 |
  4090714.6 |
  4092819.1 |
  4094923.6 |
  4097028.1 |
  4099132.7 |
  4101237.2 |
  4103341.7 |
  4105446.2 |
  4107550.7 |
  (0 below, 1 above range)

```
