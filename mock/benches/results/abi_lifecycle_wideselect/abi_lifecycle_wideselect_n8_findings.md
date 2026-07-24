# abi_lifecycle (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_wideselect_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_wideselect_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_wideselect_null_entry dominates: 65698% faster than the next best (abi_lifecycle_wideselect_held_handle)

abi_lifecycle_wideselect_null_entry (3.13 us) leads abi_lifecycle_wideselect_held_handle (2.06 ms) by 65698%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_wideselect_null_entry beats baseline by 100% (significant)

abi_lifecycle_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_lifecycle_wideselect_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_wideselect_fresh_per_batch is an outlier: 786.5x slower than the field

abi_lifecycle_wideselect_fresh_per_batch (2.46 ms) is 786.5x the fastest (3.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_wideselect_fresh_per_batch shows alternating (throttle bounce) (autocorr -0.56)

abi_lifecycle_wideselect_fresh_per_batch's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_wideselect_null_entry} vs {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} (65698% apart)

The field splits into a fast tier {abi_lifecycle_wideselect_null_entry} and a slow tier {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} with a 65698% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 786.5x the fastest

Fastest abi_lifecycle_wideselect_null_entry (3.13 us) to slowest abi_lifecycle_wideselect_fresh_per_batch (2.46 ms): 786.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_wideselect_null_entry** at 3126.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 786.53x (fastest 3126.2 ns, slowest 2458882.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2460706ns | 2461521ns | 2452360ns | 2460664ns | 2464942ns | +19.57% |
| abi_lifecycle_wideselect_fresh_per_column | 2068738ns | 2070164ns | 2059870ns | 2067703ns | 2074726ns | +0.52% |
| abi_lifecycle_wideselect_held_handle | 2058035ns | 2059485ns | 2050583ns | 2057271ns | 2062908ns | base |
| abi_lifecycle_wideselect_null_entry | 5450ns | 5434ns | 5348ns | 5407ns | 5565ns | -99.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2458128ns | 2449890ns | 2462392ns | +19.59% | 0.000 |
| abi_lifecycle_wideselect_fresh_per_column | 2066249ns | 2057432ns | 2072122ns | +0.53% | 0.000 |
| abi_lifecycle_wideselect_held_handle | 2055440ns | 2047959ns | 2060123ns | base | 0.000 |
| abi_lifecycle_wideselect_null_entry | 3138ns | 3062ns | 3208ns | -99.85% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 41254.1 | 2458548.6 | 2458128.3 | n/a |
| abi_lifecycle_wideselect_fresh_per_column | 38332.7 | 2067374.4 | 2066248.7 | 0 |
| abi_lifecycle_wideselect_held_handle | 39781.1 | 2054805.6 | 2055440.0 | n/a |
| abi_lifecycle_wideselect_null_entry | 28724.1 | 3197.0 | 3137.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_lifecycle_wideselect_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_wideselect_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_wideselect_held_handle | 0.000 | 0.1% |
| abi_lifecycle_wideselect_null_entry | 0.003 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2460706ns | 2460706ns | +19.57% |
| abi_lifecycle_wideselect_fresh_per_column | 2068738ns | 2068738ns | +0.52% |
| abi_lifecycle_wideselect_held_handle | 2058035ns | 2058035ns | base |
| abi_lifecycle_wideselect_null_entry | 5450ns | 5450ns | -99.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_held_handle | 2057018ns | base | --- | [2049179, 2060123] | --- | --- | --- | --- |
| abi_lifecycle_wideselect_fresh_per_batch | 2458882ns | +402082.7ns (+19.5%) | [+399000, +406983]ns | [2453111, 2462392] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_wideselect_fresh_per_column | 2067745ns | +11276.0ns (+0.5%) | [+2302, +18848]ns | [2058879, 2072122] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_wideselect_null_entry | 3126ns | -2053921.9ns (-99.8%) | [-2056978, -2046007]ns | [3079, 3208] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_wideselect_held_handle | abi_lifecycle_wideselect_fresh_per_batch | abi_lifecycle_wideselect_fresh_per_column | abi_lifecycle_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2050400ns | +20.0% | +0.9% | -99.8% |
| 2 | 2059827ns | +19.2% | +0.4% | -99.8% |
| 3 | 2057888ns | +19.5% | +0.5% | -99.8% |
| 4 | 2047959ns | +19.6% | +0.6% | -99.8% |
| 5 | 2060418ns | +19.6% | -0.1% | -99.9% |
| 6 | 2056148ns | +19.6% | +0.9% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | -0.558 | HIGH- (thermal bounce) |
| abi_lifecycle_wideselect_fresh_per_column | -0.143 | ok |
| abi_lifecycle_wideselect_held_handle | -0.481 | moderate- |
| abi_lifecycle_wideselect_null_entry | 0.085 | ok |

**Consistency summary:**

- **abi_lifecycle_wideselect_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 7415246.4ns | 2458128.3ns | 301.7% | HIGH |
| abi_lifecycle_wideselect_fresh_per_column | 6241358.6ns | 2066248.7ns | 302.1% | HIGH |
| abi_lifecycle_wideselect_held_handle | 6208088.8ns | 2055440.0ns | 302.0% | HIGH |
| abi_lifecycle_wideselect_null_entry | 120591.8ns | 3137.8ns | 3843.2% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_wideselect_fresh_per_batch (n=6, range 2449890.0-2462391.9 ns)
  2449890.0 |########################################
  2450515.1 |
  2451140.2 |
  2451765.3 |
  2452390.4 |
  2453015.5 |
  2453640.6 |
  2454265.7 |
  2454890.8 |
  2455515.9 |
  2456141.0 |########################################
  2456766.0 |
  2457391.1 |
  2458016.2 |########################################
  2458641.3 |
  2459266.4 |########################################
  2459891.5 |
  2460516.6 |########################################
  2461141.7 |
  2461766.8 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_fresh_per_column (n=6, range 2057432.1-2072122.3 ns)
  2057432.1 |########################################
  2058166.6 |
  2058901.1 |
  2059635.6 |########################################
  2060370.1 |
  2061104.6 |
  2061839.2 |
  2062573.7 |
  2063308.2 |
  2064042.7 |
  2064777.2 |
  2065511.7 |
  2066246.2 |
  2066980.7 |########################################
  2067715.2 |########################################
  2068449.8 |
  2069184.3 |########################################
  2069918.8 |
  2070653.3 |
  2071387.8 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_held_handle (n=6, range 2047958.8-2060122.7 ns)
  2047958.8 |########################################
  2048567.0 |
  2049175.2 |
  2049783.4 |
  2050391.6 |########################################
  2050999.8 |
  2051608.0 |
  2052216.2 |
  2052824.4 |
  2053432.6 |
  2054040.8 |
  2054648.9 |
  2055257.1 |
  2055865.3 |########################################
  2056473.5 |
  2057081.7 |
  2057689.9 |########################################
  2058298.1 |
  2058906.3 |
  2059514.5 |########################################
  (0 below, 1 above range)

abi_lifecycle_wideselect_null_entry (n=6, range 3062.1-3208.3 ns)
   3062.1 |####################
   3069.4 |
   3076.7 |
   3084.0 |
   3091.3 |########################################
   3098.7 |
   3106.0 |
   3113.3 |
   3120.6 |
   3127.9 |
   3135.2 |
   3142.5 |
   3149.8 |####################
   3157.1 |
   3164.4 |
   3171.8 |
   3179.1 |
   3186.4 |####################
   3193.7 |
   3201.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_wideselect_fresh_per_batch**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_fresh_per_column**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_held_handle**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_null_entry**: bridge=3851.0% of algo (FFI overhead may distort results)
