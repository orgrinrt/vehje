# abi_zig_entry (scatter)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_scatter_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_scatter_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_scatter_zig_null dominates: 78963% faster than the next best (abi_zig_entry_scatter_zig_dispatch)

abi_zig_entry_scatter_zig_null (2.65 us) leads abi_zig_entry_scatter_zig_dispatch (2.10 ms) by 78963%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_scatter_zig_null beats baseline by 100% (significant)

abi_zig_entry_scatter_zig_null is -2.09 ms (100%) faster than baseline abi_zig_entry_scatter_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_scatter_zig_tail_dispatch is an outlier: 1460.6x slower than the field

abi_zig_entry_scatter_zig_tail_dispatch (3.87 ms) is 1460.6x the fastest (2.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_scatter_zig_null} vs {abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_tail_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch} (78963% apart)

The field splits into a fast tier {abi_zig_entry_scatter_zig_null} and a slow tier {abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_tail_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch} with a 78963% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1460.6x the fastest

Fastest abi_zig_entry_scatter_zig_null (2.65 us) to slowest abi_zig_entry_scatter_zig_tail_dispatch (3.87 ms): 1460.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_scatter_zig_null** at 2650.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1460.64x (fastest 2650.8 ns, slowest 3871936.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2143235ns | 2106963ns | 2091462ns | 2103779ns | 2228307ns | +2.01% |
| abi_zig_entry_scatter_zig_dispatch | 2097885ns | 2098450ns | 2076833ns | 2096860ns | 2109948ns | -0.14% |
| abi_zig_entry_scatter_zig_null | 4951ns | 4940ns | 4881ns | 4931ns | 5018ns | -99.76% |
| abi_zig_entry_scatter_zig_per_w_set | 2099745ns | 2100447ns | 2088143ns | 2099465ns | 2105964ns | -0.06% |
| abi_zig_entry_scatter_zig_runtime_w | 2100914ns | 2099194ns | 2096102ns | 2098559ns | 2106852ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3875717ns | 3874906ns | 3863557ns | 3873946ns | 3884452ns | +84.48% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3941415ns | 3872329ns | 3865013ns | 3870087ns | 4086607ns | +87.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2140530ns | 2088688ns | 2225454ns | +2.01% | 0.000 |
| abi_zig_entry_scatter_zig_dispatch | 2095295ns | 2074329ns | 2107288ns | -0.14% | 0.000 |
| abi_zig_entry_scatter_zig_null | 2660ns | 2630ns | 2695ns | -99.87% | 0.048 |
| abi_zig_entry_scatter_zig_per_w_set | 2097073ns | 2085594ns | 2103245ns | -0.06% | 0.000 |
| abi_zig_entry_scatter_zig_runtime_w | 2098304ns | 2093553ns | 2104118ns | base | 0.000 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3872914ns | 3860976ns | 3881678ns | +84.57% | 0.000 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3938470ns | 3862335ns | 4083255ns | +87.70% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 183701.2 | 2109914.7 | 2140530.4 | n/a |
| abi_zig_entry_scatter_zig_dispatch | 175939.9 | 2096293.1 | 2095295.3 | 0 |
| abi_zig_entry_scatter_zig_null | 152687.3 | 2741.1 | 2660.4 | n/a |
| abi_zig_entry_scatter_zig_per_w_set | 176876.2 | 2094567.9 | 2097073.1 | 1 |
| abi_zig_entry_scatter_zig_runtime_w | 175256.9 | 2099150.4 | 2098303.6 | n/a |
| abi_zig_entry_scatter_zig_tail_dispatch | 189936.7 | 3877221.8 | 3872914.1 | n/a |
| abi_zig_entry_scatter_zig_tail_runtime_w | 202660.2 | 4075044.2 | 3938470.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_zig_entry_scatter_zig_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_null | 0.048 | 99.2% |
| abi_zig_entry_scatter_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2143235ns | 2143235ns | +2.01% |
| abi_zig_entry_scatter_zig_dispatch | 2097885ns | 2097885ns | -0.14% |
| abi_zig_entry_scatter_zig_null | 4951ns | 4951ns | -99.76% |
| abi_zig_entry_scatter_zig_per_w_set | 2099745ns | 2099745ns | -0.06% |
| abi_zig_entry_scatter_zig_runtime_w | 2100914ns | 2100914ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3875717ns | 3875717ns | +84.48% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3941415ns | 3941415ns | +87.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_runtime_w | 2096621ns | base | --- | [2094172, 2104118] | --- | --- | --- | --- |
| abi_zig_entry_scatter_zig_anchor | 2104392ns | no significant difference | [-4200, +126811]ns | [2091745, 2225454] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_dispatch | 2095831ns | no significant difference | [-12836, +5192]ns | [2082768, 2107288] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_null | 2651ns | -2093976.1ns (-99.9%) | [-2101437, -2091517]ns | [2635, 2695] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_per_w_set | 2097727ns | no significant difference | [-10831, +5916]ns | [2090247, 2103245] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3871936ns | +1773089.1ns (+84.6%) | [+1764669, +1786074]ns | [3865128, 3881678] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3869532ns | +1771927.2ns (+84.5%) | [+1766002, +1982571]ns | [3862623, 4083255] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_scatter_zig_runtime_w | abi_zig_entry_scatter_zig_anchor | abi_zig_entry_scatter_zig_dispatch | abi_zig_entry_scatter_zig_null | abi_zig_entry_scatter_zig_per_w_set | abi_zig_entry_scatter_zig_tail_dispatch | abi_zig_entry_scatter_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2107366ns | -0.2% | +0.3% | -99.9% | -0.6% | +83.6% | +84.2% |
| 2 | 2093553ns | -0.2% | -0.0% | -99.9% | +0.2% | +84.4% | +85.0% |
| 3 | 2094791ns | +0.0% | -0.2% | -99.9% | -0.4% | +85.1% | +84.5% |
| 4 | 2100869ns | +11.7% | -0.1% | -99.9% | +0.1% | +84.3% | +104.0% |
| 5 | 2096825ns | +0.4% | +0.2% | -99.9% | +0.3% | +84.7% | +84.2% |
| 6 | 2096417ns | +0.4% | -1.1% | -99.9% | +0.0% | +85.4% | +84.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | -0.220 | moderate- |
| abi_zig_entry_scatter_zig_dispatch | -0.166 | ok |
| abi_zig_entry_scatter_zig_null | -0.149 | ok |
| abi_zig_entry_scatter_zig_per_w_set | -0.170 | ok |
| abi_zig_entry_scatter_zig_runtime_w | -0.281 | moderate- |
| abi_zig_entry_scatter_zig_tail_dispatch | -0.040 | ok |
| abi_zig_entry_scatter_zig_tail_runtime_w | -0.259 | moderate- |

**Consistency summary:**

- **abi_zig_entry_scatter_zig_anchor**: won 2/6, lost 3/6
- **abi_zig_entry_scatter_zig_dispatch**: won 2/6, lost 2/6
- **abi_zig_entry_scatter_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_scatter_zig_per_w_set**: won 2/6, lost 2/6
- **abi_zig_entry_scatter_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_scatter_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 6594162.6ns | 2140530.4ns | 308.1% | HIGH |
| abi_zig_entry_scatter_zig_dispatch | 6530128.9ns | 2095295.3ns | 311.7% | HIGH |
| abi_zig_entry_scatter_zig_null | 300872.6ns | 2660.4ns | 11309.3% | HIGH |
| abi_zig_entry_scatter_zig_per_w_set | 6530794.2ns | 2097073.1ns | 311.4% | HIGH |
| abi_zig_entry_scatter_zig_runtime_w | 6537466.5ns | 2098303.6ns | 311.6% | HIGH |
| abi_zig_entry_scatter_zig_tail_dispatch | 11888894.7ns | 3872914.1ns | 307.0% | HIGH |
| abi_zig_entry_scatter_zig_tail_runtime_w | 12222665.0ns | 3938470.2ns | 310.3% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_scatter_zig_anchor (n=6, range 2088687.5-2225454.2 ns)
  2088687.5 |##########################
  2095525.8 |
  2102364.2 |########################################
  2109202.5 |
  2116040.8 |
  2122879.2 |
  2129717.5 |
  2136555.8 |
  2143394.2 |
  2150232.5 |
  2157070.9 |
  2163909.2 |
  2170747.5 |
  2177585.9 |
  2184424.2 |
  2191262.5 |
  2198100.9 |
  2204939.2 |
  2211777.5 |
  2218615.9 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_dispatch (n=6, range 2074329.2-2107287.5 ns)
  2074329.2 |########################################
  2075977.1 |
  2077625.0 |
  2079272.9 |
  2080920.9 |
  2082568.8 |
  2084216.7 |
  2085864.6 |
  2087512.5 |
  2089160.4 |
  2090808.4 |########################################
  2092456.3 |########################################
  2094104.2 |
  2095752.1 |
  2097400.0 |########################################
  2099047.9 |########################################
  2100695.8 |
  2102343.8 |
  2103991.7 |
  2105639.6 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_null (n=6, range 2630.4-2694.9 ns)
   2630.4 |####################
   2633.6 |
   2636.9 |
   2640.1 |########################################
   2643.3 |
   2646.5 |
   2649.8 |
   2653.0 |
   2656.2 |
   2659.4 |####################
   2662.7 |
   2665.9 |####################
   2669.1 |
   2672.4 |
   2675.6 |
   2678.8 |
   2682.0 |
   2685.3 |
   2688.5 |
   2691.7 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_per_w_set (n=6, range 2085593.7-2103244.6 ns)
  2085593.7 |########################################
  2086476.2 |
  2087358.8 |
  2088241.3 |
  2089123.9 |
  2090006.4 |
  2090889.0 |
  2091771.5 |
  2092654.1 |
  2093536.6 |
  2094419.1 |########################################
  2095301.7 |
  2096184.2 |
  2097066.8 |########################################
  2097949.3 |########################################
  2098831.9 |
  2099714.4 |
  2100597.0 |
  2101479.5 |
  2102362.1 |########################################
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_runtime_w (n=6, range 2093553.3-2104117.5 ns)
  2093553.3 |########################################
  2094081.5 |
  2094609.7 |########################################
  2095137.9 |
  2095666.1 |
  2096194.4 |########################################
  2096722.6 |########################################
  2097250.8 |
  2097779.0 |
  2098307.2 |
  2098835.4 |
  2099363.6 |
  2099891.8 |
  2100420.0 |########################################
  2100948.2 |
  2101476.5 |
  2102004.7 |
  2102532.9 |
  2103061.1 |
  2103589.3 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_dispatch (n=6, range 3860976.2-3881677.7 ns)
  3860976.2 |########################################
  3862011.3 |
  3863046.4 |
  3864081.4 |
  3865116.5 |
  3866151.6 |
  3867186.7 |
  3868221.7 |
  3869256.8 |########################################
  3870291.9 |
  3871327.0 |########################################
  3872362.0 |########################################
  3873397.1 |
  3874432.2 |
  3875467.2 |
  3876502.3 |########################################
  3877537.4 |
  3878572.5 |
  3879607.6 |
  3880642.6 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_runtime_w (n=6, range 3862335.4-4083255.5 ns)
  3862335.4 |########################################
  3873381.4 |##########################
  3884427.4 |
  3895473.4 |
  3906519.4 |
  3917565.4 |
  3928611.4 |
  3939657.4 |
  3950703.4 |
  3961749.4 |
  3972795.4 |
  3983841.4 |
  3994887.4 |
  4005933.4 |
  4016979.4 |
  4028025.4 |
  4039071.4 |
  4050117.4 |
  4061163.4 |
  4072209.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_scatter_zig_anchor**: bridge=310.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_dispatch**: bridge=311.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_null**: bridge=11306.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_per_w_set**: bridge=311.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_runtime_w**: bridge=311.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_dispatch**: bridge=307.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_runtime_w**: bridge=306.7% of algo (FFI overhead may distort results)
