# abi_boundary_w (real)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_real_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_real_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_real_null_entry dominates: 32650% faster than the next best (abi_boundary_w_real_soa_runtime_w)

abi_boundary_w_real_null_entry (2.71 us) leads abi_boundary_w_real_soa_runtime_w (887.59 us) by 32650%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_real_null_entry beats baseline by 100% (significant)

abi_boundary_w_real_null_entry is -2.16 ms (100%) faster than baseline abi_boundary_w_real_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_real_scalar_dispatch is an outlier: 798.5x slower than the field

abi_boundary_w_real_scalar_dispatch (2.16 ms) is 798.5x the fastest (2.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_real_scalar_dispatch shows alternating (throttle bounce) (autocorr -0.72)

abi_boundary_w_real_scalar_dispatch's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_real_null_entry} vs {abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_dispatch} (32650% apart)

The field splits into a fast tier {abi_boundary_w_real_null_entry} and a slow tier {abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_dispatch} with a 32650% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 798.5x the fastest

Fastest abi_boundary_w_real_null_entry (2.71 us) to slowest abi_boundary_w_real_scalar_dispatch (2.16 ms): 798.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_real_null_entry** at 2710.2 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 798.49x (fastest 2710.2 ns, slowest 2164054.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 4968ns | 4917ns | 4875ns | 4908ns | 5106ns | -99.77% |
| abi_boundary_w_real_scalar_anchor | 2165142ns | 2165798ns | 2153292ns | 2165155ns | 2171046ns | +0.18% |
| abi_boundary_w_real_scalar_dispatch | 2165696ns | 2166761ns | 2157250ns | 2165465ns | 2170266ns | +0.20% |
| abi_boundary_w_real_scalar_per_w | 2165424ns | 2166376ns | 2151718ns | 2164707ns | 2173352ns | +0.19% |
| abi_boundary_w_real_scalar_runtime_w | 2161318ns | 2160598ns | 2157863ns | 2160179ns | 2164755ns | base |
| abi_boundary_w_real_soa_dispatch | 894639ns | 893246ns | 888682ns | 892366ns | 901028ns | -58.61% |
| abi_boundary_w_real_soa_per_w | 895279ns | 893149ns | 887243ns | 891504ns | 904959ns | -58.58% |
| abi_boundary_w_real_soa_runtime_w | 889510ns | 890034ns | 881539ns | 887299ns | 896811ns | -58.84% |
| abi_boundary_w_real_zig_runtime_w | 2090516ns | 2091296ns | 2084910ns | 2089225ns | 2095256ns | -3.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 2727ns | 2652ns | 2805ns | -99.87% | 0.047 |
| abi_boundary_w_real_scalar_anchor | 2162420ns | 2150772ns | 2168244ns | +0.18% | 0.000 |
| abi_boundary_w_real_scalar_dispatch | 2163004ns | 2154612ns | 2167570ns | +0.20% | 0.000 |
| abi_boundary_w_real_scalar_per_w | 2162821ns | 2149243ns | 2170655ns | +0.20% | 0.000 |
| abi_boundary_w_real_scalar_runtime_w | 2158589ns | 2155135ns | 2162000ns | base | 0.000 |
| abi_boundary_w_real_soa_dispatch | 892190ns | 886232ns | 898561ns | -58.67% | 0.000 |
| abi_boundary_w_real_soa_per_w | 892780ns | 884733ns | 902364ns | -58.64% | 0.000 |
| abi_boundary_w_real_soa_runtime_w | 887069ns | 879237ns | 894240ns | -58.91% | 0.000 |
| abi_boundary_w_real_zig_runtime_w | 2087738ns | 2082343ns | 2092285ns | -3.28% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 27997.8 | 2769.3 | 2726.6 | n/a |
| abi_boundary_w_real_scalar_anchor | 44248.0 | 2160873.4 | 2162420.5 | n/a |
| abi_boundary_w_real_scalar_dispatch | 44518.6 | 2163257.6 | 2163004.0 | n/a |
| abi_boundary_w_real_scalar_per_w | 43425.5 | 2160775.3 | 2162821.2 | n/a |
| abi_boundary_w_real_scalar_runtime_w | 43079.1 | 2157872.4 | 2158588.5 | n/a |
| abi_boundary_w_real_soa_dispatch | 35170.9 | 892048.5 | 892189.7 | n/a |
| abi_boundary_w_real_soa_per_w | 37119.6 | 892670.2 | 892780.5 | n/a |
| abi_boundary_w_real_soa_runtime_w | 33928.7 | 886984.2 | 887068.9 | n/a |
| abi_boundary_w_real_zig_runtime_w | 191687.4 | 2089234.5 | 2087737.5 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_boundary_w_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.047 | 97.8% |
| abi_boundary_w_real_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_real_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_real_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_real_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_real_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_real_null_entry | 4968ns | 4968ns | -99.77% |
| abi_boundary_w_real_scalar_anchor | 2165142ns | 2165142ns | +0.18% |
| abi_boundary_w_real_scalar_dispatch | 2165696ns | 2165696ns | +0.20% |
| abi_boundary_w_real_scalar_per_w | 2165424ns | 2165424ns | +0.19% |
| abi_boundary_w_real_scalar_runtime_w | 2161318ns | 2161318ns | base |
| abi_boundary_w_real_soa_dispatch | 894639ns | 894639ns | -58.61% |
| abi_boundary_w_real_soa_per_w | 895279ns | 895279ns | -58.58% |
| abi_boundary_w_real_soa_runtime_w | 889510ns | 889510ns | -58.84% |
| abi_boundary_w_real_zig_runtime_w | 2090516ns | 2090516ns | -3.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_real_scalar_runtime_w | 2157869ns | base | --- | [2155897, 2162000] | --- | --- | --- | --- |
| abi_boundary_w_real_null_entry | 2710ns | -2155063.5ns (-99.9%) | [-2159335, -2153187]ns | [2664, 2805] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_scalar_anchor | 2163034ns | no significant difference | [-1522, +9086]ns | [2155984, 2168244] | no | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_real_scalar_dispatch | 2164054ns | no significant difference | [-3344, +11673]ns | [2157388, 2167570] | no | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_real_scalar_per_w | 2163775ns | no significant difference | [-5149, +12366]ns | [2154035, 2170655] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_real_soa_dispatch | 890836ns | -1265421.7ns (-58.6%) | [-1274047, -1259728]ns | [887171, 898561] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_soa_per_w | 890689ns | -1266567.8ns (-58.7%) | [-1276475, -1254382]ns | [885289, 902364] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_soa_runtime_w | 887589ns | -1272301.5ns (-59.0%) | [-1277367, -1264890]ns | [879378, 894240] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_zig_runtime_w | 2088564ns | -71709.8ns (-3.3%) | [-74382, -66462]ns | [2082364, 2092285] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_real_scalar_runtime_w | abi_boundary_w_real_null_entry | abi_boundary_w_real_scalar_anchor | abi_boundary_w_real_scalar_dispatch | abi_boundary_w_real_scalar_per_w | abi_boundary_w_real_soa_dispatch | abi_boundary_w_real_soa_per_w | abi_boundary_w_real_soa_runtime_w | abi_boundary_w_real_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2158356ns | -99.9% | +0.2% | +0.2% | +0.4% | -58.9% | -58.3% | -59.3% | -3.5% |
| 2 | 2155135ns | -99.9% | +0.3% | +0.6% | -0.3% | -58.6% | -58.0% | -59.2% | -3.4% |
| 3 | 2164081ns | -99.9% | +0.2% | -0.4% | -0.2% | -59.0% | -59.1% | -58.7% | -3.3% |
| 4 | 2159918ns | -99.9% | +0.2% | +0.3% | +0.6% | -58.5% | -59.0% | -58.6% | -3.3% |
| 5 | 2157382ns | -99.9% | -0.3% | +0.1% | +0.1% | -58.7% | -58.9% | -58.6% | -3.2% |
| 6 | 2156660ns | -99.9% | +0.6% | +0.5% | +0.5% | -58.3% | -58.5% | -59.1% | -3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.034 | ok |
| abi_boundary_w_real_scalar_anchor | -0.435 | moderate- |
| abi_boundary_w_real_scalar_dispatch | -0.722 | HIGH- (thermal bounce) |
| abi_boundary_w_real_scalar_per_w | -0.326 | moderate- |
| abi_boundary_w_real_scalar_runtime_w | -0.207 | moderate- |
| abi_boundary_w_real_soa_dispatch | -0.258 | moderate- |
| abi_boundary_w_real_soa_per_w | 0.199 | ok |
| abi_boundary_w_real_soa_runtime_w | 0.207 | moderate+ |
| abi_boundary_w_real_zig_runtime_w | 0.148 | ok |

**Consistency summary:**

- **abi_boundary_w_real_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_real_scalar_anchor**: won 1/6, lost 5/6
- **abi_boundary_w_real_scalar_dispatch**: won 1/6, lost 5/6
- **abi_boundary_w_real_scalar_per_w**: won 2/6, lost 3/6
- **abi_boundary_w_real_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 120278.4ns | 2726.6ns | 4411.3% | HIGH |
| abi_boundary_w_real_scalar_anchor | 6529839.0ns | 2162420.5ns | 302.0% | HIGH |
| abi_boundary_w_real_scalar_dispatch | 6534252.5ns | 2163004.0ns | 302.1% | HIGH |
| abi_boundary_w_real_scalar_per_w | 6529931.1ns | 2162821.2ns | 301.9% | HIGH |
| abi_boundary_w_real_scalar_runtime_w | 6521540.1ns | 2158588.5ns | 302.1% | HIGH |
| abi_boundary_w_real_soa_dispatch | 2713014.0ns | 892189.7ns | 304.1% | HIGH |
| abi_boundary_w_real_soa_per_w | 2717725.0ns | 892780.5ns | 304.4% | HIGH |
| abi_boundary_w_real_soa_runtime_w | 2696306.0ns | 887068.9ns | 304.0% | HIGH |
| abi_boundary_w_real_zig_runtime_w | 6525254.7ns | 2087737.5ns | 312.6% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_real_null_entry (n=6, range 2651.7-2805.2 ns)
   2651.7 |####################
   2659.4 |
   2667.0 |
   2674.7 |####################
   2682.4 |
   2690.1 |
   2697.8 |
   2705.4 |########################################
   2713.1 |
   2720.8 |
   2728.4 |
   2736.1 |####################
   2743.8 |
   2751.5 |
   2759.1 |
   2766.8 |
   2774.5 |
   2782.2 |
   2789.8 |
   2797.5 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_anchor (n=6, range 2150772.1-2168243.5 ns)
  2150772.1 |########################################
  2151645.7 |
  2152519.2 |
  2153392.8 |
  2154266.4 |
  2155140.0 |
  2156013.5 |
  2156887.1 |
  2157760.7 |
  2158634.3 |
  2159507.8 |
  2160381.4 |########################################
  2161255.0 |########################################
  2162128.5 |
  2163002.1 |
  2163875.7 |########################################
  2164749.3 |
  2165622.8 |
  2166496.4 |
  2167370.0 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_scalar_dispatch (n=6, range 2154612.1-2167569.8 ns)
  2154612.1 |########################################
  2155260.0 |
  2155907.9 |
  2156555.8 |
  2157203.6 |
  2157851.5 |
  2158499.4 |
  2159147.3 |
  2159795.2 |########################################
  2160443.1 |
  2161091.0 |
  2161738.8 |########################################
  2162386.7 |
  2163034.6 |
  2163682.5 |
  2164330.4 |
  2164978.3 |
  2165626.1 |########################################
  2166274.0 |########################################
  2166921.9 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_per_w (n=6, range 2149242.9-2170654.5 ns)
  2149242.9 |####################
  2150313.5 |
  2151384.1 |
  2152454.6 |
  2153525.2 |
  2154595.8 |
  2155666.4 |
  2156737.0 |
  2157807.6 |####################
  2158878.1 |####################
  2159948.7 |
  2161019.3 |
  2162089.9 |
  2163160.5 |
  2164231.1 |
  2165301.6 |
  2166372.2 |
  2167442.8 |########################################
  2168513.4 |
  2169584.0 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_runtime_w (n=6, range 2155134.6-2161999.8 ns)
  2155134.6 |########################################
  2155477.9 |
  2155821.1 |
  2156164.4 |
  2156507.6 |########################################
  2156850.9 |
  2157194.1 |########################################
  2157537.4 |
  2157880.7 |
  2158223.9 |########################################
  2158567.2 |
  2158910.4 |
  2159253.7 |
  2159596.9 |########################################
  2159940.2 |
  2160283.5 |
  2160626.7 |
  2160970.0 |
  2161313.2 |
  2161656.5 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_dispatch (n=6, range 886232.1-898561.2 ns)
  886232.1 |########################################
  886848.6 |
  887465.0 |
  888081.5 |########################################
  888697.9 |
  889314.4 |
  889930.8 |########################################
  890547.3 |
  891163.8 |########################################
  891780.2 |
  892396.7 |
  893013.1 |
  893629.6 |
  894246.0 |
  894862.5 |
  895479.0 |
  896095.4 |
  896711.9 |########################################
  897328.3 |
  897944.8 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_per_w (n=6, range 884732.9-902363.6 ns)
  884732.9 |####################
  885614.4 |########################################
  886496.0 |
  887377.5 |
  888259.0 |
  889140.6 |
  890022.1 |
  890903.6 |
  891785.2 |
  892666.7 |
  893548.2 |
  894429.8 |####################
  895311.3 |
  896192.8 |
  897074.4 |
  897955.9 |
  898837.4 |
  899719.0 |####################
  900600.5 |
  901482.0 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_runtime_w (n=6, range 879236.7-894239.8 ns)
  879236.7 |########################################
  879986.9 |
  880737.0 |
  881487.2 |####################
  882237.3 |
  882987.5 |
  883737.6 |
  884487.8 |
  885237.9 |
  885988.1 |
  886738.2 |
  887488.4 |
  888238.5 |
  888988.7 |
  889738.8 |
  890489.0 |
  891239.1 |
  891989.3 |
  892739.4 |
  893489.6 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_zig_runtime_w (n=6, range 2082342.9-2092285.4 ns)
  2082342.9 |########################################
  2082840.0 |
  2083337.1 |
  2083834.3 |
  2084331.4 |
  2084828.5 |
  2085325.6 |
  2085822.8 |
  2086319.9 |
  2086817.0 |
  2087314.1 |
  2087811.3 |
  2088308.4 |########################################
  2088805.5 |
  2089302.6 |
  2089799.8 |
  2090296.9 |
  2090794.0 |
  2091291.1 |####################
  2091788.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_real_null_entry**: bridge=4433.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_anchor**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_dispatch**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_per_w**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_runtime_w**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_dispatch**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_per_w**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_runtime_w**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_zig_runtime_w**: bridge=312.5% of algo (FFI overhead may distort results)
