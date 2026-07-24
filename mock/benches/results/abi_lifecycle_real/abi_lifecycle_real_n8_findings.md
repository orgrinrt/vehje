# abi_lifecycle (real)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_real_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_real_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_real_null_entry dominates: 69383% faster than the next best (abi_lifecycle_real_held_handle)

abi_lifecycle_real_null_entry (3.09 us) leads abi_lifecycle_real_held_handle (2.15 ms) by 69383%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_real_null_entry beats baseline by 100% (significant)

abi_lifecycle_real_null_entry is -2.14 ms (100%) faster than baseline abi_lifecycle_real_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_real_fresh_per_batch is an outlier: 819.0x slower than the field

abi_lifecycle_real_fresh_per_batch (2.53 ms) is 819.0x the fastest (3.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_real_fresh_per_column shows alternating (throttle bounce) (autocorr -0.74)

abi_lifecycle_real_fresh_per_column's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_real_null_entry} vs {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} (69383% apart)

The field splits into a fast tier {abi_lifecycle_real_null_entry} and a slow tier {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} with a 69383% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 819.0x the fastest

Fastest abi_lifecycle_real_null_entry (3.09 us) to slowest abi_lifecycle_real_fresh_per_batch (2.53 ms): 819.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_real_null_entry** at 3090.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 818.96x (fastest 3090.4 ns, slowest 2530912.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2535438ns | 2533610ns | 2526852ns | 2532491ns | 2544152ns | +17.93% |
| abi_lifecycle_real_fresh_per_column | 2166450ns | 2166408ns | 2160338ns | 2164596ns | 2172285ns | +0.77% |
| abi_lifecycle_real_held_handle | 2149942ns | 2149917ns | 2147899ns | 2149691ns | 2151341ns | base |
| abi_lifecycle_real_null_entry | 5372ns | 5429ns | 5148ns | 5347ns | 5521ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2532752ns | 2524302ns | 2541450ns | +17.95% | 0.000 |
| abi_lifecycle_real_fresh_per_column | 2163783ns | 2157469ns | 2169704ns | +0.77% | 0.000 |
| abi_lifecycle_real_held_handle | 2147302ns | 2145045ns | 2148698ns | base | 0.000 |
| abi_lifecycle_real_null_entry | 3053ns | 2925ns | 3142ns | -99.86% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 40169.9 | 2531777.9 | 2532752.4 | 0 |
| abi_lifecycle_real_fresh_per_column | 42097.1 | 2162962.8 | 2163782.9 | n/a |
| abi_lifecycle_real_held_handle | 40744.2 | 2149018.1 | 2147302.2 | n/a |
| abi_lifecycle_real_null_entry | 27282.4 | 3173.0 | 3053.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_lifecycle_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_real_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_real_held_handle | 0.000 | 0.1% |
| abi_lifecycle_real_null_entry | 0.003 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2535438ns | 2535438ns | +17.93% |
| abi_lifecycle_real_fresh_per_column | 2166450ns | 2166450ns | +0.77% |
| abi_lifecycle_real_held_handle | 2149942ns | 2149942ns | base |
| abi_lifecycle_real_null_entry | 5372ns | 5372ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_real_held_handle | 2147298ns | base | --- | [2145911, 2148698] | --- | --- | --- | --- |
| abi_lifecycle_real_fresh_per_batch | 2530913ns | +383273.1ns (+17.8%) | [+378898, +394180]ns | [2525894, 2541450] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_real_fresh_per_column | 2163733ns | +15597.5ns (+0.7%) | [+10353, +23491]ns | [2157912, 2169704] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_real_null_entry | 3090ns | -2144208.0ns (-99.9%) | [-2145658, -2142881]ns | [2928, 3142] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_real_held_handle | abi_lifecycle_real_fresh_per_batch | abi_lifecycle_real_fresh_per_column | abi_lifecycle_real_null_entry |
|---|---|---|---|---|
| 1 | 2149495ns | +18.6% | +0.6% | -99.9% |
| 2 | 2146777ns | +17.6% | +0.8% | -99.9% |
| 3 | 2147380ns | +17.8% | +0.9% | -99.9% |
| 4 | 2147217ns | +17.7% | +0.5% | -99.9% |
| 5 | 2145045ns | +18.1% | +1.3% | -99.9% |
| 6 | 2147900ns | +17.9% | +0.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | -0.276 | moderate- |
| abi_lifecycle_real_fresh_per_column | -0.741 | HIGH- (thermal bounce) |
| abi_lifecycle_real_held_handle | -0.223 | moderate- |
| abi_lifecycle_real_null_entry | 0.507 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **abi_lifecycle_real_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_real_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 7638862.8ns | 2532752.4ns | 301.6% | HIGH |
| abi_lifecycle_real_fresh_per_column | 6531382.9ns | 2163782.9ns | 301.9% | HIGH |
| abi_lifecycle_real_held_handle | 6487875.8ns | 2147302.2ns | 302.1% | HIGH |
| abi_lifecycle_real_null_entry | 119554.7ns | 3053.5ns | 3915.4% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_real_fresh_per_batch (n=6, range 2524302.1-2541449.8 ns)
  2524302.1 |########################################
  2525159.5 |
  2526016.9 |
  2526874.3 |########################################
  2527731.6 |
  2528589.0 |
  2529446.4 |########################################
  2530303.8 |
  2531161.2 |########################################
  2532018.6 |
  2532876.0 |########################################
  2533733.3 |
  2534590.7 |
  2535448.1 |
  2536305.5 |
  2537162.9 |
  2538020.3 |
  2538877.6 |
  2539735.0 |
  2540592.4 |
  (0 below, 1 above range)

abi_lifecycle_real_fresh_per_column (n=6, range 2157468.8-2169703.5 ns)
  2157468.8 |########################################
  2158080.5 |########################################
  2158692.3 |
  2159304.0 |
  2159915.8 |
  2160527.5 |
  2161139.2 |
  2161751.0 |
  2162362.7 |########################################
  2162974.4 |
  2163586.2 |
  2164197.9 |########################################
  2164809.6 |
  2165421.4 |
  2166033.1 |########################################
  2166644.9 |
  2167256.6 |
  2167868.3 |
  2168480.1 |
  2169091.8 |
  (0 below, 1 above range)

abi_lifecycle_real_held_handle (n=6, range 2145045.0-2148697.5 ns)
  2145045.0 |########################################
  2145227.6 |
  2145410.2 |
  2145592.9 |
  2145775.5 |
  2145958.1 |
  2146140.8 |
  2146323.4 |
  2146506.0 |
  2146688.6 |########################################
  2146871.2 |
  2147053.9 |########################################
  2147236.5 |########################################
  2147419.1 |
  2147601.8 |
  2147784.4 |########################################
  2147967.0 |
  2148149.6 |
  2148332.2 |
  2148514.9 |
  (0 below, 1 above range)

abi_lifecycle_real_null_entry (n=6, range 2925.0-3141.7 ns)
   2925.0 |########################################
   2935.8 |
   2946.7 |
   2957.5 |
   2968.3 |
   2979.2 |
   2990.0 |
   3000.8 |
   3011.7 |
   3022.5 |
   3033.3 |
   3044.2 |
   3055.0 |
   3065.8 |####################
   3076.7 |
   3087.5 |
   3098.3 |####################
   3109.2 |
   3120.0 |####################
   3130.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_real_fresh_per_batch**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_fresh_per_column**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_null_entry**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **abi_lifecycle_real_null_entry**: bridge=3872.0% of algo (FFI overhead may distort results)
