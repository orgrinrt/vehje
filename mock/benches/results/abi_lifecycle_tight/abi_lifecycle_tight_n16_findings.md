# abi_lifecycle (tight)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_tight_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_tight_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_tight_null_entry dominates: 77778% faster than the next best (abi_lifecycle_tight_held_handle)

abi_lifecycle_tight_null_entry (2.57 us) leads abi_lifecycle_tight_held_handle (2.00 ms) by 77778%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_tight_null_entry beats baseline by 100% (significant)

abi_lifecycle_tight_null_entry is -2.00 ms (100%) faster than baseline abi_lifecycle_tight_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_tight_fresh_per_batch is an outlier: 858.5x slower than the field

abi_lifecycle_tight_fresh_per_batch (2.21 ms) is 858.5x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_tight_held_handle shows alternating (throttle bounce) (autocorr -0.55)

abi_lifecycle_tight_held_handle's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_tight_null_entry} vs {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} (77778% apart)

The field splits into a fast tier {abi_lifecycle_tight_null_entry} and a slow tier {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} with a 77778% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 858.5x the fastest

Fastest abi_lifecycle_tight_null_entry (2.57 us) to slowest abi_lifecycle_tight_fresh_per_batch (2.21 ms): 858.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_tight_null_entry** at 2571.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 858.51x (fastest 2571.1 ns, slowest 2207270.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2211923ns | 2209740ns | 2204279ns | 2209256ns | 2219745ns | +10.27% |
| abi_lifecycle_tight_fresh_per_column | 2022977ns | 2021618ns | 2017890ns | 2021272ns | 2028079ns | +0.85% |
| abi_lifecycle_tight_held_handle | 2005876ns | 2004789ns | 2000429ns | 2003388ns | 2012332ns | base |
| abi_lifecycle_tight_null_entry | 4947ns | 4871ns | 4760ns | 4861ns | 5171ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2209394ns | 2201840ns | 2217148ns | +10.29% | 0.000 |
| abi_lifecycle_tight_fresh_per_column | 2020420ns | 2015441ns | 2025380ns | +0.85% | 0.000 |
| abi_lifecycle_tight_held_handle | 2003305ns | 1997900ns | 2009712ns | base | 0.000 |
| abi_lifecycle_tight_null_entry | 2600ns | 2484ns | 2707ns | -99.87% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 37711.3 | 2207457.4 | 2209393.7 | 0 |
| abi_lifecycle_tight_fresh_per_column | 36153.1 | 2020949.8 | 2020420.5 | 0 |
| abi_lifecycle_tight_held_handle | 39052.5 | 2002868.8 | 2003305.1 | n/a |
| abi_lifecycle_tight_null_entry | 26900.1 | 2670.7 | 2600.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_lifecycle_tight_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_tight_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_tight_held_handle | 0.000 | 0.1% |
| abi_lifecycle_tight_null_entry | 0.006 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2211923ns | 2211923ns | +10.27% |
| abi_lifecycle_tight_fresh_per_column | 2022977ns | 2022977ns | +0.85% |
| abi_lifecycle_tight_held_handle | 2005876ns | 2005876ns | base |
| abi_lifecycle_tight_null_entry | 4947ns | 4947ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_tight_held_handle | 2002278ns | base | --- | [1997925, 2009712] | --- | --- | --- | --- |
| abi_lifecycle_tight_fresh_per_batch | 2207271ns | +206366.9ns (+10.3%) | [+198490, +213408]ns | [2203763, 2217148] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_tight_fresh_per_column | 2019073ns | +15013.9ns (+0.7%) | [+9183, +27149]ns | [2016809, 2025380] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_tight_null_entry | 2571ns | -1999755.8ns (-99.9%) | [-2007070, -1995289]ns | [2522, 2707] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_tight_held_handle | abi_lifecycle_tight_fresh_per_batch | abi_lifecycle_tight_fresh_per_column | abi_lifecycle_tight_null_entry |
|---|---|---|---|---|
| 1 | 1997900ns | +10.6% | +0.9% | -99.9% |
| 2 | 2009847ns | +9.8% | +0.5% | -99.9% |
| 3 | 2006045ns | +10.0% | +0.6% | -99.9% |
| 4 | 1998511ns | +10.2% | +1.3% | -99.9% |
| 5 | 2009578ns | +10.7% | +0.4% | -99.9% |
| 6 | 1997950ns | +10.5% | +1.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | -0.361 | moderate- |
| abi_lifecycle_tight_fresh_per_column | -0.314 | moderate- |
| abi_lifecycle_tight_held_handle | -0.553 | HIGH- (thermal bounce) |
| abi_lifecycle_tight_null_entry | -0.347 | moderate- |

**Consistency summary:**

- **abi_lifecycle_tight_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_tight_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 6663021.2ns | 2209393.7ns | 301.6% | HIGH |
| abi_lifecycle_tight_fresh_per_column | 6097401.4ns | 2020420.5ns | 301.8% | HIGH |
| abi_lifecycle_tight_held_handle | 6052359.5ns | 2003305.1ns | 302.1% | HIGH |
| abi_lifecycle_tight_null_entry | 117657.9ns | 2600.2ns | 4525.0% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_tight_fresh_per_batch (n=6, range 2201839.6-2217147.5 ns)
  2201839.6 |########################################
  2202605.0 |
  2203370.4 |
  2204135.8 |
  2204901.2 |
  2205666.6 |########################################
  2206432.0 |########################################
  2207197.4 |########################################
  2207962.8 |
  2208728.2 |
  2209493.5 |
  2210258.9 |########################################
  2211024.3 |
  2211789.7 |
  2212555.1 |
  2213320.5 |
  2214085.9 |
  2214851.3 |
  2215616.7 |
  2216382.1 |
  (0 below, 1 above range)

abi_lifecycle_tight_fresh_per_column (n=6, range 2015441.2-2025379.6 ns)
  2015441.2 |########################################
  2015938.1 |
  2016435.0 |
  2016932.0 |
  2017428.9 |
  2017925.8 |########################################
  2018422.7 |########################################
  2018919.6 |
  2019416.6 |########################################
  2019913.5 |
  2020410.4 |
  2020907.3 |
  2021404.2 |
  2021901.2 |
  2022398.1 |
  2022895.0 |
  2023391.9 |
  2023888.8 |
  2024385.8 |
  2024882.7 |########################################
  (0 below, 1 above range)

abi_lifecycle_tight_held_handle (n=6, range 1997900.4-2009712.3 ns)
  1997900.4 |########################################
  1998491.0 |####################
  1999081.6 |
  1999672.2 |
  2000262.8 |
  2000853.4 |
  2001444.0 |
  2002034.6 |
  2002625.2 |
  2003215.8 |
  2003806.3 |
  2004396.9 |
  2004987.5 |
  2005578.1 |####################
  2006168.7 |
  2006759.3 |
  2007349.9 |
  2007940.5 |
  2008531.1 |
  2009121.7 |####################
  (0 below, 1 above range)

abi_lifecycle_tight_null_entry (n=6, range 2483.8-2707.3 ns)
   2483.8 |########################################
   2495.0 |
   2506.2 |
   2517.3 |
   2528.5 |
   2539.7 |
   2550.9 |########################################
   2562.0 |########################################
   2573.2 |########################################
   2584.4 |
   2595.6 |
   2606.7 |
   2617.9 |
   2629.1 |
   2640.2 |
   2651.4 |
   2662.6 |
   2673.8 |
   2685.0 |########################################
   2696.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_tight_fresh_per_batch**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_fresh_per_column**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_null_entry**: bridge=4589.5% of algo (FFI overhead may distort results)
