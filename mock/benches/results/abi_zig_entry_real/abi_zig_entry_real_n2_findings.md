# abi_zig_entry (real)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_real_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_real_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_real_zig_null dominates: 60858% faster than the next best (abi_zig_entry_real_zig_per_w_set)

abi_zig_entry_real_zig_null (3.40 us) leads abi_zig_entry_real_zig_per_w_set (2.07 ms) by 60858%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_real_zig_null beats baseline by 100% (significant)

abi_zig_entry_real_zig_null is -2.08 ms (100%) faster than baseline abi_zig_entry_real_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_real_zig_tail_runtime_w is an outlier: 1144.2x slower than the field

abi_zig_entry_real_zig_tail_runtime_w (3.89 ms) is 1144.2x the fastest (3.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_real_zig_null} vs {abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} (60858% apart)

The field splits into a fast tier {abi_zig_entry_real_zig_null} and a slow tier {abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} with a 60858% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1144.2x the fastest

Fastest abi_zig_entry_real_zig_null (3.40 us) to slowest abi_zig_entry_real_zig_tail_runtime_w (3.89 ms): 1144.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_real_zig_null** at 3399.8 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1144.23x (fastest 3399.8 ns, slowest 3890160.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2096152ns | 2076522ns | 2072752ns | 2075763ns | 2138436ns | +0.65% |
| abi_zig_entry_real_zig_dispatch | 2075815ns | 2076005ns | 2072514ns | 2074853ns | 2078909ns | -0.33% |
| abi_zig_entry_real_zig_null | 5703ns | 5722ns | 5625ns | 5700ns | 5746ns | -99.73% |
| abi_zig_entry_real_zig_per_w_set | 2078052ns | 2075001ns | 2074574ns | 2074867ns | 2084569ns | -0.22% |
| abi_zig_entry_real_zig_runtime_w | 2082608ns | 2081518ns | 2080483ns | 2081258ns | 2085695ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3109932ns | 3109599ns | 3104729ns | 3109324ns | 3113445ns | +49.33% |
| abi_zig_entry_real_zig_tail_runtime_w | 3891709ns | 3893074ns | 3879844ns | 3889836ns | 3900452ns | +86.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2093489ns | 2070238ns | 2135637ns | +0.65% | 0.000 |
| abi_zig_entry_real_zig_dispatch | 2073162ns | 2069850ns | 2076175ns | -0.33% | 0.000 |
| abi_zig_entry_real_zig_null | 3406ns | 3379ns | 3439ns | -99.84% | 0.001 |
| abi_zig_entry_real_zig_per_w_set | 2075344ns | 2071875ns | 2081703ns | -0.22% | 0.000 |
| abi_zig_entry_real_zig_runtime_w | 2079994ns | 2077975ns | 2082989ns | base | 0.000 |
| abi_zig_entry_real_zig_tail_dispatch | 3107186ns | 3102217ns | 3110660ns | +49.38% | 0.000 |
| abi_zig_entry_real_zig_tail_runtime_w | 3888792ns | 3877201ns | 3897425ns | +86.96% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 182815.0 | 2076276.1 | 2093489.3 | 0 |
| abi_zig_entry_real_zig_dispatch | 179494.6 | 2073622.2 | 2073162.2 | n/a |
| abi_zig_entry_real_zig_null | 154832.4 | 3570.8 | 3406.5 | n/a |
| abi_zig_entry_real_zig_per_w_set | 182981.7 | 2072525.8 | 2075343.5 | n/a |
| abi_zig_entry_real_zig_runtime_w | 183097.1 | 2079838.0 | 2079994.4 | n/a |
| abi_zig_entry_real_zig_tail_dispatch | 190059.4 | 3106632.5 | 3107186.3 | n/a |
| abi_zig_entry_real_zig_tail_runtime_w | 199318.0 | 3897558.7 | 3888792.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_real_zig_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_real_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_real_zig_null | 0.001 | 99.4% |
| abi_zig_entry_real_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_real_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_real_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2096152ns | 2096152ns | +0.65% |
| abi_zig_entry_real_zig_dispatch | 2075815ns | 2075815ns | -0.33% |
| abi_zig_entry_real_zig_null | 5703ns | 5703ns | -99.73% |
| abi_zig_entry_real_zig_per_w_set | 2078052ns | 2078052ns | -0.22% |
| abi_zig_entry_real_zig_runtime_w | 2082608ns | 2082608ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3109932ns | 3109932ns | +49.33% |
| abi_zig_entry_real_zig_tail_runtime_w | 3891709ns | 3891709ns | +86.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_runtime_w | 2078946ns | base | --- | [2078048, 2082989] | --- | --- | --- | --- |
| abi_zig_entry_real_zig_anchor | 2073855ns | no significant difference | [-8835, +55777]ns | [2070976, 2135637] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_real_zig_dispatch | 2073398ns | -7518.5ns (-0.4%) | [-11105, -1873]ns | [2069913, 2076175] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_null | 3400ns | -2075525.2ns (-99.8%) | [-2079581, -2074657]ns | [3380, 3439] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_per_w_set | 2072435ns | no significant difference | [-9998, +1771]ns | [2071892, 2081703] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_real_zig_tail_dispatch | 3106829ns | +1026225.4ns (+49.4%) | [+1023590, +1031760]ns | [3104070, 3110660] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_tail_runtime_w | 3890161ns | +1808890.9ns (+87.0%) | [+1799963, +1817539]ns | [3878791, 3897425] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_real_zig_runtime_w | abi_zig_entry_real_zig_anchor | abi_zig_entry_real_zig_dispatch | abi_zig_entry_real_zig_null | abi_zig_entry_real_zig_per_w_set | abi_zig_entry_real_zig_tail_dispatch | abi_zig_entry_real_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2077975ns | -0.1% | -0.2% | -99.8% | -0.3% | +49.3% | +86.6% |
| 2 | 2078212ns | -0.4% | -0.4% | -99.8% | -0.3% | +49.5% | +87.7% |
| 3 | 2081745ns | +5.5% | -0.6% | -99.8% | -0.2% | +49.2% | +86.8% |
| 4 | 2079680ns | -0.4% | -0.3% | -99.8% | -0.4% | +49.5% | +86.6% |
| 5 | 2078121ns | -0.2% | -0.0% | -99.8% | +0.4% | +49.8% | +87.2% |
| 6 | 2084233ns | -0.5% | -0.5% | -99.8% | -0.6% | +49.1% | +86.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_real_zig_anchor | -0.268 | moderate- |
| abi_zig_entry_real_zig_dispatch | 0.194 | ok |
| abi_zig_entry_real_zig_null | 0.188 | ok |
| abi_zig_entry_real_zig_per_w_set | -0.489 | moderate- |
| abi_zig_entry_real_zig_runtime_w | -0.233 | moderate- |
| abi_zig_entry_real_zig_tail_dispatch | 0.245 | moderate+ |
| abi_zig_entry_real_zig_tail_runtime_w | -0.372 | moderate- |

**Consistency summary:**

- **abi_zig_entry_real_zig_anchor**: won 5/6, lost 1/6
- **abi_zig_entry_real_zig_dispatch**: won 5/6, lost 0/6
- **abi_zig_entry_real_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_real_zig_per_w_set**: won 5/6, lost 1/6
- **abi_zig_entry_real_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_real_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 6533059.0ns | 2093489.3ns | 312.1% | HIGH |
| abi_zig_entry_real_zig_dispatch | 6468866.0ns | 2073162.2ns | 312.0% | HIGH |
| abi_zig_entry_real_zig_null | 304093.5ns | 3406.5ns | 8926.9% | HIGH |
| abi_zig_entry_real_zig_per_w_set | 6468407.6ns | 2075343.5ns | 311.7% | HIGH |
| abi_zig_entry_real_zig_runtime_w | 6497073.5ns | 2079994.4ns | 312.4% | HIGH |
| abi_zig_entry_real_zig_tail_dispatch | 9582056.4ns | 3107186.3ns | 308.4% | HIGH |
| abi_zig_entry_real_zig_tail_runtime_w | 11952964.4ns | 3888792.1ns | 307.4% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_real_zig_anchor (n=6, range 2070237.5-2135636.7 ns)
  2070237.5 |########################################
  2073507.5 |##########################
  2076777.4 |
  2080047.4 |
  2083317.3 |
  2086587.3 |
  2089857.3 |
  2093127.2 |
  2096397.2 |
  2099667.1 |
  2102937.1 |
  2106207.1 |
  2109477.0 |
  2112747.0 |
  2116016.9 |
  2119286.9 |
  2122556.9 |
  2125826.8 |
  2129096.8 |
  2132366.7 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_dispatch (n=6, range 2069849.6-2076175.2 ns)
  2069849.6 |########################################
  2070165.9 |
  2070482.2 |
  2070798.4 |
  2071114.7 |
  2071431.0 |
  2071747.3 |
  2072063.6 |
  2072379.8 |
  2072696.1 |####################
  2073012.4 |
  2073328.7 |
  2073645.0 |####################
  2073961.2 |
  2074277.5 |####################
  2074593.8 |
  2074910.1 |
  2075226.4 |
  2075542.6 |
  2075858.9 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_null (n=6, range 3378.8-3439.4 ns)
   3378.8 |########################################
   3381.8 |
   3384.9 |
   3387.9 |
   3390.9 |
   3393.9 |
   3397.0 |####################
   3400.0 |####################
   3403.0 |
   3406.0 |
   3409.1 |
   3412.1 |
   3415.1 |
   3418.2 |####################
   3421.2 |
   3424.2 |
   3427.2 |
   3430.3 |
   3433.3 |
   3436.3 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_per_w_set (n=6, range 2071875.4-2081703.4 ns)
  2071875.4 |########################################
  2072366.8 |#############
  2072858.2 |
  2073349.6 |
  2073841.0 |
  2074332.4 |
  2074823.8 |
  2075315.2 |
  2075806.6 |
  2076298.0 |
  2076789.4 |
  2077280.8 |#############
  2077772.2 |
  2078263.6 |
  2078755.0 |
  2079246.4 |
  2079737.8 |
  2080229.2 |
  2080720.6 |
  2081212.0 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_runtime_w (n=6, range 2077975.4-2082989.0 ns)
  2077975.4 |########################################
  2078226.1 |
  2078476.8 |
  2078727.4 |
  2078978.1 |
  2079228.8 |
  2079479.5 |#############
  2079730.1 |
  2079980.8 |
  2080231.5 |
  2080482.2 |
  2080732.9 |
  2080983.5 |
  2081234.2 |
  2081484.9 |
  2081735.6 |#############
  2081986.2 |
  2082236.9 |
  2082487.6 |
  2082738.3 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_dispatch (n=6, range 3102217.1-3110660.2 ns)
  3102217.1 |########################################
  3102639.3 |
  3103061.4 |
  3103483.6 |
  3103905.7 |
  3104327.9 |
  3104750.0 |
  3105172.2 |
  3105594.3 |########################################
  3106016.5 |
  3106438.7 |########################################
  3106860.8 |########################################
  3107283.0 |
  3107705.1 |
  3108127.3 |
  3108549.4 |########################################
  3108971.6 |
  3109393.7 |
  3109815.9 |
  3110238.0 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_runtime_w (n=6, range 3877201.2-3897424.8 ns)
  3877201.2 |########################################
  3878212.4 |
  3879223.6 |
  3880234.7 |########################################
  3881245.9 |
  3882257.1 |
  3883268.3 |
  3884279.5 |
  3885290.6 |
  3886301.8 |
  3887313.0 |
  3888324.2 |
  3889335.4 |########################################
  3890346.5 |########################################
  3891357.7 |
  3892368.9 |
  3893380.1 |########################################
  3894391.3 |
  3895402.4 |
  3896413.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_real_zig_anchor**: bridge=311.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_dispatch**: bridge=311.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_null**: bridge=8927.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_per_w_set**: bridge=311.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_runtime_w**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_dispatch**: bridge=308.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_runtime_w**: bridge=307.5% of algo (FFI overhead may distort results)
