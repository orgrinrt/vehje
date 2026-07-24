# abi_boundary_w (scatter)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_scatter_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_scatter_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_scatter_null_entry dominates: 43655% faster than the next best (abi_boundary_w_scatter_zig_runtime_w)

abi_boundary_w_scatter_null_entry (4.86 us) leads abi_boundary_w_scatter_zig_runtime_w (2.13 ms) by 43655%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_scatter_null_entry beats baseline by 100% (significant)

abi_boundary_w_scatter_null_entry is -2.16 ms (100%) faster than baseline abi_boundary_w_scatter_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_scatter_soa_runtime_w is an outlier: 445.5x slower than the field

abi_boundary_w_scatter_soa_runtime_w (2.17 ms) is 445.5x the fastest (4.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_scatter_soa_dispatch shows alternating (throttle bounce) (autocorr -0.58)

abi_boundary_w_scatter_soa_dispatch's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_scatter_null_entry} vs {abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_soa_runtime_w} (43655% apart)

The field splits into a fast tier {abi_boundary_w_scatter_null_entry} and a slow tier {abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_soa_runtime_w} with a 43655% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 445.5x the fastest

Fastest abi_boundary_w_scatter_null_entry (4.86 us) to slowest abi_boundary_w_scatter_soa_runtime_w (2.17 ms): 445.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_scatter_null_entry** at 4862.1 ns median (-99.8% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 445.54x (fastest 4862.1 ns, slowest 2166280.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 7101ns | 7111ns | 7044ns | 7097ns | 7136ns | -99.67% |
| abi_boundary_w_scatter_scalar_anchor | 2157210ns | 2157172ns | 2154770ns | 2156823ns | 2159012ns | -0.43% |
| abi_boundary_w_scatter_scalar_dispatch | 2160930ns | 2160343ns | 2157575ns | 2159522ns | 2164721ns | -0.25% |
| abi_boundary_w_scatter_scalar_per_w | 2159999ns | 2159602ns | 2156852ns | 2159359ns | 2162534ns | -0.30% |
| abi_boundary_w_scatter_scalar_runtime_w | 2166440ns | 2166688ns | 2159243ns | 2164499ns | 2172949ns | base |
| abi_boundary_w_scatter_soa_dispatch | 2157169ns | 2156404ns | 2153984ns | 2155881ns | 2160694ns | -0.43% |
| abi_boundary_w_scatter_soa_per_w | 2157994ns | 2157905ns | 2157320ns | 2157818ns | 2158596ns | -0.39% |
| abi_boundary_w_scatter_soa_runtime_w | 2167790ns | 2169128ns | 2155868ns | 2167097ns | 2174792ns | +0.06% |
| abi_boundary_w_scatter_zig_runtime_w | 2130504ns | 2130131ns | 2128128ns | 2129680ns | 2132927ns | -1.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 4863ns | 4834ns | 4885ns | -99.78% | 0.000 |
| abi_boundary_w_scatter_scalar_anchor | 2154452ns | 2152011ns | 2156260ns | -0.42% | 0.000 |
| abi_boundary_w_scatter_scalar_dispatch | 2158107ns | 2154746ns | 2161819ns | -0.25% | 0.000 |
| abi_boundary_w_scatter_scalar_per_w | 2157167ns | 2154085ns | 2159611ns | -0.30% | 0.000 |
| abi_boundary_w_scatter_scalar_runtime_w | 2163609ns | 2156608ns | 2169915ns | base | 0.000 |
| abi_boundary_w_scatter_soa_dispatch | 2154334ns | 2151415ns | 2157745ns | -0.43% | 0.000 |
| abi_boundary_w_scatter_soa_per_w | 2155242ns | 2154545ns | 2155918ns | -0.39% | 0.000 |
| abi_boundary_w_scatter_soa_runtime_w | 2164859ns | 2153025ns | 2171543ns | +0.06% | 0.000 |
| abi_boundary_w_scatter_zig_runtime_w | 2127710ns | 2125320ns | 2130118ns | -1.66% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 26700.1 | 4915.9 | 4863.5 | n/a |
| abi_boundary_w_scatter_scalar_anchor | 45463.3 | 2154546.9 | 2154452.4 | n/a |
| abi_boundary_w_scatter_scalar_dispatch | 45884.5 | 2157872.7 | 2158106.7 | n/a |
| abi_boundary_w_scatter_scalar_per_w | 46979.6 | 2156641.5 | 2157166.6 | 0 |
| abi_boundary_w_scatter_scalar_runtime_w | 46341.2 | 2163887.2 | 2163608.7 | n/a |
| abi_boundary_w_scatter_soa_dispatch | 44416.1 | 2153479.1 | 2154334.0 | n/a |
| abi_boundary_w_scatter_soa_per_w | 44660.1 | 2156018.0 | 2155242.1 | n/a |
| abi_boundary_w_scatter_soa_runtime_w | 47469.8 | 2166008.4 | 2164858.8 | n/a |
| abi_boundary_w_scatter_zig_runtime_w | 190478.3 | 2128310.9 | 2127709.8 | 4 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_boundary_w_scatter_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.000 | 99.4% |
| abi_boundary_w_scatter_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_scatter_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_scatter_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_scatter_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_scatter_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_scatter_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_scatter_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_scatter_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 7101ns | 7101ns | -99.67% |
| abi_boundary_w_scatter_scalar_anchor | 2157210ns | 2157210ns | -0.43% |
| abi_boundary_w_scatter_scalar_dispatch | 2160930ns | 2160930ns | -0.25% |
| abi_boundary_w_scatter_scalar_per_w | 2159999ns | 2159999ns | -0.30% |
| abi_boundary_w_scatter_scalar_runtime_w | 2166440ns | 2166440ns | base |
| abi_boundary_w_scatter_soa_dispatch | 2157169ns | 2157169ns | -0.43% |
| abi_boundary_w_scatter_soa_per_w | 2157994ns | 2157994ns | -0.39% |
| abi_boundary_w_scatter_soa_runtime_w | 2167790ns | 2167790ns | +0.06% |
| abi_boundary_w_scatter_zig_runtime_w | 2130504ns | 2130504ns | -1.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_scalar_runtime_w | 2163914ns | base | --- | [2156998, 2169915] | --- | --- | --- | --- |
| abi_boundary_w_scatter_null_entry | 4862ns | -2159047.2ns (-99.8%) | [-2165072, -2152116]ns | [4843, 4885] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_anchor | 2154446ns | -9164.8ns (-0.4%) | [-15098, -3206]ns | [2152652, 2156260] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_dispatch | 2157587ns | no significant difference | [-15001, +4096]ns | [2154914, 2161819] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 2156835ns | -4742.9ns (-0.2%) | [-13760, -824]ns | [2155054, 2159611] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_dispatch | 2153594ns | -6168.5ns (-0.3%) | [-17071, -4584]ns | [2151663, 2157745] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_per_w | 2155140ns | -8773.7ns (-0.4%) | [-15246, -1080]ns | [2154669, 2155918] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_runtime_w | 2166280ns | no significant difference | [-6066, +9979]ns | [2156753, 2171543] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_scatter_zig_runtime_w | 2127412ns | -36062.1ns (-1.7%) | [-40866, -30768]ns | [2125600, 2130118] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_scatter_scalar_runtime_w | abi_boundary_w_scatter_null_entry | abi_boundary_w_scatter_scalar_anchor | abi_boundary_w_scatter_scalar_dispatch | abi_boundary_w_scatter_scalar_per_w | abi_boundary_w_scatter_soa_dispatch | abi_boundary_w_scatter_soa_per_w | abi_boundary_w_scatter_soa_runtime_w | abi_boundary_w_scatter_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2156608ns | -99.8% | -0.1% | +0.0% | -0.0% | -0.2% | -0.0% | +0.2% | -1.4% |
| 2 | 2164071ns | -99.8% | -0.6% | -0.3% | -0.3% | -0.2% | -0.4% | -0.0% | -1.7% |
| 3 | 2157387ns | -99.8% | -0.2% | +0.4% | -0.0% | -0.2% | -0.1% | +0.7% | -1.5% |
| 4 | 2168816ns | -99.8% | -0.6% | -0.6% | -0.7% | -0.7% | -0.7% | +0.0% | -2.0% |
| 5 | 2163756ns | -99.8% | -0.3% | -0.2% | -0.1% | -0.3% | -0.4% | -0.5% | -1.7% |
| 6 | 2171014ns | -99.8% | -0.8% | -0.7% | -0.6% | -0.9% | -0.7% | -0.1% | -1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_scatter_null_entry | -0.066 | ok |
| abi_boundary_w_scatter_scalar_anchor | 0.282 | moderate+ |
| abi_boundary_w_scatter_scalar_dispatch | -0.352 | moderate- |
| abi_boundary_w_scatter_scalar_per_w | -0.201 | moderate- |
| abi_boundary_w_scatter_scalar_runtime_w | -0.216 | moderate- |
| abi_boundary_w_scatter_soa_dispatch | -0.576 | HIGH- (thermal bounce) |
| abi_boundary_w_scatter_soa_per_w | -0.314 | moderate- |
| abi_boundary_w_scatter_soa_runtime_w | -0.282 | moderate- |
| abi_boundary_w_scatter_zig_runtime_w | 0.114 | ok |

**Consistency summary:**

- **abi_boundary_w_scatter_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_scalar_anchor**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_scalar_dispatch**: won 4/6, lost 1/6
- **abi_boundary_w_scatter_scalar_per_w**: won 4/6, lost 0/6
- **abi_boundary_w_scatter_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_per_w**: won 4/6, lost 0/6
- **abi_boundary_w_scatter_soa_runtime_w**: won 1/6, lost 2/6
- **abi_boundary_w_scatter_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 124047.4ns | 4863.5ns | 2550.6% | HIGH |
| abi_boundary_w_scatter_scalar_anchor | 6513052.4ns | 2154452.4ns | 302.3% | HIGH |
| abi_boundary_w_scatter_scalar_dispatch | 6524679.9ns | 2158106.7ns | 302.3% | HIGH |
| abi_boundary_w_scatter_scalar_per_w | 6519350.1ns | 2157166.6ns | 302.2% | HIGH |
| abi_boundary_w_scatter_scalar_runtime_w | 6540869.6ns | 2163608.7ns | 302.3% | HIGH |
| abi_boundary_w_scatter_soa_dispatch | 6509534.2ns | 2154334.0ns | 302.2% | HIGH |
| abi_boundary_w_scatter_soa_per_w | 6511740.9ns | 2155242.1ns | 302.1% | HIGH |
| abi_boundary_w_scatter_soa_runtime_w | 6548321.1ns | 2164858.8ns | 302.5% | HIGH |
| abi_boundary_w_scatter_zig_runtime_w | 6644022.7ns | 2127709.8ns | 312.3% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_scatter_null_entry (n=6, range 4833.8-4885.4 ns)
   4833.8 |########################################
   4836.4 |
   4839.0 |
   4841.5 |
   4844.1 |
   4846.7 |
   4849.3 |
   4851.9 |########################################
   4854.4 |
   4857.0 |
   4859.6 |########################################
   4862.2 |########################################
   4864.8 |
   4867.3 |########################################
   4869.9 |
   4872.5 |
   4875.1 |
   4877.7 |
   4880.2 |
   4882.8 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_anchor (n=6, range 2152011.2-2156259.8 ns)
  2152011.2 |########################################
  2152223.6 |
  2152436.1 |
  2152648.5 |
  2152860.9 |
  2153073.3 |
  2153285.8 |########################################
  2153498.2 |
  2153710.6 |
  2153923.0 |
  2154135.5 |########################################
  2154347.9 |
  2154560.3 |########################################
  2154772.8 |
  2154985.2 |########################################
  2155197.6 |
  2155410.0 |
  2155622.5 |
  2155834.9 |
  2156047.3 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_dispatch (n=6, range 2154746.2-2161819.0 ns)
  2154746.2 |########################################
  2155099.8 |
  2155453.5 |
  2155807.1 |
  2156160.8 |
  2156514.4 |
  2156868.0 |####################
  2157221.7 |
  2157575.3 |
  2157929.0 |####################
  2158282.6 |####################
  2158636.2 |
  2158989.9 |
  2159343.5 |
  2159697.2 |
  2160050.8 |
  2160404.4 |
  2160758.1 |
  2161111.7 |
  2161465.4 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_per_w (n=6, range 2154084.6-2159610.6 ns)
  2154084.6 |########################################
  2154360.9 |
  2154637.2 |
  2154913.5 |
  2155189.8 |
  2155466.1 |
  2155742.4 |
  2156018.7 |########################################
  2156295.0 |########################################
  2156571.3 |
  2156847.6 |
  2157123.9 |########################################
  2157400.2 |
  2157676.5 |
  2157952.8 |########################################
  2158229.1 |
  2158505.4 |
  2158781.7 |
  2159058.0 |
  2159334.3 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_runtime_w (n=6, range 2156607.9-2169915.0 ns)
  2156607.9 |########################################
  2157273.3 |########################################
  2157938.6 |
  2158604.0 |
  2159269.3 |
  2159934.7 |
  2160600.0 |
  2161265.4 |
  2161930.7 |
  2162596.1 |
  2163261.5 |########################################
  2163926.8 |########################################
  2164592.2 |
  2165257.5 |
  2165922.9 |
  2166588.2 |
  2167253.6 |
  2167918.9 |
  2168584.3 |########################################
  2169249.6 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_dispatch (n=6, range 2151415.4-2157745.0 ns)
  2151415.4 |########################################
  2151731.9 |########################################
  2152048.4 |
  2152364.8 |
  2152681.3 |
  2152997.8 |
  2153314.3 |########################################
  2153630.8 |########################################
  2153947.2 |
  2154263.7 |
  2154580.2 |
  2154896.7 |
  2155213.2 |
  2155529.6 |
  2155846.1 |
  2156162.6 |
  2156479.1 |########################################
  2156795.6 |
  2157112.0 |
  2157428.5 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_per_w (n=6, range 2154545.4-2155917.7 ns)
  2154545.4 |########################################
  2154614.0 |
  2154682.6 |
  2154751.2 |########################################
  2154819.9 |
  2154888.5 |
  2154957.1 |
  2155025.7 |########################################
  2155094.3 |
  2155162.9 |########################################
  2155231.5 |
  2155300.2 |
  2155368.8 |
  2155437.4 |
  2155506.0 |
  2155574.6 |
  2155643.2 |
  2155711.9 |########################################
  2155780.5 |
  2155849.1 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_runtime_w (n=6, range 2153025.0-2171542.8 ns)
  2153025.0 |####################
  2153950.9 |
  2154876.8 |
  2155802.7 |
  2156728.5 |
  2157654.4 |
  2158580.3 |
  2159506.2 |
  2160432.1 |####################
  2161358.0 |
  2162283.9 |####################
  2163209.8 |
  2164135.6 |
  2165061.5 |
  2165987.4 |
  2166913.3 |
  2167839.2 |
  2168765.1 |########################################
  2169691.0 |
  2170616.9 |
  (0 below, 1 above range)

abi_boundary_w_scatter_zig_runtime_w (n=6, range 2125319.6-2130117.5 ns)
  2125319.6 |########################################
  2125559.5 |
  2125799.4 |########################################
  2126039.3 |
  2126279.2 |
  2126519.1 |
  2126759.0 |
  2126998.9 |########################################
  2127238.8 |
  2127478.7 |########################################
  2127718.5 |
  2127958.4 |########################################
  2128198.3 |
  2128438.2 |
  2128678.1 |
  2128918.0 |
  2129157.9 |
  2129397.8 |
  2129637.7 |
  2129877.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_scatter_null_entry**: bridge=2558.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_anchor**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_dispatch**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_per_w**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_runtime_w**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_dispatch**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_per_w**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_runtime_w**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_zig_runtime_w**: bridge=312.2% of algo (FFI overhead may distort results)
