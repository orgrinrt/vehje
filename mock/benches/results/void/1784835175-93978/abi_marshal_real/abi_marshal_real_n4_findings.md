# abi_marshal (real)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_real_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_real_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_real_marshal_null dominates: 10593% faster than the next best (abi_marshal_real_soa_native)

abi_marshal_real_marshal_null (20.22 us) leads abi_marshal_real_soa_native (2.16 ms) by 10593%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_real_marshal_null beats baseline by 99% (significant)

abi_marshal_real_marshal_null is -2.15 ms (99%) faster than baseline abi_marshal_real_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_real_soa_transposed is an outlier: 107.7x slower than the field

abi_marshal_real_soa_transposed (2.18 ms) is 107.7x the fastest (20.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_real_marshal_null} vs {abi_marshal_real_soa_native, abi_marshal_real_aos, abi_marshal_real_soa_transposed} (10593% apart)

The field splits into a fast tier {abi_marshal_real_marshal_null} and a slow tier {abi_marshal_real_soa_native, abi_marshal_real_aos, abi_marshal_real_soa_transposed} with a 10593% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 107.7x the fastest

Fastest abi_marshal_real_marshal_null (20.22 us) to slowest abi_marshal_real_soa_transposed (2.18 ms): 107.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_real_marshal_null** at 20217.0 ns median (-99.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 107.69x (fastest 20217.0 ns, slowest 2177168.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2180406ns | 2177477ns | 2167632ns | 2175154ns | 2194672ns | base |
| abi_marshal_real_marshal_null | 22583ns | 22452ns | 21939ns | 22342ns | 23266ns | -98.96% |
| abi_marshal_real_soa_native | 2166148ns | 2164477ns | 2155895ns | 2163523ns | 2175213ns | -0.65% |
| abi_marshal_real_soa_transposed | 2180230ns | 2180068ns | 2174749ns | 2179424ns | 2184180ns | -0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_real_aos | 2177522ns | 2164758ns | 2191643ns | base | 0.000 |
| abi_marshal_real_marshal_null | 20323ns | 19748ns | 20946ns | -99.07% | 0.000 |
| abi_marshal_real_soa_native | 2163339ns | 2153220ns | 2172244ns | -0.65% | 0.000 |
| abi_marshal_real_soa_transposed | 2177425ns | 2172032ns | 2181372ns | -0.00% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_real_aos | 52358.8 | 2176581.8 | 2177522.2 | n/a |
| abi_marshal_real_marshal_null | 27318.1 | 20332.3 | 20323.3 | n/a |
| abi_marshal_real_soa_native | 52403.5 | 2161157.9 | 2163339.0 | 0 |
| abi_marshal_real_soa_transposed | 50294.0 | 2177778.4 | 2177425.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_real_marshal_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_real_aos | 0.000 | 0.9% |
| abi_marshal_real_marshal_null | 0.000 | 97.7% |
| abi_marshal_real_soa_native | 0.000 | 0.9% |
| abi_marshal_real_soa_transposed | 0.000 | 0.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_real_aos | 2180406ns | 2180406ns | base |
| abi_marshal_real_marshal_null | 22583ns | 22583ns | -98.96% |
| abi_marshal_real_soa_native | 2166148ns | 2166148ns | -0.65% |
| abi_marshal_real_soa_transposed | 2180230ns | 2180230ns | -0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2174627ns | base | --- | [2166297, 2191643] | --- | --- | --- | --- |
| abi_marshal_real_marshal_null | 20217ns | -2153681.2ns (-99.0%) | [-2171823, -2146092]ns | [19807, 20946] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_real_soa_native | 2161760ns | no significant difference | [-33574, +5947]ns | [2156013, 2172244] | no | 0.6875 | 0.6875 | 0 |
| abi_marshal_real_soa_transposed | 2177168ns | no significant difference | [-17176, +13763]ns | [2173735, 2181372] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_real_aos | abi_marshal_real_marshal_null | abi_marshal_real_soa_native | abi_marshal_real_soa_transposed |
|---|---|---|---|---|
| 1 | 2181832ns | -99.1% | -0.9% | -0.4% |
| 2 | 2167837ns | -99.1% | +0.2% | +0.8% |
| 3 | 2177483ns | -99.1% | -0.9% | +0.0% |
| 4 | 2201454ns | -99.1% | -2.2% | -1.1% |
| 5 | 2164758ns | -99.1% | +0.4% | +0.5% |
| 6 | 2171770ns | -99.0% | -0.5% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_real_aos | -0.311 | moderate- |
| abi_marshal_real_marshal_null | -0.242 | moderate- |
| abi_marshal_real_soa_native | -0.411 | moderate- |
| abi_marshal_real_soa_transposed | -0.392 | moderate- |

**Consistency summary:**

- **abi_marshal_real_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_real_soa_native**: won 4/6, lost 2/6
- **abi_marshal_real_soa_transposed**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_real_aos | 6584810.0ns | 2177522.2ns | 302.4% | HIGH |
| abi_marshal_real_marshal_null | 168023.7ns | 20323.3ns | 826.8% | HIGH |
| abi_marshal_real_soa_native | 6544691.3ns | 2163339.0ns | 302.5% | HIGH |
| abi_marshal_real_soa_transposed | 6589246.0ns | 2177425.2ns | 302.6% | HIGH |

## Distribution (algo ns)

```
abi_marshal_real_aos (n=6, range 2164757.5-2191642.8 ns)
  2164757.5 |########################################
  2166101.8 |
  2167446.0 |########################################
  2168790.3 |
  2170134.5 |
  2171478.8 |########################################
  2172823.1 |
  2174167.3 |
  2175511.6 |
  2176855.9 |########################################
  2178200.1 |
  2179544.4 |
  2180888.6 |########################################
  2182232.9 |
  2183577.2 |
  2184921.4 |
  2186265.7 |
  2187610.0 |
  2188954.2 |
  2190298.5 |
  (0 below, 1 above range)

abi_marshal_real_marshal_null (n=6, range 19747.9-20945.6 ns)
  19747.9 |########################################
  19807.8 |########################################
  19867.7 |########################################
  19927.6 |
  19987.4 |
  20047.3 |
  20107.2 |
  20167.1 |
  20227.0 |
  20286.9 |
  20346.8 |
  20406.6 |
  20466.5 |
  20526.4 |########################################
  20586.3 |########################################
  20646.2 |
  20706.1 |
  20765.9 |
  20825.8 |
  20885.7 |
  (0 below, 1 above range)

abi_marshal_real_soa_native (n=6, range 2153220.4-2172243.8 ns)
  2153220.4 |########################################
  2154171.6 |
  2155122.7 |
  2156073.9 |
  2157025.1 |
  2157976.2 |########################################
  2158927.4 |
  2159878.6 |########################################
  2160829.7 |
  2161780.9 |
  2162732.1 |########################################
  2163683.2 |
  2164634.4 |
  2165585.6 |
  2166536.7 |
  2167487.9 |
  2168439.1 |
  2169390.2 |
  2170341.4 |
  2171292.6 |########################################
  (0 below, 1 above range)

abi_marshal_real_soa_transposed (n=6, range 2172032.5-2181372.1 ns)
  2172032.5 |########################################
  2172499.5 |
  2172966.5 |
  2173433.4 |
  2173900.4 |
  2174367.4 |
  2174834.4 |
  2175301.4 |########################################
  2175768.3 |
  2176235.3 |
  2176702.3 |########################################
  2177169.3 |########################################
  2177636.3 |########################################
  2178103.2 |
  2178570.2 |
  2179037.2 |
  2179504.2 |
  2179971.2 |
  2180438.1 |
  2180905.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_real_aos**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_marshal_real_marshal_null**: bridge=831.4% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_native**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_transposed**: bridge=302.6% of algo (FFI overhead may distort results)
