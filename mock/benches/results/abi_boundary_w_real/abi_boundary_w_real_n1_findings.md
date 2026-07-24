# abi_boundary_w (real)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_real_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_real_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_real_null_entry dominates: 42153% faster than the next best (abi_boundary_w_real_zig_runtime_w)

abi_boundary_w_real_null_entry (4.90 us) leads abi_boundary_w_real_zig_runtime_w (2.07 ms) by 42153%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_real_null_entry beats baseline by 100% (significant)

abi_boundary_w_real_null_entry is -2.13 ms (100%) faster than baseline abi_boundary_w_real_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_real_scalar_per_w is an outlier: 437.1x slower than the field

abi_boundary_w_real_scalar_per_w (2.14 ms) is 437.1x the fastest (4.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_real_scalar_per_w shows alternating (throttle bounce) (autocorr -0.71)

abi_boundary_w_real_scalar_per_w's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_real_null_entry} vs {abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_scalar_per_w} (42153% apart)

The field splits into a fast tier {abi_boundary_w_real_null_entry} and a slow tier {abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_scalar_per_w} with a 42153% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 437.1x the fastest

Fastest abi_boundary_w_real_null_entry (4.90 us) to slowest abi_boundary_w_real_scalar_per_w (2.14 ms): 437.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_real_null_entry** at 4902.1 ns median (-99.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 437.15x (fastest 4902.1 ns, slowest 2142940.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 7257ns | 7171ns | 6975ns | 7108ns | 7621ns | -99.66% |
| abi_boundary_w_real_scalar_anchor | 2134950ns | 2135493ns | 2123559ns | 2133380ns | 2143002ns | -0.32% |
| abi_boundary_w_real_scalar_dispatch | 2137695ns | 2139355ns | 2128010ns | 2137189ns | 2143295ns | -0.20% |
| abi_boundary_w_real_scalar_per_w | 2142707ns | 2145408ns | 2127813ns | 2142598ns | 2150318ns | +0.04% |
| abi_boundary_w_real_scalar_runtime_w | 2141892ns | 2139452ns | 2127885ns | 2138405ns | 2154127ns | base |
| abi_boundary_w_real_soa_dispatch | 2138724ns | 2139273ns | 2126050ns | 2135250ns | 2150273ns | -0.15% |
| abi_boundary_w_real_soa_per_w | 2135398ns | 2135966ns | 2129132ns | 2134119ns | 2140450ns | -0.30% |
| abi_boundary_w_real_soa_runtime_w | 2146230ns | 2142695ns | 2136846ns | 2141266ns | 2158368ns | +0.20% |
| abi_boundary_w_real_zig_runtime_w | 2070698ns | 2073769ns | 2054291ns | 2072329ns | 2076454ns | -3.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 4933ns | 4785ns | 5109ns | -99.77% | 0.000 |
| abi_boundary_w_real_scalar_anchor | 2132466ns | 2121163ns | 2140526ns | -0.32% | 0.000 |
| abi_boundary_w_real_scalar_dispatch | 2135189ns | 2125541ns | 2140826ns | -0.20% | 0.000 |
| abi_boundary_w_real_scalar_per_w | 2140235ns | 2125256ns | 2147862ns | +0.04% | 0.000 |
| abi_boundary_w_real_scalar_runtime_w | 2139384ns | 2125456ns | 2151582ns | base | 0.000 |
| abi_boundary_w_real_soa_dispatch | 2136156ns | 2123389ns | 2147529ns | -0.15% | 0.000 |
| abi_boundary_w_real_soa_per_w | 2132957ns | 2126595ns | 2137928ns | -0.30% | 0.000 |
| abi_boundary_w_real_soa_runtime_w | 2143784ns | 2134356ns | 2155979ns | +0.21% | 0.000 |
| abi_boundary_w_real_zig_runtime_w | 2068171ns | 2051847ns | 2073894ns | -3.33% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 26729.5 | 5003.6 | 4933.1 | n/a |
| abi_boundary_w_real_scalar_anchor | 37033.5 | 2133434.0 | 2132465.7 | 0 |
| abi_boundary_w_real_scalar_dispatch | 38483.7 | 2138745.6 | 2135189.2 | 0 |
| abi_boundary_w_real_scalar_per_w | 37215.1 | 2140626.8 | 2140234.9 | n/a |
| abi_boundary_w_real_scalar_runtime_w | 36699.6 | 2139303.7 | 2139383.7 | n/a |
| abi_boundary_w_real_soa_dispatch | 37025.3 | 2137248.1 | 2136155.9 | 0 |
| abi_boundary_w_real_soa_per_w | 37210.4 | 2134341.2 | 2132957.2 | 0 |
| abi_boundary_w_real_soa_runtime_w | 35491.4 | 2143357.5 | 2143784.4 | 0 |
| abi_boundary_w_real_zig_runtime_w | 177655.3 | 2068726.8 | 2068170.8 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_boundary_w_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.000 | 97.6% |
| abi_boundary_w_real_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_real_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_real_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_real_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_real_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_real_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_real_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_real_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_real_null_entry | 7257ns | 7257ns | -99.66% |
| abi_boundary_w_real_scalar_anchor | 2134950ns | 2134950ns | -0.32% |
| abi_boundary_w_real_scalar_dispatch | 2137695ns | 2137695ns | -0.20% |
| abi_boundary_w_real_scalar_per_w | 2142707ns | 2142707ns | +0.04% |
| abi_boundary_w_real_scalar_runtime_w | 2141892ns | 2141892ns | base |
| abi_boundary_w_real_soa_dispatch | 2138724ns | 2138724ns | -0.15% |
| abi_boundary_w_real_soa_per_w | 2135398ns | 2135398ns | -0.30% |
| abi_boundary_w_real_soa_runtime_w | 2146230ns | 2146230ns | +0.20% |
| abi_boundary_w_real_zig_runtime_w | 2070698ns | 2070698ns | -3.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_real_scalar_runtime_w | 2136959ns | base | --- | [2129610, 2151582] | --- | --- | --- | --- |
| abi_boundary_w_real_null_entry | 4902ns | -2132042.2ns (-99.8%) | [-2146487, -2124822]ns | [4788, 5109] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_boundary_w_real_scalar_anchor | 2132964ns | -4219.8ns (-0.2%) | [-15494, -1040]ns | [2123908, 2140526] | YES (adj: no) | 0.5833 | 0.2188 | 0 |
| abi_boundary_w_real_scalar_dispatch | 2136829ns | no significant difference | [-19306, +5998]ns | [2127912, 2140826] | no | 1.0000 | 0.6875 | 0 |
| abi_boundary_w_real_scalar_per_w | 2142941ns | no significant difference | [-17106, +15409]ns | [2129902, 2147862] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_real_soa_dispatch | 2136876ns | no significant difference | [-23289, +10570]ns | [2124062, 2147529] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_real_soa_per_w | 2133594ns | no significant difference | [-24232, +4895]ns | [2127350, 2137928] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_real_soa_runtime_w | 2140183ns | no significant difference | [-13129, +21151]ns | [2135191, 2155979] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_real_zig_runtime_w | 2071288ns | -71136.0ns (-3.3%) | [-81569, -60934]ns | [2059330, 2073894] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_real_scalar_runtime_w | abi_boundary_w_real_null_entry | abi_boundary_w_real_scalar_anchor | abi_boundary_w_real_scalar_dispatch | abi_boundary_w_real_scalar_per_w | abi_boundary_w_real_soa_dispatch | abi_boundary_w_real_soa_per_w | abi_boundary_w_real_soa_runtime_w | abi_boundary_w_real_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2148775ns | -99.8% | -0.1% | -0.7% | -0.2% | -1.1% | -1.0% | -0.4% | -3.8% |
| 2 | 2125456ns | -99.8% | -0.2% | +0.0% | +0.8% | -0.1% | +0.4% | +0.7% | -3.5% |
| 3 | 2138026ns | -99.8% | -0.5% | +0.1% | -0.2% | +0.4% | +0.0% | -0.2% | -3.2% |
| 4 | 2133765ns | -99.8% | +0.0% | +0.3% | +0.7% | +0.4% | -0.0% | +1.0% | -2.8% |
| 5 | 2154389ns | -99.8% | -0.9% | -1.1% | -1.4% | -1.0% | -1.2% | -0.9% | -3.8% |
| 6 | 2135891ns | -99.8% | -0.2% | +0.3% | +0.6% | +0.5% | +0.1% | +0.9% | -2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_real_null_entry | -0.556 | HIGH- (thermal bounce) |
| abi_boundary_w_real_scalar_anchor | -0.272 | moderate- |
| abi_boundary_w_real_scalar_dispatch | -0.334 | moderate- |
| abi_boundary_w_real_scalar_per_w | -0.710 | HIGH- (thermal bounce) |
| abi_boundary_w_real_scalar_runtime_w | -0.436 | moderate- |
| abi_boundary_w_real_soa_dispatch | -0.013 | ok |
| abi_boundary_w_real_soa_per_w | -0.170 | ok |
| abi_boundary_w_real_soa_runtime_w | -0.530 | HIGH- (thermal bounce) |
| abi_boundary_w_real_zig_runtime_w | 0.185 | ok |

**Consistency summary:**

- **abi_boundary_w_real_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_real_scalar_anchor**: won 5/6, lost 0/6
- **abi_boundary_w_real_scalar_dispatch**: won 2/6, lost 2/6
- **abi_boundary_w_real_scalar_per_w**: won 3/6, lost 3/6
- **abi_boundary_w_real_soa_dispatch**: won 2/6, lost 3/6
- **abi_boundary_w_real_soa_per_w**: won 2/6, lost 1/6
- **abi_boundary_w_real_soa_runtime_w**: won 3/6, lost 3/6
- **abi_boundary_w_real_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 123858.8ns | 4933.1ns | 2510.8% | HIGH |
| abi_boundary_w_real_scalar_anchor | 6440631.0ns | 2132465.7ns | 302.0% | HIGH |
| abi_boundary_w_real_scalar_dispatch | 6451544.0ns | 2135189.2ns | 302.2% | HIGH |
| abi_boundary_w_real_scalar_per_w | 6461712.1ns | 2140234.9ns | 301.9% | HIGH |
| abi_boundary_w_real_scalar_runtime_w | 6459757.2ns | 2139383.7ns | 301.9% | HIGH |
| abi_boundary_w_real_soa_dispatch | 6447946.2ns | 2136155.9ns | 301.8% | HIGH |
| abi_boundary_w_real_soa_per_w | 6439886.8ns | 2132957.2ns | 301.9% | HIGH |
| abi_boundary_w_real_soa_runtime_w | 6468265.4ns | 2143784.4ns | 301.7% | HIGH |
| abi_boundary_w_real_zig_runtime_w | 6454403.8ns | 2068170.8ns | 312.1% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_real_null_entry (n=6, range 4784.6-5109.1 ns)
   4784.6 |########################################
   4800.8 |####################
   4817.1 |
   4833.3 |
   4849.5 |
   4865.7 |
   4882.0 |
   4898.2 |
   4914.4 |
   4930.6 |
   4946.9 |
   4963.1 |
   4979.3 |####################
   4995.6 |
   5011.8 |####################
   5028.0 |
   5044.2 |
   5060.5 |
   5076.7 |
   5092.9 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_anchor (n=6, range 2121162.9-2140526.0 ns)
  2121162.9 |########################################
  2122131.1 |
  2123099.2 |
  2124067.4 |
  2125035.5 |
  2126003.7 |########################################
  2126971.8 |
  2127940.0 |
  2128908.2 |
  2129876.3 |
  2130844.5 |########################################
  2131812.6 |
  2132780.8 |
  2133748.9 |########################################
  2134717.1 |########################################
  2135685.3 |
  2136653.4 |
  2137621.6 |
  2138589.7 |
  2139557.9 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_dispatch (n=6, range 2125541.2-2140826.0 ns)
  2125541.2 |####################
  2126305.4 |
  2127069.7 |
  2127833.9 |
  2128598.2 |
  2129362.4 |
  2130126.7 |####################
  2130890.9 |
  2131655.1 |
  2132419.4 |
  2133183.6 |
  2133947.9 |####################
  2134712.1 |
  2135476.4 |
  2136240.6 |
  2137004.8 |
  2137769.1 |
  2138533.3 |
  2139297.6 |########################################
  2140061.8 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_per_w (n=6, range 2125256.2-2147861.5 ns)
  2125256.2 |########################################
  2126386.5 |
  2127516.7 |
  2128647.0 |
  2129777.3 |
  2130907.5 |
  2132037.8 |
  2133168.1 |
  2134298.3 |########################################
  2135428.6 |
  2136558.9 |
  2137689.1 |
  2138819.4 |
  2139949.6 |
  2141079.9 |########################################
  2142210.2 |
  2143340.4 |########################################
  2144470.7 |
  2145601.0 |
  2146731.2 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_scalar_runtime_w (n=6, range 2125456.2-2151581.9 ns)
  2125456.2 |########################################
  2126762.5 |
  2128068.8 |
  2129375.1 |
  2130681.3 |
  2131987.6 |
  2133293.9 |########################################
  2134600.2 |########################################
  2135906.5 |
  2137212.8 |########################################
  2138519.1 |
  2139825.3 |
  2141131.6 |
  2142437.9 |
  2143744.2 |
  2145050.5 |
  2146356.8 |
  2147663.0 |########################################
  2148969.3 |
  2150275.6 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_dispatch (n=6, range 2123389.2-2147529.2 ns)
  2123389.2 |########################################
  2124596.2 |########################################
  2125803.2 |
  2127010.2 |
  2128217.2 |
  2129424.2 |
  2130631.2 |
  2131838.2 |########################################
  2133045.2 |
  2134252.2 |
  2135459.2 |
  2136666.2 |
  2137873.2 |
  2139080.2 |
  2140287.2 |
  2141494.2 |########################################
  2142701.2 |
  2143908.2 |
  2145115.2 |
  2146322.2 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_soa_per_w (n=6, range 2126595.4-2137928.2 ns)
  2126595.4 |########################################
  2127162.0 |
  2127728.7 |########################################
  2128295.3 |
  2128862.0 |
  2129428.6 |
  2129995.2 |
  2130561.9 |
  2131128.5 |
  2131695.1 |
  2132261.8 |
  2132828.4 |########################################
  2133395.1 |
  2133961.7 |########################################
  2134528.3 |
  2135095.0 |
  2135661.6 |
  2136228.2 |
  2136794.9 |########################################
  2137361.5 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_runtime_w (n=6, range 2134355.8-2155979.3 ns)
  2134355.8 |########################################
  2135437.0 |########################################
  2136518.2 |
  2137599.3 |
  2138680.5 |########################################
  2139761.7 |
  2140842.9 |########################################
  2141924.0 |
  2143005.2 |
  2144086.4 |
  2145167.6 |
  2146248.8 |
  2147329.9 |
  2148411.1 |
  2149492.3 |
  2150573.5 |
  2151654.6 |
  2152735.8 |
  2153817.0 |
  2154898.2 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_zig_runtime_w (n=6, range 2051846.7-2073894.1 ns)
  2051846.7 |####################
  2052949.1 |
  2054051.4 |
  2055153.8 |
  2056256.2 |
  2057358.6 |
  2058460.9 |
  2059563.3 |
  2060665.7 |
  2061768.1 |
  2062870.4 |
  2063972.8 |
  2065075.2 |
  2066177.5 |####################
  2067279.9 |
  2068382.3 |####################
  2069484.7 |
  2070587.0 |
  2071689.4 |
  2072791.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_real_null_entry**: bridge=2513.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_anchor**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_dispatch**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_per_w**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_runtime_w**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_dispatch**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_per_w**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_runtime_w**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_zig_runtime_w**: bridge=312.2% of algo (FFI overhead may distort results)
