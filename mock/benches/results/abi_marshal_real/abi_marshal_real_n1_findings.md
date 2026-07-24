# abi_marshal (real)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_real_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_real_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_real_marshal_null dominates: 21315% faster than the next best (abi_marshal_real_aos)

abi_marshal_real_marshal_null (9.98 us) leads abi_marshal_real_aos (2.14 ms) by 21315%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_real_marshal_null beats baseline by 100% (significant)

abi_marshal_real_marshal_null is -2.13 ms (100%) faster than baseline abi_marshal_real_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_real_soa_native is an outlier: 215.3x slower than the field

abi_marshal_real_soa_native (2.15 ms) is 215.3x the fastest (9.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_real_marshal_null shows alternating (throttle bounce) (autocorr -0.66)

abi_marshal_real_marshal_null's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_real_marshal_null} vs {abi_marshal_real_aos, abi_marshal_real_soa_transposed, abi_marshal_real_soa_native} (21315% apart)

The field splits into a fast tier {abi_marshal_real_marshal_null} and a slow tier {abi_marshal_real_aos, abi_marshal_real_soa_transposed, abi_marshal_real_soa_native} with a 21315% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 215.3x the fastest

Fastest abi_marshal_real_marshal_null (9.98 us) to slowest abi_marshal_real_soa_native (2.15 ms): 215.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_real_marshal_null** at 9982.1 ns median (-99.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 215.29x (fastest 9982.1 ns, slowest 2149030.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2162453ns | 2140200ns | 2131362ns | 2137907ns | 2214817ns | base |
| abi_marshal_real_marshal_null | 12150ns | 12170ns | 11914ns | 12148ns | 12272ns | -99.44% |
| abi_marshal_real_soa_native | 2149069ns | 2151581ns | 2124909ns | 2144595ns | 2167861ns | -0.62% |
| abi_marshal_real_soa_transposed | 2205217ns | 2146095ns | 2132925ns | 2144568ns | 2332338ns | +1.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_real_aos | 2159771ns | 2128820ns | 2211932ns | base | 0.000 |
| abi_marshal_real_marshal_null | 9966ns | 9775ns | 10064ns | -99.54% | 0.000 |
| abi_marshal_real_soa_native | 2146427ns | 2122454ns | 2165051ns | -0.62% | 0.000 |
| abi_marshal_real_soa_transposed | 2202433ns | 2130300ns | 2329126ns | +1.98% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_real_aos | 45940.5 | 2177365.6 | 2159770.8 | n/a |
| abi_marshal_real_marshal_null | 26726.0 | 10092.4 | 9966.3 | n/a |
| abi_marshal_real_soa_native | 44791.9 | 2147143.8 | 2146427.1 | n/a |
| abi_marshal_real_soa_transposed | 47681.5 | 2197829.1 | 2202432.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_real_marshal_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_real_aos | 0.000 | 0.5% |
| abi_marshal_real_marshal_null | 0.000 | 97.9% |
| abi_marshal_real_soa_native | 0.000 | 0.5% |
| abi_marshal_real_soa_transposed | 0.000 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_real_aos | 2162453ns | 2162453ns | base |
| abi_marshal_real_marshal_null | 12150ns | 12150ns | -99.44% |
| abi_marshal_real_soa_native | 2149069ns | 2149069ns | -0.62% |
| abi_marshal_real_soa_transposed | 2205217ns | 2205217ns | +1.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2137629ns | base | --- | [2129751, 2211932] | --- | --- | --- | --- |
| abi_marshal_real_marshal_null | 9982ns | -2127619.2ns (-99.5%) | [-2201896, -2119899]ns | [9852, 10064] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_real_soa_native | 2149031ns | no significant difference | [-49883, +19280]ns | [2125200, 2165051] | no | 0.6875 | 0.6875 | 0 |
| abi_marshal_real_soa_transposed | 2143534ns | no significant difference | [-11312, +132724]ns | [2134638, 2329126] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_real_aos | abi_marshal_real_marshal_null | abi_marshal_real_soa_native | abi_marshal_real_soa_transposed |
|---|---|---|---|---|
| 1 | 2140017ns | -99.5% | -0.6% | +0.2% |
| 2 | 2135242ns | -99.5% | -0.6% | -0.2% |
| 3 | 2128820ns | -99.5% | +0.9% | +0.6% |
| 4 | 2258644ns | -99.6% | -3.9% | +11.2% |
| 5 | 2130682ns | -99.5% | +0.9% | +0.4% |
| 6 | 2165220ns | -99.5% | -0.3% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_real_aos | -0.385 | moderate- |
| abi_marshal_real_marshal_null | -0.664 | HIGH- (thermal bounce) |
| abi_marshal_real_soa_native | 0.342 | moderate+ |
| abi_marshal_real_soa_transposed | -0.229 | moderate- |

**Consistency summary:**

- **abi_marshal_real_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_real_soa_native**: won 4/6, lost 2/6
- **abi_marshal_real_soa_transposed**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_real_aos | 6551225.5ns | 2159770.8ns | 303.3% | HIGH |
| abi_marshal_real_marshal_null | 137064.8ns | 9966.3ns | 1375.3% | HIGH |
| abi_marshal_real_soa_native | 6490228.2ns | 2146427.1ns | 302.4% | HIGH |
| abi_marshal_real_soa_transposed | 6657795.9ns | 2202432.9ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_marshal_real_aos (n=6, range 2128820.4-2211932.0 ns)
  2128820.4 |########################################
  2132976.0 |####################
  2137131.6 |####################
  2141287.1 |
  2145442.7 |
  2149598.3 |
  2153753.9 |
  2157909.5 |
  2162065.1 |####################
  2166220.6 |
  2170376.2 |
  2174531.8 |
  2178687.4 |
  2182843.0 |
  2186998.6 |
  2191154.1 |
  2195309.7 |
  2199465.3 |
  2203620.9 |
  2207776.5 |
  (0 below, 1 above range)

abi_marshal_real_marshal_null (n=6, range 9774.6-10064.3 ns)
   9774.6 |########################################
   9789.1 |
   9803.6 |
   9818.1 |
   9832.5 |
   9847.0 |
   9861.5 |
   9876.0 |
   9890.5 |
   9905.0 |
   9919.5 |########################################
   9934.0 |
   9948.4 |
   9962.9 |########################################
   9977.4 |
   9991.9 |########################################
  10006.4 |
  10020.9 |########################################
  10035.4 |
  10049.9 |
  (0 below, 1 above range)

abi_marshal_real_soa_native (n=6, range 2122454.2-2165051.2 ns)
  2122454.2 |########################################
  2124584.1 |
  2126713.9 |########################################
  2128843.8 |
  2130973.6 |
  2133103.5 |
  2135233.3 |
  2137363.2 |
  2139493.0 |
  2141622.9 |
  2143752.7 |
  2145882.6 |########################################
  2148012.4 |
  2150142.3 |########################################
  2152272.1 |
  2154402.0 |
  2156531.8 |########################################
  2158661.7 |
  2160791.5 |
  2162921.4 |
  (0 below, 1 above range)

abi_marshal_real_soa_transposed (n=6, range 2130299.6-2329126.5 ns)
  2130299.6 |##########################
  2140240.9 |########################################
  2150182.3 |
  2160123.6 |
  2170065.0 |
  2180006.3 |
  2189947.7 |
  2199889.0 |
  2209830.3 |
  2219771.7 |
  2229713.0 |
  2239654.4 |
  2249595.7 |
  2259537.1 |
  2269478.4 |
  2279419.7 |
  2289361.1 |
  2299302.4 |
  2309243.8 |
  2319185.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_real_aos**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_marshal_real_marshal_null**: bridge=1374.9% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_native**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_transposed**: bridge=301.7% of algo (FFI overhead may distort results)
