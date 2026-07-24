# abi_zig_entry (real)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_real_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_real_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_real_zig_null dominates: 82457% faster than the next best (abi_zig_entry_real_zig_per_w_set)

abi_zig_entry_real_zig_null (2.50 us) leads abi_zig_entry_real_zig_per_w_set (2.07 ms) by 82457%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_real_zig_null beats baseline by 100% (significant)

abi_zig_entry_real_zig_null is -2.06 ms (100%) faster than baseline abi_zig_entry_real_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_real_zig_tail_runtime_w is an outlier: 1537.9x slower than the field

abi_zig_entry_real_zig_tail_runtime_w (3.85 ms) is 1537.9x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_real_zig_dispatch shows alternating (throttle bounce) (autocorr -0.53)

abi_zig_entry_real_zig_dispatch's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_real_zig_null} vs {abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} (82457% apart)

The field splits into a fast tier {abi_zig_entry_real_zig_null} and a slow tier {abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} with a 82457% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1537.9x the fastest

Fastest abi_zig_entry_real_zig_null (2.50 us) to slowest abi_zig_entry_real_zig_tail_runtime_w (3.85 ms): 1537.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_real_zig_null** at 2501.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1537.86x (fastest 2501.7 ns, slowest 3847184.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2084204ns | 2073530ns | 2070298ns | 2072639ns | 2108504ns | +0.01% |
| abi_zig_entry_real_zig_dispatch | 2072914ns | 2069242ns | 2068111ns | 2068891ns | 2081351ns | -0.53% |
| abi_zig_entry_real_zig_null | 4844ns | 4837ns | 4685ns | 4810ns | 4974ns | -99.77% |
| abi_zig_entry_real_zig_per_w_set | 2070944ns | 2067881ns | 2065235ns | 2067263ns | 2079321ns | -0.63% |
| abi_zig_entry_real_zig_runtime_w | 2084063ns | 2070011ns | 2067299ns | 2069706ns | 2113981ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3476517ns | 3478130ns | 3097663ns | 3352210ns | 3852404ns | +66.81% |
| abi_zig_entry_real_zig_tail_runtime_w | 3856139ns | 3850065ns | 3846242ns | 3849976ns | 3870334ns | +85.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2081457ns | 2067752ns | 2105524ns | +0.01% | 0.000 |
| abi_zig_entry_real_zig_dispatch | 2070213ns | 2065500ns | 2078559ns | -0.53% | 0.000 |
| abi_zig_entry_real_zig_null | 2519ns | 2456ns | 2592ns | -99.88% | 0.025 |
| abi_zig_entry_real_zig_per_w_set | 2068261ns | 2062621ns | 2076553ns | -0.62% | 0.000 |
| abi_zig_entry_real_zig_runtime_w | 2081267ns | 2064718ns | 2110908ns | base | 0.000 |
| abi_zig_entry_real_zig_tail_dispatch | 3473538ns | 3094820ns | 3849389ns | +66.90% | 0.000 |
| abi_zig_entry_real_zig_tail_runtime_w | 3853197ns | 3843370ns | 3867244ns | +85.14% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 185736.9 | 2098736.4 | 2081457.0 | 23 |
| abi_zig_entry_real_zig_dispatch | 182955.5 | 2070551.4 | 2070213.1 | n/a |
| abi_zig_entry_real_zig_null | 158840.2 | 2752.6 | 2519.3 | n/a |
| abi_zig_entry_real_zig_per_w_set | 185475.9 | 2068661.7 | 2068261.2 | n/a |
| abi_zig_entry_real_zig_runtime_w | 190147.1 | 2090693.0 | 2081266.5 | n/a |
| abi_zig_entry_real_zig_tail_dispatch | 199791.2 | 3479046.3 | 3473538.4 | n/a |
| abi_zig_entry_real_zig_tail_runtime_w | 196913.3 | 3860965.6 | 3853197.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_zig_entry_real_zig_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_real_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_null | 0.026 | 98.2% |
| abi_zig_entry_real_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_real_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2084204ns | 2084204ns | +0.01% |
| abi_zig_entry_real_zig_dispatch | 2072914ns | 2072914ns | -0.53% |
| abi_zig_entry_real_zig_null | 4844ns | 4844ns | -99.77% |
| abi_zig_entry_real_zig_per_w_set | 2070944ns | 2070944ns | -0.63% |
| abi_zig_entry_real_zig_runtime_w | 2084063ns | 2084063ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3476517ns | 3476517ns | +66.81% |
| abi_zig_entry_real_zig_tail_runtime_w | 3856139ns | 3856139ns | +85.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_runtime_w | 2067296ns | base | --- | [2065595, 2110908] | --- | --- | --- | --- |
| abi_zig_entry_real_zig_anchor | 2070877ns | no significant difference | [-6619, +5196]ns | [2067969, 2105524] | no | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_real_zig_dispatch | 2066574ns | no significant difference | [-43716, +11278]ns | [2065506, 2078559] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_real_zig_null | 2502ns | -2064813.4ns (-99.9%) | [-2108426, -2063002]ns | [2464, 2592] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_real_zig_per_w_set | 2065276ns | no significant difference | [-39135, +3309]ns | [2062955, 2076553] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_real_zig_tail_dispatch | 3475062ns | +1408144.6ns (+68.1%) | [+987820, +1780851]ns | [3096165, 3849389] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_real_zig_tail_runtime_w | 3847184ns | +1780204.6ns (+86.1%) | [+1749001, +1786587]ns | [3845163, 3867244] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_real_zig_runtime_w | abi_zig_entry_real_zig_anchor | abi_zig_entry_real_zig_dispatch | abi_zig_entry_real_zig_null | abi_zig_entry_real_zig_per_w_set | abi_zig_entry_real_zig_tail_dispatch | abi_zig_entry_real_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2066472ns | +0.3% | -0.0% | -99.9% | +0.3% | +50.3% | +86.2% |
| 2 | 2064718ns | +0.1% | +0.5% | -99.9% | +0.0% | +50.0% | +86.3% |
| 3 | 2151972ns | -0.7% | -4.0% | -99.9% | -3.3% | +43.8% | +80.0% |
| 4 | 2069845ns | +0.0% | +0.6% | -99.9% | -0.3% | +86.1% | +86.5% |
| 5 | 2067231ns | +0.2% | -0.0% | -99.9% | -0.2% | +86.1% | +86.1% |
| 6 | 2067362ns | +0.0% | -0.0% | -99.9% | -0.1% | +86.0% | +85.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_real_zig_anchor | -0.263 | moderate- |
| abi_zig_entry_real_zig_dispatch | -0.533 | HIGH- (thermal bounce) |
| abi_zig_entry_real_zig_null | 0.468 | moderate+ |
| abi_zig_entry_real_zig_per_w_set | -0.298 | moderate- |
| abi_zig_entry_real_zig_runtime_w | -0.229 | moderate- |
| abi_zig_entry_real_zig_tail_dispatch | 0.497 | moderate+ |
| abi_zig_entry_real_zig_tail_runtime_w | 0.116 | ok |

**Consistency summary:**

- **abi_zig_entry_real_zig_anchor**: won 1/6, lost 3/6
- **abi_zig_entry_real_zig_dispatch**: won 1/6, lost 2/6
- **abi_zig_entry_real_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_real_zig_per_w_set**: won 3/6, lost 1/6
- **abi_zig_entry_real_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_real_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 6626903.2ns | 2081457.0ns | 318.4% | HIGH |
| abi_zig_entry_real_zig_dispatch | 6465615.1ns | 2070213.1ns | 312.3% | HIGH |
| abi_zig_entry_real_zig_null | 302804.9ns | 2519.3ns | 12019.4% | HIGH |
| abi_zig_entry_real_zig_per_w_set | 6463222.3ns | 2068261.2ns | 312.5% | HIGH |
| abi_zig_entry_real_zig_runtime_w | 6549108.7ns | 2081266.5ns | 314.7% | HIGH |
| abi_zig_entry_real_zig_tail_dispatch | 10705407.9ns | 3473538.4ns | 308.2% | HIGH |
| abi_zig_entry_real_zig_tail_runtime_w | 11838775.8ns | 3853197.2ns | 307.2% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_real_zig_anchor (n=6, range 2067751.7-2105524.4 ns)
  2067751.7 |########################################
  2069640.3 |########################################
  2071529.0 |####################
  2073417.6 |
  2075306.2 |
  2077194.9 |
  2079083.5 |
  2080972.1 |
  2082860.8 |
  2084749.4 |
  2086638.0 |
  2088526.7 |
  2090415.3 |
  2092304.0 |
  2094192.6 |
  2096081.2 |
  2097969.9 |
  2099858.5 |
  2101747.1 |
  2103635.8 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_dispatch (n=6, range 2065500.0-2078558.8 ns)
  2065500.0 |########################################
  2066152.9 |########################################
  2066805.9 |
  2067458.8 |
  2068111.8 |
  2068764.7 |
  2069417.6 |
  2070070.6 |
  2070723.5 |
  2071376.4 |
  2072029.4 |
  2072682.3 |
  2073335.2 |
  2073988.2 |####################
  2074641.1 |
  2075294.1 |
  2075947.0 |
  2076599.9 |
  2077252.9 |
  2077905.8 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_null (n=6, range 2455.8-2592.5 ns)
   2455.8 |########################################
   2462.6 |
   2469.5 |########################################
   2476.3 |
   2483.1 |
   2490.0 |########################################
   2496.8 |
   2503.6 |########################################
   2510.5 |
   2517.3 |
   2524.2 |
   2531.0 |
   2537.8 |
   2544.7 |
   2551.5 |
   2558.3 |
   2565.2 |
   2572.0 |
   2578.8 |
   2585.7 |########################################
  (0 below, 1 above range)

abi_zig_entry_real_zig_per_w_set (n=6, range 2062621.2-2076552.7 ns)
  2062621.2 |########################################
  2063317.8 |
  2064014.4 |
  2064710.9 |####################
  2065407.5 |####################
  2066104.1 |
  2066800.6 |
  2067497.2 |
  2068193.8 |
  2068890.4 |
  2069587.0 |
  2070283.5 |
  2070980.1 |
  2071676.7 |
  2072373.2 |####################
  2073069.8 |
  2073766.4 |
  2074463.0 |
  2075159.6 |
  2075856.1 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_runtime_w (n=6, range 2064717.5-2110908.2 ns)
  2064717.5 |########################################
  2067027.0 |########################################
  2069336.6 |####################
  2071646.1 |
  2073955.6 |
  2076265.2 |
  2078574.7 |
  2080884.2 |
  2083193.8 |
  2085503.3 |
  2087812.8 |
  2090122.4 |
  2092431.9 |
  2094741.4 |
  2097051.0 |
  2099360.5 |
  2101670.0 |
  2103979.6 |
  2106289.1 |
  2108598.6 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_dispatch (n=6, range 3094820.0-3849388.8 ns)
  3094820.0 |########################################
  3132548.4 |
  3170276.9 |
  3208005.3 |
  3245733.8 |
  3283462.2 |
  3321190.6 |
  3358919.1 |
  3396647.5 |
  3434375.9 |
  3472104.4 |
  3509832.8 |
  3547561.2 |
  3585289.7 |
  3623018.1 |
  3660746.6 |
  3698475.0 |
  3736203.4 |
  3773931.9 |
  3811660.3 |##########################
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_runtime_w (n=6, range 3843369.6-3867244.1 ns)
  3843369.6 |#############
  3844563.3 |
  3845757.1 |
  3846950.8 |########################################
  3848144.5 |
  3849338.2 |
  3850532.0 |
  3851725.7 |
  3852919.4 |
  3854113.1 |
  3855306.9 |
  3856500.6 |
  3857694.3 |
  3858888.1 |
  3860081.8 |#############
  3861275.5 |
  3862469.2 |
  3863663.0 |
  3864856.7 |
  3866050.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_real_zig_anchor**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_dispatch**: bridge=312.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_null**: bridge=12138.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_per_w_set**: bridge=312.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_runtime_w**: bridge=312.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_dispatch**: bridge=308.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_runtime_w**: bridge=307.4% of algo (FFI overhead may distort results)
