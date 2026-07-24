# abi_boundary_w (scatter)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_scatter_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_scatter_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_scatter_null_entry dominates: 35866% faster than the next best (abi_boundary_w_scatter_soa_runtime_w)

abi_boundary_w_scatter_null_entry (2.50 us) leads abi_boundary_w_scatter_soa_runtime_w (897.35 us) by 35866%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_scatter_null_entry beats baseline by 100% (significant)

abi_boundary_w_scatter_null_entry is -2.18 ms (100%) faster than baseline abi_boundary_w_scatter_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_scatter_scalar_per_w is an outlier: 877.2x slower than the field

abi_boundary_w_scatter_scalar_per_w (2.19 ms) is 877.2x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_scatter_null_entry} vs {abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_scalar_per_w} (35866% apart)

The field splits into a fast tier {abi_boundary_w_scatter_null_entry} and a slow tier {abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_scalar_per_w} with a 35866% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 877.2x the fastest

Fastest abi_boundary_w_scatter_null_entry (2.50 us) to slowest abi_boundary_w_scatter_scalar_per_w (2.19 ms): 877.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_scatter_null_entry** at 2495.0 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 877.23x (fastest 2495.0 ns, slowest 2188680.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 4810ns | 4809ns | 4627ns | 4783ns | 4944ns | -99.78% |
| abi_boundary_w_scatter_scalar_anchor | 2175081ns | 2174350ns | 2157197ns | 2171636ns | 2189190ns | -0.18% |
| abi_boundary_w_scatter_scalar_dispatch | 2190770ns | 2188756ns | 2177251ns | 2186759ns | 2203546ns | +0.54% |
| abi_boundary_w_scatter_scalar_per_w | 2191040ns | 2191901ns | 2165466ns | 2184352ns | 2213859ns | +0.55% |
| abi_boundary_w_scatter_scalar_runtime_w | 2179059ns | 2181009ns | 2162215ns | 2176354ns | 2191538ns | base |
| abi_boundary_w_scatter_soa_dispatch | 904899ns | 903732ns | 898196ns | 901970ns | 912644ns | -58.47% |
| abi_boundary_w_scatter_soa_per_w | 925848ns | 914113ns | 893468ns | 911662ns | 963318ns | -57.51% |
| abi_boundary_w_scatter_soa_runtime_w | 983827ns | 900092ns | 895036ns | 898511ns | 1156197ns | -54.85% |
| abi_boundary_w_scatter_zig_runtime_w | 2137563ns | 2135876ns | 2129552ns | 2135326ns | 2144923ns | -1.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 2518ns | 2429ns | 2607ns | -99.88% | 0.025 |
| abi_boundary_w_scatter_scalar_anchor | 2172009ns | 2154431ns | 2185708ns | -0.17% | 0.000 |
| abi_boundary_w_scatter_scalar_dispatch | 2187616ns | 2174350ns | 2200215ns | +0.54% | 0.000 |
| abi_boundary_w_scatter_scalar_per_w | 2187983ns | 2162499ns | 2210941ns | +0.56% | 0.000 |
| abi_boundary_w_scatter_scalar_runtime_w | 2175796ns | 2159400ns | 2188067ns | base | 0.000 |
| abi_boundary_w_scatter_soa_dispatch | 902160ns | 895646ns | 909691ns | -58.54% | 0.000 |
| abi_boundary_w_scatter_soa_per_w | 923015ns | 891092ns | 960312ns | -57.58% | 0.000 |
| abi_boundary_w_scatter_soa_runtime_w | 980722ns | 892619ns | 1152076ns | -54.93% | 0.000 |
| abi_boundary_w_scatter_zig_runtime_w | 2134503ns | 2126845ns | 2141648ns | -1.90% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 28036.9 | 2771.6 | 2518.1 | n/a |
| abi_boundary_w_scatter_scalar_anchor | 57634.3 | 2169681.9 | 2172009.0 | n/a |
| abi_boundary_w_scatter_scalar_dispatch | 54578.1 | 2189070.2 | 2187616.1 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 56482.0 | 2203463.2 | 2187982.5 | 0 |
| abi_boundary_w_scatter_scalar_runtime_w | 59514.8 | 2173150.8 | 2175796.0 | n/a |
| abi_boundary_w_scatter_soa_dispatch | 40946.7 | 902032.4 | 902160.3 | n/a |
| abi_boundary_w_scatter_soa_per_w | 46071.1 | 923601.2 | 923015.0 | n/a |
| abi_boundary_w_scatter_soa_runtime_w | 59167.6 | 987926.8 | 980722.5 | n/a |
| abi_boundary_w_scatter_zig_runtime_w | 212539.4 | 2134650.9 | 2134503.0 | 4 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_boundary_w_scatter_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.026 | 97.3% |
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
| abi_boundary_w_scatter_null_entry | 4810ns | 4810ns | -99.78% |
| abi_boundary_w_scatter_scalar_anchor | 2175081ns | 2175081ns | -0.18% |
| abi_boundary_w_scatter_scalar_dispatch | 2190770ns | 2190770ns | +0.54% |
| abi_boundary_w_scatter_scalar_per_w | 2191040ns | 2191040ns | +0.55% |
| abi_boundary_w_scatter_scalar_runtime_w | 2179059ns | 2179059ns | base |
| abi_boundary_w_scatter_soa_dispatch | 904899ns | 904899ns | -58.47% |
| abi_boundary_w_scatter_soa_per_w | 925848ns | 925848ns | -57.51% |
| abi_boundary_w_scatter_soa_runtime_w | 983827ns | 983827ns | -54.85% |
| abi_boundary_w_scatter_zig_runtime_w | 2137563ns | 2137563ns | -1.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_scalar_runtime_w | 2177640ns | base | --- | [2161681, 2188067] | --- | --- | --- | --- |
| abi_boundary_w_scatter_null_entry | 2495ns | -2175101.8ns (-99.9%) | [-2185576, -2159156]ns | [2452, 2607] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_anchor | 2171413ns | no significant difference | [-21348, +16640]ns | [2158906, 2185708] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_scatter_scalar_dispatch | 2185616ns | +8867.4ns (+0.4%) | [+3535, +23058]ns | [2177018, 2200215] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 2188680ns | no significant difference | [-974, +24397]ns | [2164326, 2210941] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_scatter_soa_dispatch | 900990ns | -1274453.8ns (-58.5%) | [-1285854, -1260599]ns | [895800, 909691] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_per_w | 911270ns | -1255967.9ns (-57.7%) | [-1274620, -1227755]ns | [897464, 960312] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_runtime_w | 897354ns | -1264327.1ns (-58.1%) | [-1294903, -1025991]ns | [892738, 1152076] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_zig_runtime_w | 2132761ns | -39793.8ns (-1.8%) | [-54687, -29399]ns | [2129099, 2141648] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_scatter_scalar_runtime_w | abi_boundary_w_scatter_null_entry | abi_boundary_w_scatter_scalar_anchor | abi_boundary_w_scatter_scalar_dispatch | abi_boundary_w_scatter_scalar_per_w | abi_boundary_w_scatter_soa_dispatch | abi_boundary_w_scatter_soa_per_w | abi_boundary_w_scatter_soa_runtime_w | abi_boundary_w_scatter_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2179589ns | -99.9% | -0.2% | +0.9% | +0.9% | -58.9% | -57.0% | -36.4% | -2.2% |
| 2 | 2196545ns | -99.9% | -1.5% | +0.2% | +0.9% | -58.7% | -55.2% | -59.4% | -2.7% |
| 3 | 2178736ns | -99.9% | +0.6% | +0.3% | -0.0% | -58.1% | -58.0% | -59.0% | -1.5% |
| 4 | 2159400ns | -99.9% | +0.9% | +1.2% | +0.3% | -58.1% | -58.1% | -58.3% | -1.2% |
| 5 | 2176544ns | -99.9% | -0.4% | +0.1% | +1.3% | -58.9% | -59.1% | -57.8% | -2.3% |
| 6 | 2163963ns | -99.9% | -0.4% | +0.5% | -0.1% | -58.6% | -58.1% | -58.6% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.168 | ok |
| abi_boundary_w_scatter_scalar_anchor | -0.004 | ok |
| abi_boundary_w_scatter_scalar_dispatch | 0.474 | moderate+ |
| abi_boundary_w_scatter_scalar_per_w | -0.237 | moderate- |
| abi_boundary_w_scatter_scalar_runtime_w | 0.082 | ok |
| abi_boundary_w_scatter_soa_dispatch | 0.306 | moderate+ |
| abi_boundary_w_scatter_soa_per_w | 0.293 | moderate+ |
| abi_boundary_w_scatter_soa_runtime_w | -0.052 | ok |
| abi_boundary_w_scatter_zig_runtime_w | 0.185 | ok |

**Consistency summary:**

- **abi_boundary_w_scatter_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_scalar_anchor**: won 4/6, lost 2/6
- **abi_boundary_w_scatter_scalar_dispatch**: won 0/6, lost 6/6
- **abi_boundary_w_scatter_scalar_per_w**: won 0/6, lost 4/6
- **abi_boundary_w_scatter_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 113515.1ns | 2518.1ns | 4507.9% | HIGH |
| abi_boundary_w_scatter_scalar_anchor | 6566941.5ns | 2172009.0ns | 302.3% | HIGH |
| abi_boundary_w_scatter_scalar_dispatch | 6623782.5ns | 2187616.1ns | 302.8% | HIGH |
| abi_boundary_w_scatter_scalar_per_w | 6640321.5ns | 2187982.5ns | 303.5% | HIGH |
| abi_boundary_w_scatter_scalar_runtime_w | 6578243.5ns | 2175796.0ns | 302.3% | HIGH |
| abi_boundary_w_scatter_soa_dispatch | 2748606.9ns | 902160.3ns | 304.7% | HIGH |
| abi_boundary_w_scatter_soa_per_w | 2821076.9ns | 923015.0ns | 305.6% | HIGH |
| abi_boundary_w_scatter_soa_runtime_w | 3144111.1ns | 980722.5ns | 320.6% | HIGH |
| abi_boundary_w_scatter_zig_runtime_w | 6691304.2ns | 2134503.0ns | 313.5% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_scatter_null_entry (n=6, range 2428.8-2607.1 ns)
   2428.8 |########################################
   2437.7 |
   2446.6 |
   2455.5 |
   2464.5 |
   2473.4 |########################################
   2482.3 |########################################
   2491.2 |
   2500.1 |########################################
   2509.0 |
   2517.9 |
   2526.9 |
   2535.8 |
   2544.7 |
   2553.6 |
   2562.5 |
   2571.4 |
   2580.4 |
   2589.3 |########################################
   2598.2 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_anchor (n=6, range 2154430.8-2185708.1 ns)
  2154430.8 |########################################
  2155994.7 |
  2157558.5 |
  2159122.4 |
  2160686.3 |
  2162250.1 |########################################
  2163814.0 |
  2165377.9 |
  2166941.7 |########################################
  2168505.6 |
  2170069.5 |
  2171633.3 |
  2173197.2 |########################################
  2174761.0 |
  2176324.9 |
  2177888.8 |
  2179452.6 |########################################
  2181016.5 |
  2182580.4 |
  2184144.2 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_dispatch (n=6, range 2174349.6-2200214.8 ns)
  2174349.6 |########################################
  2175642.9 |
  2176936.1 |
  2178229.4 |
  2179522.6 |########################################
  2180815.9 |
  2182109.2 |
  2183402.4 |
  2184695.7 |########################################
  2185988.9 |########################################
  2187282.2 |
  2188575.5 |
  2189868.7 |
  2191162.0 |
  2192455.2 |
  2193748.5 |
  2195041.8 |
  2196335.0 |
  2197628.3 |
  2198921.5 |########################################
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_per_w (n=6, range 2162499.2-2210941.5 ns)
  2162499.2 |########################################
  2164921.3 |########################################
  2167343.4 |
  2169765.5 |
  2172187.7 |
  2174609.8 |
  2177031.9 |########################################
  2179454.0 |
  2181876.1 |
  2184298.2 |
  2186720.3 |
  2189142.4 |
  2191564.6 |
  2193986.7 |
  2196408.8 |
  2198830.9 |########################################
  2201253.0 |
  2203675.1 |########################################
  2206097.2 |
  2208519.3 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_runtime_w (n=6, range 2159399.6-2188066.9 ns)
  2159399.6 |########################################
  2160833.0 |
  2162266.3 |
  2163699.7 |########################################
  2165133.1 |
  2166566.4 |
  2167999.8 |
  2169433.1 |
  2170866.5 |
  2172299.9 |
  2173733.2 |
  2175166.6 |########################################
  2176600.0 |
  2178033.3 |########################################
  2179466.7 |########################################
  2180900.0 |
  2182333.4 |
  2183766.8 |
  2185200.1 |
  2186633.5 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_dispatch (n=6, range 895645.8-909691.2 ns)
  895645.8 |########################################
  896348.1 |####################
  897050.3 |
  897752.6 |
  898454.9 |
  899157.2 |
  899859.4 |
  900561.7 |
  901264.0 |
  901966.3 |
  902668.5 |
  903370.8 |
  904073.1 |
  904775.3 |####################
  905477.6 |
  906179.9 |
  906882.2 |####################
  907584.4 |
  908286.7 |
  908989.0 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_per_w (n=6, range 891092.1-960311.7 ns)
  891092.1 |########################################
  894553.1 |
  898014.1 |
  901475.0 |########################################
  904936.0 |########################################
  908397.0 |
  911858.0 |########################################
  915318.9 |
  918779.9 |
  922240.9 |
  925701.9 |
  929162.9 |
  932623.8 |
  936084.8 |########################################
  939545.8 |
  943006.8 |
  946467.7 |
  949928.7 |
  953389.7 |
  956850.7 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_runtime_w (n=6, range 892619.2-1152075.9 ns)
  892619.2 |########################################
  905592.0 |##########
  918564.9 |
  931537.7 |
  944510.5 |
  957483.4 |
  970456.2 |
  983429.0 |
  996401.9 |
  1009374.7 |
  1022347.5 |
  1035320.4 |
  1048293.2 |
  1061266.0 |
  1074238.9 |
  1087211.7 |
  1100184.5 |
  1113157.4 |
  1126130.2 |
  1139103.0 |
  (0 below, 1 above range)

abi_boundary_w_scatter_zig_runtime_w (n=6, range 2126845.4-2141648.3 ns)
  2126845.4 |########################################
  2127585.5 |
  2128325.7 |
  2129065.8 |
  2129806.0 |
  2130546.1 |
  2131286.3 |########################################
  2132026.4 |########################################
  2132766.6 |########################################
  2133506.7 |
  2134246.8 |
  2134987.0 |
  2135727.1 |
  2136467.3 |########################################
  2137207.4 |
  2137947.6 |
  2138687.7 |
  2139427.9 |
  2140168.0 |
  2140908.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_scatter_null_entry**: bridge=4557.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_anchor**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_dispatch**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_per_w**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_runtime_w**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_dispatch**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_per_w**: bridge=304.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_runtime_w**: bridge=305.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_zig_runtime_w**: bridge=313.7% of algo (FFI overhead may distort results)
