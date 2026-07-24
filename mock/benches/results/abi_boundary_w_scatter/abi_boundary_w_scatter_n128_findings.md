# abi_boundary_w (scatter)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_scatter_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_scatter_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_scatter_null_entry dominates: 33348% faster than the next best (abi_boundary_w_scatter_soa_runtime_w)

abi_boundary_w_scatter_null_entry (2.67 us) leads abi_boundary_w_scatter_soa_runtime_w (893.90 us) by 33348%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_scatter_null_entry beats baseline by 100% (significant)

abi_boundary_w_scatter_null_entry is -2.15 ms (100%) faster than baseline abi_boundary_w_scatter_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_scatter_scalar_dispatch is an outlier: 808.0x slower than the field

abi_boundary_w_scatter_scalar_dispatch (2.16 ms) is 808.0x the fastest (2.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_scatter_soa_dispatch shows alternating (throttle bounce) (autocorr -0.66)

abi_boundary_w_scatter_soa_dispatch's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_scatter_null_entry} vs {abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_scalar_dispatch} (33348% apart)

The field splits into a fast tier {abi_boundary_w_scatter_null_entry} and a slow tier {abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_scalar_dispatch} with a 33348% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 808.0x the fastest

Fastest abi_boundary_w_scatter_null_entry (2.67 us) to slowest abi_boundary_w_scatter_scalar_dispatch (2.16 ms): 808.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_scatter_null_entry** at 2672.5 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 808.02x (fastest 2672.5 ns, slowest 2159431.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 4930ns | 4931ns | 4839ns | 4907ns | 5009ns | -99.77% |
| abi_boundary_w_scatter_scalar_anchor | 2161580ns | 2161549ns | 2150859ns | 2160973ns | 2167853ns | +0.16% |
| abi_boundary_w_scatter_scalar_dispatch | 2163395ns | 2162189ns | 2152402ns | 2159843ns | 2174218ns | +0.25% |
| abi_boundary_w_scatter_scalar_per_w | 2156648ns | 2154840ns | 2151568ns | 2154507ns | 2162399ns | -0.07% |
| abi_boundary_w_scatter_scalar_runtime_w | 2158107ns | 2157976ns | 2149197ns | 2156831ns | 2164474ns | base |
| abi_boundary_w_scatter_soa_dispatch | 901412ns | 901677ns | 893881ns | 899521ns | 908014ns | -58.23% |
| abi_boundary_w_scatter_soa_per_w | 904439ns | 904300ns | 898106ns | 903541ns | 908951ns | -58.09% |
| abi_boundary_w_scatter_soa_runtime_w | 898361ns | 896474ns | 892930ns | 895617ns | 905194ns | -58.37% |
| abi_boundary_w_scatter_zig_runtime_w | 2134505ns | 2133275ns | 2131078ns | 2132759ns | 2138838ns | -1.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 2689ns | 2651ns | 2738ns | -99.88% | 0.048 |
| abi_boundary_w_scatter_scalar_anchor | 2158800ns | 2148242ns | 2164946ns | +0.16% | 0.000 |
| abi_boundary_w_scatter_scalar_dispatch | 2160554ns | 2149587ns | 2171250ns | +0.25% | 0.000 |
| abi_boundary_w_scatter_scalar_per_w | 2153919ns | 2148849ns | 2159573ns | -0.06% | 0.000 |
| abi_boundary_w_scatter_scalar_runtime_w | 2155248ns | 2146500ns | 2161490ns | base | 0.000 |
| abi_boundary_w_scatter_soa_dispatch | 898857ns | 891471ns | 905356ns | -58.29% | 0.000 |
| abi_boundary_w_scatter_soa_per_w | 901880ns | 895558ns | 906292ns | -58.15% | 0.000 |
| abi_boundary_w_scatter_soa_runtime_w | 895744ns | 890454ns | 902480ns | -58.44% | 0.000 |
| abi_boundary_w_scatter_zig_runtime_w | 2131540ns | 2128208ns | 2135910ns | -1.10% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 26461.8 | 2748.7 | 2688.7 | n/a |
| abi_boundary_w_scatter_scalar_anchor | 47320.0 | 2155761.4 | 2158800.5 | n/a |
| abi_boundary_w_scatter_scalar_dispatch | 47787.6 | 2159514.4 | 2160553.8 | n/a |
| abi_boundary_w_scatter_scalar_per_w | 47131.6 | 2156264.6 | 2153918.6 | 0 |
| abi_boundary_w_scatter_scalar_runtime_w | 46612.2 | 2156714.9 | 2155248.1 | n/a |
| abi_boundary_w_scatter_soa_dispatch | 36309.7 | 898920.1 | 898856.6 | n/a |
| abi_boundary_w_scatter_soa_per_w | 37949.1 | 901620.9 | 901879.5 | n/a |
| abi_boundary_w_scatter_soa_runtime_w | 37798.5 | 896001.5 | 895744.1 | n/a |
| abi_boundary_w_scatter_zig_runtime_w | 206737.4 | 2129919.9 | 2131539.8 | 7 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_boundary_w_scatter_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.048 | 99.2% |
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
| abi_boundary_w_scatter_null_entry | 4930ns | 4930ns | -99.77% |
| abi_boundary_w_scatter_scalar_anchor | 2161580ns | 2161580ns | +0.16% |
| abi_boundary_w_scatter_scalar_dispatch | 2163395ns | 2163395ns | +0.25% |
| abi_boundary_w_scatter_scalar_per_w | 2156648ns | 2156648ns | -0.07% |
| abi_boundary_w_scatter_scalar_runtime_w | 2158107ns | 2158107ns | base |
| abi_boundary_w_scatter_soa_dispatch | 901412ns | 901412ns | -58.23% |
| abi_boundary_w_scatter_soa_per_w | 904439ns | 904439ns | -58.09% |
| abi_boundary_w_scatter_soa_runtime_w | 898361ns | 898361ns | -58.37% |
| abi_boundary_w_scatter_zig_runtime_w | 2134505ns | 2134505ns | -1.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_scalar_runtime_w | 2155108ns | base | --- | [2149147, 2161490] | --- | --- | --- | --- |
| abi_boundary_w_scatter_null_entry | 2672ns | -2152399.0ns (-99.9%) | [-2158825, -2146454]ns | [2656, 2738] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_anchor | 2158739ns | +2898.1ns (+0.1%) | [+1289, +6470]ns | [2152716, 2164946] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_dispatch | 2159432ns | no significant difference | [-2664, +12136]ns | [2150980, 2171250] | no | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 2152254ns | no significant difference | [-9259, +5385]ns | [2149929, 2159573] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_scatter_soa_dispatch | 899098ns | -1256342.1ns (-58.3%) | [-1264888, -1247945]ns | [892116, 905356] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_per_w | 901765ns | -1252360.3ns (-58.1%) | [-1260003, -1247742]ns | [897582, 906292] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_runtime_w | 893903ns | -1258345.6ns (-58.4%) | [-1266228, -1253938]ns | [890849, 902480] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_zig_runtime_w | 2130243ns | -24460.0ns (-1.1%) | [-27494, -19171]ns | [2128466, 2135910] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_scatter_scalar_runtime_w | abi_boundary_w_scatter_null_entry | abi_boundary_w_scatter_scalar_anchor | abi_boundary_w_scatter_scalar_dispatch | abi_boundary_w_scatter_scalar_per_w | abi_boundary_w_scatter_soa_dispatch | abi_boundary_w_scatter_soa_per_w | abi_boundary_w_scatter_soa_runtime_w | abi_boundary_w_scatter_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2159326ns | -99.9% | +0.3% | +0.2% | +0.3% | -58.2% | -57.9% | -58.2% | -1.3% |
| 2 | 2146500ns | -99.9% | +0.1% | +0.4% | +0.2% | -58.5% | -58.1% | -58.4% | -0.8% |
| 3 | 2154721ns | -99.9% | +0.1% | +0.4% | -0.2% | -57.8% | -58.2% | -58.7% | -1.0% |
| 4 | 2163655ns | -99.9% | +0.0% | +0.7% | -0.7% | -58.6% | -58.2% | -58.6% | -1.2% |
| 5 | 2151793ns | -99.9% | +0.3% | +0.0% | +0.1% | -58.1% | -58.0% | -58.6% | -1.1% |
| 6 | 2155494ns | -99.9% | +0.2% | -0.3% | -0.1% | -58.6% | -58.5% | -58.2% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.138 | ok |
| abi_boundary_w_scatter_scalar_anchor | -0.330 | moderate- |
| abi_boundary_w_scatter_scalar_dispatch | -0.047 | ok |
| abi_boundary_w_scatter_scalar_per_w | -0.023 | ok |
| abi_boundary_w_scatter_scalar_runtime_w | -0.371 | moderate- |
| abi_boundary_w_scatter_soa_dispatch | -0.656 | HIGH- (thermal bounce) |
| abi_boundary_w_scatter_soa_per_w | -0.208 | moderate- |
| abi_boundary_w_scatter_soa_runtime_w | -0.209 | moderate- |
| abi_boundary_w_scatter_zig_runtime_w | -0.044 | ok |

**Consistency summary:**

- **abi_boundary_w_scatter_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_scalar_anchor**: won 0/6, lost 4/6
- **abi_boundary_w_scatter_scalar_dispatch**: won 1/6, lost 4/6
- **abi_boundary_w_scatter_scalar_per_w**: won 2/6, lost 2/6
- **abi_boundary_w_scatter_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 118641.9ns | 2688.7ns | 4412.7% | HIGH |
| abi_boundary_w_scatter_scalar_anchor | 6519713.8ns | 2158800.5ns | 302.0% | HIGH |
| abi_boundary_w_scatter_scalar_dispatch | 6531163.5ns | 2160553.8ns | 302.3% | HIGH |
| abi_boundary_w_scatter_scalar_per_w | 6514700.1ns | 2153918.6ns | 302.5% | HIGH |
| abi_boundary_w_scatter_scalar_runtime_w | 6521140.6ns | 2155248.1ns | 302.6% | HIGH |
| abi_boundary_w_scatter_soa_dispatch | 2734576.0ns | 898856.6ns | 304.2% | HIGH |
| abi_boundary_w_scatter_soa_per_w | 2744522.9ns | 901879.5ns | 304.3% | HIGH |
| abi_boundary_w_scatter_soa_runtime_w | 2727379.1ns | 895744.1ns | 304.5% | HIGH |
| abi_boundary_w_scatter_zig_runtime_w | 6671101.0ns | 2131539.8ns | 313.0% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_scatter_null_entry (n=6, range 2650.8-2737.7 ns)
   2650.8 |########################################
   2655.1 |
   2659.5 |########################################
   2663.8 |########################################
   2668.2 |
   2672.5 |
   2676.9 |########################################
   2681.2 |
   2685.6 |
   2689.9 |
   2694.2 |
   2698.6 |
   2702.9 |
   2707.3 |
   2711.6 |
   2716.0 |########################################
   2720.3 |
   2724.7 |
   2729.0 |
   2733.4 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_anchor (n=6, range 2148241.7-2164946.2 ns)
  2148241.7 |####################
  2149076.9 |
  2149912.2 |
  2150747.4 |
  2151582.6 |
  2152417.8 |
  2153253.1 |
  2154088.3 |
  2154923.5 |
  2155758.7 |
  2156594.0 |####################
  2157429.2 |
  2158264.4 |########################################
  2159099.7 |
  2159934.9 |
  2160770.1 |
  2161605.3 |
  2162440.6 |
  2163275.8 |
  2164111.0 |####################
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_dispatch (n=6, range 2149587.1-2171250.0 ns)
  2149587.1 |########################################
  2150670.2 |
  2151753.4 |########################################
  2152836.5 |
  2153919.7 |
  2155002.8 |########################################
  2156086.0 |
  2157169.1 |
  2158252.3 |
  2159335.4 |
  2160418.5 |
  2161501.7 |
  2162584.8 |########################################
  2163668.0 |########################################
  2164751.1 |
  2165834.3 |
  2166917.4 |
  2168000.6 |
  2169083.7 |
  2170166.9 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_per_w (n=6, range 2148848.8-2159572.9 ns)
  2148848.8 |####################
  2149385.0 |
  2149921.2 |
  2150457.4 |
  2150993.6 |########################################
  2151529.8 |
  2152066.0 |
  2152602.2 |
  2153138.4 |####################
  2153674.6 |####################
  2154210.8 |
  2154747.1 |
  2155283.3 |
  2155819.5 |
  2156355.7 |
  2156891.9 |
  2157428.1 |
  2157964.3 |
  2158500.5 |
  2159036.7 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_runtime_w (n=6, range 2146500.4-2161490.2 ns)
  2146500.4 |########################################
  2147249.9 |
  2147999.4 |
  2148748.9 |
  2149498.4 |
  2150247.9 |
  2150997.3 |
  2151746.8 |########################################
  2152496.3 |
  2153245.8 |
  2153995.3 |########################################
  2154744.8 |########################################
  2155494.3 |
  2156243.8 |
  2156993.3 |
  2157742.8 |
  2158492.2 |
  2159241.7 |########################################
  2159991.2 |
  2160740.7 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_dispatch (n=6, range 891471.2-905356.0 ns)
  891471.2 |####################
  892165.4 |####################
  892859.7 |
  893553.9 |
  894248.2 |
  894942.4 |
  895636.6 |
  896330.9 |####################
  897025.1 |
  897719.4 |
  898413.6 |
  899107.8 |
  899802.1 |
  900496.3 |
  901190.6 |########################################
  901884.8 |
  902579.0 |
  903273.3 |
  903967.5 |
  904661.8 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_per_w (n=6, range 895557.5-906291.9 ns)
  895557.5 |####################
  896094.2 |
  896630.9 |
  897167.7 |
  897704.4 |
  898241.1 |
  898777.8 |
  899314.5 |####################
  899851.3 |####################
  900388.0 |
  900924.7 |
  901461.4 |
  901998.1 |
  902534.9 |
  903071.6 |########################################
  903608.3 |
  904145.0 |
  904681.7 |
  905218.5 |
  905755.2 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_runtime_w (n=6, range 890453.7-902480.0 ns)
  890453.7 |########################################
  891055.0 |########################################
  891656.3 |
  892257.6 |########################################
  892859.0 |
  893460.3 |
  894061.6 |
  894662.9 |
  895264.2 |########################################
  895865.5 |
  896466.8 |
  897068.2 |
  897669.5 |
  898270.8 |
  898872.1 |
  899473.4 |
  900074.7 |
  900676.1 |
  901277.4 |########################################
  901878.7 |
  (0 below, 1 above range)

abi_boundary_w_scatter_zig_runtime_w (n=6, range 2128207.5-2135910.0 ns)
  2128207.5 |########################################
  2128592.6 |########################################
  2128977.8 |
  2129362.9 |########################################
  2129748.0 |
  2130133.1 |
  2130518.2 |
  2130903.4 |########################################
  2131288.5 |
  2131673.6 |
  2132058.8 |
  2132443.9 |
  2132829.0 |
  2133214.1 |########################################
  2133599.2 |
  2133984.4 |
  2134369.5 |
  2134754.6 |
  2135139.8 |
  2135524.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_scatter_null_entry**: bridge=4437.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_anchor**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_dispatch**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_per_w**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_runtime_w**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_dispatch**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_per_w**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_runtime_w**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_zig_runtime_w**: bridge=313.0% of algo (FFI overhead may distort results)
