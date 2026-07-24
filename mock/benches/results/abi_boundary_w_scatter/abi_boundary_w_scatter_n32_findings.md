# abi_boundary_w (scatter)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_scatter_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_scatter_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_scatter_null_entry dominates: 38667% faster than the next best (abi_boundary_w_scatter_soa_runtime_w)

abi_boundary_w_scatter_null_entry (2.30 us) leads abi_boundary_w_scatter_soa_runtime_w (889.97 us) by 38667%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_scatter_null_entry beats baseline by 100% (significant)

abi_boundary_w_scatter_null_entry is -2.16 ms (100%) faster than baseline abi_boundary_w_scatter_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_scatter_scalar_anchor is an outlier: 941.9x slower than the field

abi_boundary_w_scatter_scalar_anchor (2.16 ms) is 941.9x the fastest (2.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_scatter_null_entry shows alternating (throttle bounce) (autocorr -0.60)

abi_boundary_w_scatter_null_entry's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_scatter_null_entry} vs {abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_anchor} (38667% apart)

The field splits into a fast tier {abi_boundary_w_scatter_null_entry} and a slow tier {abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_anchor} with a 38667% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 941.9x the fastest

Fastest abi_boundary_w_scatter_null_entry (2.30 us) to slowest abi_boundary_w_scatter_scalar_anchor (2.16 ms): 941.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_scatter_null_entry** at 2295.6 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 941.86x (fastest 2295.6 ns, slowest 2162190.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 4605ns | 4575ns | 4429ns | 4558ns | 4763ns | -99.79% |
| abi_boundary_w_scatter_scalar_anchor | 2164273ns | 2165204ns | 2156051ns | 2163625ns | 2169357ns | -2.77% |
| abi_boundary_w_scatter_scalar_dispatch | 2192645ns | 2160643ns | 2155189ns | 2158904ns | 2261984ns | -1.50% |
| abi_boundary_w_scatter_scalar_per_w | 2161928ns | 2159891ns | 2147520ns | 2159389ns | 2172941ns | -2.88% |
| abi_boundary_w_scatter_scalar_runtime_w | 2226018ns | 2163559ns | 2157436ns | 2161769ns | 2356681ns | base |
| abi_boundary_w_scatter_soa_dispatch | 902893ns | 901760ns | 889805ns | 899913ns | 913908ns | -59.44% |
| abi_boundary_w_scatter_soa_per_w | 901434ns | 903818ns | 891347ns | 901864ns | 905833ns | -59.50% |
| abi_boundary_w_scatter_soa_runtime_w | 897115ns | 892438ns | 888838ns | 891598ns | 909530ns | -59.70% |
| abi_boundary_w_scatter_zig_runtime_w | 2133522ns | 2133114ns | 2121995ns | 2131240ns | 2142709ns | -4.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 2290ns | 2210ns | 2339ns | -99.90% | 0.014 |
| abi_boundary_w_scatter_scalar_anchor | 2161404ns | 2153378ns | 2166461ns | -2.77% | 0.000 |
| abi_boundary_w_scatter_scalar_dispatch | 2189551ns | 2152460ns | 2258472ns | -1.50% | 0.000 |
| abi_boundary_w_scatter_scalar_per_w | 2159214ns | 2144974ns | 2170087ns | -2.87% | 0.000 |
| abi_boundary_w_scatter_scalar_runtime_w | 2222983ns | 2154641ns | 2353375ns | base | 0.000 |
| abi_boundary_w_scatter_soa_dispatch | 900217ns | 887412ns | 911002ns | -59.50% | 0.000 |
| abi_boundary_w_scatter_soa_per_w | 898888ns | 888930ns | 903301ns | -59.56% | 0.000 |
| abi_boundary_w_scatter_soa_runtime_w | 894375ns | 886386ns | 906225ns | -59.77% | 0.000 |
| abi_boundary_w_scatter_zig_runtime_w | 2130497ns | 2119334ns | 2139567ns | -4.16% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 27078.2 | 2416.9 | 2289.8 | n/a |
| abi_boundary_w_scatter_scalar_anchor | 49228.9 | 2160240.8 | 2161403.8 | n/a |
| abi_boundary_w_scatter_scalar_dispatch | 53942.7 | 2184820.4 | 2189550.9 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 45693.0 | 2160233.2 | 2159214.2 | n/a |
| abi_boundary_w_scatter_scalar_runtime_w | 51591.0 | 2195973.9 | 2222982.6 | n/a |
| abi_boundary_w_scatter_soa_dispatch | 38493.4 | 900506.8 | 900217.4 | n/a |
| abi_boundary_w_scatter_soa_per_w | 35985.5 | 900120.2 | 898888.3 | n/a |
| abi_boundary_w_scatter_soa_runtime_w | 41684.6 | 894466.0 | 894375.4 | n/a |
| abi_boundary_w_scatter_zig_runtime_w | 205687.1 | 2130223.9 | 2130496.6 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_boundary_w_scatter_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.014 | 96.3% |
| abi_boundary_w_scatter_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_scatter_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_scatter_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_scatter_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_scatter_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_scatter_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_scatter_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_scatter_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 4605ns | 4605ns | -99.79% |
| abi_boundary_w_scatter_scalar_anchor | 2164273ns | 2164273ns | -2.77% |
| abi_boundary_w_scatter_scalar_dispatch | 2192645ns | 2192645ns | -1.50% |
| abi_boundary_w_scatter_scalar_per_w | 2161928ns | 2161928ns | -2.88% |
| abi_boundary_w_scatter_scalar_runtime_w | 2226018ns | 2226018ns | base |
| abi_boundary_w_scatter_soa_dispatch | 902893ns | 902893ns | -59.44% |
| abi_boundary_w_scatter_soa_per_w | 901434ns | 901434ns | -59.50% |
| abi_boundary_w_scatter_soa_runtime_w | 897115ns | 897115ns | -59.70% |
| abi_boundary_w_scatter_zig_runtime_w | 2133522ns | 2133522ns | -4.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_scalar_runtime_w | 2160590ns | base | --- | [2154983, 2353375] | --- | --- | --- | --- |
| abi_boundary_w_scatter_null_entry | 2296ns | -2158287.5ns (-99.9%) | [-2351043, -2152748]ns | [2235, 2339] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_anchor | 2162190ns | no significant difference | [-189717, +5714]ns | [2155560, 2166461] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_scatter_scalar_dispatch | 2157690ns | no significant difference | [-195300, +96219]ns | [2152490, 2258472] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 2157282ns | no significant difference | [-197141, +9985]ns | [2150273, 2170087] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_scatter_soa_dispatch | 899064ns | -1264397.3ns (-58.5%) | [-1450835, -1253063]ns | [890586, 911002] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_per_w | 901142ns | -1261694.9ns (-58.4%) | [-1458140, -1252447]ns | [892221, 903301] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_runtime_w | 889966ns | -1268042.1ns (-58.7%) | [-1463902, -1253878]ns | [886936, 906225] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_zig_runtime_w | 2129948ns | -37455.2ns (-1.7%) | [-218440, -21563]ns | [2121975, 2139567] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_scatter_scalar_runtime_w | abi_boundary_w_scatter_null_entry | abi_boundary_w_scatter_scalar_anchor | abi_boundary_w_scatter_scalar_dispatch | abi_boundary_w_scatter_scalar_per_w | abi_boundary_w_scatter_soa_dispatch | abi_boundary_w_scatter_soa_per_w | abi_boundary_w_scatter_soa_runtime_w | abi_boundary_w_scatter_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2165563ns | -99.9% | -0.0% | +8.7% | +0.4% | -58.6% | -58.4% | -58.1% | -1.8% |
| 2 | 2155325ns | -99.9% | +0.1% | -0.1% | -0.5% | -58.5% | -58.2% | -58.9% | -1.7% |
| 3 | 2541187ns | -99.9% | -14.7% | -14.9% | -15.1% | -64.2% | -64.5% | -64.9% | -15.6% |
| 4 | 2156172ns | -99.9% | +0.4% | +0.0% | -0.0% | -58.2% | -58.0% | -58.8% | -1.1% |
| 5 | 2154641ns | -99.9% | -0.1% | +0.2% | +0.5% | -58.8% | -58.4% | -58.0% | -0.9% |
| 6 | 2165007ns | -99.9% | -0.3% | -0.6% | -0.4% | -57.8% | -58.9% | -59.0% | -1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_scatter_null_entry | -0.602 | HIGH- (thermal bounce) |
| abi_boundary_w_scatter_scalar_anchor | -0.175 | ok |
| abi_boundary_w_scatter_scalar_dispatch | -0.064 | ok |
| abi_boundary_w_scatter_scalar_per_w | -0.438 | moderate- |
| abi_boundary_w_scatter_scalar_runtime_w | -0.250 | moderate- |
| abi_boundary_w_scatter_soa_dispatch | -0.441 | moderate- |
| abi_boundary_w_scatter_soa_per_w | 0.246 | moderate+ |
| abi_boundary_w_scatter_soa_runtime_w | -0.432 | moderate- |
| abi_boundary_w_scatter_zig_runtime_w | -0.222 | moderate- |

**Consistency summary:**

- **abi_boundary_w_scatter_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_scalar_anchor**: won 2/6, lost 2/6
- **abi_boundary_w_scatter_scalar_dispatch**: won 3/6, lost 2/6
- **abi_boundary_w_scatter_scalar_per_w**: won 3/6, lost 2/6
- **abi_boundary_w_scatter_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 116603.7ns | 2289.8ns | 5092.3% | HIGH |
| abi_boundary_w_scatter_scalar_anchor | 6534617.1ns | 2161403.8ns | 302.3% | HIGH |
| abi_boundary_w_scatter_scalar_dispatch | 6629345.0ns | 2189550.9ns | 302.8% | HIGH |
| abi_boundary_w_scatter_scalar_per_w | 6526275.3ns | 2159214.2ns | 302.3% | HIGH |
| abi_boundary_w_scatter_scalar_runtime_w | 6619461.2ns | 2222982.6ns | 297.8% | HIGH |
| abi_boundary_w_scatter_soa_dispatch | 2740640.8ns | 900217.4ns | 304.4% | HIGH |
| abi_boundary_w_scatter_soa_per_w | 2735823.1ns | 898888.3ns | 304.4% | HIGH |
| abi_boundary_w_scatter_soa_runtime_w | 2726569.8ns | 894375.4ns | 304.9% | HIGH |
| abi_boundary_w_scatter_zig_runtime_w | 6677465.6ns | 2130496.6ns | 313.4% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_scatter_null_entry (n=6, range 2210.4-2338.8 ns)
   2210.4 |########################################
   2216.8 |
   2223.2 |
   2229.7 |
   2236.1 |
   2242.5 |
   2248.9 |
   2255.3 |########################################
   2261.7 |########################################
   2268.2 |
   2274.6 |
   2281.0 |
   2287.4 |
   2293.8 |
   2300.2 |
   2306.7 |
   2313.1 |
   2319.5 |########################################
   2325.9 |
   2332.3 |########################################
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_anchor (n=6, range 2153378.3-2166461.2 ns)
  2153378.3 |####################
  2154032.4 |
  2154686.6 |
  2155340.7 |
  2155994.9 |
  2156649.0 |
  2157303.2 |####################
  2157957.3 |
  2158611.5 |####################
  2159265.6 |
  2159919.8 |
  2160573.9 |
  2161228.1 |
  2161882.2 |
  2162536.4 |
  2163190.5 |
  2163844.7 |
  2164498.8 |
  2165153.0 |########################################
  2165807.1 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_dispatch (n=6, range 2152460.4-2258472.3 ns)
  2152460.4 |########################################
  2157761.0 |#############
  2163061.6 |#############
  2168362.2 |
  2173662.8 |
  2178963.4 |
  2184264.0 |
  2189564.6 |
  2194865.2 |
  2200165.8 |
  2205466.3 |
  2210766.9 |
  2216067.5 |
  2221368.1 |
  2226668.7 |
  2231969.3 |
  2237269.9 |
  2242570.5 |
  2247871.1 |
  2253171.7 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_per_w (n=6, range 2144974.2-2170087.2 ns)
  2144974.2 |####################
  2146229.9 |
  2147485.5 |
  2148741.2 |
  2149996.8 |
  2151252.5 |
  2152508.1 |
  2153763.8 |
  2155019.4 |####################
  2156275.1 |########################################
  2157530.7 |
  2158786.4 |
  2160042.0 |
  2161297.7 |
  2162553.3 |
  2163809.0 |
  2165064.6 |####################
  2166320.3 |
  2167575.9 |
  2168831.6 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_runtime_w (n=6, range 2154641.2-2353375.0 ns)
  2154641.2 |########################################
  2164577.9 |##########################
  2174514.6 |
  2184451.3 |
  2194388.0 |
  2204324.7 |
  2214261.3 |
  2224198.0 |
  2234134.7 |
  2244071.4 |
  2254008.1 |
  2263944.8 |
  2273881.5 |
  2283818.2 |
  2293754.9 |
  2303691.5 |
  2313628.2 |
  2323564.9 |
  2333501.6 |
  2343438.3 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_dispatch (n=6, range 887412.5-911002.5 ns)
  887412.5 |########################################
  888592.0 |
  889771.5 |
  890951.0 |
  892130.5 |
  893310.0 |########################################
  894489.5 |
  895669.0 |########################################
  896848.5 |
  898028.0 |
  899207.5 |
  900387.0 |
  901566.5 |########################################
  902746.0 |
  903925.5 |
  905105.0 |
  906284.5 |
  907464.0 |########################################
  908643.5 |
  909823.0 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_per_w (n=6, range 888930.4-903301.2 ns)
  888930.4 |########################################
  889648.9 |
  890367.5 |
  891086.0 |
  891804.6 |
  892523.1 |
  893241.7 |
  893960.2 |
  894678.7 |
  895397.3 |########################################
  896115.8 |
  896834.4 |
  897552.9 |
  898271.5 |
  898990.0 |
  899708.5 |
  900427.1 |########################################
  901145.6 |########################################
  901864.2 |########################################
  902582.7 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_runtime_w (n=6, range 886385.8-906224.6 ns)
  886385.8 |########################################
  887377.7 |########################################
  888369.7 |########################################
  889361.6 |
  890353.6 |########################################
  891345.5 |
  892337.4 |
  893329.4 |
  894321.3 |
  895313.2 |
  896305.2 |
  897297.1 |
  898289.1 |
  899281.0 |
  900272.9 |
  901264.9 |
  902256.8 |
  903248.7 |
  904240.7 |########################################
  905232.6 |
  (0 below, 1 above range)

abi_boundary_w_scatter_zig_runtime_w (n=6, range 2119334.2-2139567.3 ns)
  2119334.2 |########################################
  2120345.9 |
  2121357.5 |
  2122369.2 |
  2123380.8 |
  2124392.5 |########################################
  2125404.1 |
  2126415.8 |########################################
  2127427.4 |
  2128439.1 |
  2129450.8 |
  2130462.4 |
  2131474.1 |
  2132485.7 |########################################
  2133497.4 |########################################
  2134509.0 |
  2135520.7 |
  2136532.3 |
  2137544.0 |
  2138555.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_scatter_null_entry**: bridge=5079.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_anchor**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_dispatch**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_per_w**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_runtime_w**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_dispatch**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_per_w**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_runtime_w**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_zig_runtime_w**: bridge=313.7% of algo (FFI overhead may distort results)
