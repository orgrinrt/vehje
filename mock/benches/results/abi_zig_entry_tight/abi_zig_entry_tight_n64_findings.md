# abi_zig_entry (tight)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_tight_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_tight_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_tight_zig_null dominates: 79635% faster than the next best (abi_zig_entry_tight_zig_dispatch)

abi_zig_entry_tight_zig_null (2.48 us) leads abi_zig_entry_tight_zig_dispatch (1.98 ms) by 79635%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_tight_zig_null beats baseline by 100% (significant)

abi_zig_entry_tight_zig_null is -1.99 ms (100%) faster than baseline abi_zig_entry_tight_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_tight_zig_tail_dispatch is an outlier: 1245.3x slower than the field

abi_zig_entry_tight_zig_tail_dispatch (3.09 ms) is 1245.3x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_tight_zig_null} vs {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} (79635% apart)

The field splits into a fast tier {abi_zig_entry_tight_zig_null} and a slow tier {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} with a 79635% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1245.3x the fastest

Fastest abi_zig_entry_tight_zig_null (2.48 us) to slowest abi_zig_entry_tight_zig_tail_dispatch (3.09 ms): 1245.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_tight_zig_null** at 2484.6 ns median (-99.9% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1245.25x (fastest 2484.6 ns, slowest 3093888.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2000543ns | 1995590ns | 1990432ns | 1993958ns | 2015477ns | -0.62% |
| abi_zig_entry_tight_zig_dispatch | 1984360ns | 1983693ns | 1979787ns | 1983588ns | 1987804ns | -1.43% |
| abi_zig_entry_tight_zig_null | 4779ns | 4811ns | 4624ns | 4786ns | 4846ns | -99.76% |
| abi_zig_entry_tight_zig_per_w_set | 1984009ns | 1983835ns | 1982344ns | 1983492ns | 1985615ns | -1.44% |
| abi_zig_entry_tight_zig_runtime_w | 2013067ns | 1994350ns | 1984299ns | 1991815ns | 2059329ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3084866ns | 3096928ns | 3034144ns | 3093133ns | 3097825ns | +53.24% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3093594ns | 3094089ns | 3083472ns | 3093576ns | 3098682ns | +53.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1997746ns | 1987779ns | 2012447ns | -0.62% | 0.000 |
| abi_zig_entry_tight_zig_dispatch | 1981670ns | 1977121ns | 1985068ns | -1.42% | 0.000 |
| abi_zig_entry_tight_zig_null | 2470ns | 2376ns | 2513ns | -99.88% | 0.026 |
| abi_zig_entry_tight_zig_per_w_set | 1981292ns | 1979577ns | 1982899ns | -1.44% | 0.000 |
| abi_zig_entry_tight_zig_runtime_w | 2010164ns | 1981570ns | 2056212ns | base | 0.000 |
| abi_zig_entry_tight_zig_tail_dispatch | 3081941ns | 3031254ns | 3094813ns | +53.32% | 0.000 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3090742ns | 3080555ns | 3095854ns | +53.76% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 186015.1 | 1995392.1 | 1997745.8 | n/a |
| abi_zig_entry_tight_zig_dispatch | 178718.1 | 1980971.3 | 1981669.7 | n/a |
| abi_zig_entry_tight_zig_null | 155338.0 | 2759.0 | 2469.6 | n/a |
| abi_zig_entry_tight_zig_per_w_set | 180483.8 | 1978841.4 | 1981292.5 | n/a |
| abi_zig_entry_tight_zig_runtime_w | 195733.6 | 2006753.5 | 2010163.8 | n/a |
| abi_zig_entry_tight_zig_tail_dispatch | 193826.7 | 3079009.6 | 3081941.1 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 191664.7 | 3086681.8 | 3090742.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_zig_entry_tight_zig_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_null | 0.026 | 95.6% |
| abi_zig_entry_tight_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2000543ns | 2000543ns | -0.62% |
| abi_zig_entry_tight_zig_dispatch | 1984360ns | 1984360ns | -1.43% |
| abi_zig_entry_tight_zig_null | 4779ns | 4779ns | -99.76% |
| abi_zig_entry_tight_zig_per_w_set | 1984009ns | 1984009ns | -1.44% |
| abi_zig_entry_tight_zig_runtime_w | 2013067ns | 2013067ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3084866ns | 3084866ns | +53.24% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3093594ns | 3093594ns | +53.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_runtime_w | 1991513ns | base | --- | [1982766, 2056212] | --- | --- | --- | --- |
| abi_zig_entry_tight_zig_anchor | 1992940ns | no significant difference | [-46884, +13122]ns | [1987851, 2012447] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_tight_zig_dispatch | 1981054ns | -10613.1ns (-0.5%) | [-72795, -2074]ns | [1978887, 1985068] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_null | 2485ns | -1989065.9ns (-99.9%) | [-2053713, -1980304]ns | [2411, 2513] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_per_w_set | 1981098ns | -10759.6ns (-0.5%) | [-73313, -2541]ns | [1979881, 1982899] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_dispatch | 3093888ns | +1099678.0ns (+55.2%) | [+1007827, +1107827]ns | [3057122, 3094813] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3091285ns | +1102666.8ns (+55.4%) | [+1030313, +1108755]ns | [3085087, 3095854] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_tight_zig_runtime_w | abi_zig_entry_tight_zig_anchor | abi_zig_entry_tight_zig_dispatch | abi_zig_entry_tight_zig_null | abi_zig_entry_tight_zig_per_w_set | abi_zig_entry_tight_zig_tail_dispatch | abi_zig_entry_tight_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1981570ns | +0.7% | -0.0% | -99.9% | -0.1% | +55.6% | +55.9% |
| 2 | 1988264ns | -0.0% | -0.6% | -99.9% | -0.4% | +55.6% | +55.4% |
| 3 | 1996601ns | -0.3% | -0.5% | -99.9% | -0.7% | +55.0% | +54.3% |
| 4 | 1983962ns | +0.6% | -0.2% | -99.9% | -0.2% | +55.9% | +55.9% |
| 5 | 1994762ns | -0.3% | -0.5% | -99.9% | -0.7% | +52.0% | +55.3% |
| 6 | 2115823ns | -4.1% | -6.4% | -99.9% | -6.2% | +46.3% | +46.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | -0.146 | ok |
| abi_zig_entry_tight_zig_dispatch | -0.465 | moderate- |
| abi_zig_entry_tight_zig_null | 0.026 | ok |
| abi_zig_entry_tight_zig_per_w_set | 0.053 | ok |
| abi_zig_entry_tight_zig_runtime_w | 0.004 | ok |
| abi_zig_entry_tight_zig_tail_dispatch | -0.288 | moderate- |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.120 | ok |

**Consistency summary:**

- **abi_zig_entry_tight_zig_anchor**: won 3/6, lost 2/6
- **abi_zig_entry_tight_zig_dispatch**: won 5/6, lost 0/6
- **abi_zig_entry_tight_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_per_w_set**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 6242247.6ns | 1997745.8ns | 312.5% | HIGH |
| abi_zig_entry_tight_zig_dispatch | 6196556.9ns | 1981669.7ns | 312.7% | HIGH |
| abi_zig_entry_tight_zig_null | 297605.3ns | 2469.6ns | 12050.7% | HIGH |
| abi_zig_entry_tight_zig_per_w_set | 6190230.9ns | 1981292.5ns | 312.4% | HIGH |
| abi_zig_entry_tight_zig_runtime_w | 6302130.0ns | 2010163.8ns | 313.5% | HIGH |
| abi_zig_entry_tight_zig_tail_dispatch | 9509285.5ns | 3081941.1ns | 308.5% | HIGH |
| abi_zig_entry_tight_zig_tail_runtime_w | 9527758.5ns | 3090742.1ns | 308.3% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_tight_zig_anchor (n=6, range 1987778.8-2012446.9 ns)
  1987778.8 |########################################
  1989012.2 |####################
  1990245.6 |
  1991479.0 |
  1992712.4 |
  1993945.8 |
  1995179.2 |########################################
  1996412.6 |
  1997646.0 |
  1998879.4 |
  2000112.8 |
  2001346.2 |
  2002579.6 |
  2003813.0 |
  2005046.4 |
  2006279.8 |
  2007513.2 |
  2008746.6 |
  2009980.0 |
  2011213.4 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_dispatch (n=6, range 1977121.2-1985068.4 ns)
  1977121.2 |########################################
  1977518.6 |
  1977915.9 |
  1978313.3 |
  1978710.6 |
  1979108.0 |
  1979505.3 |
  1979902.7 |
  1980300.1 |########################################
  1980697.4 |########################################
  1981094.8 |########################################
  1981492.1 |
  1981889.5 |
  1982286.8 |
  1982684.2 |
  1983081.6 |
  1983478.9 |
  1983876.3 |
  1984273.6 |########################################
  1984671.0 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_null (n=6, range 2375.8-2513.3 ns)
   2375.8 |########################################
   2382.7 |
   2389.6 |
   2396.4 |
   2403.3 |
   2410.2 |
   2417.1 |
   2423.9 |
   2430.8 |
   2437.7 |
   2444.6 |########################################
   2451.4 |
   2458.3 |
   2465.2 |
   2472.1 |########################################
   2478.9 |
   2485.8 |########################################
   2492.7 |
   2499.6 |
   2506.4 |########################################
  (0 below, 1 above range)

abi_zig_entry_tight_zig_per_w_set (n=6, range 1979577.1-1982898.9 ns)
  1979577.1 |########################################
  1979743.2 |
  1979909.3 |
  1980075.4 |########################################
  1980241.5 |
  1980407.6 |
  1980573.7 |
  1980739.7 |########################################
  1980905.8 |
  1981071.9 |
  1981238.0 |########################################
  1981404.1 |
  1981570.2 |
  1981736.3 |
  1981902.4 |
  1982068.5 |########################################
  1982234.6 |
  1982400.7 |
  1982566.8 |
  1982732.9 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_runtime_w (n=6, range 1981570.4-2056212.2 ns)
  1981570.4 |########################################
  1985302.5 |####################
  1989034.6 |
  1992766.7 |####################
  1996498.8 |####################
  2000230.9 |
  2003963.0 |
  2007695.0 |
  2011427.1 |
  2015159.2 |
  2018891.3 |
  2022623.4 |
  2026355.5 |
  2030087.6 |
  2033819.7 |
  2037551.8 |
  2041283.9 |
  2045016.0 |
  2048748.1 |
  2052480.2 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_dispatch (n=6, range 3031254.2-3094813.2 ns)
  3031254.2 |#############
  3034432.1 |
  3037610.1 |
  3040788.0 |
  3043966.0 |
  3047143.9 |
  3050321.9 |
  3053499.8 |
  3056677.8 |
  3059855.7 |
  3063033.7 |
  3066211.6 |
  3069389.6 |
  3072567.5 |
  3075745.5 |
  3078923.4 |
  3082101.4 |#############
  3085279.3 |
  3088457.3 |
  3091635.2 |########################################
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_runtime_w (n=6, range 3080555.0-3095854.3 ns)
  3080555.0 |########################################
  3081320.0 |
  3082084.9 |
  3082849.9 |
  3083614.9 |
  3084379.8 |
  3085144.8 |
  3085909.8 |
  3086674.7 |
  3087439.7 |
  3088204.7 |
  3088969.6 |########################################
  3089734.6 |########################################
  3090499.6 |
  3091264.5 |
  3092029.5 |########################################
  3092794.5 |########################################
  3093559.4 |
  3094324.4 |
  3095089.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_tight_zig_anchor**: bridge=312.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_dispatch**: bridge=312.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_null**: bridge=11981.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_per_w_set**: bridge=312.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_runtime_w**: bridge=312.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_dispatch**: bridge=308.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_runtime_w**: bridge=308.1% of algo (FFI overhead may distort results)
