# abi_boundary_w (wideselect)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_wideselect_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_wideselect_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_wideselect_null_entry dominates: 41264% faster than the next best (abi_boundary_w_wideselect_zig_runtime_w)

abi_boundary_w_wideselect_null_entry (4.87 us) leads abi_boundary_w_wideselect_zig_runtime_w (2.01 ms) by 41264%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_wideselect_null_entry beats baseline by 100% (significant)

abi_boundary_w_wideselect_null_entry is -2.09 ms (100%) faster than baseline abi_boundary_w_wideselect_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_wideselect_soa_runtime_w is an outlier: 433.2x slower than the field

abi_boundary_w_wideselect_soa_runtime_w (2.11 ms) is 433.2x the fastest (4.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_wideselect_scalar_per_w shows alternating (throttle bounce) (autocorr -0.59)

abi_boundary_w_wideselect_scalar_per_w's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_wideselect_null_entry} vs {abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_soa_runtime_w} (41264% apart)

The field splits into a fast tier {abi_boundary_w_wideselect_null_entry} and a slow tier {abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_soa_runtime_w} with a 41264% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 433.2x the fastest

Fastest abi_boundary_w_wideselect_null_entry (4.87 us) to slowest abi_boundary_w_wideselect_soa_runtime_w (2.11 ms): 433.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_wideselect_null_entry** at 4866.6 ns median (-99.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 433.22x (fastest 4866.6 ns, slowest 2108327.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 7160ns | 7157ns | 6986ns | 7125ns | 7298ns | -99.66% |
| abi_boundary_w_wideselect_scalar_anchor | 2085684ns | 2085108ns | 2074562ns | 2083367ns | 2094720ns | -0.60% |
| abi_boundary_w_wideselect_scalar_dispatch | 2095086ns | 2093162ns | 2086586ns | 2092156ns | 2103731ns | -0.15% |
| abi_boundary_w_wideselect_scalar_per_w | 2089609ns | 2090278ns | 2079943ns | 2088112ns | 2096688ns | -0.42% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2098319ns | 2098503ns | 2093859ns | 2097699ns | 2101479ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 2100800ns | 2100981ns | 2078950ns | 2096032ns | 2118877ns | +0.12% |
| abi_boundary_w_wideselect_soa_per_w | 2091690ns | 2093089ns | 2075654ns | 2088916ns | 2103868ns | -0.32% |
| abi_boundary_w_wideselect_soa_runtime_w | 2108292ns | 2111657ns | 2085922ns | 2107885ns | 2120088ns | +0.48% |
| abi_boundary_w_wideselect_zig_runtime_w | 2017661ns | 2016311ns | 2013403ns | 2015343ns | 2023267ns | -3.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 4884ns | 4783ns | 4971ns | -99.77% | 0.000 |
| abi_boundary_w_wideselect_scalar_anchor | 2082593ns | 2071768ns | 2091472ns | -0.59% | 0.000 |
| abi_boundary_w_wideselect_scalar_dispatch | 2091871ns | 2083785ns | 2100283ns | -0.15% | 0.000 |
| abi_boundary_w_wideselect_scalar_per_w | 2086533ns | 2077078ns | 2093587ns | -0.41% | 0.000 |
| abi_boundary_w_wideselect_scalar_runtime_w | 2095050ns | 2090732ns | 2098098ns | base | 0.000 |
| abi_boundary_w_wideselect_soa_dispatch | 2097417ns | 2075785ns | 2115131ns | +0.11% | 0.000 |
| abi_boundary_w_wideselect_soa_per_w | 2088539ns | 2072964ns | 2100291ns | -0.31% | 0.000 |
| abi_boundary_w_wideselect_soa_runtime_w | 2104822ns | 2083037ns | 2116211ns | +0.47% | 0.000 |
| abi_boundary_w_wideselect_zig_runtime_w | 2014296ns | 2009765ns | 2019768ns | -3.85% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 28844.4 | 4920.6 | 4883.8 | n/a |
| abi_boundary_w_wideselect_scalar_anchor | 56549.4 | 2084906.4 | 2082593.4 | n/a |
| abi_boundary_w_wideselect_scalar_dispatch | 60811.9 | 2090573.7 | 2091870.8 | n/a |
| abi_boundary_w_wideselect_scalar_per_w | 62665.8 | 2088283.2 | 2086532.8 | n/a |
| abi_boundary_w_wideselect_scalar_runtime_w | 65028.8 | 2096757.1 | 2095050.5 | n/a |
| abi_boundary_w_wideselect_soa_dispatch | 66403.7 | 2101387.7 | 2097417.2 | n/a |
| abi_boundary_w_wideselect_soa_per_w | 62735.0 | 2091683.4 | 2088538.5 | n/a |
| abi_boundary_w_wideselect_soa_runtime_w | 70835.8 | 2102989.3 | 2104822.3 | n/a |
| abi_boundary_w_wideselect_zig_runtime_w | 224476.4 | 2016903.4 | 2014295.5 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_boundary_w_wideselect_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.000 | 98.3% |
| abi_boundary_w_wideselect_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_wideselect_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_wideselect_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 7160ns | 7160ns | -99.66% |
| abi_boundary_w_wideselect_scalar_anchor | 2085684ns | 2085684ns | -0.60% |
| abi_boundary_w_wideselect_scalar_dispatch | 2095086ns | 2095086ns | -0.15% |
| abi_boundary_w_wideselect_scalar_per_w | 2089609ns | 2089609ns | -0.42% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2098319ns | 2098319ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 2100800ns | 2100800ns | +0.12% |
| abi_boundary_w_wideselect_soa_per_w | 2091690ns | 2091690ns | -0.32% |
| abi_boundary_w_wideselect_soa_runtime_w | 2108292ns | 2108292ns | +0.48% |
| abi_boundary_w_wideselect_zig_runtime_w | 2017661ns | 2017661ns | -3.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_scalar_runtime_w | 2095270ns | base | --- | [2091784, 2098098] | --- | --- | --- | --- |
| abi_boundary_w_wideselect_null_entry | 4867ns | -2090340.6ns (-99.8%) | [-2093195, -2086964]ns | [4814, 4971] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_boundary_w_wideselect_scalar_anchor | 2081913ns | -12399.2ns (-0.6%) | [-23703, -1269]ns | [2074395, 2091472] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 2089842ns | no significant difference | [-10451, +3811]ns | [2085487, 2100283] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_wideselect_scalar_per_w | 2087059ns | no significant difference | [-18995, +1322]ns | [2078952, 2093587] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_dispatch | 2097486ns | no significant difference | [-17794, +20668]ns | [2079635, 2115131] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_wideselect_soa_per_w | 2089949ns | no significant difference | [-20563, +5067]ns | [2075375, 2100291] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_runtime_w | 2108327ns | no significant difference | [-6010, +24427]ns | [2089929, 2116211] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_wideselect_zig_runtime_w | 2013063ns | -84884.6ns (-4.1%) | [-85364, -72016]ns | [2010056, 2019768] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_wideselect_scalar_runtime_w | abi_boundary_w_wideselect_null_entry | abi_boundary_w_wideselect_scalar_anchor | abi_boundary_w_wideselect_scalar_dispatch | abi_boundary_w_wideselect_scalar_per_w | abi_boundary_w_wideselect_soa_dispatch | abi_boundary_w_wideselect_soa_per_w | abi_boundary_w_wideselect_soa_runtime_w | abi_boundary_w_wideselect_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2090732ns | -99.8% | +0.1% | -0.1% | +0.2% | +0.2% | +0.6% | +1.1% | -3.3% |
| 2 | 2100107ns | -99.8% | -1.3% | +0.1% | -0.9% | -0.8% | -0.1% | +0.4% | -4.0% |
| 3 | 2096089ns | -99.8% | -0.9% | -0.4% | -0.7% | +1.1% | -0.9% | +0.0% | -4.1% |
| 4 | 2094751ns | -99.8% | -0.2% | -0.2% | -0.1% | -0.9% | -0.3% | +0.6% | -4.1% |
| 5 | 2095788ns | -99.8% | -0.5% | -0.6% | -0.9% | +0.2% | -1.1% | -0.6% | -4.0% |
| 6 | 2092836ns | -99.8% | -0.7% | +0.3% | -0.0% | +0.9% | -0.1% | +1.3% | -3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | -0.325 | moderate- |
| abi_boundary_w_wideselect_scalar_anchor | -0.254 | moderate- |
| abi_boundary_w_wideselect_scalar_dispatch | -0.448 | moderate- |
| abi_boundary_w_wideselect_scalar_per_w | -0.589 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_scalar_runtime_w | -0.369 | moderate- |
| abi_boundary_w_wideselect_soa_dispatch | -0.572 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_soa_per_w | -0.027 | ok |
| abi_boundary_w_wideselect_soa_runtime_w | -0.463 | moderate- |
| abi_boundary_w_wideselect_zig_runtime_w | 0.209 | moderate+ |

**Consistency summary:**

- **abi_boundary_w_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_scalar_anchor**: won 5/6, lost 1/6
- **abi_boundary_w_wideselect_scalar_dispatch**: won 3/6, lost 1/6
- **abi_boundary_w_wideselect_scalar_per_w**: won 3/6, lost 1/6
- **abi_boundary_w_wideselect_soa_dispatch**: won 2/6, lost 4/6
- **abi_boundary_w_wideselect_soa_per_w**: won 5/6, lost 1/6
- **abi_boundary_w_wideselect_soa_runtime_w**: won 1/6, lost 4/6
- **abi_boundary_w_wideselect_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 126328.6ns | 4883.8ns | 2586.7% | HIGH |
| abi_boundary_w_wideselect_scalar_anchor | 6313008.3ns | 2082593.4ns | 303.1% | HIGH |
| abi_boundary_w_wideselect_scalar_dispatch | 6334823.6ns | 2091870.8ns | 302.8% | HIGH |
| abi_boundary_w_wideselect_scalar_per_w | 6327913.6ns | 2086532.8ns | 303.3% | HIGH |
| abi_boundary_w_wideselect_scalar_runtime_w | 6359342.7ns | 2095050.5ns | 303.5% | HIGH |
| abi_boundary_w_wideselect_soa_dispatch | 6368738.7ns | 2097417.2ns | 303.6% | HIGH |
| abi_boundary_w_wideselect_soa_per_w | 6335436.7ns | 2088538.5ns | 303.3% | HIGH |
| abi_boundary_w_wideselect_soa_runtime_w | 6379650.4ns | 2104822.3ns | 303.1% | HIGH |
| abi_boundary_w_wideselect_zig_runtime_w | 6358036.1ns | 2014295.5ns | 315.6% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_wideselect_null_entry (n=6, range 4783.3-4971.0 ns)
   4783.3 |########################################
   4792.7 |
   4802.1 |
   4811.5 |
   4820.9 |
   4830.2 |
   4839.6 |########################################
   4849.0 |########################################
   4858.4 |
   4867.8 |
   4877.2 |########################################
   4886.6 |
   4895.9 |
   4905.3 |
   4914.7 |
   4924.1 |
   4933.5 |
   4942.9 |
   4952.3 |
   4961.7 |########################################
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_anchor (n=6, range 2071768.3-2091472.5 ns)
  2071768.3 |########################################
  2072753.5 |
  2073738.7 |
  2074723.9 |
  2075709.1 |
  2076694.4 |########################################
  2077679.6 |########################################
  2078664.8 |
  2079650.0 |
  2080635.2 |
  2081620.4 |
  2082605.6 |
  2083590.8 |
  2084576.0 |
  2085561.2 |########################################
  2086546.4 |
  2087531.7 |
  2088516.9 |
  2089502.1 |########################################
  2090487.3 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_dispatch (n=6, range 2083785.0-2100283.1 ns)
  2083785.0 |########################################
  2084609.9 |
  2085434.8 |
  2086259.7 |
  2087084.6 |########################################
  2087909.5 |
  2088734.4 |########################################
  2089559.3 |########################################
  2090384.2 |
  2091209.1 |
  2092034.1 |
  2092859.0 |
  2093683.9 |
  2094508.8 |
  2095333.7 |
  2096158.6 |
  2096983.5 |
  2097808.4 |
  2098633.3 |########################################
  2099458.2 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_per_w (n=6, range 2077078.3-2093586.9 ns)
  2077078.3 |########################################
  2077903.7 |
  2078729.2 |
  2079554.6 |
  2080380.0 |########################################
  2081205.4 |
  2082030.9 |########################################
  2082856.3 |
  2083681.7 |
  2084507.1 |
  2085332.6 |
  2086158.0 |
  2086983.4 |
  2087808.9 |
  2088634.3 |
  2089459.7 |
  2090285.1 |
  2091110.6 |
  2091936.0 |########################################
  2092761.4 |########################################
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_runtime_w (n=6, range 2090731.7-2098097.9 ns)
  2090731.7 |########################################
  2091100.0 |
  2091468.3 |
  2091836.6 |
  2092204.9 |
  2092573.2 |########################################
  2092941.6 |
  2093309.9 |
  2093678.2 |
  2094046.5 |
  2094414.8 |########################################
  2094783.1 |
  2095151.4 |
  2095519.7 |########################################
  2095888.0 |########################################
  2096256.3 |
  2096624.7 |
  2096993.0 |
  2097361.3 |
  2097729.6 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_dispatch (n=6, range 2075785.0-2115130.7 ns)
  2075785.0 |########################################
  2077752.3 |
  2079719.6 |
  2081686.8 |########################################
  2083654.1 |
  2085621.4 |
  2087588.7 |
  2089556.0 |
  2091523.3 |
  2093490.5 |########################################
  2095457.8 |
  2097425.1 |
  2099392.4 |########################################
  2101359.7 |
  2103327.0 |
  2105294.2 |
  2107261.5 |
  2109228.8 |########################################
  2111196.1 |
  2113163.4 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_per_w (n=6, range 2072963.7-2100291.2 ns)
  2072963.7 |########################################
  2074330.1 |
  2075696.5 |
  2077062.8 |########################################
  2078429.2 |
  2079795.6 |
  2081162.0 |
  2082528.3 |
  2083894.7 |
  2085261.1 |
  2086627.5 |
  2087993.9 |########################################
  2089360.2 |########################################
  2090726.6 |
  2092093.0 |
  2093459.4 |
  2094825.7 |
  2096192.1 |########################################
  2097558.5 |
  2098924.9 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_runtime_w (n=6, range 2083036.7-2116211.2 ns)
  2083036.7 |########################################
  2084695.4 |
  2086354.2 |
  2088012.9 |
  2089671.6 |
  2091330.3 |
  2092989.1 |
  2094647.8 |
  2096306.5 |########################################
  2097965.2 |
  2099624.0 |
  2101282.7 |
  2102941.4 |
  2104600.2 |
  2106258.9 |########################################
  2107917.6 |########################################
  2109576.3 |
  2111235.1 |
  2112893.8 |########################################
  2114552.5 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_zig_runtime_w (n=6, range 2009765.0-2019767.5 ns)
  2009765.0 |########################################
  2010265.1 |########################################
  2010765.2 |########################################
  2011265.4 |
  2011765.5 |
  2012265.6 |
  2012765.8 |
  2013265.9 |
  2013766.0 |
  2014266.1 |
  2014766.2 |########################################
  2015266.4 |
  2015766.5 |
  2016266.6 |
  2016766.8 |
  2017266.9 |
  2017767.0 |########################################
  2018267.1 |
  2018767.2 |
  2019267.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_wideselect_null_entry**: bridge=2598.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_anchor**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_dispatch**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_per_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_runtime_w**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_dispatch**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_per_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_runtime_w**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_zig_runtime_w**: bridge=315.7% of algo (FFI overhead may distort results)
