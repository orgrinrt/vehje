# abi_boundary_w (scatter)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_scatter_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_scatter_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_scatter_null_entry dominates: 29090% faster than the next best (abi_boundary_w_scatter_soa_runtime_w)

abi_boundary_w_scatter_null_entry (3.07 us) leads abi_boundary_w_scatter_soa_runtime_w (896.18 us) by 29090%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_scatter_null_entry beats baseline by 100% (significant)

abi_boundary_w_scatter_null_entry is -2.16 ms (100%) faster than baseline abi_boundary_w_scatter_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_scatter_scalar_anchor is an outlier: 704.4x slower than the field

abi_boundary_w_scatter_scalar_anchor (2.16 ms) is 704.4x the fastest (3.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_scatter_scalar_dispatch shows alternating (throttle bounce) (autocorr -0.56)

abi_boundary_w_scatter_scalar_dispatch's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_scatter_null_entry} vs {abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_anchor} (29090% apart)

The field splits into a fast tier {abi_boundary_w_scatter_null_entry} and a slow tier {abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_anchor} with a 29090% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 704.4x the fastest

Fastest abi_boundary_w_scatter_null_entry (3.07 us) to slowest abi_boundary_w_scatter_scalar_anchor (2.16 ms): 704.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_scatter_null_entry** at 3070.2 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 704.40x (fastest 3070.2 ns, slowest 2162643.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 5441ns | 5428ns | 5247ns | 5377ns | 5636ns | -99.75% |
| abi_boundary_w_scatter_scalar_anchor | 2165528ns | 2165598ns | 2156504ns | 2163569ns | 2172977ns | -0.05% |
| abi_boundary_w_scatter_scalar_dispatch | 2160202ns | 2157524ns | 2151390ns | 2156978ns | 2169442ns | -0.30% |
| abi_boundary_w_scatter_scalar_per_w | 2163152ns | 2160012ns | 2151825ns | 2158742ns | 2175430ns | -0.16% |
| abi_boundary_w_scatter_scalar_runtime_w | 2166692ns | 2163828ns | 2159365ns | 2162692ns | 2176358ns | base |
| abi_boundary_w_scatter_soa_dispatch | 912647ns | 912147ns | 905096ns | 911158ns | 918657ns | -57.88% |
| abi_boundary_w_scatter_soa_per_w | 901406ns | 901740ns | 892159ns | 900869ns | 906835ns | -58.40% |
| abi_boundary_w_scatter_soa_runtime_w | 901936ns | 898838ns | 895594ns | 897871ns | 911205ns | -58.37% |
| abi_boundary_w_scatter_zig_runtime_w | 2134369ns | 2135190ns | 2128760ns | 2133515ns | 2138456ns | -1.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 3102ns | 3014ns | 3218ns | -99.86% | 0.003 |
| abi_boundary_w_scatter_scalar_anchor | 2162567ns | 2153639ns | 2169866ns | -0.05% | 0.000 |
| abi_boundary_w_scatter_scalar_dispatch | 2157298ns | 2148738ns | 2166237ns | -0.29% | 0.000 |
| abi_boundary_w_scatter_scalar_per_w | 2160298ns | 2149059ns | 2172278ns | -0.16% | 0.000 |
| abi_boundary_w_scatter_scalar_runtime_w | 2163665ns | 2156481ns | 2173137ns | base | 0.000 |
| abi_boundary_w_scatter_soa_dispatch | 909942ns | 902438ns | 915847ns | -57.94% | 0.000 |
| abi_boundary_w_scatter_soa_per_w | 898818ns | 889776ns | 904100ns | -58.46% | 0.000 |
| abi_boundary_w_scatter_soa_runtime_w | 899378ns | 893125ns | 908651ns | -58.43% | 0.000 |
| abi_boundary_w_scatter_zig_runtime_w | 2131301ns | 2125880ns | 2135382ns | -1.50% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 28282.9 | 3238.2 | 3101.8 | n/a |
| abi_boundary_w_scatter_scalar_anchor | 52210.1 | 2161085.2 | 2162566.6 | n/a |
| abi_boundary_w_scatter_scalar_dispatch | 49015.7 | 2156246.4 | 2157298.5 | n/a |
| abi_boundary_w_scatter_scalar_per_w | 50854.9 | 2160109.3 | 2160297.6 | n/a |
| abi_boundary_w_scatter_scalar_runtime_w | 52582.9 | 2162144.7 | 2163665.3 | n/a |
| abi_boundary_w_scatter_soa_dispatch | 40897.0 | 909644.6 | 909942.3 | n/a |
| abi_boundary_w_scatter_soa_per_w | 40725.8 | 898519.5 | 898818.4 | n/a |
| abi_boundary_w_scatter_soa_runtime_w | 38167.4 | 901703.3 | 899377.8 | n/a |
| abi_boundary_w_scatter_zig_runtime_w | 210155.5 | 2133046.9 | 2131300.8 | 5 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_boundary_w_scatter_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.003 | 98.2% |
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
| abi_boundary_w_scatter_null_entry | 5441ns | 5441ns | -99.75% |
| abi_boundary_w_scatter_scalar_anchor | 2165528ns | 2165528ns | -0.05% |
| abi_boundary_w_scatter_scalar_dispatch | 2160202ns | 2160202ns | -0.30% |
| abi_boundary_w_scatter_scalar_per_w | 2163152ns | 2163152ns | -0.16% |
| abi_boundary_w_scatter_scalar_runtime_w | 2166692ns | 2166692ns | base |
| abi_boundary_w_scatter_soa_dispatch | 912647ns | 912647ns | -57.88% |
| abi_boundary_w_scatter_soa_per_w | 901406ns | 901406ns | -58.40% |
| abi_boundary_w_scatter_soa_runtime_w | 901936ns | 901936ns | -58.37% |
| abi_boundary_w_scatter_zig_runtime_w | 2134369ns | 2134369ns | -1.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_scalar_runtime_w | 2160803ns | base | --- | [2157055, 2173137] | --- | --- | --- | --- |
| abi_boundary_w_scatter_null_entry | 3070ns | -2157701.0ns (-99.9%) | [-2170067, -2153922]ns | [3017, 3218] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_anchor | 2162643ns | no significant difference | [-7408, +6338]ns | [2155190, 2169866] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_scatter_scalar_dispatch | 2154703ns | no significant difference | [-19906, +6253]ns | [2150956, 2166237] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 2157332ns | no significant difference | [-14853, +6846]ns | [2151282, 2172278] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_scatter_soa_dispatch | 909521ns | -1250024.4ns (-57.8%) | [-1265340, -1245805]ns | [904459, 915847] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_per_w | 899215ns | -1261794.5ns (-58.4%) | [-1279790, -1252956]ns | [893140, 904100] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_runtime_w | 896178ns | -1264570.1ns (-58.5%) | [-1271001, -1257291]ns | [893305, 908651] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_zig_runtime_w | 2131993ns | -30528.7ns (-1.4%) | [-39462, -27102]ns | [2126527, 2135382] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_scatter_scalar_runtime_w | abi_boundary_w_scatter_null_entry | abi_boundary_w_scatter_scalar_anchor | abi_boundary_w_scatter_scalar_dispatch | abi_boundary_w_scatter_scalar_per_w | abi_boundary_w_scatter_soa_dispatch | abi_boundary_w_scatter_soa_per_w | abi_boundary_w_scatter_soa_runtime_w | abi_boundary_w_scatter_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2157630ns | -99.9% | +0.2% | +0.3% | -0.4% | -57.7% | -58.1% | -58.4% | -1.5% |
| 2 | 2177747ns | -99.9% | -0.1% | -1.1% | -1.0% | -58.4% | -59.1% | -58.3% | -2.2% |
| 3 | 2156481ns | -99.8% | +0.3% | -0.0% | -0.1% | -58.0% | -58.1% | -58.6% | -1.4% |
| 4 | 2159268ns | -99.9% | -0.1% | -0.5% | -0.1% | -57.8% | -58.2% | -58.6% | -1.2% |
| 5 | 2162338ns | -99.9% | -0.4% | +0.3% | +0.6% | -58.3% | -58.5% | -58.0% | -1.3% |
| 6 | 2168527ns | -99.9% | -0.3% | -0.7% | +0.0% | -57.6% | -58.6% | -58.7% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.251 | moderate+ |
| abi_boundary_w_scatter_scalar_anchor | 0.232 | moderate+ |
| abi_boundary_w_scatter_scalar_dispatch | -0.562 | HIGH- (thermal bounce) |
| abi_boundary_w_scatter_scalar_per_w | 0.369 | moderate+ |
| abi_boundary_w_scatter_scalar_runtime_w | -0.469 | moderate- |
| abi_boundary_w_scatter_soa_dispatch | -0.526 | HIGH- (thermal bounce) |
| abi_boundary_w_scatter_soa_per_w | -0.535 | HIGH- (thermal bounce) |
| abi_boundary_w_scatter_soa_runtime_w | -0.499 | moderate- |
| abi_boundary_w_scatter_zig_runtime_w | 0.107 | ok |

**Consistency summary:**

- **abi_boundary_w_scatter_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_scalar_anchor**: won 3/6, lost 2/6
- **abi_boundary_w_scatter_scalar_dispatch**: won 3/6, lost 2/6
- **abi_boundary_w_scatter_scalar_per_w**: won 3/6, lost 1/6
- **abi_boundary_w_scatter_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 120089.1ns | 3101.8ns | 3871.5% | HIGH |
| abi_boundary_w_scatter_scalar_anchor | 6540977.2ns | 2162566.6ns | 302.5% | HIGH |
| abi_boundary_w_scatter_scalar_dispatch | 6520320.4ns | 2157298.5ns | 302.2% | HIGH |
| abi_boundary_w_scatter_scalar_per_w | 6537149.2ns | 2160297.6ns | 302.6% | HIGH |
| abi_boundary_w_scatter_scalar_runtime_w | 6543074.3ns | 2163665.3ns | 302.4% | HIGH |
| abi_boundary_w_scatter_soa_dispatch | 2771282.2ns | 909942.3ns | 304.6% | HIGH |
| abi_boundary_w_scatter_soa_per_w | 2739165.3ns | 898818.4ns | 304.8% | HIGH |
| abi_boundary_w_scatter_soa_runtime_w | 2740490.4ns | 899377.8ns | 304.7% | HIGH |
| abi_boundary_w_scatter_zig_runtime_w | 6686555.0ns | 2131300.8ns | 313.7% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_scatter_null_entry (n=6, range 3013.7-3218.3 ns)
   3013.7 |########################################
   3023.9 |####################
   3034.2 |
   3044.4 |
   3054.6 |
   3064.8 |
   3075.1 |
   3085.3 |
   3095.5 |
   3105.8 |####################
   3116.0 |
   3126.2 |
   3136.5 |
   3146.7 |
   3156.9 |
   3167.2 |
   3177.4 |
   3187.6 |####################
   3197.8 |
   3208.1 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_anchor (n=6, range 2153638.8-2169866.2 ns)
  2153638.8 |########################################
  2154450.2 |
  2155261.5 |
  2156072.9 |########################################
  2156884.3 |
  2157695.7 |
  2158507.0 |
  2159318.4 |
  2160129.8 |
  2160941.2 |
  2161752.5 |########################################
  2162563.9 |########################################
  2163375.3 |########################################
  2164186.6 |
  2164998.0 |
  2165809.4 |
  2166620.8 |
  2167432.1 |
  2168243.5 |
  2169054.9 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_dispatch (n=6, range 2148737.9-2166236.7 ns)
  2148737.9 |####################
  2149612.8 |
  2150487.8 |
  2151362.7 |
  2152237.6 |
  2153112.6 |########################################
  2153987.5 |
  2154862.5 |
  2155737.4 |####################
  2156612.3 |
  2157487.3 |
  2158362.2 |
  2159237.2 |
  2160112.1 |
  2160987.0 |
  2161862.0 |
  2162736.9 |####################
  2163611.8 |
  2164486.8 |
  2165361.7 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_per_w (n=6, range 2149058.8-2172278.4 ns)
  2149058.8 |########################################
  2150219.8 |
  2151380.8 |
  2152541.7 |########################################
  2153702.7 |
  2154863.7 |
  2156024.7 |########################################
  2157185.6 |########################################
  2158346.6 |
  2159507.6 |
  2160668.6 |
  2161829.6 |
  2162990.5 |
  2164151.5 |
  2165312.5 |
  2166473.5 |
  2167634.4 |
  2168795.4 |########################################
  2169956.4 |
  2171117.4 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_runtime_w (n=6, range 2156481.2-2173137.1 ns)
  2156481.2 |########################################
  2157314.0 |########################################
  2158146.8 |
  2158979.6 |########################################
  2159812.4 |
  2160645.2 |
  2161478.0 |
  2162310.8 |########################################
  2163143.6 |
  2163976.4 |
  2164809.2 |
  2165641.9 |
  2166474.7 |
  2167307.5 |
  2168140.3 |########################################
  2168973.1 |
  2169805.9 |
  2170638.7 |
  2171471.5 |
  2172304.3 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_dispatch (n=6, range 902437.9-915847.3 ns)
  902437.9 |####################
  903108.4 |
  903778.8 |
  904449.3 |
  905119.8 |
  905790.2 |
  906460.7 |########################################
  907131.2 |
  907801.7 |
  908472.1 |
  909142.6 |
  909813.1 |
  910483.5 |
  911154.0 |
  911824.5 |####################
  912495.0 |
  913165.4 |####################
  913835.9 |
  914506.4 |
  915176.8 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_per_w (n=6, range 889776.2-904099.8 ns)
  889776.2 |####################
  890492.4 |
  891208.6 |
  891924.7 |
  892640.9 |
  893357.1 |
  894073.3 |
  894789.5 |
  895505.6 |
  896221.8 |########################################
  896938.0 |
  897654.2 |
  898370.4 |
  899086.5 |
  899802.7 |
  900518.9 |
  901235.1 |####################
  901951.3 |
  902667.4 |
  903383.6 |####################
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_runtime_w (n=6, range 893125.0-908650.8 ns)
  893125.0 |########################################
  893901.3 |
  894677.6 |####################
  895453.9 |
  896230.2 |
  897006.4 |####################
  897782.7 |
  898559.0 |
  899335.3 |
  900111.6 |
  900887.9 |
  901664.2 |
  902440.5 |
  903216.8 |
  903993.1 |
  904769.4 |
  905545.6 |
  906321.9 |
  907098.2 |
  907874.5 |####################
  (0 below, 1 above range)

abi_boundary_w_scatter_zig_runtime_w (n=6, range 2125879.6-2135382.3 ns)
  2125879.6 |########################################
  2126354.7 |
  2126829.9 |########################################
  2127305.0 |
  2127780.1 |
  2128255.3 |
  2128730.4 |
  2129205.5 |
  2129680.7 |
  2130155.8 |
  2130631.0 |########################################
  2131106.1 |
  2131581.2 |
  2132056.4 |
  2132531.5 |
  2133006.6 |########################################
  2133481.8 |
  2133956.9 |########################################
  2134432.0 |
  2134907.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_scatter_null_entry**: bridge=3915.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_anchor**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_dispatch**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_per_w**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_runtime_w**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_dispatch**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_per_w**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_runtime_w**: bridge=305.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_zig_runtime_w**: bridge=313.3% of algo (FFI overhead may distort results)
