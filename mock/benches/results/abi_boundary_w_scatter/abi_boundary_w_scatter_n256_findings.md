# abi_boundary_w (scatter)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_scatter_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_scatter_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_scatter_null_entry dominates: 28120% faster than the next best (abi_boundary_w_scatter_soa_runtime_w)

abi_boundary_w_scatter_null_entry (3.19 us) leads abi_boundary_w_scatter_soa_runtime_w (899.26 us) by 28120%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_scatter_null_entry beats baseline by 100% (significant)

abi_boundary_w_scatter_null_entry is -2.17 ms (100%) faster than baseline abi_boundary_w_scatter_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_scatter_scalar_anchor is an outlier: 681.3x slower than the field

abi_boundary_w_scatter_scalar_anchor (2.17 ms) is 681.3x the fastest (3.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_scatter_scalar_anchor shows alternating (throttle bounce) (autocorr -0.57)

abi_boundary_w_scatter_scalar_anchor's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_scatter_null_entry} vs {abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_anchor} (28120% apart)

The field splits into a fast tier {abi_boundary_w_scatter_null_entry} and a slow tier {abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_anchor} with a 28120% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 681.3x the fastest

Fastest abi_boundary_w_scatter_null_entry (3.19 us) to slowest abi_boundary_w_scatter_scalar_anchor (2.17 ms): 681.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_scatter_null_entry** at 3186.7 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 681.26x (fastest 3186.7 ns, slowest 2170928.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 5472ns | 5469ns | 5292ns | 5454ns | 5588ns | -99.75% |
| abi_boundary_w_scatter_scalar_anchor | 2170476ns | 2174159ns | 2157680ns | 2169696ns | 2178045ns | -0.09% |
| abi_boundary_w_scatter_scalar_dispatch | 2170203ns | 2167790ns | 2156478ns | 2165575ns | 2184007ns | -0.11% |
| abi_boundary_w_scatter_scalar_per_w | 2172139ns | 2169676ns | 2159772ns | 2168886ns | 2183204ns | -0.02% |
| abi_boundary_w_scatter_scalar_runtime_w | 2172526ns | 2172295ns | 2157748ns | 2171346ns | 2181684ns | base |
| abi_boundary_w_scatter_soa_dispatch | 904419ns | 905315ns | 891953ns | 904936ns | 909877ns | -58.37% |
| abi_boundary_w_scatter_soa_per_w | 905844ns | 905626ns | 896625ns | 903782ns | 913547ns | -58.30% |
| abi_boundary_w_scatter_soa_runtime_w | 902095ns | 902104ns | 893890ns | 901294ns | 907399ns | -58.48% |
| abi_boundary_w_scatter_zig_runtime_w | 2145933ns | 2145422ns | 2134750ns | 2143536ns | 2155119ns | -1.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 3182ns | 3114ns | 3241ns | -99.85% | 0.080 |
| abi_boundary_w_scatter_scalar_anchor | 2167211ns | 2154761ns | 2174657ns | -0.10% | 0.000 |
| abi_boundary_w_scatter_scalar_dispatch | 2167078ns | 2153731ns | 2180619ns | -0.10% | 0.000 |
| abi_boundary_w_scatter_scalar_per_w | 2168999ns | 2157072ns | 2179759ns | -0.01% | 0.000 |
| abi_boundary_w_scatter_scalar_runtime_w | 2169282ns | 2154927ns | 2178419ns | base | 0.000 |
| abi_boundary_w_scatter_soa_dispatch | 901614ns | 889410ns | 906880ns | -58.44% | 0.000 |
| abi_boundary_w_scatter_soa_per_w | 903229ns | 894215ns | 910781ns | -58.36% | 0.000 |
| abi_boundary_w_scatter_soa_runtime_w | 899315ns | 891232ns | 904541ns | -58.54% | 0.000 |
| abi_boundary_w_scatter_zig_runtime_w | 2142354ns | 2131568ns | 2151324ns | -1.24% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 28032.6 | 3237.1 | 3182.1 | n/a |
| abi_boundary_w_scatter_scalar_anchor | 62412.7 | 2168934.7 | 2167211.1 | 1 |
| abi_boundary_w_scatter_scalar_dispatch | 59217.2 | 2166406.3 | 2167078.1 | n/a |
| abi_boundary_w_scatter_scalar_per_w | 62607.0 | 2166614.5 | 2168998.8 | 7 |
| abi_boundary_w_scatter_scalar_runtime_w | 60537.8 | 2169761.7 | 2169281.8 | n/a |
| abi_boundary_w_scatter_soa_dispatch | 44703.0 | 902107.4 | 901613.5 | n/a |
| abi_boundary_w_scatter_soa_per_w | 40854.9 | 903797.3 | 903229.0 | n/a |
| abi_boundary_w_scatter_soa_runtime_w | 45229.6 | 899083.4 | 899315.3 | n/a |
| abi_boundary_w_scatter_zig_runtime_w | 237772.5 | 2142099.4 | 2142353.9 | 7 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_boundary_w_scatter_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.080 | 97.7% |
| abi_boundary_w_scatter_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_scatter_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_scatter_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_scatter_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_scatter_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_scatter_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_scatter_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_scatter_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 5472ns | 5472ns | -99.75% |
| abi_boundary_w_scatter_scalar_anchor | 2170476ns | 2170476ns | -0.09% |
| abi_boundary_w_scatter_scalar_dispatch | 2170203ns | 2170203ns | -0.11% |
| abi_boundary_w_scatter_scalar_per_w | 2172139ns | 2172139ns | -0.02% |
| abi_boundary_w_scatter_scalar_runtime_w | 2172526ns | 2172526ns | base |
| abi_boundary_w_scatter_soa_dispatch | 904419ns | 904419ns | -58.37% |
| abi_boundary_w_scatter_soa_per_w | 905844ns | 905844ns | -58.30% |
| abi_boundary_w_scatter_soa_runtime_w | 902095ns | 902095ns | -58.48% |
| abi_boundary_w_scatter_zig_runtime_w | 2145933ns | 2145933ns | -1.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_scalar_runtime_w | 2168938ns | base | --- | [2160488, 2178419] | --- | --- | --- | --- |
| abi_boundary_w_scatter_null_entry | 3187ns | -2165751.9ns (-99.9%) | [-2175244, -2157303]ns | [3119, 3241] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_anchor | 2170928ns | no significant difference | [-18028, +12014]ns | [2156048, 2174657] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_scatter_scalar_dispatch | 2164679ns | no significant difference | [-10519, +7886]ns | [2155936, 2180619] | no | 0.9167 | 0.6875 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 2166449ns | no significant difference | [-9976, +8062]ns | [2160788, 2179759] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_scatter_soa_dispatch | 902640ns | -1270503.7ns (-58.6%) | [-1277009, -1255492]ns | [895321, 906880] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_per_w | 902939ns | -1264934.4ns (-58.3%) | [-1278675, -1254549]ns | [895967, 910781] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_runtime_w | 899263ns | -1271127.7ns (-58.6%) | [-1276748, -1262023]ns | [894142, 904541] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_zig_runtime_w | 2141788ns | -25263.9ns (-1.2%) | [-40127, -15393]ns | [2133950, 2151324] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_scatter_scalar_runtime_w | abi_boundary_w_scatter_null_entry | abi_boundary_w_scatter_scalar_anchor | abi_boundary_w_scatter_scalar_dispatch | abi_boundary_w_scatter_scalar_per_w | abi_boundary_w_scatter_soa_dispatch | abi_boundary_w_scatter_soa_per_w | abi_boundary_w_scatter_soa_runtime_w | abi_boundary_w_scatter_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2154927ns | -99.8% | +0.8% | +0.1% | +0.1% | -58.1% | -58.3% | -58.4% | -0.5% |
| 2 | 2169216ns | -99.9% | +0.3% | +0.6% | -0.1% | -58.4% | -58.5% | -58.4% | -1.4% |
| 3 | 2166049ns | -99.9% | -0.5% | -0.2% | -0.0% | -58.9% | -57.8% | -58.5% | -1.6% |
| 4 | 2168661ns | -99.9% | +0.2% | -0.7% | +0.6% | -58.1% | -58.2% | -58.9% | -1.0% |
| 5 | 2182105ns | -99.9% | -1.1% | -0.1% | -0.8% | -58.5% | -59.0% | -58.5% | -2.1% |
| 6 | 2174733ns | -99.9% | -0.2% | -0.3% | +0.1% | -58.6% | -58.4% | -58.6% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.004 | ok |
| abi_boundary_w_scatter_scalar_anchor | -0.571 | HIGH- (thermal bounce) |
| abi_boundary_w_scatter_scalar_dispatch | -0.427 | moderate- |
| abi_boundary_w_scatter_scalar_per_w | -0.248 | moderate- |
| abi_boundary_w_scatter_scalar_runtime_w | 0.158 | ok |
| abi_boundary_w_scatter_soa_dispatch | -0.320 | moderate- |
| abi_boundary_w_scatter_soa_per_w | -0.095 | ok |
| abi_boundary_w_scatter_soa_runtime_w | -0.477 | moderate- |
| abi_boundary_w_scatter_zig_runtime_w | -0.407 | moderate- |

**Consistency summary:**

- **abi_boundary_w_scatter_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_scalar_anchor**: won 3/6, lost 3/6
- **abi_boundary_w_scatter_scalar_dispatch**: won 4/6, lost 2/6
- **abi_boundary_w_scatter_scalar_per_w**: won 2/6, lost 2/6
- **abi_boundary_w_scatter_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 121677.0ns | 3182.1ns | 3823.8% | HIGH |
| abi_boundary_w_scatter_scalar_anchor | 6571232.6ns | 2167211.1ns | 303.2% | HIGH |
| abi_boundary_w_scatter_scalar_dispatch | 6560626.6ns | 2167078.1ns | 302.7% | HIGH |
| abi_boundary_w_scatter_scalar_per_w | 6567678.5ns | 2168998.8ns | 302.8% | HIGH |
| abi_boundary_w_scatter_scalar_runtime_w | 6576198.5ns | 2169281.8ns | 303.2% | HIGH |
| abi_boundary_w_scatter_soa_dispatch | 2752352.1ns | 901613.5ns | 305.3% | HIGH |
| abi_boundary_w_scatter_soa_per_w | 2753058.8ns | 903229.0ns | 304.8% | HIGH |
| abi_boundary_w_scatter_soa_runtime_w | 2746844.4ns | 899315.3ns | 305.4% | HIGH |
| abi_boundary_w_scatter_zig_runtime_w | 6748318.7ns | 2142353.9ns | 315.0% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_scatter_null_entry (n=6, range 3113.8-3240.8 ns)
   3113.8 |########################################
   3120.2 |########################################
   3126.5 |
   3132.9 |
   3139.2 |
   3145.6 |
   3151.9 |
   3158.2 |########################################
   3164.6 |
   3171.0 |
   3177.3 |
   3183.7 |
   3190.0 |
   3196.4 |
   3202.7 |
   3209.1 |########################################
   3215.4 |
   3221.8 |########################################
   3228.1 |
   3234.5 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_anchor (n=6, range 2154761.2-2174656.6 ns)
  2154761.2 |########################################
  2155756.0 |
  2156750.7 |########################################
  2157745.5 |
  2158740.3 |
  2159735.1 |
  2160729.8 |
  2161724.6 |
  2162719.4 |
  2163714.2 |
  2164708.9 |
  2165703.7 |
  2166698.5 |
  2167693.2 |
  2168688.0 |
  2169682.8 |########################################
  2170677.6 |
  2171672.3 |########################################
  2172667.1 |########################################
  2173661.9 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_dispatch (n=6, range 2153730.8-2180619.0 ns)
  2153730.8 |########################################
  2155075.2 |
  2156419.6 |
  2157764.0 |########################################
  2159108.4 |
  2160452.8 |########################################
  2161797.3 |
  2163141.7 |
  2164486.1 |
  2165830.5 |
  2167174.9 |
  2168519.3 |########################################
  2169863.7 |
  2171208.1 |
  2172552.5 |
  2173897.0 |
  2175241.4 |
  2176585.8 |
  2177930.2 |
  2179274.6 |########################################
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_per_w (n=6, range 2157071.7-2179759.3 ns)
  2157071.7 |########################################
  2158206.1 |
  2159340.5 |
  2160474.8 |
  2161609.2 |
  2162743.6 |
  2163878.0 |########################################
  2165012.4 |########################################
  2166146.8 |########################################
  2167281.1 |
  2168415.5 |
  2169549.9 |
  2170684.3 |
  2171818.7 |
  2172953.1 |
  2174087.4 |
  2175221.8 |
  2176356.2 |
  2177490.6 |########################################
  2178625.0 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_runtime_w (n=6, range 2154927.1-2178419.0 ns)
  2154927.1 |########################################
  2156101.7 |
  2157276.3 |
  2158450.9 |
  2159625.5 |
  2160800.1 |
  2161974.7 |
  2163149.2 |
  2164323.8 |
  2165498.4 |########################################
  2166673.0 |
  2167847.6 |########################################
  2169022.2 |########################################
  2170196.8 |
  2171371.4 |
  2172546.0 |
  2173720.6 |########################################
  2174895.2 |
  2176069.8 |
  2177244.4 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_dispatch (n=6, range 889410.0-906879.8 ns)
  889410.0 |########################################
  890283.5 |
  891157.0 |
  892030.5 |
  892904.0 |
  893777.4 |
  894650.9 |
  895524.4 |
  896397.9 |
  897271.4 |
  898144.9 |
  899018.4 |
  899891.9 |
  900765.4 |########################################
  901638.9 |########################################
  902512.4 |
  903385.8 |########################################
  904259.3 |########################################
  905132.8 |
  906006.3 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_per_w (n=6, range 894215.0-910780.8 ns)
  894215.0 |########################################
  895043.3 |
  895871.6 |
  896699.9 |
  897528.2 |########################################
  898356.5 |
  899184.8 |
  900013.0 |########################################
  900841.3 |
  901669.6 |
  902497.9 |
  903326.2 |
  904154.5 |
  904982.8 |########################################
  905811.1 |
  906639.4 |########################################
  907467.7 |
  908296.0 |
  909124.3 |
  909952.6 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_runtime_w (n=6, range 891232.1-904540.6 ns)
  891232.1 |########################################
  891897.5 |
  892563.0 |
  893228.4 |
  893893.8 |
  894559.2 |
  895224.7 |
  895890.1 |
  896555.5 |########################################
  897220.9 |
  897886.4 |
  898551.8 |########################################
  899217.2 |########################################
  899882.7 |
  900548.1 |
  901213.5 |
  901878.9 |
  902544.4 |########################################
  903209.8 |
  903875.2 |
  (0 below, 1 above range)

abi_boundary_w_scatter_zig_runtime_w (n=6, range 2131568.3-2151324.0 ns)
  2131568.3 |########################################
  2132556.1 |
  2133543.9 |
  2134531.6 |
  2135519.4 |########################################
  2136507.2 |
  2137495.0 |
  2138482.8 |########################################
  2139470.6 |
  2140458.3 |
  2141446.1 |
  2142433.9 |
  2143421.7 |########################################
  2144409.5 |
  2145397.3 |
  2146385.0 |
  2147372.8 |########################################
  2148360.6 |
  2149348.4 |
  2150336.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_scatter_null_entry**: bridge=3789.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_anchor**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_dispatch**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_per_w**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_runtime_w**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_dispatch**: bridge=305.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_per_w**: bridge=305.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_runtime_w**: bridge=305.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_zig_runtime_w**: bridge=315.3% of algo (FFI overhead may distort results)
