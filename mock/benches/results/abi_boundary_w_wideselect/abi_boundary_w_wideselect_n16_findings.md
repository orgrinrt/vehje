# abi_boundary_w (wideselect)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_wideselect_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_wideselect_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_wideselect_null_entry dominates: 36622% faster than the next best (abi_boundary_w_wideselect_soa_per_w)

abi_boundary_w_wideselect_null_entry (2.62 us) leads abi_boundary_w_wideselect_soa_per_w (962.86 us) by 36622%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_wideselect_null_entry beats baseline by 100% (significant)

abi_boundary_w_wideselect_null_entry is -2.25 ms (100%) faster than baseline abi_boundary_w_wideselect_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_wideselect_scalar_per_w is an outlier: 861.0x slower than the field

abi_boundary_w_wideselect_scalar_per_w (2.26 ms) is 861.0x the fastest (2.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_wideselect_soa_per_w shows alternating (throttle bounce) (autocorr -0.52)

abi_boundary_w_wideselect_soa_per_w's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_wideselect_null_entry} vs {abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_per_w} (36622% apart)

The field splits into a fast tier {abi_boundary_w_wideselect_null_entry} and a slow tier {abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_per_w} with a 36622% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 861.0x the fastest

Fastest abi_boundary_w_wideselect_null_entry (2.62 us) to slowest abi_boundary_w_wideselect_scalar_per_w (2.26 ms): 861.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_wideselect_null_entry** at 2622.1 ns median (-99.9% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 860.95x (fastest 2622.1 ns, slowest 2257461.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 5059ns | 4979ns | 4799ns | 4964ns | 5332ns | -99.77% |
| abi_boundary_w_wideselect_scalar_anchor | 2228968ns | 2228809ns | 2107171ns | 2194452ns | 2341641ns | +0.43% |
| abi_boundary_w_wideselect_scalar_dispatch | 2184946ns | 2204598ns | 2085056ns | 2170315ns | 2256836ns | -1.55% |
| abi_boundary_w_wideselect_scalar_per_w | 2273105ns | 2261783ns | 2106712ns | 2241311ns | 2403994ns | +2.42% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2219326ns | 2258559ns | 2082968ns | 2215165ns | 2293746ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 981001ns | 983626ns | 939870ns | 973092ns | 1013430ns | -55.80% |
| abi_boundary_w_wideselect_soa_per_w | 968757ns | 966332ns | 936044ns | 961271ns | 996344ns | -56.35% |
| abi_boundary_w_wideselect_soa_runtime_w | 980917ns | 979128ns | 933113ns | 973039ns | 1016638ns | -55.80% |
| abi_boundary_w_wideselect_zig_runtime_w | 2174345ns | 2137886ns | 2024475ns | 2110509ns | 2345033ns | -2.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 2647ns | 2529ns | 2750ns | -99.88% | 0.006 |
| abi_boundary_w_wideselect_scalar_anchor | 2224310ns | 2103829ns | 2336203ns | +0.43% | 0.000 |
| abi_boundary_w_wideselect_scalar_dispatch | 2180427ns | 2081570ns | 2251508ns | -1.55% | 0.000 |
| abi_boundary_w_wideselect_scalar_per_w | 2268376ns | 2103130ns | 2398360ns | +2.42% | 0.000 |
| abi_boundary_w_wideselect_scalar_runtime_w | 2214720ns | 2079696ns | 2288671ns | base | 0.000 |
| abi_boundary_w_wideselect_soa_dispatch | 977217ns | 936895ns | 1009111ns | -55.88% | 0.000 |
| abi_boundary_w_wideselect_soa_per_w | 965326ns | 933160ns | 992685ns | -56.41% | 0.000 |
| abi_boundary_w_wideselect_soa_runtime_w | 977074ns | 930200ns | 1011968ns | -55.88% | 0.000 |
| abi_boundary_w_wideselect_zig_runtime_w | 2169187ns | 2021087ns | 2338505ns | -2.06% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 31923.3 | 2747.2 | 2647.3 | n/a |
| abi_boundary_w_wideselect_scalar_anchor | 102774.2 | 2212714.9 | 2224309.8 | n/a |
| abi_boundary_w_wideselect_scalar_dispatch | 100459.4 | 2192029.1 | 2180426.6 | n/a |
| abi_boundary_w_wideselect_scalar_per_w | 112177.6 | 2272689.0 | 2268376.2 | n/a |
| abi_boundary_w_wideselect_scalar_runtime_w | 101919.9 | 2213758.3 | 2214720.3 | n/a |
| abi_boundary_w_wideselect_soa_dispatch | 76144.1 | 979040.2 | 977216.8 | n/a |
| abi_boundary_w_wideselect_soa_per_w | 66192.1 | 968285.3 | 965326.5 | n/a |
| abi_boundary_w_wideselect_soa_runtime_w | 74278.7 | 976557.8 | 977074.0 | n/a |
| abi_boundary_w_wideselect_zig_runtime_w | 364981.7 | 2166985.5 | 2169186.7 | 6 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_boundary_w_wideselect_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.006 | 96.4% |
| abi_boundary_w_wideselect_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_wideselect_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_wideselect_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_wideselect_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_wideselect_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 5059ns | 5059ns | -99.77% |
| abi_boundary_w_wideselect_scalar_anchor | 2228968ns | 2228968ns | +0.43% |
| abi_boundary_w_wideselect_scalar_dispatch | 2184946ns | 2184946ns | -1.55% |
| abi_boundary_w_wideselect_scalar_per_w | 2273105ns | 2273105ns | +2.42% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2219326ns | 2219326ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 981001ns | 981001ns | -55.80% |
| abi_boundary_w_wideselect_soa_per_w | 968757ns | 968757ns | -56.35% |
| abi_boundary_w_wideselect_soa_runtime_w | 980917ns | 980917ns | -55.80% |
| abi_boundary_w_wideselect_zig_runtime_w | 2174345ns | 2174345ns | -2.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_scalar_runtime_w | 2253783ns | base | --- | [2101707, 2288671] | --- | --- | --- | --- |
| abi_boundary_w_wideselect_null_entry | 2622ns | -2251154.0ns (-99.9%) | [-2285980, -2099085]ns | [2570, 2750] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_scalar_anchor | 2223925ns | no significant difference | [-34825, +47531]ns | [2112802, 2336203] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 2199890ns | no significant difference | [-85079, +6451]ns | [2089881, 2251508] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_wideselect_scalar_per_w | 2257462ns | no significant difference | [-47495, +140875]ns | [2149306, 2398360] | no | 0.3500 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_dispatch | 979557ns | -1255036.9ns (-55.7%) | [-1298750, -1158724]ns | [942983, 1009111] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_per_w | 962858ns | -1288581.8ns (-57.2%) | [-1328985, -1130615]ns | [940436, 992685] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_runtime_w | 975283ns | -1255971.2ns (-55.7%) | [-1299232, -1157735]ns | [943971, 1011968] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_zig_runtime_w | 2132830ns | no significant difference | [-196047, +136805]ns | [2036225, 2338505] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_wideselect_scalar_runtime_w | abi_boundary_w_wideselect_null_entry | abi_boundary_w_wideselect_scalar_anchor | abi_boundary_w_wideselect_scalar_dispatch | abi_boundary_w_wideselect_scalar_per_w | abi_boundary_w_wideselect_soa_dispatch | abi_boundary_w_wideselect_soa_per_w | abi_boundary_w_wideselect_soa_runtime_w | abi_boundary_w_wideselect_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2256031ns | -99.9% | -3.0% | -3.7% | +2.7% | -57.5% | -57.2% | -56.7% | -10.4% |
| 2 | 2079696ns | -99.9% | +1.2% | +0.1% | +1.1% | -54.4% | -55.1% | -55.3% | +4.4% |
| 3 | 2123717ns | -99.9% | -0.1% | -1.2% | +3.5% | -55.9% | -52.5% | -54.9% | -3.4% |
| 4 | 2313908ns | -99.9% | +1.4% | -3.7% | -5.1% | -56.2% | -58.5% | -57.0% | +7.9% |
| 5 | 2251535ns | -99.9% | +0.4% | +0.5% | +8.3% | -55.4% | -57.9% | -56.8% | -7.0% |
| 6 | 2263435ns | -99.9% | +2.7% | -1.0% | +4.1% | -55.8% | -56.9% | -54.5% | -3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.021 | ok |
| abi_boundary_w_wideselect_scalar_anchor | 0.227 | moderate+ |
| abi_boundary_w_wideselect_scalar_dispatch | 0.475 | moderate+ |
| abi_boundary_w_wideselect_scalar_per_w | 0.149 | ok |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.075 | ok |
| abi_boundary_w_wideselect_soa_dispatch | 0.345 | moderate+ |
| abi_boundary_w_wideselect_soa_per_w | -0.518 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_soa_runtime_w | 0.053 | ok |
| abi_boundary_w_wideselect_zig_runtime_w | -0.434 | moderate- |

**Consistency summary:**

- **abi_boundary_w_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_scalar_anchor**: won 1/6, lost 4/6
- **abi_boundary_w_wideselect_scalar_dispatch**: won 4/6, lost 1/6
- **abi_boundary_w_wideselect_scalar_per_w**: won 1/6, lost 5/6
- **abi_boundary_w_wideselect_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_zig_runtime_w**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 122343.1ns | 2647.3ns | 4621.4% | HIGH |
| abi_boundary_w_wideselect_scalar_anchor | 6753893.8ns | 2224309.8ns | 303.6% | HIGH |
| abi_boundary_w_wideselect_scalar_dispatch | 6662813.8ns | 2180426.6ns | 305.6% | HIGH |
| abi_boundary_w_wideselect_scalar_per_w | 6912418.2ns | 2268376.2ns | 304.7% | HIGH |
| abi_boundary_w_wideselect_scalar_runtime_w | 6748009.2ns | 2214720.3ns | 304.7% | HIGH |
| abi_boundary_w_wideselect_soa_dispatch | 3018762.4ns | 977216.8ns | 308.9% | HIGH |
| abi_boundary_w_wideselect_soa_per_w | 2964563.8ns | 965326.5ns | 307.1% | HIGH |
| abi_boundary_w_wideselect_soa_runtime_w | 3014859.4ns | 977074.0ns | 308.6% | HIGH |
| abi_boundary_w_wideselect_zig_runtime_w | 7017831.2ns | 2169186.7ns | 323.5% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_wideselect_null_entry (n=6, range 2528.7-2749.6 ns)
   2528.7 |####################
   2539.7 |
   2550.8 |
   2561.8 |
   2572.9 |
   2583.9 |
   2595.0 |
   2606.0 |########################################
   2617.0 |####################
   2628.1 |
   2639.1 |####################
   2650.2 |
   2661.2 |
   2672.3 |
   2683.3 |
   2694.3 |
   2705.4 |
   2716.4 |
   2727.5 |
   2738.5 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_anchor (n=6, range 2103829.2-2336202.7 ns)
  2103829.2 |########################################
  2115447.9 |########################################
  2127066.6 |
  2138685.2 |
  2150303.9 |
  2161922.6 |
  2173541.2 |
  2185159.9 |########################################
  2196778.6 |
  2208397.3 |
  2220016.0 |
  2231634.6 |
  2243253.3 |
  2254872.0 |########################################
  2266490.7 |
  2278109.3 |
  2289728.0 |
  2301346.7 |
  2312965.4 |
  2324584.0 |########################################
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_dispatch (n=6, range 2081569.6-2251508.1 ns)
  2081569.6 |########################################
  2090066.5 |########################################
  2098563.5 |
  2107060.4 |
  2115557.3 |
  2124054.2 |
  2132551.1 |
  2141048.1 |
  2149545.0 |
  2158041.9 |
  2166538.8 |########################################
  2175035.8 |
  2183532.7 |
  2192029.6 |
  2200526.5 |
  2209023.5 |
  2217520.4 |
  2226017.3 |########################################
  2234514.2 |########################################
  2243011.2 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_per_w (n=6, range 2103130.0-2398360.2 ns)
  2103130.0 |####################
  2117891.5 |
  2132653.0 |
  2147414.5 |
  2162176.0 |
  2176937.5 |
  2191699.1 |########################################
  2206460.6 |
  2221222.1 |
  2235983.6 |
  2250745.1 |
  2265506.6 |
  2280268.1 |
  2295029.6 |
  2309791.1 |####################
  2324552.7 |
  2339314.2 |
  2354075.7 |####################
  2368837.2 |
  2383598.7 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_runtime_w (n=6, range 2079696.2-2288671.5 ns)
  2079696.2 |####################
  2090145.0 |
  2100593.7 |
  2111042.5 |
  2121491.2 |####################
  2131940.0 |
  2142388.8 |
  2152837.5 |
  2163286.3 |
  2173735.1 |
  2184183.8 |
  2194632.6 |
  2205081.4 |
  2215530.1 |
  2225978.9 |
  2236427.6 |
  2246876.4 |########################################
  2257325.2 |####################
  2267773.9 |
  2278222.7 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_dispatch (n=6, range 936894.6-1009110.9 ns)
  936894.6 |########################################
  940505.4 |
  944116.2 |
  947727.0 |########################################
  951337.8 |
  954948.7 |
  958559.5 |########################################
  962170.3 |
  965781.1 |
  969391.9 |
  973002.7 |
  976613.5 |
  980224.4 |
  983835.2 |
  987446.0 |
  991056.8 |
  994667.6 |
  998278.4 |########################################
  1001889.2 |########################################
  1005500.0 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_per_w (n=6, range 933160.4-992685.4 ns)
  933160.4 |########################################
  936136.7 |
  939112.9 |
  942089.2 |
  945065.4 |########################################
  948041.7 |
  951017.9 |
  953994.2 |
  956970.4 |########################################
  959946.7 |
  962922.9 |
  965899.2 |########################################
  968875.4 |
  971851.7 |
  974827.9 |########################################
  977804.2 |
  980780.4 |
  983756.7 |
  986732.9 |
  989709.2 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_runtime_w (n=6, range 930200.0-1011967.9 ns)
  930200.0 |########################################
  934288.4 |
  938376.8 |
  942465.2 |
  946553.6 |
  950642.0 |
  954730.4 |########################################
  958818.8 |
  962907.2 |
  966995.6 |
  971083.9 |########################################
  975172.3 |########################################
  979260.7 |
  983349.1 |
  987437.5 |
  991525.9 |########################################
  995614.3 |
  999702.7 |
  1003791.1 |
  1007879.5 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_zig_runtime_w (n=6, range 2021087.1-2338505.0 ns)
  2021087.1 |########################################
  2036958.0 |########################################
  2052828.9 |
  2068699.8 |
  2084570.7 |########################################
  2100441.6 |
  2116312.5 |
  2132183.4 |
  2148054.3 |
  2163925.2 |########################################
  2179796.0 |########################################
  2195666.9 |
  2211537.8 |
  2227408.7 |
  2243279.6 |
  2259150.5 |
  2275021.4 |
  2290892.3 |
  2306763.2 |
  2322634.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_wideselect_null_entry**: bridge=4614.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_anchor**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_dispatch**: bridge=305.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_per_w**: bridge=304.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_runtime_w**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_dispatch**: bridge=309.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_per_w**: bridge=307.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_runtime_w**: bridge=306.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_zig_runtime_w**: bridge=323.4% of algo (FFI overhead may distort results)
