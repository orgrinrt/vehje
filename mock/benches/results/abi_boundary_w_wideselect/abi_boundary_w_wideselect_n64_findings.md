# abi_boundary_w (wideselect)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_wideselect_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_wideselect_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_wideselect_null_entry dominates: 37347% faster than the next best (abi_boundary_w_wideselect_soa_dispatch)

abi_boundary_w_wideselect_null_entry (2.51 us) leads abi_boundary_w_wideselect_soa_dispatch (941.00 us) by 37347%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_wideselect_null_entry beats baseline by 100% (significant)

abi_boundary_w_wideselect_null_entry is -2.12 ms (100%) faster than baseline abi_boundary_w_wideselect_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_wideselect_scalar_anchor is an outlier: 845.5x slower than the field

abi_boundary_w_wideselect_scalar_anchor (2.12 ms) is 845.5x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_wideselect_soa_dispatch shows alternating (throttle bounce) (autocorr -0.60)

abi_boundary_w_wideselect_soa_dispatch's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_wideselect_null_entry} vs {abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_anchor} (37347% apart)

The field splits into a fast tier {abi_boundary_w_wideselect_null_entry} and a slow tier {abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_anchor} with a 37347% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 845.5x the fastest

Fastest abi_boundary_w_wideselect_null_entry (2.51 us) to slowest abi_boundary_w_wideselect_scalar_anchor (2.12 ms): 845.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_wideselect_null_entry** at 2512.9 ns median (-99.9% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 845.46x (fastest 2512.9 ns, slowest 2124551.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 4844ns | 4775ns | 4736ns | 4769ns | 5009ns | -99.77% |
| abi_boundary_w_wideselect_scalar_anchor | 2161264ns | 2128556ns | 2107007ns | 2123008ns | 2245777ns | +1.88% |
| abi_boundary_w_wideselect_scalar_dispatch | 2121838ns | 2119724ns | 2105397ns | 2115087ns | 2140186ns | +0.02% |
| abi_boundary_w_wideselect_scalar_per_w | 2109360ns | 2109221ns | 2097132ns | 2106285ns | 2120087ns | -0.57% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2121445ns | 2126612ns | 2102852ns | 2122016ns | 2129884ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 945454ns | 944080ns | 937480ns | 943175ns | 952860ns | -55.43% |
| abi_boundary_w_wideselect_soa_per_w | 950554ns | 952172ns | 941111ns | 949222ns | 957274ns | -55.19% |
| abi_boundary_w_wideselect_soa_runtime_w | 948436ns | 945058ns | 939931ns | 944248ns | 958970ns | -55.29% |
| abi_boundary_w_wideselect_zig_runtime_w | 2093698ns | 2037415ns | 2024456ns | 2033537ns | 2218560ns | -1.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 2534ns | 2478ns | 2607ns | -99.88% | 0.025 |
| abi_boundary_w_wideselect_scalar_anchor | 2157235ns | 2103284ns | 2241568ns | +1.88% | 0.000 |
| abi_boundary_w_wideselect_scalar_dispatch | 2117859ns | 2101475ns | 2136002ns | +0.02% | 0.000 |
| abi_boundary_w_wideselect_scalar_per_w | 2105735ns | 2093429ns | 2116501ns | -0.55% | 0.000 |
| abi_boundary_w_wideselect_scalar_runtime_w | 2117442ns | 2099164ns | 2125882ns | base | 0.000 |
| abi_boundary_w_wideselect_soa_dispatch | 942382ns | 934702ns | 949554ns | -55.49% | 0.000 |
| abi_boundary_w_wideselect_soa_per_w | 947428ns | 938013ns | 954267ns | -55.26% | 0.000 |
| abi_boundary_w_wideselect_soa_runtime_w | 945200ns | 936984ns | 955550ns | -55.36% | 0.000 |
| abi_boundary_w_wideselect_zig_runtime_w | 2089607ns | 2020785ns | 2214169ns | -1.31% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 29669.5 | 2747.3 | 2533.7 | n/a |
| abi_boundary_w_wideselect_scalar_anchor | 86382.2 | 2162829.4 | 2157234.7 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 88059.7 | 2113507.8 | 2117859.2 | 2 |
| abi_boundary_w_wideselect_scalar_per_w | 76718.1 | 2105434.6 | 2105735.0 | n/a |
| abi_boundary_w_wideselect_scalar_runtime_w | 89101.7 | 2120564.6 | 2117441.9 | n/a |
| abi_boundary_w_wideselect_soa_dispatch | 54575.0 | 942152.4 | 942381.6 | n/a |
| abi_boundary_w_wideselect_soa_per_w | 59138.2 | 946790.9 | 947427.8 | n/a |
| abi_boundary_w_wideselect_soa_runtime_w | 60590.2 | 945358.8 | 945200.1 | n/a |
| abi_boundary_w_wideselect_zig_runtime_w | 282139.9 | 2100727.4 | 2089607.4 | 7 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_boundary_w_wideselect_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.025 | 98.6% |
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
| abi_boundary_w_wideselect_null_entry | 4844ns | 4844ns | -99.77% |
| abi_boundary_w_wideselect_scalar_anchor | 2161264ns | 2161264ns | +1.88% |
| abi_boundary_w_wideselect_scalar_dispatch | 2121838ns | 2121838ns | +0.02% |
| abi_boundary_w_wideselect_scalar_per_w | 2109360ns | 2109360ns | -0.57% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2121445ns | 2121445ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 945454ns | 945454ns | -55.43% |
| abi_boundary_w_wideselect_soa_per_w | 950554ns | 950554ns | -55.19% |
| abi_boundary_w_wideselect_soa_runtime_w | 948436ns | 948436ns | -55.29% |
| abi_boundary_w_wideselect_zig_runtime_w | 2093698ns | 2093698ns | -1.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_scalar_runtime_w | 2122322ns | base | --- | [2104122, 2125882] | --- | --- | --- | --- |
| abi_boundary_w_wideselect_null_entry | 2513ns | -2119831.0ns (-99.9%) | [-2123295, -2101599]ns | [2481, 2607] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_scalar_anchor | 2124552ns | no significant difference | [-9273, +122953]ns | [2105585, 2241568] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 2115721ns | no significant difference | [-17190, +26101]ns | [2101855, 2136002] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_wideselect_scalar_per_w | 2105590ns | no significant difference | [-27410, +3718]ns | [2095114, 2116501] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_dispatch | 941003ns | -1176327.9ns (-55.4%) | [-1181661, -1167192]ns | [936588, 949554] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_per_w | 949044ns | -1172793.4ns (-55.3%) | [-1186232, -1151017]ns | [938972, 954267] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_runtime_w | 941691ns | -1174956.9ns (-55.4%) | [-1187418, -1154351]ns | [938360, 955550] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_zig_runtime_w | 2033386ns | no significant difference | [-101733, +97626]ns | [2021267, 2214169] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_wideselect_scalar_runtime_w | abi_boundary_w_wideselect_null_entry | abi_boundary_w_wideselect_scalar_anchor | abi_boundary_w_wideselect_scalar_dispatch | abi_boundary_w_wideselect_scalar_per_w | abi_boundary_w_wideselect_soa_dispatch | abi_boundary_w_wideselect_soa_per_w | abi_boundary_w_wideselect_soa_runtime_w | abi_boundary_w_wideselect_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2124008ns | -99.9% | +10.6% | -0.5% | -1.1% | -55.6% | -55.8% | -55.8% | +12.3% |
| 2 | 2126401ns | -99.9% | +0.1% | -0.4% | -0.3% | -55.3% | -55.8% | -55.9% | -4.0% |
| 3 | 2099164ns | -99.9% | +1.0% | +1.5% | +0.7% | -55.5% | -54.6% | -54.6% | -3.5% |
| 4 | 2125362ns | -99.9% | +0.4% | -1.1% | -1.5% | -55.4% | -55.1% | -55.8% | -4.9% |
| 5 | 2109079ns | -99.9% | -0.3% | -0.3% | -0.6% | -55.5% | -54.8% | -55.3% | -3.1% |
| 6 | 2120637ns | -99.9% | -0.6% | +1.0% | -0.5% | -55.7% | -55.4% | -54.8% | -4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.015 | ok |
| abi_boundary_w_wideselect_scalar_anchor | 0.006 | ok |
| abi_boundary_w_wideselect_scalar_dispatch | -0.253 | moderate- |
| abi_boundary_w_wideselect_scalar_per_w | 0.036 | ok |
| abi_boundary_w_wideselect_scalar_runtime_w | -0.571 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_soa_dispatch | -0.600 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_soa_per_w | 0.364 | moderate+ |
| abi_boundary_w_wideselect_soa_runtime_w | -0.184 | ok |
| abi_boundary_w_wideselect_zig_runtime_w | -0.003 | ok |

**Consistency summary:**

- **abi_boundary_w_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_scalar_anchor**: won 2/6, lost 4/6
- **abi_boundary_w_wideselect_scalar_dispatch**: won 4/6, lost 2/6
- **abi_boundary_w_wideselect_scalar_per_w**: won 5/6, lost 1/6
- **abi_boundary_w_wideselect_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_zig_runtime_w**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 115631.8ns | 2533.7ns | 4563.7% | HIGH |
| abi_boundary_w_wideselect_scalar_anchor | 6570799.7ns | 2157234.7ns | 304.6% | HIGH |
| abi_boundary_w_wideselect_scalar_dispatch | 6431014.0ns | 2117859.2ns | 303.7% | HIGH |
| abi_boundary_w_wideselect_scalar_per_w | 6391849.3ns | 2105735.0ns | 303.5% | HIGH |
| abi_boundary_w_wideselect_scalar_runtime_w | 6448284.4ns | 2117441.9ns | 304.5% | HIGH |
| abi_boundary_w_wideselect_soa_dispatch | 2882329.4ns | 942381.6ns | 305.9% | HIGH |
| abi_boundary_w_wideselect_soa_per_w | 2899897.9ns | 947427.8ns | 306.1% | HIGH |
| abi_boundary_w_wideselect_soa_runtime_w | 2897042.3ns | 945200.1ns | 306.5% | HIGH |
| abi_boundary_w_wideselect_zig_runtime_w | 6676728.4ns | 2089607.4ns | 319.5% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_wideselect_null_entry (n=6, range 2477.5-2606.9 ns)
   2477.5 |########################################
   2484.0 |########################################
   2490.4 |
   2496.9 |########################################
   2503.4 |
   2509.8 |
   2516.3 |
   2522.8 |########################################
   2529.2 |
   2535.7 |
   2542.2 |
   2548.6 |
   2555.1 |
   2561.6 |
   2568.0 |########################################
   2574.5 |
   2581.0 |
   2587.4 |
   2593.9 |
   2600.4 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_anchor (n=6, range 2103283.8-2241568.0 ns)
  2103283.8 |########################################
  2110198.0 |
  2117112.2 |####################
  2124026.4 |####################
  2130940.6 |####################
  2137854.8 |
  2144769.0 |
  2151683.3 |
  2158597.5 |
  2165511.7 |
  2172425.9 |
  2179340.1 |
  2186254.3 |
  2193168.5 |
  2200082.7 |
  2206996.9 |
  2213911.1 |
  2220825.3 |
  2227739.5 |
  2234653.7 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_dispatch (n=6, range 2101475.4-2136002.1 ns)
  2101475.4 |########################################
  2103201.7 |
  2104928.1 |
  2106654.4 |
  2108380.7 |
  2110107.1 |
  2111833.4 |####################
  2113559.7 |
  2115286.1 |
  2117012.4 |####################
  2118738.8 |
  2120465.1 |
  2122191.4 |
  2123917.8 |
  2125644.1 |
  2127370.4 |
  2129096.8 |####################
  2130823.1 |
  2132549.4 |
  2134275.8 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_per_w (n=6, range 2093428.8-2116500.6 ns)
  2093428.8 |########################################
  2094582.4 |
  2095736.0 |########################################
  2096889.6 |
  2098043.2 |
  2099196.8 |
  2100350.4 |########################################
  2101503.9 |
  2102657.5 |
  2103811.1 |
  2104964.7 |
  2106118.3 |
  2107271.9 |
  2108425.5 |
  2109579.1 |########################################
  2110732.7 |
  2111886.3 |
  2113039.9 |########################################
  2114193.5 |
  2115347.1 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_runtime_w (n=6, range 2099164.2-2125881.9 ns)
  2099164.2 |########################################
  2100500.1 |
  2101836.0 |
  2103171.8 |
  2104507.7 |
  2105843.6 |
  2107179.5 |
  2108515.4 |########################################
  2109851.3 |
  2111187.1 |
  2112523.0 |
  2113858.9 |
  2115194.8 |
  2116530.7 |
  2117866.6 |
  2119202.4 |
  2120538.3 |########################################
  2121874.2 |
  2123210.1 |########################################
  2124546.0 |########################################
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_dispatch (n=6, range 934702.5-949553.9 ns)
  934702.5 |####################
  935445.1 |
  936187.6 |
  936930.2 |
  937672.8 |
  938415.4 |########################################
  939157.9 |
  939900.5 |
  940643.1 |
  941385.7 |
  942128.2 |####################
  942870.8 |
  943613.4 |
  944355.9 |
  945098.5 |
  945841.1 |
  946583.7 |
  947326.2 |
  948068.8 |####################
  948811.4 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_per_w (n=6, range 938012.9-954267.1 ns)
  938012.9 |########################################
  938825.6 |
  939638.3 |########################################
  940451.0 |
  941263.7 |
  942076.4 |
  942889.2 |
  943701.9 |
  944514.6 |
  945327.3 |########################################
  946140.0 |
  946952.7 |
  947765.4 |
  948578.1 |
  949390.8 |
  950203.6 |
  951016.3 |
  951829.0 |########################################
  952641.7 |
  953454.4 |########################################
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_runtime_w (n=6, range 936983.8-955549.8 ns)
  936983.8 |########################################
  937912.1 |
  938840.4 |########################################
  939768.7 |########################################
  940697.0 |
  941625.3 |
  942553.6 |########################################
  943481.9 |
  944410.2 |
  945338.5 |
  946266.8 |
  947195.1 |
  948123.4 |
  949051.7 |
  949980.0 |
  950908.3 |
  951836.6 |########################################
  952764.9 |
  953693.2 |
  954621.5 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_zig_runtime_w (n=6, range 2020784.6-2214169.0 ns)
  2020784.6 |########################################
  2030453.8 |
  2040123.0 |##########################
  2049792.3 |
  2059461.5 |
  2069130.7 |
  2078799.9 |
  2088469.1 |
  2098138.3 |
  2107807.6 |
  2117476.8 |
  2127146.0 |
  2136815.2 |
  2146484.4 |
  2156153.6 |
  2165822.9 |
  2175492.1 |
  2185161.3 |
  2194830.5 |
  2204499.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_wideselect_null_entry**: bridge=4608.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_anchor**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_dispatch**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_per_w**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_runtime_w**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_dispatch**: bridge=305.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_per_w**: bridge=305.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_runtime_w**: bridge=307.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_zig_runtime_w**: bridge=319.5% of algo (FFI overhead may distort results)
