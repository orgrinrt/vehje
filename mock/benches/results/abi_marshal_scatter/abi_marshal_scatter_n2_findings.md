# abi_marshal (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_scatter_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_scatter_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_scatter_marshal_null dominates: 17251% faster than the next best (abi_marshal_scatter_soa_native)

abi_marshal_scatter_marshal_null (12.17 us) leads abi_marshal_scatter_soa_native (2.11 ms) by 17251%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_scatter_marshal_null beats baseline by 99% (significant)

abi_marshal_scatter_marshal_null is -2.11 ms (99%) faster than baseline abi_marshal_scatter_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_scatter_soa_transposed is an outlier: 174.6x slower than the field

abi_marshal_scatter_soa_transposed (2.12 ms) is 174.6x the fastest (12.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_scatter_marshal_null} vs {abi_marshal_scatter_soa_native, abi_marshal_scatter_aos, abi_marshal_scatter_soa_transposed} (17251% apart)

The field splits into a fast tier {abi_marshal_scatter_marshal_null} and a slow tier {abi_marshal_scatter_soa_native, abi_marshal_scatter_aos, abi_marshal_scatter_soa_transposed} with a 17251% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 174.6x the fastest

Fastest abi_marshal_scatter_marshal_null (12.17 us) to slowest abi_marshal_scatter_soa_transposed (2.12 ms): 174.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_scatter_marshal_null** at 12166.2 ns median (-99.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 174.64x (fastest 12166.2 ns, slowest 2124726.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2125509ns | 2125121ns | 2114515ns | 2123195ns | 2134477ns | base |
| abi_marshal_scatter_marshal_null | 14564ns | 14391ns | 14014ns | 14285ns | 15256ns | -99.31% |
| abi_marshal_scatter_soa_native | 2116368ns | 2113545ns | 2109324ns | 2112817ns | 2125217ns | -0.43% |
| abi_marshal_scatter_soa_transposed | 2128475ns | 2127354ns | 2121677ns | 2126377ns | 2135021ns | +0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2122996ns | 2111977ns | 2131895ns | base | 0.000 |
| abi_marshal_scatter_marshal_null | 12298ns | 11835ns | 12888ns | -99.42% | 0.000 |
| abi_marshal_scatter_soa_native | 2113860ns | 2106798ns | 2122706ns | -0.43% | 0.000 |
| abi_marshal_scatter_soa_transposed | 2125887ns | 2119098ns | 2132375ns | +0.14% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_scatter_aos | 39342.8 | 2122842.9 | 2122995.8 | n/a |
| abi_marshal_scatter_marshal_null | 26608.2 | 12455.0 | 12297.9 | n/a |
| abi_marshal_scatter_soa_native | 38969.4 | 2113364.8 | 2113859.6 | n/a |
| abi_marshal_scatter_soa_transposed | 40057.0 | 2126204.4 | 2125887.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_scatter_marshal_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_scatter_aos | 0.000 | 0.6% |
| abi_marshal_scatter_marshal_null | 0.000 | 97.3% |
| abi_marshal_scatter_soa_native | 0.000 | 0.6% |
| abi_marshal_scatter_soa_transposed | 0.000 | 0.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_scatter_aos | 2125509ns | 2125509ns | base |
| abi_marshal_scatter_marshal_null | 14564ns | 14564ns | -99.31% |
| abi_marshal_scatter_soa_native | 2116368ns | 2116368ns | -0.43% |
| abi_marshal_scatter_soa_transposed | 2128475ns | 2128475ns | +0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2122727ns | base | --- | [2114366, 2131895] | --- | --- | --- | --- |
| abi_marshal_scatter_marshal_null | 12166ns | -2110097.3ns (-99.4%) | [-2119530, -2102466]ns | [11840, 12888] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_scatter_soa_native | 2110992ns | no significant difference | [-24014, +3419]ns | [2107881, 2122706] | no | 0.6875 | 0.6875 | 0 |
| abi_marshal_scatter_soa_transposed | 2124727ns | no significant difference | [-4616, +8986]ns | [2120560, 2132375] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_scatter_aos | abi_marshal_scatter_marshal_null | abi_marshal_scatter_soa_native | abi_marshal_scatter_soa_transposed |
|---|---|---|---|---|
| 1 | 2123635ns | -99.4% | -0.6% | +0.2% |
| 2 | 2132506ns | -99.4% | -1.2% | +0.2% |
| 3 | 2121818ns | -99.4% | +0.1% | -0.1% |
| 4 | 2111977ns | -99.4% | -0.0% | +0.5% |
| 5 | 2131283ns | -99.4% | -1.0% | -0.3% |
| 6 | 2116755ns | -99.4% | +0.2% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_scatter_aos | -0.421 | moderate- |
| abi_marshal_scatter_marshal_null | 0.386 | moderate+ |
| abi_marshal_scatter_soa_native | -0.410 | moderate- |
| abi_marshal_scatter_soa_transposed | -0.106 | ok |

**Consistency summary:**

- **abi_marshal_scatter_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_scatter_soa_native**: won 3/6, lost 2/6
- **abi_marshal_scatter_soa_transposed**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_scatter_aos | 6413611.4ns | 2122995.8ns | 302.1% | HIGH |
| abi_marshal_scatter_marshal_null | 144272.4ns | 12297.9ns | 1173.1% | HIGH |
| abi_marshal_scatter_soa_native | 6383684.4ns | 2113859.6ns | 302.0% | HIGH |
| abi_marshal_scatter_soa_transposed | 6421476.1ns | 2125887.3ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_marshal_scatter_aos (n=6, range 2111977.1-2131894.5 ns)
  2111977.1 |########################################
  2112973.0 |
  2113968.8 |
  2114964.7 |
  2115960.6 |########################################
  2116956.5 |
  2117952.3 |
  2118948.2 |
  2119944.1 |
  2120940.0 |########################################
  2121935.8 |
  2122931.7 |########################################
  2123927.6 |
  2124923.4 |
  2125919.3 |
  2126915.2 |
  2127911.1 |
  2128906.9 |
  2129902.8 |
  2130898.7 |########################################
  (0 below, 1 above range)

abi_marshal_scatter_marshal_null (n=6, range 11835.4-12887.7 ns)
  11835.4 |########################################
  11888.0 |
  11940.6 |####################
  11993.2 |
  12045.9 |
  12098.5 |
  12151.1 |
  12203.7 |
  12256.3 |
  12308.9 |
  12361.5 |####################
  12414.2 |
  12466.8 |
  12519.4 |
  12572.0 |
  12624.6 |
  12677.2 |
  12729.9 |
  12782.5 |
  12835.1 |####################
  (0 below, 1 above range)

abi_marshal_scatter_soa_native (n=6, range 2106798.3-2122706.0 ns)
  2106798.3 |####################
  2107593.7 |
  2108389.1 |####################
  2109184.5 |
  2109979.8 |
  2110775.2 |########################################
  2111570.6 |
  2112366.0 |
  2113161.4 |
  2113956.8 |
  2114752.2 |
  2115547.6 |
  2116342.9 |
  2117138.3 |
  2117933.7 |
  2118729.1 |
  2119524.5 |
  2120319.9 |####################
  2121115.3 |
  2121910.7 |
  (0 below, 1 above range)

abi_marshal_scatter_soa_transposed (n=6, range 2119098.3-2132375.2 ns)
  2119098.3 |####################
  2119762.1 |
  2120426.0 |
  2121089.8 |
  2121753.7 |####################
  2122417.5 |
  2123081.4 |
  2123745.2 |
  2124409.1 |########################################
  2125072.9 |
  2125736.8 |
  2126400.6 |
  2127064.5 |
  2127728.3 |####################
  2128392.2 |
  2129056.0 |
  2129719.9 |
  2130383.7 |
  2131047.6 |
  2131711.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_scatter_aos**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_marshal_null**: bridge=1184.8% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_soa_native**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_soa_transposed**: bridge=302.1% of algo (FFI overhead may distort results)
