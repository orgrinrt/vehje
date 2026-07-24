# abi_marshal (real)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_real_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_real_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_real_marshal_null dominates: 16820% faster than the next best (abi_marshal_real_aos)

abi_marshal_real_marshal_null (12.69 us) leads abi_marshal_real_aos (2.15 ms) by 16820%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_real_marshal_null beats baseline by 99% (significant)

abi_marshal_real_marshal_null is -2.13 ms (99%) faster than baseline abi_marshal_real_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_real_soa_transposed is an outlier: 169.6x slower than the field

abi_marshal_real_soa_transposed (2.15 ms) is 169.6x the fastest (12.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_real_marshal_null} vs {abi_marshal_real_aos, abi_marshal_real_soa_native, abi_marshal_real_soa_transposed} (16820% apart)

The field splits into a fast tier {abi_marshal_real_marshal_null} and a slow tier {abi_marshal_real_aos, abi_marshal_real_soa_native, abi_marshal_real_soa_transposed} with a 16820% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 169.6x the fastest

Fastest abi_marshal_real_marshal_null (12.69 us) to slowest abi_marshal_real_soa_transposed (2.15 ms): 169.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_real_marshal_null** at 12690.2 ns median (-99.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 169.60x (fastest 12690.2 ns, slowest 2152306.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2168412ns | 2149714ns | 2134458ns | 2145527ns | 2219715ns | base |
| abi_marshal_real_marshal_null | 14905ns | 15037ns | 13887ns | 14842ns | 15511ns | -99.31% |
| abi_marshal_real_soa_native | 2169189ns | 2150305ns | 2141914ns | 2148639ns | 2213650ns | +0.04% |
| abi_marshal_real_soa_transposed | 2165994ns | 2154894ns | 2143514ns | 2154048ns | 2195152ns | -0.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_real_aos | 2165791ns | 2131882ns | 2216918ns | base | 0.000 |
| abi_marshal_real_marshal_null | 12559ns | 11751ns | 13056ns | -99.42% | 0.000 |
| abi_marshal_real_soa_native | 2166463ns | 2139369ns | 2210796ns | +0.03% | 0.000 |
| abi_marshal_real_soa_transposed | 2163319ns | 2140867ns | 2192358ns | -0.11% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_real_aos | 44854.3 | 2153401.9 | 2165791.2 | n/a |
| abi_marshal_real_marshal_null | 28569.4 | 12731.2 | 12559.2 | n/a |
| abi_marshal_real_soa_native | 45969.4 | 2163601.2 | 2166462.7 | n/a |
| abi_marshal_real_soa_transposed | 43778.5 | 2390148.1 | 2163319.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_real_marshal_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_real_aos | 0.000 | 0.5% |
| abi_marshal_real_marshal_null | 0.000 | 92.6% |
| abi_marshal_real_soa_native | 0.000 | 0.5% |
| abi_marshal_real_soa_transposed | 0.000 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_real_aos | 2168412ns | 2168412ns | base |
| abi_marshal_real_marshal_null | 14905ns | 14905ns | -99.31% |
| abi_marshal_real_soa_native | 2169189ns | 2169189ns | +0.04% |
| abi_marshal_real_soa_transposed | 2165994ns | 2165994ns | -0.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2147160ns | base | --- | [2133295, 2216918] | --- | --- | --- | --- |
| abi_marshal_real_marshal_null | 12690ns | -2134456.1ns (-99.4%) | [-2204440, -2120800]ns | [11932, 13056] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_real_soa_native | 2147668ns | no significant difference | [-14971, +14372]ns | [2140925, 2210796] | no | 1.0000 | 1.0000 | 0 |
| abi_marshal_real_soa_transposed | 2152306ns | no significant difference | [-33162, +19011]ns | [2145293, 2192358] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_real_aos | abi_marshal_real_marshal_null | abi_marshal_real_soa_native | abi_marshal_real_soa_transposed |
|---|---|---|---|---|
| 1 | 2131882ns | -99.4% | +0.5% | +1.0% |
| 2 | 2134708ns | -99.4% | +0.8% | +0.8% |
| 3 | 2149877ns | -99.5% | +0.4% | -0.0% |
| 4 | 2148945ns | -99.4% | -0.4% | -0.4% |
| 5 | 2145374ns | -99.4% | -0.1% | +0.6% |
| 6 | 2283960ns | -99.4% | -0.9% | -2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_real_aos | -0.015 | ok |
| abi_marshal_real_marshal_null | -0.101 | ok |
| abi_marshal_real_soa_native | -0.086 | ok |
| abi_marshal_real_soa_transposed | 0.087 | ok |

**Consistency summary:**

- **abi_marshal_real_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_real_soa_native**: won 3/6, lost 3/6
- **abi_marshal_real_soa_transposed**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_real_aos | 6505474.2ns | 2165791.2ns | 300.4% | HIGH |
| abi_marshal_real_marshal_null | 145137.8ns | 12559.2ns | 1155.6% | HIGH |
| abi_marshal_real_soa_native | 6661868.8ns | 2166462.7ns | 307.5% | HIGH |
| abi_marshal_real_soa_transposed | 6758179.5ns | 2163319.2ns | 312.4% | HIGH |

## Distribution (algo ns)

```
abi_marshal_real_aos (n=6, range 2131882.5-2216918.4 ns)
  2131882.5 |########################################
  2136134.3 |
  2140386.1 |
  2144637.9 |####################
  2148889.7 |########################################
  2153141.5 |
  2157393.3 |
  2161645.0 |
  2165896.8 |
  2170148.6 |
  2174400.4 |
  2178652.2 |
  2182904.0 |
  2187155.8 |
  2191407.6 |
  2195659.4 |
  2199911.2 |
  2204163.0 |
  2208414.8 |
  2212666.6 |
  (0 below, 1 above range)

abi_marshal_real_marshal_null (n=6, range 11751.2-13055.6 ns)
  11751.2 |####################
  11816.4 |
  11881.6 |
  11946.9 |
  12012.1 |
  12077.3 |####################
  12142.5 |
  12207.7 |
  12273.0 |
  12338.2 |
  12403.4 |
  12468.6 |####################
  12533.8 |
  12599.1 |
  12664.3 |
  12729.5 |
  12794.7 |
  12859.9 |########################################
  12925.2 |
  12990.4 |
  (0 below, 1 above range)

abi_marshal_real_soa_native (n=6, range 2139369.2-2210795.6 ns)
  2139369.2 |########################################
  2142940.5 |####################
  2146511.8 |
  2150083.2 |####################
  2153654.5 |
  2157225.8 |####################
  2160797.1 |
  2164368.5 |
  2167939.8 |
  2171511.1 |
  2175082.4 |
  2178653.7 |
  2182225.1 |
  2185796.4 |
  2189367.7 |
  2192939.0 |
  2196510.4 |
  2200081.7 |
  2203653.0 |
  2207224.3 |
  (0 below, 1 above range)

abi_marshal_real_soa_transposed (n=6, range 2140867.1-2192358.0 ns)
  2140867.1 |####################
  2143441.6 |
  2146016.2 |
  2148590.7 |########################################
  2151165.3 |####################
  2153739.8 |
  2156314.4 |
  2158888.9 |####################
  2161463.4 |
  2164038.0 |
  2166612.5 |
  2169187.1 |
  2171761.6 |
  2174336.2 |
  2176910.7 |
  2179485.2 |
  2182059.8 |
  2184634.3 |
  2187208.9 |
  2189783.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_real_aos**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_marshal_real_marshal_null**: bridge=1136.4% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_native**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_transposed**: bridge=302.0% of algo (FFI overhead may distort results)
