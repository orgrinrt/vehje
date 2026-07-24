# abi_lifecycle (tight)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_tight_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_tight_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_tight_null_entry dominates: 86409% faster than the next best (abi_lifecycle_tight_held_handle)

abi_lifecycle_tight_null_entry (2.32 us) leads abi_lifecycle_tight_held_handle (2.01 ms) by 86409%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_tight_null_entry beats baseline by 100% (significant)

abi_lifecycle_tight_null_entry is -2.00 ms (100%) faster than baseline abi_lifecycle_tight_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_tight_fresh_per_batch is an outlier: 908.5x slower than the field

abi_lifecycle_tight_fresh_per_batch (2.11 ms) is 908.5x the fastest (2.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_tight_null_entry} vs {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} (86409% apart)

The field splits into a fast tier {abi_lifecycle_tight_null_entry} and a slow tier {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} with a 86409% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 908.5x the fastest

Fastest abi_lifecycle_tight_null_entry (2.32 us) to slowest abi_lifecycle_tight_fresh_per_batch (2.11 ms): 908.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_tight_null_entry** at 2319.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 908.53x (fastest 2319.6 ns, slowest 2107389.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2115240ns | 2110059ns | 2099032ns | 2108036ns | 2134148ns | +0.90% |
| abi_lifecycle_tight_fresh_per_column | 2036903ns | 2025952ns | 2016091ns | 2023755ns | 2067029ns | -2.84% |
| abi_lifecycle_tight_held_handle | 2096351ns | 2009176ns | 1999733ns | 2008024ns | 2277150ns | base |
| abi_lifecycle_tight_null_entry | 4658ns | 4612ns | 4414ns | 4560ns | 4927ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2112563ns | 2096485ns | 2131285ns | +0.90% | 0.000 |
| abi_lifecycle_tight_fresh_per_column | 2034303ns | 2013713ns | 2064230ns | -2.83% | 0.000 |
| abi_lifecycle_tight_held_handle | 2093652ns | 1997242ns | 2274205ns | base | 0.000 |
| abi_lifecycle_tight_null_entry | 2334ns | 2225ns | 2451ns | -99.89% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 40187.6 | 2193234.8 | 2112562.9 | 0 |
| abi_lifecycle_tight_fresh_per_column | 40761.0 | 2033694.2 | 2034302.6 | n/a |
| abi_lifecycle_tight_held_handle | 45575.6 | 2015710.6 | 2093651.9 | n/a |
| abi_lifecycle_tight_null_entry | 27206.6 | 2443.7 | 2334.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_lifecycle_tight_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_tight_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_tight_held_handle | 0.000 | 0.1% |
| abi_lifecycle_tight_null_entry | 0.014 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2115240ns | 2115240ns | +0.90% |
| abi_lifecycle_tight_fresh_per_column | 2036903ns | 2036903ns | -2.84% |
| abi_lifecycle_tight_held_handle | 2096351ns | 2096351ns | base |
| abi_lifecycle_tight_null_entry | 4658ns | 4658ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_tight_held_handle | 2006609ns | base | --- | [2000141, 2274205] | --- | --- | --- | --- |
| abi_lifecycle_tight_fresh_per_batch | 2107389ns | no significant difference | [-150431, +111007]ns | [2099014, 2131285] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_tight_fresh_per_column | 2023432ns | no significant difference | [-215443, +22766]ns | [2015245, 2064230] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_tight_null_entry | 2320ns | -2004249.4ns (-99.9%) | [-2271794, -1997910]ns | [2231, 2451] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_tight_held_handle | abi_lifecycle_tight_fresh_per_batch | abi_lifecycle_tight_fresh_per_column | abi_lifecycle_tight_null_entry |
|---|---|---|---|---|
| 1 | 2007125ns | +4.7% | +0.6% | -99.9% |
| 2 | 1997242ns | +5.7% | +0.8% | -99.9% |
| 3 | 2006092ns | +4.9% | +0.5% | -99.9% |
| 4 | 2003041ns | +5.4% | +1.4% | -99.9% |
| 5 | 2009460ns | +4.3% | +0.9% | -99.9% |
| 6 | 2538950ns | -15.3% | -17.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | -0.288 | moderate- |
| abi_lifecycle_tight_fresh_per_column | 0.055 | ok |
| abi_lifecycle_tight_held_handle | -0.022 | ok |
| abi_lifecycle_tight_null_entry | -0.013 | ok |

**Consistency summary:**

- **abi_lifecycle_tight_fresh_per_batch**: won 1/6, lost 5/6
- **abi_lifecycle_tight_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 6461204.3ns | 2112562.9ns | 305.8% | HIGH |
| abi_lifecycle_tight_fresh_per_column | 6145002.6ns | 2034302.6ns | 302.1% | HIGH |
| abi_lifecycle_tight_held_handle | 6092473.7ns | 2093651.9ns | 291.0% | HIGH |
| abi_lifecycle_tight_null_entry | 115977.8ns | 2334.0ns | 4969.0% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_tight_fresh_per_batch (n=6, range 2096484.6-2131285.0 ns)
  2096484.6 |####################
  2098224.6 |
  2099964.6 |####################
  2101704.7 |
  2103444.7 |####################
  2105184.7 |
  2106924.7 |
  2108664.7 |
  2110404.8 |########################################
  2112144.8 |
  2113884.8 |
  2115624.8 |
  2117364.8 |
  2119104.9 |
  2120844.9 |
  2122584.9 |
  2124324.9 |
  2126064.9 |
  2127805.0 |
  2129545.0 |
  (0 below, 1 above range)

abi_lifecycle_tight_fresh_per_column (n=6, range 2013712.9-2064230.2 ns)
  2013712.9 |########################################
  2016238.8 |########################################
  2018764.6 |########################################
  2021290.5 |
  2023816.4 |
  2026342.2 |########################################
  2028868.1 |########################################
  2031394.0 |
  2033919.8 |
  2036445.7 |
  2038971.5 |
  2041497.4 |
  2044023.3 |
  2046549.1 |
  2049075.0 |
  2051600.9 |
  2054126.7 |
  2056652.6 |
  2059178.5 |
  2061704.3 |
  (0 below, 1 above range)

abi_lifecycle_tight_held_handle (n=6, range 1997241.7-2274205.4 ns)
  1997241.7 |########################################
  2011089.9 |
  2024938.1 |
  2038786.3 |
  2052634.4 |
  2066482.6 |
  2080330.8 |
  2094179.0 |
  2108027.2 |
  2121875.4 |
  2135723.5 |
  2149571.7 |
  2163419.9 |
  2177268.1 |
  2191116.3 |
  2204964.5 |
  2218812.7 |
  2232660.8 |
  2246509.0 |
  2260357.2 |
  (0 below, 1 above range)

abi_lifecycle_tight_null_entry (n=6, range 2225.0-2451.4 ns)
   2225.0 |########################################
   2236.3 |########################################
   2247.6 |
   2259.0 |
   2270.3 |
   2281.6 |########################################
   2292.9 |
   2304.3 |
   2315.6 |
   2326.9 |
   2338.2 |########################################
   2349.5 |
   2360.9 |
   2372.2 |
   2383.5 |
   2394.8 |
   2406.2 |
   2417.5 |########################################
   2428.8 |
   2440.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_tight_fresh_per_batch**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_fresh_per_column**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_held_handle**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_null_entry**: bridge=5015.9% of algo (FFI overhead may distort results)
