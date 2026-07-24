# abi_boundary_w (wideselect)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_wideselect_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_wideselect_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_wideselect_null_entry dominates: 40838% faster than the next best (abi_boundary_w_wideselect_soa_dispatch)

abi_boundary_w_wideselect_null_entry (2.33 us) leads abi_boundary_w_wideselect_soa_dispatch (955.04 us) by 40838%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_wideselect_null_entry beats baseline by 100% (significant)

abi_boundary_w_wideselect_null_entry is -2.14 ms (100%) faster than baseline abi_boundary_w_wideselect_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_wideselect_scalar_anchor is an outlier: 935.0x slower than the field

abi_boundary_w_wideselect_scalar_anchor (2.18 ms) is 935.0x the fastest (2.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_wideselect_null_entry} vs {abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_anchor} (40838% apart)

The field splits into a fast tier {abi_boundary_w_wideselect_null_entry} and a slow tier {abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_anchor} with a 40838% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 935.0x the fastest

Fastest abi_boundary_w_wideselect_null_entry (2.33 us) to slowest abi_boundary_w_wideselect_scalar_anchor (2.18 ms): 935.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_wideselect_null_entry** at 2332.9 ns median (-99.9% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 935.04x (fastest 2332.9 ns, slowest 2181359.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 4735ns | 4702ns | 4515ns | 4688ns | 4916ns | -99.78% |
| abi_boundary_w_wideselect_scalar_anchor | 2183148ns | 2185802ns | 2121561ns | 2173255ns | 2228781ns | -0.83% |
| abi_boundary_w_wideselect_scalar_dispatch | 2198669ns | 2167505ns | 2133129ns | 2160228ns | 2289101ns | -0.13% |
| abi_boundary_w_wideselect_scalar_per_w | 2198131ns | 2165007ns | 2129245ns | 2154517ns | 2297994ns | -0.15% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2201497ns | 2149944ns | 2109835ns | 2139459ns | 2340384ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 976984ns | 958495ns | 944827ns | 955024ns | 1026002ns | -55.62% |
| abi_boundary_w_wideselect_soa_per_w | 996418ns | 968907ns | 953120ns | 965813ns | 1063974ns | -54.74% |
| abi_boundary_w_wideselect_soa_runtime_w | 997476ns | 964582ns | 947460ns | 961219ns | 1076870ns | -54.69% |
| abi_boundary_w_wideselect_zig_runtime_w | 2131661ns | 2103388ns | 2044474ns | 2092155ns | 2234514ns | -3.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 2353ns | 2251ns | 2458ns | -99.89% | 0.014 |
| abi_boundary_w_wideselect_scalar_anchor | 2178728ns | 2117731ns | 2223900ns | -0.84% | 0.000 |
| abi_boundary_w_wideselect_scalar_dispatch | 2194071ns | 2128859ns | 2283994ns | -0.14% | 0.000 |
| abi_boundary_w_wideselect_scalar_per_w | 2193637ns | 2125242ns | 2292746ns | -0.16% | 0.000 |
| abi_boundary_w_wideselect_scalar_runtime_w | 2197094ns | 2105398ns | 2335368ns | base | 0.000 |
| abi_boundary_w_wideselect_soa_dispatch | 973076ns | 941363ns | 1021059ns | -55.71% | 0.000 |
| abi_boundary_w_wideselect_soa_per_w | 992470ns | 949995ns | 1059221ns | -54.83% | 0.000 |
| abi_boundary_w_wideselect_soa_runtime_w | 993619ns | 944075ns | 1072269ns | -54.78% | 0.000 |
| abi_boundary_w_wideselect_zig_runtime_w | 2126909ns | 2040328ns | 2228893ns | -3.19% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 31823.0 | 2523.0 | 2353.0 | n/a |
| abi_boundary_w_wideselect_scalar_anchor | 105964.1 | 2183868.2 | 2178727.9 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 106131.8 | 2204334.0 | 2194071.3 | 3 |
| abi_boundary_w_wideselect_scalar_per_w | 106693.7 | 2194701.3 | 2193636.9 | 2 |
| abi_boundary_w_wideselect_scalar_runtime_w | 98339.1 | 2201892.9 | 2197093.7 | n/a |
| abi_boundary_w_wideselect_soa_dispatch | 79134.8 | 972607.2 | 973075.6 | n/a |
| abi_boundary_w_wideselect_soa_per_w | 83658.0 | 992237.0 | 992470.5 | n/a |
| abi_boundary_w_wideselect_soa_runtime_w | 75162.6 | 994211.0 | 993618.8 | n/a |
| abi_boundary_w_wideselect_zig_runtime_w | 324903.4 | 2141397.0 | 2126908.7 | 3 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_boundary_w_wideselect_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.014 | 96.5% |
| abi_boundary_w_wideselect_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_wideselect_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 4735ns | 4735ns | -99.78% |
| abi_boundary_w_wideselect_scalar_anchor | 2183148ns | 2183148ns | -0.83% |
| abi_boundary_w_wideselect_scalar_dispatch | 2198669ns | 2198669ns | -0.13% |
| abi_boundary_w_wideselect_scalar_per_w | 2198131ns | 2198131ns | -0.15% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2201497ns | 2201497ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 976984ns | 976984ns | -55.62% |
| abi_boundary_w_wideselect_soa_per_w | 996418ns | 996418ns | -54.74% |
| abi_boundary_w_wideselect_soa_runtime_w | 997476ns | 997476ns | -54.69% |
| abi_boundary_w_wideselect_zig_runtime_w | 2131661ns | 2131661ns | -3.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_scalar_runtime_w | 2145820ns | base | --- | [2110093, 2335368] | --- | --- | --- | --- |
| abi_boundary_w_wideselect_null_entry | 2333ns | -2143487.0ns (-99.9%) | [-2332924, -2107811]ns | [2269, 2458] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_scalar_anchor | 2181360ns | no significant difference | [-111468, +62546]ns | [2130924, 2223900] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 2163211ns | no significant difference | [-51374, +41480]ns | [2135009, 2283994] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_wideselect_scalar_per_w | 2160790ns | no significant difference | [-66635, +38983]ns | [2127375, 2292746] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_dispatch | 955042ns | -1192406.9ns (-55.6%) | [-1314308, -1165339]ns | [943125, 1021059] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_per_w | 965279ns | -1170540.0ns (-54.5%) | [-1291204, -1152126]ns | [952911, 1059221] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_runtime_w | 961124ns | -1194048.3ns (-55.6%) | [-1263099, -1153277]ns | [947464, 1072269] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_zig_runtime_w | 2098894ns | no significant difference | [-157498, +1011]ns | [2052940, 2228893] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_wideselect_scalar_runtime_w | abi_boundary_w_wideselect_null_entry | abi_boundary_w_wideselect_scalar_anchor | abi_boundary_w_wideselect_scalar_dispatch | abi_boundary_w_wideselect_scalar_per_w | abi_boundary_w_wideselect_soa_dispatch | abi_boundary_w_wideselect_soa_per_w | abi_boundary_w_wideselect_soa_runtime_w | abi_boundary_w_wideselect_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2360248ns | -99.9% | -5.3% | -0.9% | -5.8% | -58.6% | -58.4% | -54.7% | -9.0% |
| 2 | 2310488ns | -99.9% | -4.2% | -3.5% | +2.2% | -53.9% | -50.8% | -53.4% | -0.0% |
| 3 | 2123405ns | -99.9% | +1.0% | +2.2% | +1.3% | -55.5% | -55.0% | -54.5% | +0.1% |
| 4 | 2168236ns | -99.9% | -0.7% | -0.6% | +0.1% | -55.6% | -55.5% | -56.1% | -4.7% |
| 5 | 2105398ns | -99.9% | +5.0% | +1.7% | +0.9% | -55.3% | -54.1% | -54.6% | -3.1% |
| 6 | 2114788ns | -99.9% | +0.1% | +0.7% | +0.7% | -55.2% | -55.1% | -55.4% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | -0.216 | moderate- |
| abi_boundary_w_wideselect_scalar_anchor | -0.099 | ok |
| abi_boundary_w_wideselect_scalar_dispatch | 0.343 | moderate+ |
| abi_boundary_w_wideselect_scalar_per_w | 0.118 | ok |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.369 | moderate+ |
| abi_boundary_w_wideselect_soa_dispatch | -0.069 | ok |
| abi_boundary_w_wideselect_soa_per_w | -0.156 | ok |
| abi_boundary_w_wideselect_soa_runtime_w | 0.456 | moderate+ |
| abi_boundary_w_wideselect_zig_runtime_w | 0.287 | moderate+ |

**Consistency summary:**

- **abi_boundary_w_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_scalar_anchor**: won 3/6, lost 3/6
- **abi_boundary_w_wideselect_scalar_dispatch**: won 3/6, lost 3/6
- **abi_boundary_w_wideselect_scalar_per_w**: won 1/6, lost 5/6
- **abi_boundary_w_wideselect_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_zig_runtime_w**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 120434.7ns | 2353.0ns | 5118.2% | HIGH |
| abi_boundary_w_wideselect_scalar_anchor | 6667166.0ns | 2178727.9ns | 306.0% | HIGH |
| abi_boundary_w_wideselect_scalar_dispatch | 6715953.9ns | 2194071.3ns | 306.1% | HIGH |
| abi_boundary_w_wideselect_scalar_per_w | 6721621.2ns | 2193636.9ns | 306.4% | HIGH |
| abi_boundary_w_wideselect_scalar_runtime_w | 6681545.7ns | 2197093.7ns | 304.1% | HIGH |
| abi_boundary_w_wideselect_soa_dispatch | 2995977.7ns | 973075.6ns | 307.9% | HIGH |
| abi_boundary_w_wideselect_soa_per_w | 3066246.8ns | 992470.5ns | 309.0% | HIGH |
| abi_boundary_w_wideselect_soa_runtime_w | 3072498.8ns | 993618.8ns | 309.2% | HIGH |
| abi_boundary_w_wideselect_zig_runtime_w | 6856057.2ns | 2126908.7ns | 322.3% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_wideselect_null_entry (n=6, range 2250.8-2457.5 ns)
   2250.8 |########################################
   2261.1 |
   2271.5 |
   2281.8 |########################################
   2292.1 |
   2302.5 |
   2312.8 |########################################
   2323.1 |
   2333.5 |
   2343.8 |########################################
   2354.2 |
   2364.5 |
   2374.8 |########################################
   2385.2 |
   2395.5 |
   2405.8 |
   2416.2 |
   2426.5 |
   2436.8 |
   2447.2 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_anchor (n=6, range 2117730.8-2223899.8 ns)
  2117730.8 |########################################
  2123039.2 |
  2128347.7 |
  2133656.1 |
  2138964.6 |########################################
  2144273.0 |
  2149581.5 |########################################
  2154889.9 |
  2160198.4 |
  2165506.8 |
  2170815.3 |
  2176123.8 |
  2181432.2 |
  2186740.6 |
  2192049.1 |
  2197357.5 |
  2202666.0 |
  2207974.4 |########################################
  2213282.9 |########################################
  2218591.3 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_dispatch (n=6, range 2128858.8-2283993.8 ns)
  2128858.8 |########################################
  2136615.5 |########################################
  2144372.3 |
  2152129.0 |########################################
  2159885.8 |
  2167642.5 |########################################
  2175399.3 |
  2183156.0 |
  2190912.8 |
  2198669.5 |
  2206426.3 |
  2214183.0 |
  2221939.8 |
  2229696.5 |########################################
  2237453.3 |
  2245210.0 |
  2252966.8 |
  2260723.5 |
  2268480.3 |
  2276237.0 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_per_w (n=6, range 2125242.5-2292746.0 ns)
  2125242.5 |########################################
  2133617.7 |
  2141992.9 |
  2150368.0 |####################
  2158743.2 |
  2167118.4 |####################
  2175493.6 |
  2183868.7 |
  2192243.9 |
  2200619.1 |
  2208994.3 |
  2217369.5 |####################
  2225744.6 |
  2234119.8 |
  2242495.0 |
  2250870.2 |
  2259245.3 |
  2267620.5 |
  2275995.7 |
  2284370.9 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_runtime_w (n=6, range 2105397.9-2335367.9 ns)
  2105397.9 |########################################
  2116896.4 |####################
  2128394.9 |
  2139893.4 |
  2151391.9 |
  2162890.4 |####################
  2174388.9 |
  2185887.4 |
  2197385.9 |
  2208884.4 |
  2220382.9 |
  2231881.4 |
  2243379.9 |
  2254878.4 |
  2266376.9 |
  2277875.4 |
  2289373.9 |
  2300872.4 |####################
  2312370.9 |
  2323869.4 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_dispatch (n=6, range 941363.3-1021059.4 ns)
  941363.3 |########################################
  945348.1 |####################
  949332.9 |
  953317.7 |
  957302.5 |
  961287.3 |####################
  965272.1 |
  969256.9 |
  973241.7 |####################
  977226.5 |
  981211.3 |
  985196.2 |
  989181.0 |
  993165.8 |
  997150.6 |
  1001135.4 |
  1005120.2 |
  1009105.0 |
  1013089.8 |
  1017074.6 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_per_w (n=6, range 949994.6-1059220.9 ns)
  949994.6 |####################
  955455.9 |####################
  960917.2 |########################################
  966378.5 |
  971839.8 |
  977301.2 |####################
  982762.5 |
  988223.8 |
  993685.1 |
  999146.4 |
  1004607.7 |
  1010069.0 |
  1015530.4 |
  1020991.7 |
  1026453.0 |
  1031914.3 |
  1037375.6 |
  1042836.9 |
  1048298.2 |
  1053759.5 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_runtime_w (n=6, range 944075.0-1072268.5 ns)
  944075.0 |####################
  950484.7 |########################################
  956894.4 |
  963304.0 |####################
  969713.7 |
  976123.4 |
  982533.1 |
  988942.7 |
  995352.4 |
  1001762.1 |
  1008171.8 |
  1014581.5 |
  1020991.1 |
  1027400.8 |
  1033810.5 |
  1040220.2 |
  1046629.8 |
  1053039.5 |
  1059449.2 |
  1065858.9 |####################
  (0 below, 1 above range)

abi_boundary_w_wideselect_zig_runtime_w (n=6, range 2040327.9-2228892.7 ns)
  2040327.9 |########################################
  2049756.1 |
  2059184.4 |########################################
  2068612.6 |########################################
  2078040.9 |
  2087469.1 |
  2096897.3 |
  2106325.6 |
  2115753.8 |
  2125182.1 |########################################
  2134610.3 |
  2144038.5 |########################################
  2153466.8 |
  2162895.0 |
  2172323.3 |
  2181751.5 |
  2191179.7 |
  2200608.0 |
  2210036.2 |
  2219464.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_wideselect_null_entry**: bridge=5136.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_anchor**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_dispatch**: bridge=305.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_per_w**: bridge=304.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_runtime_w**: bridge=304.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_dispatch**: bridge=306.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_per_w**: bridge=307.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_runtime_w**: bridge=308.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_zig_runtime_w**: bridge=319.1% of algo (FFI overhead may distort results)
