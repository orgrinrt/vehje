# abi_boundary_w (real)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_real_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_real_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_real_null_entry dominates: 51873% faster than the next best (abi_boundary_w_real_zig_runtime_w)

abi_boundary_w_real_null_entry (3.99 us) leads abi_boundary_w_real_zig_runtime_w (2.07 ms) by 51873%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_real_null_entry beats baseline by 100% (significant)

abi_boundary_w_real_null_entry is -2.13 ms (100%) faster than baseline abi_boundary_w_real_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_real_soa_runtime_w is an outlier: 537.0x slower than the field

abi_boundary_w_real_soa_runtime_w (2.14 ms) is 537.0x the fastest (3.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_real_scalar_per_w shows alternating (throttle bounce) (autocorr -0.64)

abi_boundary_w_real_scalar_per_w's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_real_null_entry} vs {abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_soa_runtime_w} (51873% apart)

The field splits into a fast tier {abi_boundary_w_real_null_entry} and a slow tier {abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_soa_runtime_w} with a 51873% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 537.0x the fastest

Fastest abi_boundary_w_real_null_entry (3.99 us) to slowest abi_boundary_w_real_soa_runtime_w (2.14 ms): 537.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_real_null_entry** at 3986.7 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 536.95x (fastest 3986.7 ns, slowest 2140632.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 6279ns | 6261ns | 5951ns | 6218ns | 6533ns | -99.71% |
| abi_boundary_w_real_scalar_anchor | 2138558ns | 2137293ns | 2128860ns | 2135172ns | 2148488ns | -0.02% |
| abi_boundary_w_real_scalar_dispatch | 2132447ns | 2133736ns | 2124990ns | 2132394ns | 2136255ns | -0.31% |
| abi_boundary_w_real_scalar_per_w | 2128761ns | 2132964ns | 2107818ns | 2129862ns | 2137581ns | -0.48% |
| abi_boundary_w_real_scalar_runtime_w | 2139082ns | 2137194ns | 2130531ns | 2136130ns | 2147785ns | base |
| abi_boundary_w_real_soa_dispatch | 2141148ns | 2139354ns | 2134698ns | 2138651ns | 2148119ns | +0.10% |
| abi_boundary_w_real_soa_per_w | 2132905ns | 2133059ns | 2125378ns | 2132868ns | 2136724ns | -0.29% |
| abi_boundary_w_real_soa_runtime_w | 2142028ns | 2143087ns | 2123629ns | 2141355ns | 2152238ns | +0.14% |
| abi_boundary_w_real_zig_runtime_w | 2073725ns | 2074608ns | 2063504ns | 2073969ns | 2078471ns | -3.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 4017ns | 3835ns | 4176ns | -99.81% | 0.001 |
| abi_boundary_w_real_scalar_anchor | 2136078ns | 2126417ns | 2145835ns | -0.02% | 0.000 |
| abi_boundary_w_real_scalar_dispatch | 2129955ns | 2122362ns | 2133807ns | -0.31% | 0.000 |
| abi_boundary_w_real_scalar_per_w | 2126264ns | 2105299ns | 2134983ns | -0.48% | 0.000 |
| abi_boundary_w_real_scalar_runtime_w | 2136560ns | 2128104ns | 2145279ns | base | 0.000 |
| abi_boundary_w_real_soa_dispatch | 2138618ns | 2132238ns | 2145425ns | +0.10% | 0.000 |
| abi_boundary_w_real_soa_per_w | 2130413ns | 2122855ns | 2134239ns | -0.29% | 0.000 |
| abi_boundary_w_real_soa_runtime_w | 2139556ns | 2121202ns | 2149763ns | +0.14% | 0.000 |
| abi_boundary_w_real_zig_runtime_w | 2071119ns | 2060839ns | 2075831ns | -3.06% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 26844.0 | 4144.8 | 4016.8 | n/a |
| abi_boundary_w_real_scalar_anchor | 36399.5 | 2134604.9 | 2136078.1 | 1 |
| abi_boundary_w_real_scalar_dispatch | 36306.5 | 2130440.0 | 2129954.9 | 0 |
| abi_boundary_w_real_scalar_per_w | 37496.7 | 2127109.1 | 2126263.8 | 0 |
| abi_boundary_w_real_scalar_runtime_w | 35782.8 | 2138936.9 | 2136559.9 | n/a |
| abi_boundary_w_real_soa_dispatch | 35948.1 | 2137800.5 | 2138617.5 | n/a |
| abi_boundary_w_real_soa_per_w | 37704.9 | 2131121.3 | 2130412.6 | 0 |
| abi_boundary_w_real_soa_runtime_w | 35933.2 | 2138984.1 | 2139556.0 | n/a |
| abi_boundary_w_real_zig_runtime_w | 177495.3 | 2070694.8 | 2071118.9 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.001 | 96.2% |
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
| abi_boundary_w_real_null_entry | 6279ns | 6279ns | -99.71% |
| abi_boundary_w_real_scalar_anchor | 2138558ns | 2138558ns | -0.02% |
| abi_boundary_w_real_scalar_dispatch | 2132447ns | 2132447ns | -0.31% |
| abi_boundary_w_real_scalar_per_w | 2128761ns | 2128761ns | -0.48% |
| abi_boundary_w_real_scalar_runtime_w | 2139082ns | 2139082ns | base |
| abi_boundary_w_real_soa_dispatch | 2141148ns | 2141148ns | +0.10% |
| abi_boundary_w_real_soa_per_w | 2132905ns | 2132905ns | -0.29% |
| abi_boundary_w_real_soa_runtime_w | 2142028ns | 2142028ns | +0.14% |
| abi_boundary_w_real_zig_runtime_w | 2073725ns | 2073725ns | -3.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_real_scalar_runtime_w | 2134606ns | base | --- | [2129795, 2145279] | --- | --- | --- | --- |
| abi_boundary_w_real_null_entry | 3987ns | -2130567.5ns (-99.8%) | [-2141391, -2125670]ns | [3887, 4176] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_boundary_w_real_scalar_anchor | 2134922ns | no significant difference | [-11782, +8087]ns | [2127478, 2145835] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_real_scalar_dispatch | 2131206ns | no significant difference | [-14073, +2826]ns | [2124852, 2133807] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_real_scalar_per_w | 2130504ns | no significant difference | [-25207, +3929]ns | [2113304, 2134983] | no | 0.5833 | 0.2188 | 0 |
| abi_boundary_w_real_soa_dispatch | 2136940ns | no significant difference | [-11791, +12004]ns | [2133488, 2145425] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_real_soa_per_w | 2130703ns | no significant difference | [-15078, +1581]ns | [2126296, 2134239] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_real_soa_runtime_w | 2140633ns | no significant difference | [-7261, +10667]ns | [2128272, 2149763] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_real_zig_runtime_w | 2071987ns | -64877.9ns (-3.0%) | [-74604, -56841]ns | [2065539, 2075831] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_real_scalar_runtime_w | abi_boundary_w_real_null_entry | abi_boundary_w_real_scalar_anchor | abi_boundary_w_real_scalar_dispatch | abi_boundary_w_real_scalar_per_w | abi_boundary_w_real_soa_dispatch | abi_boundary_w_real_soa_per_w | abi_boundary_w_real_soa_runtime_w | abi_boundary_w_real_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2147392ns | -99.8% | +0.4% | -0.8% | -0.7% | -0.6% | -0.8% | -0.2% | -3.6% |
| 2 | 2133858ns | -99.8% | +0.1% | +0.1% | -1.3% | +0.2% | -0.1% | +0.2% | -2.6% |
| 3 | 2128104ns | -99.8% | +0.4% | +0.2% | +0.5% | +0.3% | +0.1% | +0.3% | -3.2% |
| 4 | 2143165ns | -99.8% | -0.8% | -0.6% | -1.0% | -0.5% | -0.3% | +0.3% | -3.4% |
| 5 | 2135355ns | -99.8% | -0.3% | -0.4% | -0.2% | +0.7% | -0.6% | +0.7% | -2.9% |
| 6 | 2131486ns | -99.8% | +0.2% | -0.4% | -0.1% | +0.4% | +0.0% | -0.5% | -2.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_real_null_entry | -0.136 | ok |
| abi_boundary_w_real_scalar_anchor | 0.112 | ok |
| abi_boundary_w_real_scalar_dispatch | 0.393 | moderate+ |
| abi_boundary_w_real_scalar_per_w | -0.643 | HIGH- (thermal bounce) |
| abi_boundary_w_real_scalar_runtime_w | -0.240 | moderate- |
| abi_boundary_w_real_soa_dispatch | -0.159 | ok |
| abi_boundary_w_real_soa_per_w | -0.626 | HIGH- (thermal bounce) |
| abi_boundary_w_real_soa_runtime_w | -0.209 | moderate- |
| abi_boundary_w_real_zig_runtime_w | -0.440 | moderate- |

**Consistency summary:**

- **abi_boundary_w_real_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_real_scalar_anchor**: won 2/6, lost 3/6
- **abi_boundary_w_real_scalar_dispatch**: won 4/6, lost 1/6
- **abi_boundary_w_real_scalar_per_w**: won 4/6, lost 1/6
- **abi_boundary_w_real_soa_dispatch**: won 2/6, lost 4/6
- **abi_boundary_w_real_soa_per_w**: won 4/6, lost 1/6
- **abi_boundary_w_real_soa_runtime_w**: won 2/6, lost 4/6
- **abi_boundary_w_real_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 122090.6ns | 4016.8ns | 3039.5% | HIGH |
| abi_boundary_w_real_scalar_anchor | 6446055.4ns | 2136078.1ns | 301.8% | HIGH |
| abi_boundary_w_real_scalar_dispatch | 6430808.9ns | 2129954.9ns | 301.9% | HIGH |
| abi_boundary_w_real_scalar_per_w | 6419283.3ns | 2126263.8ns | 301.9% | HIGH |
| abi_boundary_w_real_scalar_runtime_w | 6449927.6ns | 2136559.9ns | 301.9% | HIGH |
| abi_boundary_w_real_soa_dispatch | 6450626.8ns | 2138617.5ns | 301.6% | HIGH |
| abi_boundary_w_real_soa_per_w | 6432150.8ns | 2130412.6ns | 301.9% | HIGH |
| abi_boundary_w_real_soa_runtime_w | 6453869.7ns | 2139556.0ns | 301.6% | HIGH |
| abi_boundary_w_real_zig_runtime_w | 6456815.7ns | 2071118.9ns | 311.8% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_real_null_entry (n=6, range 3834.6-4176.5 ns)
   3834.6 |########################################
   3851.7 |
   3868.8 |
   3885.9 |
   3903.0 |
   3920.1 |
   3937.2 |########################################
   3954.3 |########################################
   3971.4 |
   3988.5 |
   4005.6 |########################################
   4022.6 |
   4039.7 |
   4056.8 |########################################
   4073.9 |
   4091.0 |
   4108.1 |
   4125.2 |
   4142.3 |
   4159.4 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_anchor (n=6, range 2126417.1-2145834.8 ns)
  2126417.1 |####################
  2127388.0 |
  2128358.9 |####################
  2129329.8 |
  2130300.6 |
  2131271.5 |
  2132242.4 |
  2133213.3 |
  2134184.2 |########################################
  2135155.1 |
  2136126.0 |####################
  2137096.8 |
  2138067.7 |
  2139038.6 |
  2140009.5 |
  2140980.4 |
  2141951.3 |
  2142922.1 |
  2143893.0 |
  2144863.9 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_dispatch (n=6, range 2122362.1-2133806.8 ns)
  2122362.1 |####################
  2122934.3 |
  2123506.6 |
  2124078.8 |
  2124651.0 |
  2125223.3 |
  2125795.5 |
  2126367.8 |
  2126940.0 |####################
  2127512.2 |
  2128084.5 |
  2128656.7 |
  2129228.9 |
  2129801.2 |
  2130373.4 |
  2130945.7 |########################################
  2131517.9 |
  2132090.1 |####################
  2132662.4 |
  2133234.6 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_per_w (n=6, range 2105298.8-2134983.0 ns)
  2105298.8 |########################################
  2106783.0 |
  2108267.2 |
  2109751.4 |
  2111235.6 |
  2112719.8 |
  2114204.0 |
  2115688.3 |
  2117172.5 |
  2118656.7 |
  2120140.9 |########################################
  2121625.1 |
  2123109.3 |
  2124593.5 |
  2126077.7 |
  2127561.9 |
  2129046.1 |########################################
  2130530.3 |########################################
  2132014.5 |########################################
  2133498.7 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_runtime_w (n=6, range 2128103.8-2145278.5 ns)
  2128103.8 |########################################
  2128962.5 |
  2129821.3 |
  2130680.0 |########################################
  2131538.8 |
  2132397.5 |
  2133256.2 |########################################
  2134115.0 |
  2134973.7 |########################################
  2135832.4 |
  2136691.2 |
  2137549.9 |
  2138408.6 |
  2139267.4 |
  2140126.1 |
  2140984.9 |
  2141843.6 |
  2142702.3 |########################################
  2143561.1 |
  2144419.8 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_dispatch (n=6, range 2132238.3-2145424.6 ns)
  2132238.3 |########################################
  2132897.6 |
  2133556.9 |
  2134216.2 |########################################
  2134875.6 |########################################
  2135534.9 |
  2136194.2 |
  2136853.5 |
  2137512.8 |
  2138172.1 |
  2138831.5 |########################################
  2139490.8 |
  2140150.1 |########################################
  2140809.4 |
  2141468.7 |
  2142128.0 |
  2142787.3 |
  2143446.7 |
  2144106.0 |
  2144765.3 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_per_w (n=6, range 2122855.0-2134238.5 ns)
  2122855.0 |########################################
  2123424.2 |
  2123993.4 |
  2124562.5 |
  2125131.7 |
  2125700.9 |
  2126270.1 |
  2126839.2 |
  2127408.4 |
  2127977.6 |
  2128546.8 |
  2129116.0 |
  2129685.1 |########################################
  2130254.3 |########################################
  2130823.5 |########################################
  2131392.7 |
  2131961.8 |########################################
  2132531.0 |
  2133100.2 |
  2133669.4 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_runtime_w (n=6, range 2121202.1-2149763.3 ns)
  2121202.1 |########################################
  2122630.2 |
  2124058.2 |
  2125486.3 |
  2126914.3 |
  2128342.4 |
  2129770.5 |
  2131198.5 |
  2132626.6 |
  2134054.6 |########################################
  2135482.7 |
  2136910.8 |########################################
  2138338.8 |
  2139766.9 |
  2141194.9 |
  2142623.0 |########################################
  2144051.1 |
  2145479.1 |
  2146907.2 |
  2148335.2 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_zig_runtime_w (n=6, range 2060838.7-2075830.6 ns)
  2060838.7 |########################################
  2061588.3 |
  2062337.9 |
  2063087.5 |
  2063837.1 |
  2064586.7 |
  2065336.3 |
  2066085.9 |
  2066835.5 |
  2067585.1 |
  2068334.6 |
  2069084.2 |
  2069833.8 |########################################
  2070583.4 |########################################
  2071333.0 |
  2072082.6 |
  2072832.2 |########################################
  2073581.8 |########################################
  2074331.4 |
  2075081.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_real_null_entry**: bridge=3059.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_anchor**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_dispatch**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_per_w**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_runtime_w**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_dispatch**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_per_w**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_runtime_w**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_zig_runtime_w**: bridge=311.8% of algo (FFI overhead may distort results)
