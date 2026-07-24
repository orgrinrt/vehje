# abi_lifecycle (tight)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_tight_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_tight_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_tight_null_entry dominates: 71151% faster than the next best (abi_lifecycle_tight_held_handle)

abi_lifecycle_tight_null_entry (2.82 us) leads abi_lifecycle_tight_held_handle (2.01 ms) by 71151%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_tight_null_entry beats baseline by 100% (significant)

abi_lifecycle_tight_null_entry is -2.00 ms (100%) faster than baseline abi_lifecycle_tight_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_tight_fresh_per_batch is an outlier: 721.5x slower than the field

abi_lifecycle_tight_fresh_per_batch (2.03 ms) is 721.5x the fastest (2.82 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_tight_null_entry} vs {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} (71151% apart)

The field splits into a fast tier {abi_lifecycle_tight_null_entry} and a slow tier {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} with a 71151% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 721.5x the fastest

Fastest abi_lifecycle_tight_null_entry (2.82 us) to slowest abi_lifecycle_tight_fresh_per_batch (2.03 ms): 721.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_tight_null_entry** at 2816.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 721.46x (fastest 2816.2 ns, slowest 2031811.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2047327ns | 2034511ns | 2026866ns | 2033527ns | 2078259ns | +0.73% |
| abi_lifecycle_tight_fresh_per_column | 2018964ns | 2018472ns | 2013071ns | 2018091ns | 2023220ns | -0.67% |
| abi_lifecycle_tight_held_handle | 2032560ns | 2009321ns | 1998449ns | 2008079ns | 2086338ns | base |
| abi_lifecycle_tight_null_entry | 5098ns | 5184ns | 4851ns | 5111ns | 5201ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2044659ns | 2024383ns | 2075420ns | +0.73% | 0.000 |
| abi_lifecycle_tight_fresh_per_column | 2016502ns | 2010543ns | 2020672ns | -0.66% | 0.000 |
| abi_lifecycle_tight_held_handle | 2029932ns | 1995966ns | 2083670ns | base | 0.000 |
| abi_lifecycle_tight_null_entry | 2783ns | 2644ns | 2845ns | -99.86% | 0.046 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 41046.4 | 2113328.1 | 2044659.3 | n/a |
| abi_lifecycle_tight_fresh_per_column | 36071.2 | 2015035.8 | 2016502.2 | n/a |
| abi_lifecycle_tight_held_handle | 39550.9 | 2005938.2 | 2029931.6 | n/a |
| abi_lifecycle_tight_null_entry | 27662.6 | 2824.2 | 2782.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_lifecycle_tight_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_tight_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_tight_held_handle | 0.000 | 0.1% |
| abi_lifecycle_tight_null_entry | 0.045 | 93.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2047327ns | 2047327ns | +0.73% |
| abi_lifecycle_tight_fresh_per_column | 2018964ns | 2018964ns | -0.67% |
| abi_lifecycle_tight_held_handle | 2032560ns | 2032560ns | base |
| abi_lifecycle_tight_null_entry | 5098ns | 5098ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_tight_held_handle | 2006609ns | base | --- | [1999516, 2083670] | --- | --- | --- | --- |
| abi_lifecycle_tight_fresh_per_batch | 2031811ns | no significant difference | [-18850, +36820]ns | [2026747, 2075420] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_tight_fresh_per_column | 2016052ns | no significant difference | [-67618, +17700]ns | [2012783, 2020672] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_tight_null_entry | 2816ns | -2003841.5ns (-99.9%) | [-2080837, -1996769]ns | [2687, 2845] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_tight_held_handle | abi_lifecycle_tight_fresh_per_batch | abi_lifecycle_tight_fresh_per_column | abi_lifecycle_tight_null_entry |
|---|---|---|---|---|
| 1 | 1995966ns | +1.4% | +1.0% | -99.9% |
| 2 | 2155458ns | -2.6% | -6.5% | -99.9% |
| 3 | 2007204ns | +2.3% | +0.8% | -99.9% |
| 4 | 2003066ns | +1.3% | +0.7% | -99.9% |
| 5 | 2011882ns | +1.0% | +0.2% | -99.9% |
| 6 | 2006013ns | +1.3% | +0.2% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | -0.107 | ok |
| abi_lifecycle_tight_fresh_per_column | 0.087 | ok |
| abi_lifecycle_tight_held_handle | -0.293 | moderate- |
| abi_lifecycle_tight_null_entry | -0.328 | moderate- |

**Consistency summary:**

- **abi_lifecycle_tight_fresh_per_batch**: won 1/6, lost 5/6
- **abi_lifecycle_tight_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 6245591.0ns | 2044659.3ns | 305.5% | HIGH |
| abi_lifecycle_tight_fresh_per_column | 6084301.0ns | 2016502.2ns | 301.7% | HIGH |
| abi_lifecycle_tight_held_handle | 6064884.0ns | 2029931.6ns | 298.8% | HIGH |
| abi_lifecycle_tight_null_entry | 120276.0ns | 2782.6ns | 4322.5% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_tight_fresh_per_batch (n=6, range 2024382.9-2075420.2 ns)
  2024382.9 |########################################
  2026934.8 |########################################
  2029486.6 |########################################
  2032038.5 |########################################
  2034590.4 |
  2037142.2 |
  2039694.1 |
  2042246.0 |
  2044797.8 |
  2047349.7 |
  2049901.6 |########################################
  2052453.4 |
  2055005.3 |
  2057557.2 |
  2060109.0 |
  2062660.9 |
  2065212.8 |
  2067764.6 |
  2070316.5 |
  2072868.4 |
  (0 below, 1 above range)

abi_lifecycle_tight_fresh_per_column (n=6, range 2010542.9-2020672.1 ns)
  2010542.9 |########################################
  2011049.4 |
  2011555.8 |
  2012062.3 |
  2012568.7 |
  2013075.2 |
  2013581.7 |
  2014088.1 |
  2014594.6 |########################################
  2015101.0 |
  2015607.5 |########################################
  2016114.0 |########################################
  2016620.4 |
  2017126.9 |
  2017633.3 |########################################
  2018139.8 |
  2018646.3 |
  2019152.7 |
  2019659.2 |
  2020165.6 |
  (0 below, 1 above range)

abi_lifecycle_tight_held_handle (n=6, range 1995965.8-2083670.2 ns)
  1995965.8 |####################
  2000351.0 |####################
  2004736.2 |########################################
  2009121.5 |####################
  2013506.7 |
  2017891.9 |
  2022277.1 |
  2026662.3 |
  2031047.6 |
  2035432.8 |
  2039818.0 |
  2044203.2 |
  2048588.4 |
  2052973.7 |
  2057358.9 |
  2061744.1 |
  2066129.3 |
  2070514.5 |
  2074899.8 |
  2079285.0 |
  (0 below, 1 above range)

abi_lifecycle_tight_null_entry (n=6, range 2644.2-2844.8 ns)
   2644.2 |########################################
   2654.2 |
   2664.3 |
   2674.3 |
   2684.3 |
   2694.3 |
   2704.4 |
   2714.4 |
   2724.4 |########################################
   2734.5 |
   2744.5 |
   2754.5 |
   2764.6 |
   2774.6 |
   2784.6 |
   2794.7 |
   2804.7 |########################################
   2814.7 |
   2824.7 |########################################
   2834.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_tight_fresh_per_batch**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_fresh_per_column**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_null_entry**: bridge=4283.5% of algo (FFI overhead may distort results)
