# abi_zig_entry (real)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_real_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_real_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_real_zig_null dominates: 66158% faster than the next best (abi_zig_entry_real_zig_dispatch)

abi_zig_entry_real_zig_null (3.11 us) leads abi_zig_entry_real_zig_dispatch (2.06 ms) by 66158%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_real_zig_null beats baseline by 100% (significant)

abi_zig_entry_real_zig_null is -2.06 ms (100%) faster than baseline abi_zig_entry_real_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_real_zig_tail_runtime_w is an outlier: 1234.7x slower than the field

abi_zig_entry_real_zig_tail_runtime_w (3.85 ms) is 1234.7x the fastest (3.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_real_zig_anchor shows alternating (throttle bounce) (autocorr -0.64)

abi_zig_entry_real_zig_anchor's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_real_zig_null} vs {abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} (66158% apart)

The field splits into a fast tier {abi_zig_entry_real_zig_null} and a slow tier {abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} with a 66158% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1234.7x the fastest

Fastest abi_zig_entry_real_zig_null (3.11 us) to slowest abi_zig_entry_real_zig_tail_runtime_w (3.85 ms): 1234.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_real_zig_null** at 3114.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1234.68x (fastest 3114.2 ns, slowest 3845035.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2077130ns | 2078158ns | 2069165ns | 2076943ns | 2081393ns | +0.44% |
| abi_zig_entry_real_zig_dispatch | 2083031ns | 2066088ns | 2063134ns | 2065652ns | 2119048ns | +0.72% |
| abi_zig_entry_real_zig_null | 5425ns | 5396ns | 5349ns | 5391ns | 5512ns | -99.74% |
| abi_zig_entry_real_zig_per_w_set | 2127229ns | 2068301ns | 2060902ns | 2068065ns | 2249139ns | +2.86% |
| abi_zig_entry_real_zig_runtime_w | 2068079ns | 2068282ns | 2060882ns | 2067303ns | 2072842ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3723254ns | 3843945ns | 3101420ns | 3843561ns | 3853711ns | +80.03% |
| abi_zig_entry_real_zig_tail_runtime_w | 3724807ns | 3847919ns | 3095676ns | 3844339ns | 3860074ns | +80.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2074425ns | 2066512ns | 2078717ns | +0.44% | 0.000 |
| abi_zig_entry_real_zig_dispatch | 2080288ns | 2060628ns | 2116094ns | +0.72% | 0.000 |
| abi_zig_entry_real_zig_null | 3112ns | 3066ns | 3138ns | -99.85% | 0.082 |
| abi_zig_entry_real_zig_per_w_set | 2124535ns | 2058453ns | 2246216ns | +2.86% | 0.000 |
| abi_zig_entry_real_zig_runtime_w | 2065386ns | 2058345ns | 2070034ns | base | 0.000 |
| abi_zig_entry_real_zig_tail_dispatch | 3720426ns | 3098536ns | 3850919ns | +80.13% | 0.000 |
| abi_zig_entry_real_zig_tail_runtime_w | 3721826ns | 3092805ns | 3856902ns | +80.20% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 181909.6 | 2070826.3 | 2074424.7 | n/a |
| abi_zig_entry_real_zig_dispatch | 191179.0 | 2093576.9 | 2080287.6 | n/a |
| abi_zig_entry_real_zig_null | 155996.2 | 3192.0 | 3112.4 | n/a |
| abi_zig_entry_real_zig_per_w_set | 185752.6 | 2131195.2 | 2124534.6 | n/a |
| abi_zig_entry_real_zig_runtime_w | 178332.9 | 2065204.4 | 2065385.8 | n/a |
| abi_zig_entry_real_zig_tail_dispatch | 190595.1 | 3727514.9 | 3720426.2 | n/a |
| abi_zig_entry_real_zig_tail_runtime_w | 196269.4 | 3727164.4 | 3721826.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.084 Gops/s** (abi_zig_entry_real_zig_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_real_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_null | 0.082 | 98.4% |
| abi_zig_entry_real_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_real_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2077130ns | 2077130ns | +0.44% |
| abi_zig_entry_real_zig_dispatch | 2083031ns | 2083031ns | +0.72% |
| abi_zig_entry_real_zig_null | 5425ns | 5425ns | -99.74% |
| abi_zig_entry_real_zig_per_w_set | 2127229ns | 2127229ns | +2.86% |
| abi_zig_entry_real_zig_runtime_w | 2068079ns | 2068079ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3723254ns | 3723254ns | +80.03% |
| abi_zig_entry_real_zig_tail_runtime_w | 3724807ns | 3724807ns | +80.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_runtime_w | 2065601ns | base | --- | [2060522, 2070034] | --- | --- | --- | --- |
| abi_zig_entry_real_zig_anchor | 2075397ns | +9372.9ns (+0.5%) | [+4929, +12815]ns | [2069160, 2078717] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_dispatch | 2063397ns | no significant difference | [-7543, +55571]ns | [2061372, 2116094] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_real_zig_null | 3114ns | -2062490.4ns (-99.8%) | [-2066917, -2057413]ns | [3085, 3138] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_per_w_set | 2065655ns | no significant difference | [-3059, +180192]ns | [2061732, 2246216] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_real_zig_tail_dispatch | 3840993ns | +1775768.8ns (+86.0%) | [+1402022, +1787330]ns | [3469366, 3850919] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_tail_runtime_w | 3845036ns | +1779434.6ns (+86.1%) | [+1399694, +1790193]ns | [3463541, 3856902] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_real_zig_runtime_w | abi_zig_entry_real_zig_anchor | abi_zig_entry_real_zig_dispatch | abi_zig_entry_real_zig_null | abi_zig_entry_real_zig_per_w_set | abi_zig_entry_real_zig_tail_dispatch | abi_zig_entry_real_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2070719ns | +0.4% | -0.3% | -99.8% | -0.2% | +85.5% | +86.4% |
| 2 | 2064493ns | +0.1% | -0.1% | -99.9% | +0.0% | +50.1% | +86.1% |
| 3 | 2069348ns | +0.4% | -0.4% | -99.8% | +0.1% | +85.9% | +85.3% |
| 4 | 2062700ns | +0.5% | +4.9% | -99.8% | +17.4% | +86.9% | +86.9% |
| 5 | 2066710ns | +0.6% | -0.2% | -99.8% | -0.1% | +85.9% | +86.2% |
| 6 | 2058345ns | +0.7% | +0.5% | -99.8% | +0.0% | +86.6% | +50.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_real_zig_anchor | -0.637 | HIGH- (thermal bounce) |
| abi_zig_entry_real_zig_dispatch | -0.274 | moderate- |
| abi_zig_entry_real_zig_null | -0.163 | ok |
| abi_zig_entry_real_zig_per_w_set | -0.215 | moderate- |
| abi_zig_entry_real_zig_runtime_w | -0.307 | moderate- |
| abi_zig_entry_real_zig_tail_dispatch | -0.227 | moderate- |
| abi_zig_entry_real_zig_tail_runtime_w | -0.036 | ok |

**Consistency summary:**

- **abi_zig_entry_real_zig_anchor**: won 0/6, lost 5/6
- **abi_zig_entry_real_zig_dispatch**: won 3/6, lost 2/6
- **abi_zig_entry_real_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_real_zig_per_w_set**: won 1/6, lost 2/6
- **abi_zig_entry_real_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_real_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 6467075.9ns | 2074424.7ns | 311.8% | HIGH |
| abi_zig_entry_real_zig_dispatch | 6540702.4ns | 2080287.6ns | 314.4% | HIGH |
| abi_zig_entry_real_zig_null | 305637.2ns | 3112.4ns | 9820.1% | HIGH |
| abi_zig_entry_real_zig_per_w_set | 6549377.2ns | 2124534.6ns | 308.3% | HIGH |
| abi_zig_entry_real_zig_runtime_w | 6444247.6ns | 2065385.8ns | 312.0% | HIGH |
| abi_zig_entry_real_zig_tail_dispatch | 11431178.7ns | 3720426.2ns | 307.3% | HIGH |
| abi_zig_entry_real_zig_tail_runtime_w | 11442609.2ns | 3721826.3ns | 307.4% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_real_zig_anchor (n=6, range 2066512.1-2078717.1 ns)
  2066512.1 |########################################
  2067122.4 |
  2067732.6 |
  2068342.9 |
  2068953.1 |
  2069563.4 |
  2070173.6 |
  2070783.9 |
  2071394.1 |########################################
  2072004.4 |
  2072614.6 |
  2073224.9 |########################################
  2073835.1 |
  2074445.4 |
  2075055.6 |
  2075665.9 |
  2076276.1 |
  2076886.4 |########################################
  2077496.6 |
  2078106.9 |########################################
  (0 below, 1 above range)

abi_zig_entry_real_zig_dispatch (n=6, range 2060627.5-2116094.0 ns)
  2060627.5 |########################################
  2063400.8 |#############
  2066174.1 |#############
  2068947.5 |
  2071720.8 |
  2074494.1 |
  2077267.4 |
  2080040.8 |
  2082814.1 |
  2085587.4 |
  2088360.7 |
  2091134.0 |
  2093907.4 |
  2096680.7 |
  2099454.0 |
  2102227.3 |
  2105000.7 |
  2107774.0 |
  2110547.3 |
  2113320.6 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_null (n=6, range 3065.8-3137.5 ns)
   3065.8 |####################
   3069.4 |
   3073.0 |
   3076.6 |
   3080.1 |
   3083.7 |
   3087.3 |
   3090.9 |
   3094.5 |
   3098.1 |
   3101.7 |####################
   3105.2 |
   3108.8 |
   3112.4 |########################################
   3116.0 |####################
   3119.6 |
   3123.2 |
   3126.7 |
   3130.3 |
   3133.9 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_per_w_set (n=6, range 2058452.9-2246216.5 ns)
  2058452.9 |########################################
  2067841.1 |##########
  2077229.3 |
  2086617.4 |
  2096005.6 |
  2105393.8 |
  2114782.0 |
  2124170.1 |
  2133558.3 |
  2142946.5 |
  2152334.7 |
  2161722.9 |
  2171111.0 |
  2180499.2 |
  2189887.4 |
  2199275.6 |
  2208663.7 |
  2218051.9 |
  2227440.1 |
  2236828.3 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_runtime_w (n=6, range 2058345.0-2070033.8 ns)
  2058345.0 |########################################
  2058929.4 |
  2059513.9 |
  2060098.3 |
  2060682.8 |
  2061267.2 |
  2061851.6 |
  2062436.1 |########################################
  2063020.5 |
  2063604.9 |
  2064189.4 |########################################
  2064773.8 |
  2065358.2 |
  2065942.7 |
  2066527.1 |########################################
  2067111.6 |
  2067696.0 |
  2068280.4 |
  2068864.9 |########################################
  2069449.3 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_dispatch (n=6, range 3098535.8-3850919.4 ns)
  3098535.8 |##########
  3136155.0 |
  3173774.2 |
  3211393.3 |
  3249012.5 |
  3286631.7 |
  3324250.9 |
  3361870.0 |
  3399489.2 |
  3437108.4 |
  3474727.6 |
  3512346.8 |
  3549965.9 |
  3587585.1 |
  3625204.3 |
  3662823.5 |
  3700442.6 |
  3738061.8 |
  3775681.0 |
  3813300.2 |########################################
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_runtime_w (n=6, range 3092805.4-3856902.3 ns)
  3092805.4 |##########
  3131010.2 |
  3169215.1 |
  3207419.9 |
  3245624.8 |
  3283829.6 |
  3322034.5 |
  3360239.3 |
  3398444.2 |
  3436649.0 |
  3474853.8 |
  3513058.7 |
  3551263.5 |
  3589468.4 |
  3627673.2 |
  3665878.1 |
  3704082.9 |
  3742287.8 |
  3780492.6 |
  3818697.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_real_zig_anchor**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_dispatch**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_null**: bridge=9782.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_per_w_set**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_runtime_w**: bridge=311.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_dispatch**: bridge=307.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_runtime_w**: bridge=307.1% of algo (FFI overhead may distort results)
