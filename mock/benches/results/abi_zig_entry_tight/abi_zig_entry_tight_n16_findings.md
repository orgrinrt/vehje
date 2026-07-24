# abi_zig_entry (tight)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_tight_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_tight_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_tight_zig_null dominates: 80126% faster than the next best (abi_zig_entry_tight_zig_dispatch)

abi_zig_entry_tight_zig_null (2.49 us) leads abi_zig_entry_tight_zig_dispatch (2.00 ms) by 80126%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_tight_zig_null beats baseline by 100% (significant)

abi_zig_entry_tight_zig_null is -2.00 ms (100%) faster than baseline abi_zig_entry_tight_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_tight_zig_tail_runtime_w is an outlier: 1248.3x slower than the field

abi_zig_entry_tight_zig_tail_runtime_w (3.11 ms) is 1248.3x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_tight_zig_null} vs {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_tail_dispatch, abi_zig_entry_tight_zig_tail_runtime_w} (80126% apart)

The field splits into a fast tier {abi_zig_entry_tight_zig_null} and a slow tier {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_tail_dispatch, abi_zig_entry_tight_zig_tail_runtime_w} with a 80126% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1248.3x the fastest

Fastest abi_zig_entry_tight_zig_null (2.49 us) to slowest abi_zig_entry_tight_zig_tail_runtime_w (3.11 ms): 1248.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_tight_zig_null** at 2488.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1248.28x (fastest 2488.6 ns, slowest 3106411.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2019732ns | 2007071ns | 1999781ns | 2004851ns | 2052031ns | +0.39% |
| abi_zig_entry_tight_zig_dispatch | 2003578ns | 1999611ns | 1989850ns | 1998423ns | 2018176ns | -0.41% |
| abi_zig_entry_tight_zig_null | 4803ns | 4804ns | 4732ns | 4782ns | 4870ns | -99.76% |
| abi_zig_entry_tight_zig_per_w_set | 2021378ns | 2016757ns | 1997421ns | 2015374ns | 2042364ns | +0.48% |
| abi_zig_entry_tight_zig_runtime_w | 2011796ns | 2001943ns | 1991178ns | 1999383ns | 2040724ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3108102ns | 3105158ns | 3090742ns | 3102485ns | 3125208ns | +54.49% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3113115ns | 3109563ns | 3104291ns | 3108564ns | 3124355ns | +54.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2016743ns | 1996892ns | 2049017ns | +0.40% | 0.000 |
| abi_zig_entry_tight_zig_dispatch | 2000435ns | 1986997ns | 2014818ns | -0.41% | 0.000 |
| abi_zig_entry_tight_zig_null | 2490ns | 2463ns | 2516ns | -99.88% | 0.006 |
| abi_zig_entry_tight_zig_per_w_set | 2018328ns | 1994750ns | 2039229ns | +0.48% | 0.000 |
| abi_zig_entry_tight_zig_runtime_w | 2008725ns | 1988388ns | 2037287ns | base | 0.000 |
| abi_zig_entry_tight_zig_tail_dispatch | 3104968ns | 3087912ns | 3121935ns | +54.57% | 0.000 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3109960ns | 3101243ns | 3121088ns | +54.82% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 199692.3 | 2014302.4 | 2016742.8 | 1 |
| abi_zig_entry_tight_zig_dispatch | 207942.5 | 2000433.5 | 2000435.4 | 0 |
| abi_zig_entry_tight_zig_null | 155461.7 | 2722.5 | 2490.2 | n/a |
| abi_zig_entry_tight_zig_per_w_set | 207660.7 | 2016714.0 | 2018328.5 | n/a |
| abi_zig_entry_tight_zig_runtime_w | 205483.3 | 2015420.7 | 2008724.8 | n/a |
| abi_zig_entry_tight_zig_tail_dispatch | 213436.1 | 3101818.7 | 3104968.5 | n/a |
| abi_zig_entry_tight_zig_tail_runtime_w | 219368.9 | 3104725.6 | 3109960.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_zig_entry_tight_zig_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_null | 0.006 | 99.0% |
| abi_zig_entry_tight_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2019732ns | 2019732ns | +0.39% |
| abi_zig_entry_tight_zig_dispatch | 2003578ns | 2003578ns | -0.41% |
| abi_zig_entry_tight_zig_null | 4803ns | 4803ns | -99.76% |
| abi_zig_entry_tight_zig_per_w_set | 2021378ns | 2021378ns | +0.48% |
| abi_zig_entry_tight_zig_runtime_w | 2011796ns | 2011796ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3108102ns | 3108102ns | +54.49% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3113115ns | 3113115ns | +54.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_runtime_w | 1998942ns | base | --- | [1989946, 2037287] | --- | --- | --- | --- |
| abi_zig_entry_tight_zig_anchor | 2004062ns | no significant difference | [-5866, +21040]ns | [1997149, 2049017] | no | 0.8250 | 0.6875 | 0 |
| abi_zig_entry_tight_zig_dispatch | 1996460ns | no significant difference | [-40133, +14306]ns | [1990029, 2014818] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_tight_zig_null | 2489ns | -1996444.1ns (-99.9%) | [-2034797, -1987463]ns | [2466, 2516] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_per_w_set | 2013574ns | no significant difference | [-706, +17608]ns | [2002183, 2039229] | no | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_tight_zig_tail_dispatch | 3102003ns | +1101828.8ns (+55.1%) | [+1061452, +1125450]ns | [3090968, 3121935] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3106412ns | +1109481.2ns (+55.5%) | [+1069606, +1124618]ns | [3102380, 3121088] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_tight_zig_runtime_w | abi_zig_entry_tight_zig_anchor | abi_zig_entry_tight_zig_dispatch | abi_zig_entry_tight_zig_null | abi_zig_entry_tight_zig_per_w_set | abi_zig_entry_tight_zig_tail_dispatch | abi_zig_entry_tight_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1991504ns | +0.5% | +0.3% | -99.9% | +0.9% | +55.7% | +56.0% |
| 2 | 1999106ns | -0.1% | -0.2% | -99.9% | +0.9% | +57.1% | +55.2% |
| 3 | 2070022ns | -0.5% | -3.7% | -99.9% | -0.4% | +49.5% | +50.1% |
| 4 | 1998778ns | +0.4% | +0.2% | -99.9% | +0.7% | +55.2% | +55.2% |
| 5 | 2004552ns | +1.6% | +1.1% | -99.9% | +0.5% | +54.8% | +55.7% |
| 6 | 1988388ns | +0.5% | -0.1% | -99.9% | +0.3% | +55.3% | +56.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | -0.447 | moderate- |
| abi_zig_entry_tight_zig_dispatch | -0.281 | moderate- |
| abi_zig_entry_tight_zig_null | -0.096 | ok |
| abi_zig_entry_tight_zig_per_w_set | -0.099 | ok |
| abi_zig_entry_tight_zig_runtime_w | -0.194 | ok |
| abi_zig_entry_tight_zig_tail_dispatch | -0.281 | moderate- |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.236 | moderate+ |

**Consistency summary:**

- **abi_zig_entry_tight_zig_anchor**: won 2/6, lost 4/6
- **abi_zig_entry_tight_zig_dispatch**: won 2/6, lost 3/6
- **abi_zig_entry_tight_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_per_w_set**: won 1/6, lost 5/6
- **abi_zig_entry_tight_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 6324628.0ns | 2016742.8ns | 313.6% | HIGH |
| abi_zig_entry_tight_zig_dispatch | 6290648.4ns | 2000435.4ns | 314.5% | HIGH |
| abi_zig_entry_tight_zig_null | 301071.6ns | 2490.2ns | 12090.2% | HIGH |
| abi_zig_entry_tight_zig_per_w_set | 6385479.2ns | 2018328.5ns | 316.4% | HIGH |
| abi_zig_entry_tight_zig_runtime_w | 6299875.5ns | 2008724.8ns | 313.6% | HIGH |
| abi_zig_entry_tight_zig_tail_dispatch | 9600458.2ns | 3104968.5ns | 309.2% | HIGH |
| abi_zig_entry_tight_zig_tail_runtime_w | 9616120.9ns | 3109960.0ns | 309.2% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_tight_zig_anchor (n=6, range 1996891.7-2049017.4 ns)
  1996891.7 |########################################
  1999498.0 |####################
  2002104.3 |
  2004710.6 |
  2007316.8 |####################
  2009923.1 |
  2012529.4 |
  2015135.7 |
  2017742.0 |
  2020348.3 |
  2022954.6 |
  2025560.9 |
  2028167.1 |
  2030773.4 |
  2033379.7 |
  2035986.0 |####################
  2038592.3 |
  2041198.6 |
  2043804.9 |
  2046411.2 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_dispatch (n=6, range 1986996.7-2014817.7 ns)
  1986996.7 |########################################
  1988387.8 |
  1989778.8 |
  1991169.9 |
  1992560.9 |########################################
  1993951.9 |
  1995343.0 |########################################
  1996734.1 |########################################
  1998125.1 |
  1999516.2 |
  2000907.2 |########################################
  2002298.2 |
  2003689.3 |
  2005080.4 |
  2006471.4 |
  2007862.5 |
  2009253.5 |
  2010644.6 |
  2012035.6 |
  2013426.7 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_null (n=6, range 2463.3-2515.8 ns)
   2463.3 |########################################
   2465.9 |
   2468.6 |########################################
   2471.2 |
   2473.8 |
   2476.4 |
   2479.1 |########################################
   2481.7 |
   2484.3 |
   2486.9 |
   2489.6 |
   2492.2 |
   2494.8 |########################################
   2497.5 |
   2500.1 |
   2502.7 |
   2505.3 |
   2508.0 |
   2510.6 |
   2513.2 |########################################
  (0 below, 1 above range)

abi_zig_entry_tight_zig_per_w_set (n=6, range 1994749.6-2039228.9 ns)
  1994749.6 |####################
  1996973.6 |
  1999197.5 |
  2001421.5 |
  2003645.5 |
  2005869.4 |
  2008093.4 |####################
  2010317.4 |####################
  2012541.3 |
  2014765.3 |########################################
  2016989.3 |
  2019213.2 |
  2021437.2 |
  2023661.2 |
  2025885.1 |
  2028109.1 |
  2030333.1 |
  2032557.0 |
  2034781.0 |
  2037005.0 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_runtime_w (n=6, range 1988387.5-2037286.9 ns)
  1988387.5 |####################
  1990832.5 |####################
  1993277.4 |
  1995722.4 |
  1998167.4 |########################################
  2000612.4 |
  2003057.3 |####################
  2005502.3 |
  2007947.3 |
  2010392.2 |
  2012837.2 |
  2015282.2 |
  2017727.1 |
  2020172.1 |
  2022617.1 |
  2025062.0 |
  2027507.0 |
  2029952.0 |
  2032397.0 |
  2034841.9 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_dispatch (n=6, range 3087911.7-3121935.2 ns)
  3087911.7 |########################################
  3089612.9 |
  3091314.1 |
  3093015.2 |########################################
  3094716.4 |
  3096417.6 |
  3098118.8 |
  3099819.9 |########################################
  3101521.1 |########################################
  3103222.3 |########################################
  3104923.5 |
  3106624.6 |
  3108325.8 |
  3110027.0 |
  3111728.2 |
  3113429.3 |
  3115130.5 |
  3116831.7 |
  3118532.9 |
  3120234.0 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_runtime_w (n=6, range 3101242.9-3121087.7 ns)
  3101242.9 |########################################
  3102235.1 |
  3103227.4 |########################################
  3104219.6 |
  3105211.9 |########################################
  3106204.1 |########################################
  3107196.3 |
  3108188.6 |
  3109180.8 |
  3110173.1 |
  3111165.3 |
  3112157.5 |
  3113149.8 |
  3114142.0 |
  3115134.3 |
  3116126.5 |
  3117118.7 |
  3118111.0 |
  3119103.2 |
  3120095.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_tight_zig_anchor**: bridge=313.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_dispatch**: bridge=314.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_null**: bridge=12122.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_per_w_set**: bridge=313.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_runtime_w**: bridge=313.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_dispatch**: bridge=308.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_runtime_w**: bridge=309.2% of algo (FFI overhead may distort results)
