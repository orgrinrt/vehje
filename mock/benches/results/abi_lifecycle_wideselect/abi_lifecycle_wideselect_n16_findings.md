# abi_lifecycle (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_wideselect_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_wideselect_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_wideselect_null_entry dominates: 79718% faster than the next best (abi_lifecycle_wideselect_held_handle)

abi_lifecycle_wideselect_null_entry (2.57 us) leads abi_lifecycle_wideselect_held_handle (2.05 ms) by 79718%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_wideselect_null_entry beats baseline by 100% (significant)

abi_lifecycle_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_lifecycle_wideselect_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_wideselect_fresh_per_batch is an outlier: 877.1x slower than the field

abi_lifecycle_wideselect_fresh_per_batch (2.26 ms) is 877.1x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_wideselect_null_entry} vs {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} (79718% apart)

The field splits into a fast tier {abi_lifecycle_wideselect_null_entry} and a slow tier {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} with a 79718% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 877.1x the fastest

Fastest abi_lifecycle_wideselect_null_entry (2.57 us) to slowest abi_lifecycle_wideselect_fresh_per_batch (2.26 ms): 877.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_wideselect_null_entry** at 2572.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 877.13x (fastest 2572.1 ns, slowest 2256056.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2302120ns | 2258580ns | 2240940ns | 2257042ns | 2400325ns | +11.96% |
| abi_lifecycle_wideselect_fresh_per_column | 2071667ns | 2070454ns | 2066048ns | 2069233ns | 2078128ns | +0.76% |
| abi_lifecycle_wideselect_held_handle | 2056111ns | 2055705ns | 2046890ns | 2054701ns | 2062836ns | base |
| abi_lifecycle_wideselect_null_entry | 4885ns | 4878ns | 4678ns | 4844ns | 5049ns | -99.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2299402ns | 2238570ns | 2397171ns | +11.97% | 0.000 |
| abi_lifecycle_wideselect_fresh_per_column | 2069072ns | 2063388ns | 2075516ns | +0.76% | 0.000 |
| abi_lifecycle_wideselect_held_handle | 2053522ns | 2044514ns | 2060249ns | base | 0.000 |
| abi_lifecycle_wideselect_null_entry | 2573ns | 2468ns | 2643ns | -99.87% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 45080.9 | 2380647.8 | 2299401.9 | n/a |
| abi_lifecycle_wideselect_fresh_per_column | 38907.7 | 2069358.4 | 2069072.0 | 0 |
| abi_lifecycle_wideselect_held_handle | 40876.7 | 2055000.8 | 2053521.7 | n/a |
| abi_lifecycle_wideselect_null_entry | 28884.0 | 2663.1 | 2573.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_lifecycle_wideselect_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_wideselect_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_wideselect_held_handle | 0.000 | 0.1% |
| abi_lifecycle_wideselect_null_entry | 0.006 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2302120ns | 2302120ns | +11.96% |
| abi_lifecycle_wideselect_fresh_per_column | 2071667ns | 2071667ns | +0.76% |
| abi_lifecycle_wideselect_held_handle | 2056111ns | 2056111ns | base |
| abi_lifecycle_wideselect_null_entry | 4885ns | 4885ns | -99.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_held_handle | 2052996ns | base | --- | [2047320, 2060249] | --- | --- | --- | --- |
| abi_lifecycle_wideselect_fresh_per_batch | 2256056ns | +203351.1ns (+9.9%) | [+188884, +345406]ns | [2244979, 2397171] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_wideselect_fresh_per_column | 2067881ns | +16206.6ns (+0.8%) | [+7633, +22812]ns | [2063818, 2075516] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_wideselect_null_entry | 2572ns | -2050381.2ns (-99.9%) | [-2057709, -2044756]ns | [2505, 2643] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_wideselect_held_handle | abi_lifecycle_wideselect_fresh_per_batch | abi_lifecycle_wideselect_fresh_per_column | abi_lifecycle_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2044514ns | +11.2% | +0.9% | -99.9% |
| 2 | 2061481ns | +8.6% | +0.4% | -99.9% |
| 3 | 2059017ns | +22.4% | +0.3% | -99.9% |
| 4 | 2055284ns | +9.9% | +1.2% | -99.9% |
| 5 | 2050126ns | +10.0% | +1.1% | -99.9% |
| 6 | 2050709ns | +9.8% | +0.7% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | -0.287 | moderate- |
| abi_lifecycle_wideselect_fresh_per_column | -0.139 | ok |
| abi_lifecycle_wideselect_held_handle | -0.075 | ok |
| abi_lifecycle_wideselect_null_entry | -0.180 | ok |

**Consistency summary:**

- **abi_lifecycle_wideselect_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 6999085.8ns | 2299401.9ns | 304.4% | HIGH |
| abi_lifecycle_wideselect_fresh_per_column | 6248795.7ns | 2069072.0ns | 302.0% | HIGH |
| abi_lifecycle_wideselect_held_handle | 6204104.0ns | 2053521.7ns | 302.1% | HIGH |
| abi_lifecycle_wideselect_null_entry | 119010.8ns | 2573.3ns | 4624.9% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_wideselect_fresh_per_batch (n=6, range 2238570.0-2397171.2 ns)
  2238570.0 |####################
  2246500.1 |########################################
  2254430.1 |####################
  2262360.2 |
  2270290.2 |####################
  2278220.3 |
  2286150.4 |
  2294080.4 |
  2302010.5 |
  2309940.6 |
  2317870.6 |
  2325800.7 |
  2333730.8 |
  2341660.8 |
  2349590.9 |
  2357520.9 |
  2365451.0 |
  2373381.1 |
  2381311.1 |
  2389241.2 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_fresh_per_column (n=6, range 2063387.9-2075516.4 ns)
  2063387.9 |########################################
  2063994.3 |########################################
  2064600.8 |
  2065207.2 |
  2065813.6 |########################################
  2066420.0 |
  2067026.5 |
  2067632.9 |
  2068239.3 |
  2068845.7 |
  2069452.2 |########################################
  2070058.6 |
  2070665.0 |
  2071271.5 |
  2071877.9 |########################################
  2072484.3 |
  2073090.7 |
  2073697.2 |
  2074303.6 |
  2074910.0 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_held_handle (n=6, range 2044514.2-2060248.8 ns)
  2044514.2 |####################
  2045300.9 |
  2046087.7 |
  2046874.4 |
  2047661.1 |
  2048447.8 |
  2049234.6 |
  2050021.3 |########################################
  2050808.0 |
  2051594.7 |
  2052381.5 |
  2053168.2 |
  2053954.9 |
  2054741.7 |####################
  2055528.4 |
  2056315.1 |
  2057101.8 |
  2057888.6 |
  2058675.3 |####################
  2059462.0 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_null_entry (n=6, range 2468.3-2642.7 ns)
   2468.3 |########################################
   2477.0 |
   2485.7 |
   2494.5 |
   2503.2 |
   2511.9 |
   2520.6 |
   2529.3 |
   2538.1 |########################################
   2546.8 |
   2555.5 |########################################
   2564.2 |
   2572.9 |
   2581.7 |########################################
   2590.4 |
   2599.1 |
   2607.8 |########################################
   2616.5 |
   2625.3 |
   2634.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_wideselect_fresh_per_batch**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_fresh_per_column**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_held_handle**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_null_entry**: bridge=4635.0% of algo (FFI overhead may distort results)
