# abi_zig_entry (scatter)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_scatter_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_scatter_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_scatter_zig_null dominates: 60911% faster than the next best (abi_zig_entry_scatter_zig_per_w_set)

abi_zig_entry_scatter_zig_null (3.44 us) leads abi_zig_entry_scatter_zig_per_w_set (2.10 ms) by 60911%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_scatter_zig_null beats baseline by 100% (significant)

abi_zig_entry_scatter_zig_null is -2.11 ms (100%) faster than baseline abi_zig_entry_scatter_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_scatter_zig_tail_runtime_w is an outlier: 1127.9x slower than the field

abi_zig_entry_scatter_zig_tail_runtime_w (3.88 ms) is 1127.9x the fastest (3.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_scatter_zig_null} vs {abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch, abi_zig_entry_scatter_zig_tail_runtime_w} (60911% apart)

The field splits into a fast tier {abi_zig_entry_scatter_zig_null} and a slow tier {abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch, abi_zig_entry_scatter_zig_tail_runtime_w} with a 60911% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1127.9x the fastest

Fastest abi_zig_entry_scatter_zig_null (3.44 us) to slowest abi_zig_entry_scatter_zig_tail_runtime_w (3.88 ms): 1127.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_scatter_zig_null** at 3443.5 ns median (-99.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1127.86x (fastest 3443.5 ns, slowest 3883772.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2105154ns | 2105772ns | 2096544ns | 2103814ns | 2111467ns | -1.00% |
| abi_zig_entry_scatter_zig_dispatch | 2109124ns | 2112107ns | 2084048ns | 2107403ns | 2124244ns | -0.81% |
| abi_zig_entry_scatter_zig_null | 5751ns | 5735ns | 5659ns | 5725ns | 5835ns | -99.73% |
| abi_zig_entry_scatter_zig_per_w_set | 2105369ns | 2103600ns | 2099708ns | 2102937ns | 2111849ns | -0.99% |
| abi_zig_entry_scatter_zig_runtime_w | 2126453ns | 2113216ns | 2110202ns | 2112274ns | 2155847ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3868315ns | 3868849ns | 3844395ns | 3860731ns | 3891651ns | +81.91% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3953767ns | 3886660ns | 3876241ns | 3884932ns | 4095782ns | +85.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2102581ns | 2094032ns | 2108894ns | -1.00% | 0.000 |
| abi_zig_entry_scatter_zig_dispatch | 2106450ns | 2081583ns | 2121403ns | -0.82% | 0.000 |
| abi_zig_entry_scatter_zig_null | 3438ns | 3385ns | 3475ns | -99.84% | 0.001 |
| abi_zig_entry_scatter_zig_per_w_set | 2102620ns | 2096995ns | 2108998ns | -1.00% | 0.000 |
| abi_zig_entry_scatter_zig_runtime_w | 2123807ns | 2107515ns | 2153067ns | base | 0.000 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3865501ns | 3841797ns | 3888724ns | +82.01% | 0.000 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3950877ns | 3873436ns | 4092827ns | +86.03% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 172674.3 | 2102547.1 | 2102581.2 | n/a |
| abi_zig_entry_scatter_zig_dispatch | 182830.9 | 2107691.0 | 2106449.9 | 0 |
| abi_zig_entry_scatter_zig_null | 154671.1 | 3618.5 | 3438.0 | n/a |
| abi_zig_entry_scatter_zig_per_w_set | 181956.2 | 2104688.6 | 2102620.0 | 0 |
| abi_zig_entry_scatter_zig_runtime_w | 178729.2 | 2117083.4 | 2123806.8 | n/a |
| abi_zig_entry_scatter_zig_tail_dispatch | 189492.2 | 3877927.0 | 3865501.4 | n/a |
| abi_zig_entry_scatter_zig_tail_runtime_w | 195685.5 | 3909202.1 | 3950877.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_scatter_zig_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_null | 0.001 | 98.3% |
| abi_zig_entry_scatter_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2105154ns | 2105154ns | -1.00% |
| abi_zig_entry_scatter_zig_dispatch | 2109124ns | 2109124ns | -0.81% |
| abi_zig_entry_scatter_zig_null | 5751ns | 5751ns | -99.73% |
| abi_zig_entry_scatter_zig_per_w_set | 2105369ns | 2105369ns | -0.99% |
| abi_zig_entry_scatter_zig_runtime_w | 2126453ns | 2126453ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3868315ns | 3868315ns | +81.91% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3953767ns | 3953767ns | +85.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_runtime_w | 2110678ns | base | --- | [2107675, 2153067] | --- | --- | --- | --- |
| abi_zig_entry_scatter_zig_anchor | 2103182ns | -8741.9ns (-0.4%) | [-53150, -1785]ns | [2095668, 2108894] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_dispatch | 2109506ns | no significant difference | [-43454, +1475]ns | [2088441, 2121403] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_null | 3444ns | -2107232.6ns (-99.8%) | [-2149654, -2104219]ns | [3395, 3475] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_per_w_set | 2100922ns | -9950.7ns (-0.5%) | [-50060, -3550]ns | [2097940, 2108998] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3865952ns | +1742694.3ns (+82.6%) | [+1721072, +1761318]ns | [3841829, 3888724] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3883773ns | +1776098.1ns (+84.1%) | [+1765354, +1939760]ns | [3876032, 4092827] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_scatter_zig_runtime_w | abi_zig_entry_scatter_zig_anchor | abi_zig_entry_scatter_zig_dispatch | abi_zig_entry_scatter_zig_null | abi_zig_entry_scatter_zig_per_w_set | abi_zig_entry_scatter_zig_tail_dispatch | abi_zig_entry_scatter_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2192127ns | -4.1% | -2.8% | -99.8% | -3.8% | +78.0% | +95.8% |
| 2 | 2108942ns | -0.2% | +0.1% | -99.8% | -0.4% | +82.2% | +83.7% |
| 3 | 2107835ns | -0.2% | +0.0% | -99.8% | -0.4% | +83.5% | +84.2% |
| 4 | 2112415ns | -0.0% | -0.8% | -99.8% | -0.5% | +83.5% | +83.6% |
| 5 | 2114008ns | -0.8% | -0.1% | -99.8% | -0.8% | +82.8% | +84.2% |
| 6 | 2107515ns | -0.6% | -1.2% | -99.8% | +0.1% | +82.3% | +84.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.046 | ok |
| abi_zig_entry_scatter_zig_dispatch | -0.031 | ok |
| abi_zig_entry_scatter_zig_null | 0.296 | moderate+ |
| abi_zig_entry_scatter_zig_per_w_set | -0.232 | moderate- |
| abi_zig_entry_scatter_zig_runtime_w | -0.058 | ok |
| abi_zig_entry_scatter_zig_tail_dispatch | -0.342 | moderate- |
| abi_zig_entry_scatter_zig_tail_runtime_w | -0.059 | ok |

**Consistency summary:**

- **abi_zig_entry_scatter_zig_anchor**: won 5/6, lost 0/6
- **abi_zig_entry_scatter_zig_dispatch**: won 4/6, lost 1/6
- **abi_zig_entry_scatter_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_scatter_zig_per_w_set**: won 5/6, lost 0/6
- **abi_zig_entry_scatter_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_scatter_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 6546028.5ns | 2102581.2ns | 311.3% | HIGH |
| abi_zig_entry_scatter_zig_dispatch | 6578135.3ns | 2106449.9ns | 312.3% | HIGH |
| abi_zig_entry_scatter_zig_null | 304719.9ns | 3438.0ns | 8863.4% | HIGH |
| abi_zig_entry_scatter_zig_per_w_set | 6561054.1ns | 2102620.0ns | 312.0% | HIGH |
| abi_zig_entry_scatter_zig_runtime_w | 6597944.1ns | 2123806.8ns | 310.7% | HIGH |
| abi_zig_entry_scatter_zig_tail_dispatch | 11988343.9ns | 3865501.4ns | 310.1% | HIGH |
| abi_zig_entry_scatter_zig_tail_runtime_w | 11985812.7ns | 3950877.3ns | 303.4% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_scatter_zig_anchor (n=6, range 2094031.7-2108893.8 ns)
  2094031.7 |########################################
  2094774.8 |
  2095517.9 |
  2096261.0 |
  2097004.1 |########################################
  2097747.2 |
  2098490.3 |
  2099233.4 |
  2099976.5 |
  2100719.6 |
  2101462.7 |
  2102205.8 |########################################
  2102948.9 |
  2103692.0 |########################################
  2104435.1 |
  2105178.2 |########################################
  2105921.3 |
  2106664.4 |
  2107407.5 |
  2108150.6 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_dispatch (n=6, range 2081582.9-2121403.0 ns)
  2081582.9 |########################################
  2083573.9 |
  2085564.9 |
  2087555.9 |
  2089546.9 |
  2091537.9 |
  2093528.9 |########################################
  2095519.9 |
  2097510.9 |
  2099501.9 |
  2101492.9 |
  2103483.9 |
  2105474.9 |
  2107465.9 |########################################
  2109456.9 |########################################
  2111447.9 |########################################
  2113438.9 |
  2115429.9 |
  2117420.9 |
  2119411.9 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_null (n=6, range 3385.0-3475.0 ns)
   3385.0 |########################################
   3389.5 |
   3394.0 |
   3398.5 |
   3403.0 |########################################
   3407.5 |
   3412.0 |
   3416.5 |
   3421.0 |
   3425.5 |
   3430.0 |
   3434.5 |
   3439.0 |########################################
   3443.5 |########################################
   3448.0 |
   3452.5 |
   3457.0 |
   3461.5 |########################################
   3466.0 |
   3470.5 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_per_w_set (n=6, range 2096995.0-2108997.5 ns)
  2096995.0 |########################################
  2097595.1 |
  2098195.2 |
  2098795.4 |########################################
  2099395.5 |
  2099995.6 |########################################
  2100595.8 |
  2101195.9 |########################################
  2101796.0 |
  2102396.1 |
  2102996.2 |
  2103596.4 |
  2104196.5 |
  2104796.6 |
  2105396.8 |
  2105996.9 |
  2106597.0 |
  2107197.1 |
  2107797.2 |
  2108397.4 |########################################
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_runtime_w (n=6, range 2107514.6-2153067.3 ns)
  2107514.6 |########################################
  2109792.2 |
  2112069.9 |##########################
  2114347.5 |
  2116625.1 |
  2118902.8 |
  2121180.4 |
  2123458.0 |
  2125735.7 |
  2128013.3 |
  2130291.0 |
  2132568.6 |
  2134846.2 |
  2137123.9 |
  2139401.5 |
  2141679.1 |
  2143956.8 |
  2146234.4 |
  2148512.0 |
  2150789.7 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_dispatch (n=6, range 3841797.1-3888723.5 ns)
  3841797.1 |########################################
  3844143.4 |
  3846489.7 |
  3848836.1 |
  3851182.4 |
  3853528.7 |
  3855875.0 |
  3858221.3 |
  3860567.7 |
  3862914.0 |####################
  3865260.3 |####################
  3867606.6 |
  3869952.9 |
  3872299.3 |
  3874645.6 |####################
  3876991.9 |
  3879338.2 |
  3881684.5 |
  3884030.9 |
  3886377.2 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_runtime_w (n=6, range 3873435.8-4092827.1 ns)
  3873435.8 |########################################
  3884405.4 |##########################
  3895374.9 |
  3906344.5 |
  3917314.0 |
  3928283.6 |
  3939253.2 |
  3950222.7 |
  3961192.3 |
  3972161.9 |
  3983131.4 |
  3994101.0 |
  4005070.6 |
  4016040.1 |
  4027009.7 |
  4037979.2 |
  4048948.8 |
  4059918.4 |
  4070887.9 |
  4081857.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_scatter_zig_anchor**: bridge=311.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_dispatch**: bridge=311.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_null**: bridge=8863.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_per_w_set**: bridge=312.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_runtime_w**: bridge=311.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_dispatch**: bridge=306.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_runtime_w**: bridge=307.6% of algo (FFI overhead may distort results)
