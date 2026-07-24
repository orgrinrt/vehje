# abi_lifecycle (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_wideselect_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_wideselect_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_wideselect_null_entry dominates: 74431% faster than the next best (abi_lifecycle_wideselect_held_handle)

abi_lifecycle_wideselect_null_entry (2.75 us) leads abi_lifecycle_wideselect_held_handle (2.05 ms) by 74431%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_wideselect_null_entry beats baseline by 100% (significant)

abi_lifecycle_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_lifecycle_wideselect_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_wideselect_fresh_per_batch is an outlier: 755.4x slower than the field

abi_lifecycle_wideselect_fresh_per_batch (2.08 ms) is 755.4x the fastest (2.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_wideselect_null_entry shows alternating (throttle bounce) (autocorr -0.80)

abi_lifecycle_wideselect_null_entry's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_wideselect_null_entry} vs {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} (74431% apart)

The field splits into a fast tier {abi_lifecycle_wideselect_null_entry} and a slow tier {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} with a 74431% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 755.4x the fastest

Fastest abi_lifecycle_wideselect_null_entry (2.75 us) to slowest abi_lifecycle_wideselect_fresh_per_batch (2.08 ms): 755.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_wideselect_null_entry** at 2752.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 755.42x (fastest 2752.1 ns, slowest 2078996.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2091057ns | 2081554ns | 2071017ns | 2078490ns | 2119928ns | +2.07% |
| abi_lifecycle_wideselect_fresh_per_column | 2117671ns | 2072972ns | 2057876ns | 2069571ns | 2219718ns | +3.37% |
| abi_lifecycle_wideselect_held_handle | 2048553ns | 2053732ns | 2033753ns | 2047919ns | 2056905ns | base |
| abi_lifecycle_wideselect_null_entry | 5036ns | 5043ns | 4825ns | 4985ns | 5219ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2088407ns | 2068458ns | 2117024ns | +2.07% | 0.000 |
| abi_lifecycle_wideselect_fresh_per_column | 2114968ns | 2055393ns | 2216638ns | +3.37% | 0.000 |
| abi_lifecycle_wideselect_held_handle | 2046076ns | 2031360ns | 2054436ns | base | 0.000 |
| abi_lifecycle_wideselect_null_entry | 2738ns | 2640ns | 2823ns | -99.87% | 0.047 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 42596.3 | 2088649.6 | 2088406.8 | n/a |
| abi_lifecycle_wideselect_fresh_per_column | 48619.9 | 2161137.6 | 2114967.9 | n/a |
| abi_lifecycle_wideselect_held_handle | 38721.6 | 2045675.1 | 2046076.1 | n/a |
| abi_lifecycle_wideselect_null_entry | 27351.0 | 2793.7 | 2738.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_lifecycle_wideselect_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_wideselect_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_wideselect_held_handle | 0.000 | 0.1% |
| abi_lifecycle_wideselect_null_entry | 0.047 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2091057ns | 2091057ns | +2.07% |
| abi_lifecycle_wideselect_fresh_per_column | 2117671ns | 2117671ns | +3.37% |
| abi_lifecycle_wideselect_held_handle | 2048553ns | 2048553ns | base |
| abi_lifecycle_wideselect_null_entry | 5036ns | 5036ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_held_handle | 2051173ns | base | --- | [2032619, 2054436] | --- | --- | --- | --- |
| abi_lifecycle_wideselect_fresh_per_batch | 2078997ns | +33166.0ns (+1.6%) | [+19936, +73890]ns | [2069199, 2117024] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_wideselect_fresh_per_column | 2070428ns | +25957.1ns (+1.3%) | [+11432, +169286]ns | [2057837, 2216638] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_wideselect_null_entry | 2752ns | -2048472.7ns (-99.9%) | [-2051700, -2029840]ns | [2641, 2823] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_wideselect_held_handle | abi_lifecycle_wideselect_fresh_per_batch | abi_lifecycle_wideselect_fresh_per_column | abi_lifecycle_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2052392ns | +4.8% | +14.5% | -99.9% |
| 2 | 2055257ns | +1.0% | +0.2% | -99.9% |
| 3 | 2053616ns | +1.4% | +1.5% | -99.9% |
| 4 | 2049954ns | +0.9% | +0.9% | -99.9% |
| 5 | 2033878ns | +2.5% | +1.1% | -99.9% |
| 6 | 2031360ns | +1.9% | +2.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | -0.069 | ok |
| abi_lifecycle_wideselect_fresh_per_column | -0.066 | ok |
| abi_lifecycle_wideselect_held_handle | 0.514 | HIGH+ (drift/warm-up) |
| abi_lifecycle_wideselect_null_entry | -0.800 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_lifecycle_wideselect_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 6315272.5ns | 2088406.8ns | 302.4% | HIGH |
| abi_lifecycle_wideselect_fresh_per_column | 6425942.5ns | 2114967.9ns | 303.8% | HIGH |
| abi_lifecycle_wideselect_held_handle | 6178552.6ns | 2046076.1ns | 302.0% | HIGH |
| abi_lifecycle_wideselect_null_entry | 119569.7ns | 2738.5ns | 4366.3% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_wideselect_fresh_per_batch (n=6, range 2068458.3-2117024.4 ns)
  2068458.3 |########################################
  2070886.6 |
  2073314.9 |
  2075743.2 |####################
  2078171.5 |
  2080599.8 |####################
  2083028.1 |####################
  2085456.4 |
  2087884.7 |
  2090313.0 |
  2092741.4 |
  2095169.7 |
  2097598.0 |
  2100026.3 |
  2102454.6 |
  2104882.9 |
  2107311.2 |
  2109739.5 |
  2112167.8 |
  2114596.1 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_fresh_per_column (n=6, range 2055392.9-2216638.4 ns)
  2055392.9 |########################################
  2063455.2 |####################
  2071517.4 |####################
  2079579.7 |####################
  2087642.0 |
  2095704.3 |
  2103766.5 |
  2111828.8 |
  2119891.1 |
  2127953.4 |
  2136015.6 |
  2144077.9 |
  2152140.2 |
  2160202.4 |
  2168264.7 |
  2176327.0 |
  2184389.3 |
  2192451.5 |
  2200513.8 |
  2208576.1 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_held_handle (n=6, range 2031360.0-2054436.5 ns)
  2031360.0 |########################################
  2032513.8 |
  2033667.6 |########################################
  2034821.5 |
  2035975.3 |
  2037129.1 |
  2038282.9 |
  2039436.8 |
  2040590.6 |
  2041744.4 |
  2042898.2 |
  2044052.0 |
  2045205.9 |
  2046359.7 |
  2047513.5 |
  2048667.3 |
  2049821.2 |########################################
  2050975.0 |
  2052128.8 |########################################
  2053282.6 |########################################
  (0 below, 1 above range)

abi_lifecycle_wideselect_null_entry (n=6, range 2639.6-2822.7 ns)
   2639.6 |########################################
   2648.8 |
   2657.9 |
   2667.1 |
   2676.2 |
   2685.4 |
   2694.5 |
   2703.7 |
   2712.8 |
   2722.0 |
   2731.1 |
   2740.3 |####################
   2749.5 |
   2758.6 |####################
   2767.8 |
   2776.9 |
   2786.1 |
   2795.2 |
   2804.4 |####################
   2813.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_wideselect_fresh_per_batch**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_fresh_per_column**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_held_handle**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **abi_lifecycle_wideselect_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_null_entry**: bridge=4326.7% of algo (FFI overhead may distort results)
