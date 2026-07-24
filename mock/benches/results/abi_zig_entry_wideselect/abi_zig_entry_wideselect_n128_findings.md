# abi_zig_entry (wideselect)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_wideselect_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_wideselect_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_wideselect_zig_null dominates: 73469% faster than the next best (abi_zig_entry_wideselect_zig_runtime_w)

abi_zig_entry_wideselect_zig_null (2.68 us) leads abi_zig_entry_wideselect_zig_runtime_w (1.97 ms) by 73469%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_wideselect_zig_null beats baseline by 100% (significant)

abi_zig_entry_wideselect_zig_null is -1.97 ms (100%) faster than baseline abi_zig_entry_wideselect_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_wideselect_zig_tail_dispatch is an outlier: 1235.9x slower than the field

abi_zig_entry_wideselect_zig_tail_dispatch (3.31 ms) is 1235.9x the fastest (2.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_wideselect_zig_dispatch shows alternating (throttle bounce) (autocorr -0.50)

abi_zig_entry_wideselect_zig_dispatch's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_wideselect_zig_null} vs {abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch} (73469% apart)

The field splits into a fast tier {abi_zig_entry_wideselect_zig_null} and a slow tier {abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch} with a 73469% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1235.9x the fastest

Fastest abi_zig_entry_wideselect_zig_null (2.68 us) to slowest abi_zig_entry_wideselect_zig_tail_dispatch (3.31 ms): 1235.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_wideselect_zig_null** at 2679.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1235.91x (fastest 2679.8 ns, slowest 3311934.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1992114ns | 1980254ns | 1976097ns | 1979292ns | 2019355ns | +0.90% |
| abi_zig_entry_wideselect_zig_dispatch | 1976955ns | 1975147ns | 1972536ns | 1974566ns | 1982747ns | +0.13% |
| abi_zig_entry_wideselect_zig_null | 4975ns | 4976ns | 4866ns | 4965ns | 5044ns | -99.75% |
| abi_zig_entry_wideselect_zig_per_w_set | 1976281ns | 1975705ns | 1970389ns | 1974428ns | 1982007ns | +0.10% |
| abi_zig_entry_wideselect_zig_runtime_w | 1974365ns | 1974106ns | 1971477ns | 1973832ns | 1976610ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3393591ns | 3314889ns | 3307942ns | 3313193ns | 3557014ns | +71.88% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3308690ns | 3305484ns | 3304410ns | 3305419ns | 3315735ns | +67.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1989404ns | 1973512ns | 2016458ns | +0.90% | 0.000 |
| abi_zig_entry_wideselect_zig_dispatch | 1974221ns | 1969960ns | 1979878ns | +0.13% | 0.000 |
| abi_zig_entry_wideselect_zig_null | 2678ns | 2630ns | 2710ns | -99.86% | 0.048 |
| abi_zig_entry_wideselect_zig_per_w_set | 1973659ns | 1967878ns | 1979311ns | +0.10% | 0.000 |
| abi_zig_entry_wideselect_zig_runtime_w | 1971713ns | 1968829ns | 1973936ns | base | 0.000 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3390678ns | 3305199ns | 3554018ns | +71.97% | 0.000 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3305973ns | 3301741ns | 3312951ns | +67.67% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 190959.5 | 1987422.9 | 1989404.1 | n/a |
| abi_zig_entry_wideselect_zig_dispatch | 181882.1 | 1974582.3 | 1974220.8 | n/a |
| abi_zig_entry_wideselect_zig_null | 159259.4 | 2762.0 | 2678.3 | n/a |
| abi_zig_entry_wideselect_zig_per_w_set | 179705.6 | 1973137.6 | 1973659.2 | n/a |
| abi_zig_entry_wideselect_zig_runtime_w | 177069.4 | 1971442.9 | 1971713.0 | n/a |
| abi_zig_entry_wideselect_zig_tail_dispatch | 195982.9 | 3405598.4 | 3390678.3 | n/a |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 188453.8 | 3304529.0 | 3305973.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_zig_entry_wideselect_zig_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_null | 0.048 | 98.1% |
| abi_zig_entry_wideselect_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1992114ns | 1992114ns | +0.90% |
| abi_zig_entry_wideselect_zig_dispatch | 1976955ns | 1976955ns | +0.13% |
| abi_zig_entry_wideselect_zig_null | 4975ns | 4975ns | -99.75% |
| abi_zig_entry_wideselect_zig_per_w_set | 1976281ns | 1976281ns | +0.10% |
| abi_zig_entry_wideselect_zig_runtime_w | 1974365ns | 1974365ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3393591ns | 3393591ns | +71.88% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3308690ns | 3308690ns | +67.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_runtime_w | 1971467ns | base | --- | [1969736, 1973936] | --- | --- | --- | --- |
| abi_zig_entry_wideselect_zig_anchor | 1977642ns | +5089.2ns (+0.3%) | [+1799, +46185]ns | [1974112, 2016458] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_dispatch | 1972462ns | no significant difference | [-856, +7345]ns | [1970323, 1979878] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_wideselect_zig_null | 2680ns | -1968780.4ns (-99.9%) | [-1971277, -1967046]ns | [2645, 2710] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_per_w_set | 1973050ns | no significant difference | [-4392, +9576]ns | [1968616, 1979311] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3311934ns | +1340732.9ns (+68.0%) | [+1335153, +1581011]ns | [3306083, 3554018] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3302767ns | +1332689.5ns (+67.6%) | [+1329785, +1340307]ns | [3302201, 3312951] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_wideselect_zig_runtime_w | abi_zig_entry_wideselect_zig_anchor | abi_zig_entry_wideselect_zig_dispatch | abi_zig_entry_wideselect_zig_null | abi_zig_entry_wideselect_zig_per_w_set | abi_zig_entry_wideselect_zig_tail_dispatch | abi_zig_entry_wideselect_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1974300ns | +0.0% | +0.1% | -99.9% | -0.3% | +80.7% | +67.3% |
| 2 | 1971716ns | +3.9% | -0.1% | -99.9% | -0.2% | +79.6% | +67.9% |
| 3 | 1973573ns | +0.3% | -0.0% | -99.9% | -0.0% | +67.8% | +68.0% |
| 4 | 1970642ns | +0.1% | -0.0% | -99.9% | +0.7% | +67.8% | +67.5% |
| 5 | 1971218ns | +0.2% | +0.6% | -99.9% | +0.1% | +67.7% | +67.5% |
| 6 | 1968829ns | +0.8% | +0.1% | -99.9% | +0.3% | +68.2% | +67.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | -0.218 | moderate- |
| abi_zig_entry_wideselect_zig_dispatch | -0.503 | HIGH- (thermal bounce) |
| abi_zig_entry_wideselect_zig_null | -0.287 | moderate- |
| abi_zig_entry_wideselect_zig_per_w_set | 0.096 | ok |
| abi_zig_entry_wideselect_zig_runtime_w | -0.001 | ok |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.436 | moderate+ |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 0.065 | ok |

**Consistency summary:**

- **abi_zig_entry_wideselect_zig_anchor**: won 0/6, lost 5/6
- **abi_zig_entry_wideselect_zig_dispatch**: won 0/6, lost 3/6
- **abi_zig_entry_wideselect_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_wideselect_zig_per_w_set**: won 2/6, lost 3/6
- **abi_zig_entry_wideselect_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 6219697.6ns | 1989404.1ns | 312.6% | HIGH |
| abi_zig_entry_wideselect_zig_dispatch | 6176213.3ns | 1974220.8ns | 312.8% | HIGH |
| abi_zig_entry_wideselect_zig_null | 309467.3ns | 2678.3ns | 11554.5% | HIGH |
| abi_zig_entry_wideselect_zig_per_w_set | 6169006.4ns | 1973659.2ns | 312.6% | HIGH |
| abi_zig_entry_wideselect_zig_runtime_w | 6163089.1ns | 1971713.0ns | 312.6% | HIGH |
| abi_zig_entry_wideselect_zig_tail_dispatch | 10435574.0ns | 3390678.3ns | 307.8% | HIGH |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 10176932.2ns | 3305973.3ns | 307.8% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_wideselect_zig_anchor (n=6, range 1973511.7-2016457.7 ns)
  1973511.7 |########################################
  1975659.0 |
  1977806.3 |
  1979953.6 |#############
  1982100.9 |#############
  1984248.2 |
  1986395.5 |
  1988542.8 |
  1990690.1 |
  1992837.4 |
  1994984.7 |
  1997132.0 |
  1999279.3 |
  2001426.6 |
  2003573.9 |
  2005721.2 |
  2007868.5 |
  2010015.8 |
  2012163.1 |
  2014310.4 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_dispatch (n=6, range 1969959.6-1979877.9 ns)
  1969959.6 |########################################
  1970455.5 |########################################
  1970951.4 |
  1971447.3 |########################################
  1971943.3 |
  1972439.2 |
  1972935.1 |########################################
  1973431.0 |
  1973926.9 |
  1974422.8 |
  1974918.8 |
  1975414.7 |
  1975910.6 |
  1976406.5 |########################################
  1976902.4 |
  1977398.3 |
  1977894.2 |
  1978390.2 |
  1978886.1 |
  1979382.0 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_null (n=6, range 2630.0-2710.4 ns)
   2630.0 |########################################
   2634.0 |
   2638.0 |
   2642.1 |
   2646.1 |
   2650.1 |
   2654.1 |
   2658.1 |########################################
   2662.2 |
   2666.2 |
   2670.2 |########################################
   2674.2 |
   2678.2 |
   2682.3 |
   2686.3 |########################################
   2690.3 |
   2694.3 |
   2698.3 |
   2702.4 |
   2706.4 |########################################
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_per_w_set (n=6, range 1967877.9-1979311.5 ns)
  1967877.9 |########################################
  1968449.6 |
  1969021.3 |########################################
  1969592.9 |
  1970164.6 |
  1970736.3 |
  1971308.0 |
  1971879.6 |
  1972451.3 |########################################
  1973023.0 |########################################
  1973594.7 |########################################
  1974166.4 |
  1974738.0 |
  1975309.7 |
  1975881.4 |
  1976453.1 |
  1977024.7 |
  1977596.4 |
  1978168.1 |
  1978739.8 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_runtime_w (n=6, range 1968828.7-1973936.5 ns)
  1968828.7 |########################################
  1969084.1 |
  1969339.5 |
  1969594.9 |
  1969850.2 |
  1970105.6 |
  1970361.0 |
  1970616.4 |########################################
  1970871.8 |
  1971127.2 |########################################
  1971382.6 |
  1971638.0 |########################################
  1971893.4 |
  1972148.7 |
  1972404.1 |
  1972659.5 |
  1972914.9 |
  1973170.3 |
  1973425.7 |########################################
  1973681.1 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_dispatch (n=6, range 3305198.8-3554018.4 ns)
  3305198.8 |########################################
  3317639.8 |
  3330080.8 |
  3342521.7 |
  3354962.7 |
  3367403.7 |
  3379844.7 |
  3392285.6 |
  3404726.6 |
  3417167.6 |
  3429608.6 |
  3442049.6 |
  3454490.5 |
  3466931.5 |
  3479372.5 |
  3491813.5 |
  3504254.4 |
  3516695.4 |
  3529136.4 |##########
  3541577.4 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_runtime_w (n=6, range 3301741.2-3312951.5 ns)
  3301741.2 |#############
  3302301.7 |########################################
  3302862.2 |
  3303422.7 |
  3303983.2 |
  3304543.8 |
  3305104.3 |
  3305664.8 |
  3306225.3 |
  3306785.8 |
  3307346.3 |
  3307906.8 |
  3308467.4 |
  3309027.9 |
  3309588.4 |
  3310148.9 |#############
  3310709.4 |
  3311269.9 |
  3311830.4 |
  3312390.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_wideselect_zig_anchor**: bridge=312.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_dispatch**: bridge=312.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_null**: bridge=11427.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_per_w_set**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_runtime_w**: bridge=312.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_dispatch**: bridge=307.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: bridge=307.9% of algo (FFI overhead may distort results)
