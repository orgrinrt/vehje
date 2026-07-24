# abi_boundary_w (real)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_real_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_real_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_real_null_entry dominates: 39297% faster than the next best (abi_boundary_w_real_soa_runtime_w)

abi_boundary_w_real_null_entry (2.26 us) leads abi_boundary_w_real_soa_runtime_w (888.49 us) by 39297%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_real_null_entry beats baseline by 100% (significant)

abi_boundary_w_real_null_entry is -2.17 ms (100%) faster than baseline abi_boundary_w_real_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_real_scalar_anchor is an outlier: 965.7x slower than the field

abi_boundary_w_real_scalar_anchor (2.18 ms) is 965.7x the fastest (2.26 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_real_soa_dispatch shows alternating (throttle bounce) (autocorr -0.83)

abi_boundary_w_real_soa_dispatch's per-pass series has lag-1 autocorrelation -0.83, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_real_null_entry} vs {abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_anchor} (39297% apart)

The field splits into a fast tier {abi_boundary_w_real_null_entry} and a slow tier {abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_anchor} with a 39297% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 965.7x the fastest

Fastest abi_boundary_w_real_null_entry (2.26 us) to slowest abi_boundary_w_real_scalar_anchor (2.18 ms): 965.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_real_null_entry** at 2255.2 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 965.71x (fastest 2255.2 ns, slowest 2177867.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 4488ns | 4467ns | 4378ns | 4460ns | 4584ns | -99.79% |
| abi_boundary_w_real_scalar_anchor | 2297325ns | 2180999ns | 2170811ns | 2177877ns | 2539755ns | +5.63% |
| abi_boundary_w_real_scalar_dispatch | 2171584ns | 2170689ns | 2165286ns | 2169692ns | 2177572ns | -0.15% |
| abi_boundary_w_real_scalar_per_w | 2171195ns | 2174950ns | 2160601ns | 2171350ns | 2176261ns | -0.17% |
| abi_boundary_w_real_scalar_runtime_w | 2174792ns | 2173325ns | 2158115ns | 2170739ns | 2189210ns | base |
| abi_boundary_w_real_soa_dispatch | 894354ns | 893332ns | 889489ns | 892118ns | 900141ns | -58.88% |
| abi_boundary_w_real_soa_per_w | 896407ns | 897475ns | 885922ns | 896405ns | 901654ns | -58.78% |
| abi_boundary_w_real_soa_runtime_w | 892263ns | 891159ns | 882680ns | 890345ns | 899932ns | -58.97% |
| abi_boundary_w_real_zig_runtime_w | 2115781ns | 2104665ns | 2098728ns | 2103234ns | 2143127ns | -2.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 2254ns | 2215ns | 2292ns | -99.90% | 0.014 |
| abi_boundary_w_real_scalar_anchor | 2294117ns | 2167972ns | 2536132ns | +5.63% | 0.000 |
| abi_boundary_w_real_scalar_dispatch | 2168499ns | 2162109ns | 2174409ns | -0.15% | 0.000 |
| abi_boundary_w_real_scalar_per_w | 2168207ns | 2157798ns | 2173141ns | -0.17% | 0.000 |
| abi_boundary_w_real_scalar_runtime_w | 2171815ns | 2155473ns | 2186128ns | base | 0.000 |
| abi_boundary_w_real_soa_dispatch | 891719ns | 886748ns | 897592ns | -58.94% | 0.000 |
| abi_boundary_w_real_soa_per_w | 893738ns | 883318ns | 898962ns | -58.85% | 0.000 |
| abi_boundary_w_real_soa_runtime_w | 889639ns | 880264ns | 897273ns | -59.04% | 0.000 |
| abi_boundary_w_real_zig_runtime_w | 2112550ns | 2095679ns | 2139636ns | -2.73% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 27129.1 | 2387.7 | 2253.9 | n/a |
| abi_boundary_w_real_scalar_anchor | 60423.0 | 2224195.8 | 2294116.6 | n/a |
| abi_boundary_w_real_scalar_dispatch | 55679.8 | 2167251.8 | 2168498.9 | 1 |
| abi_boundary_w_real_scalar_per_w | 58084.4 | 2169467.1 | 2168207.1 | 2 |
| abi_boundary_w_real_scalar_runtime_w | 52212.5 | 2170148.3 | 2171815.4 | n/a |
| abi_boundary_w_real_soa_dispatch | 40746.8 | 890566.5 | 891719.2 | n/a |
| abi_boundary_w_real_soa_per_w | 42667.3 | 893949.2 | 893737.8 | n/a |
| abi_boundary_w_real_soa_runtime_w | 39723.1 | 890226.2 | 889639.4 | n/a |
| abi_boundary_w_real_zig_runtime_w | 223599.7 | 2112196.4 | 2112550.4 | 3 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_boundary_w_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.014 | 98.2% |
| abi_boundary_w_real_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_real_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_real_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_real_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_real_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_real_null_entry | 4488ns | 4488ns | -99.79% |
| abi_boundary_w_real_scalar_anchor | 2297325ns | 2297325ns | +5.63% |
| abi_boundary_w_real_scalar_dispatch | 2171584ns | 2171584ns | -0.15% |
| abi_boundary_w_real_scalar_per_w | 2171195ns | 2171195ns | -0.17% |
| abi_boundary_w_real_scalar_runtime_w | 2174792ns | 2174792ns | base |
| abi_boundary_w_real_soa_dispatch | 894354ns | 894354ns | -58.88% |
| abi_boundary_w_real_soa_per_w | 896407ns | 896407ns | -58.78% |
| abi_boundary_w_real_soa_runtime_w | 892263ns | 892263ns | -58.97% |
| abi_boundary_w_real_zig_runtime_w | 2115781ns | 2115781ns | -2.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_real_scalar_runtime_w | 2170366ns | base | --- | [2158953, 2186128] | --- | --- | --- | --- |
| abi_boundary_w_real_null_entry | 2255ns | -2168095.0ns (-99.9%) | [-2183872, -2156718]ns | [2215, 2292] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_scalar_anchor | 2177867ns | +12751.3ns (+0.6%) | [+2397, +351755]ns | [2168351, 2536132] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_real_scalar_dispatch | 2167672ns | no significant difference | [-20412, +11431]ns | [2163416, 2174409] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_real_scalar_per_w | 2171872ns | no significant difference | [-18866, +7123]ns | [2159608, 2173141] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_real_soa_dispatch | 890645ns | -1279932.3ns (-59.0%) | [-1296679, -1263677]ns | [886920, 897592] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_soa_per_w | 894830ns | -1275106.9ns (-58.8%) | [-1295003, -1264123]ns | [887421, 898962] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_soa_runtime_w | 888490ns | -1276787.3ns (-58.8%) | [-1294522, -1275218]ns | [883156, 897273] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_zig_runtime_w | 2101606ns | -64957.5ns (-3.0%) | [-72544, -40294]ns | [2096409, 2139636] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_real_scalar_runtime_w | abi_boundary_w_real_null_entry | abi_boundary_w_real_scalar_anchor | abi_boundary_w_real_scalar_dispatch | abi_boundary_w_real_scalar_per_w | abi_boundary_w_real_soa_dispatch | abi_boundary_w_real_soa_per_w | abi_boundary_w_real_soa_runtime_w | abi_boundary_w_real_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2197801ns | -99.9% | +31.3% | -1.5% | -1.1% | -59.4% | -59.1% | -59.5% | -1.3% |
| 2 | 2173666ns | -99.9% | +0.6% | +0.1% | -0.0% | -59.1% | -59.0% | -58.7% | -3.0% |
| 3 | 2155473ns | -99.9% | +0.6% | +0.6% | +0.1% | -58.4% | -58.5% | -59.2% | -2.5% |
| 4 | 2162432ns | -99.9% | +0.7% | +0.4% | +0.4% | -59.0% | -58.6% | -59.0% | -3.0% |
| 5 | 2167066ns | -99.9% | +0.5% | -0.2% | +0.2% | -58.6% | -58.5% | -59.1% | -3.3% |
| 6 | 2174454ns | -99.9% | -0.3% | -0.4% | -0.6% | -59.2% | -59.4% | -58.8% | -3.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.195 | ok |
| abi_boundary_w_real_scalar_anchor | -0.018 | ok |
| abi_boundary_w_real_scalar_dispatch | -0.297 | moderate- |
| abi_boundary_w_real_scalar_per_w | -0.335 | moderate- |
| abi_boundary_w_real_scalar_runtime_w | 0.191 | ok |
| abi_boundary_w_real_soa_dispatch | -0.826 | HIGH- (thermal bounce) |
| abi_boundary_w_real_soa_per_w | -0.354 | moderate- |
| abi_boundary_w_real_soa_runtime_w | -0.315 | moderate- |
| abi_boundary_w_real_zig_runtime_w | 0.109 | ok |

**Consistency summary:**

- **abi_boundary_w_real_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_real_scalar_anchor**: won 1/6, lost 5/6
- **abi_boundary_w_real_scalar_dispatch**: won 3/6, lost 3/6
- **abi_boundary_w_real_scalar_per_w**: won 2/6, lost 3/6
- **abi_boundary_w_real_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 116493.8ns | 2253.9ns | 5168.6% | HIGH |
| abi_boundary_w_real_scalar_anchor | 6758927.9ns | 2294116.6ns | 294.6% | HIGH |
| abi_boundary_w_real_scalar_dispatch | 6565483.3ns | 2168498.9ns | 302.8% | HIGH |
| abi_boundary_w_real_scalar_per_w | 6569106.0ns | 2168207.1ns | 303.0% | HIGH |
| abi_boundary_w_real_scalar_runtime_w | 6564246.5ns | 2171815.4ns | 302.2% | HIGH |
| abi_boundary_w_real_soa_dispatch | 2715207.4ns | 891719.2ns | 304.5% | HIGH |
| abi_boundary_w_real_soa_per_w | 2723585.9ns | 893737.8ns | 304.7% | HIGH |
| abi_boundary_w_real_soa_runtime_w | 2712092.4ns | 889639.4ns | 304.9% | HIGH |
| abi_boundary_w_real_zig_runtime_w | 6636182.6ns | 2112550.4ns | 314.1% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_real_null_entry (n=6, range 2214.6-2291.8 ns)
   2214.6 |##########################
   2218.5 |
   2222.3 |
   2226.2 |
   2230.0 |
   2233.9 |
   2237.8 |
   2241.6 |
   2245.5 |
   2249.4 |
   2253.2 |########################################
   2257.1 |
   2260.9 |
   2264.8 |
   2268.7 |
   2272.5 |
   2276.4 |
   2280.3 |
   2284.1 |
   2288.0 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_anchor (n=6, range 2167972.1-2536131.6 ns)
  2167972.1 |########################################
  2186380.1 |
  2204788.1 |
  2223196.0 |
  2241604.0 |
  2260012.0 |
  2278420.0 |
  2296827.9 |
  2315235.9 |
  2333643.9 |
  2352051.9 |
  2370459.9 |
  2388867.8 |
  2407275.8 |
  2425683.8 |
  2444091.8 |
  2462499.7 |
  2480907.7 |
  2499315.7 |
  2517723.7 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_dispatch (n=6, range 2162108.8-2174409.0 ns)
  2162108.8 |########################################
  2162723.8 |
  2163338.8 |
  2163953.8 |
  2164568.8 |########################################
  2165183.8 |
  2165798.8 |
  2166413.9 |########################################
  2167028.9 |
  2167643.9 |
  2168258.9 |########################################
  2168873.9 |
  2169488.9 |
  2170103.9 |
  2170718.9 |
  2171333.9 |
  2171948.9 |########################################
  2172563.9 |
  2173178.9 |
  2173793.9 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_per_w (n=6, range 2157798.3-2173141.2 ns)
  2157798.3 |####################
  2158565.4 |
  2159332.6 |
  2160099.7 |
  2160866.9 |####################
  2161634.0 |
  2162401.2 |
  2163168.3 |
  2163935.5 |
  2164702.6 |
  2165469.8 |
  2166236.9 |
  2167004.1 |
  2167771.2 |
  2168538.4 |
  2169305.5 |
  2170072.7 |
  2170839.8 |
  2171607.0 |########################################
  2172374.1 |####################
  (0 below, 1 above range)

abi_boundary_w_real_scalar_runtime_w (n=6, range 2155473.3-2186127.5 ns)
  2155473.3 |########################################
  2157006.0 |
  2158538.7 |
  2160071.4 |
  2161604.1 |########################################
  2163136.8 |
  2164669.6 |
  2166202.3 |########################################
  2167735.0 |
  2169267.7 |
  2170800.4 |
  2172333.1 |########################################
  2173865.8 |########################################
  2175398.5 |
  2176931.2 |
  2178464.0 |
  2179996.7 |
  2181529.4 |
  2183062.1 |
  2184594.8 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_dispatch (n=6, range 886747.9-897592.5 ns)
  886747.9 |########################################
  887290.1 |
  887832.4 |
  888374.6 |
  888916.8 |
  889459.1 |####################
  890001.3 |
  890543.5 |
  891085.7 |
  891628.0 |####################
  892170.2 |
  892712.4 |
  893254.7 |
  893796.9 |
  894339.1 |
  894881.3 |
  895423.6 |
  895965.8 |
  896508.0 |
  897050.3 |####################
  (0 below, 1 above range)

abi_boundary_w_real_soa_per_w (n=6, range 883317.5-898962.2 ns)
  883317.5 |####################
  884099.7 |
  884882.0 |
  885664.2 |
  886446.4 |
  887228.7 |
  888010.9 |
  888793.2 |
  889575.4 |
  890357.6 |
  891139.9 |####################
  891922.1 |
  892704.3 |
  893486.6 |
  894268.8 |########################################
  895051.1 |
  895833.3 |
  896615.5 |
  897397.8 |
  898180.0 |####################
  (0 below, 1 above range)

abi_boundary_w_real_soa_runtime_w (n=6, range 880263.8-897272.7 ns)
  880263.8 |########################################
  881114.2 |
  881964.7 |
  882815.1 |
  883665.6 |
  884516.0 |
  885366.5 |########################################
  886216.9 |
  887067.4 |########################################
  887917.8 |
  888768.2 |
  889618.7 |########################################
  890469.1 |
  891319.6 |
  892170.0 |
  893020.5 |
  893870.9 |
  894721.4 |
  895571.8 |
  896422.3 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_zig_runtime_w (n=6, range 2095678.8-2139636.2 ns)
  2095678.8 |########################################
  2097876.7 |
  2100074.5 |####################
  2102272.4 |####################
  2104470.3 |
  2106668.2 |
  2108866.0 |####################
  2111063.9 |
  2113261.8 |
  2115459.7 |
  2117657.5 |
  2119855.4 |
  2122053.3 |
  2124251.1 |
  2126449.0 |
  2128646.9 |
  2130844.8 |
  2133042.6 |
  2135240.5 |
  2137438.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_real_null_entry**: bridge=5162.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_anchor**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_dispatch**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_per_w**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_runtime_w**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_dispatch**: bridge=304.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_per_w**: bridge=304.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_runtime_w**: bridge=305.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_zig_runtime_w**: bridge=314.2% of algo (FFI overhead may distort results)
