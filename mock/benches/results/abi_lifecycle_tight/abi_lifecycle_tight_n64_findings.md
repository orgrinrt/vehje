# abi_lifecycle (tight)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_tight_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_tight_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_tight_null_entry dominates: 79248% faster than the next best (abi_lifecycle_tight_held_handle)

abi_lifecycle_tight_null_entry (2.52 us) leads abi_lifecycle_tight_held_handle (2.00 ms) by 79248%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_tight_null_entry beats baseline by 100% (significant)

abi_lifecycle_tight_null_entry is -2.00 ms (100%) faster than baseline abi_lifecycle_tight_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_tight_fresh_per_batch is an outlier: 815.9x slower than the field

abi_lifecycle_tight_fresh_per_batch (2.06 ms) is 815.9x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_tight_fresh_per_column shows warm-up / thermal drift (autocorr +0.55)

abi_lifecycle_tight_fresh_per_column's per-pass series has lag-1 autocorrelation +0.55, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_tight_null_entry} vs {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} (79248% apart)

The field splits into a fast tier {abi_lifecycle_tight_null_entry} and a slow tier {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} with a 79248% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 815.9x the fastest

Fastest abi_lifecycle_tight_null_entry (2.52 us) to slowest abi_lifecycle_tight_fresh_per_batch (2.06 ms): 815.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_tight_null_entry** at 2523.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 815.88x (fastest 2523.8 ns, slowest 2059067.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2060189ns | 2061627ns | 2051057ns | 2059653ns | 2065558ns | +2.77% |
| abi_lifecycle_tight_fresh_per_column | 2021088ns | 2020121ns | 2016727ns | 2019005ns | 2026394ns | +0.82% |
| abi_lifecycle_tight_held_handle | 2004591ns | 2005067ns | 1999495ns | 2003643ns | 2008561ns | base |
| abi_lifecycle_tight_null_entry | 4833ns | 4815ns | 4605ns | 4786ns | 5016ns | -99.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2057625ns | 2048471ns | 2062967ns | +2.78% | 0.000 |
| abi_lifecycle_tight_fresh_per_column | 2018528ns | 2014132ns | 2023798ns | +0.83% | 0.000 |
| abi_lifecycle_tight_held_handle | 2002001ns | 1996805ns | 2005974ns | base | 0.000 |
| abi_lifecycle_tight_null_entry | 2539ns | 2411ns | 2639ns | -99.87% | 0.025 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 38443.1 | 2056714.6 | 2057625.3 | 0 |
| abi_lifecycle_tight_fresh_per_column | 37753.4 | 2018188.0 | 2018527.8 | 0 |
| abi_lifecycle_tight_held_handle | 38826.6 | 2002935.3 | 2002000.7 | n/a |
| abi_lifecycle_tight_null_entry | 27033.2 | 2792.9 | 2538.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_lifecycle_tight_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_tight_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_tight_held_handle | 0.000 | 0.1% |
| abi_lifecycle_tight_null_entry | 0.025 | 95.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2060189ns | 2060189ns | +2.77% |
| abi_lifecycle_tight_fresh_per_column | 2021088ns | 2021088ns | +0.82% |
| abi_lifecycle_tight_held_handle | 2004591ns | 2004591ns | base |
| abi_lifecycle_tight_null_entry | 4833ns | 4833ns | -99.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_tight_held_handle | 2002544ns | base | --- | [1997484, 2005974] | --- | --- | --- | --- |
| abi_lifecycle_tight_fresh_per_batch | 2059068ns | +55520.0ns (+2.8%) | [+50883, +60471]ns | [2050841, 2062967] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_tight_fresh_per_column | 2017574ns | +17476.0ns (+0.9%) | [+12298, +19807]ns | [2014211, 2023798] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_tight_null_entry | 2524ns | -1999981.9ns (-99.9%) | [-2003373, -1995030]ns | [2454, 2639] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_tight_held_handle | abi_lifecycle_tight_fresh_per_batch | abi_lifecycle_tight_fresh_per_column | abi_lifecycle_tight_null_entry |
|---|---|---|---|---|
| 1 | 2006285ns | +3.0% | +0.9% | -99.9% |
| 2 | 2003112ns | +2.5% | +1.0% | -99.9% |
| 3 | 2001976ns | +2.9% | +0.8% | -99.9% |
| 4 | 2005663ns | +2.7% | +0.4% | -99.9% |
| 5 | 1998164ns | +3.0% | +0.8% | -99.9% |
| 6 | 1996805ns | +2.6% | +1.0% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | -0.278 | moderate- |
| abi_lifecycle_tight_fresh_per_column | 0.553 | HIGH+ (drift/warm-up) |
| abi_lifecycle_tight_held_handle | 0.141 | ok |
| abi_lifecycle_tight_null_entry | 0.132 | ok |

**Consistency summary:**

- **abi_lifecycle_tight_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_tight_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 6209376.6ns | 2057625.3ns | 301.8% | HIGH |
| abi_lifecycle_tight_fresh_per_column | 6091553.4ns | 2018527.8ns | 301.8% | HIGH |
| abi_lifecycle_tight_held_handle | 6048144.6ns | 2002000.7ns | 302.1% | HIGH |
| abi_lifecycle_tight_null_entry | 112799.3ns | 2538.9ns | 4442.9% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_tight_fresh_per_batch (n=6, range 2048470.8-2062966.9 ns)
  2048470.8 |####################
  2049195.6 |
  2049920.4 |
  2050645.2 |
  2051370.0 |
  2052094.8 |
  2052819.6 |####################
  2053544.4 |
  2054269.2 |
  2054994.0 |
  2055718.9 |
  2056443.7 |
  2057168.5 |
  2057893.3 |
  2058618.1 |########################################
  2059342.9 |####################
  2060067.7 |
  2060792.5 |
  2061517.3 |
  2062242.1 |
  (0 below, 1 above range)

abi_lifecycle_tight_fresh_per_column (n=6, range 2014131.7-2023797.5 ns)
  2014131.7 |########################################
  2014615.0 |
  2015098.3 |
  2015581.6 |
  2016064.9 |####################
  2016548.1 |
  2017031.4 |
  2017514.7 |
  2017998.0 |
  2018481.3 |####################
  2018964.6 |
  2019447.9 |
  2019931.2 |
  2020414.5 |
  2020897.8 |
  2021381.1 |
  2021864.3 |
  2022347.6 |
  2022830.9 |####################
  2023314.2 |
  (0 below, 1 above range)

abi_lifecycle_tight_held_handle (n=6, range 1996805.0-2005974.0 ns)
  1996805.0 |########################################
  1997263.4 |
  1997721.9 |########################################
  1998180.3 |
  1998638.8 |
  1999097.2 |
  1999555.7 |
  2000014.1 |
  2000472.6 |
  2000931.0 |
  2001389.5 |
  2001847.9 |########################################
  2002306.4 |
  2002764.8 |########################################
  2003223.3 |
  2003681.7 |
  2004140.2 |
  2004598.6 |
  2005057.1 |
  2005515.5 |########################################
  (0 below, 1 above range)

abi_lifecycle_tight_null_entry (n=6, range 2410.8-2638.8 ns)
   2410.8 |########################################
   2422.2 |
   2433.6 |
   2445.0 |
   2456.4 |
   2467.8 |
   2479.2 |
   2490.6 |########################################
   2502.0 |########################################
   2513.4 |
   2524.8 |
   2536.2 |########################################
   2547.6 |
   2559.0 |
   2570.4 |########################################
   2581.8 |
   2593.2 |
   2604.6 |
   2616.0 |
   2627.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_tight_fresh_per_batch**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_fresh_per_column**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **abi_lifecycle_tight_fresh_per_column**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_null_entry**: bridge=4471.3% of algo (FFI overhead may distort results)
