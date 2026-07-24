# abi_zig_entry (wideselect)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_wideselect_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_wideselect_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_wideselect_zig_null dominates: 62775% faster than the next best (abi_zig_entry_wideselect_zig_per_w_set)

abi_zig_entry_wideselect_zig_null (3.14 us) leads abi_zig_entry_wideselect_zig_per_w_set (1.97 ms) by 62775%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_wideselect_zig_null beats baseline by 100% (significant)

abi_zig_entry_wideselect_zig_null is -1.97 ms (100%) faster than baseline abi_zig_entry_wideselect_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_wideselect_zig_tail_dispatch is an outlier: 1055.3x slower than the field

abi_zig_entry_wideselect_zig_tail_dispatch (3.31 ms) is 1055.3x the fastest (3.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_wideselect_zig_dispatch shows alternating (throttle bounce) (autocorr -0.69)

abi_zig_entry_wideselect_zig_dispatch's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_wideselect_zig_null} vs {abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch} (62775% apart)

The field splits into a fast tier {abi_zig_entry_wideselect_zig_null} and a slow tier {abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch} with a 62775% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1055.3x the fastest

Fastest abi_zig_entry_wideselect_zig_null (3.14 us) to slowest abi_zig_entry_wideselect_zig_tail_dispatch (3.31 ms): 1055.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_wideselect_zig_null** at 3135.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1055.30x (fastest 3135.2 ns, slowest 3308577.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1982128ns | 1982293ns | 1977856ns | 1981338ns | 1985448ns | +0.25% |
| abi_zig_entry_wideselect_zig_dispatch | 1980830ns | 1980093ns | 1976816ns | 1979520ns | 1984802ns | +0.18% |
| abi_zig_entry_wideselect_zig_null | 5471ns | 5464ns | 5380ns | 5446ns | 5553ns | -99.72% |
| abi_zig_entry_wideselect_zig_per_w_set | 1975415ns | 1973897ns | 1971696ns | 1973453ns | 1980216ns | -0.09% |
| abi_zig_entry_wideselect_zig_runtime_w | 1977241ns | 1976448ns | 1972891ns | 1975684ns | 1981752ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3311141ns | 3311506ns | 3304423ns | 3310954ns | 3314780ns | +67.46% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3351868ns | 3308330ns | 3306488ns | 3307831ns | 3440613ns | +69.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1979420ns | 1975264ns | 1982745ns | +0.25% | 0.000 |
| abi_zig_entry_wideselect_zig_dispatch | 1978101ns | 1974177ns | 1982029ns | +0.18% | 0.000 |
| abi_zig_entry_wideselect_zig_null | 3144ns | 3107ns | 3179ns | -99.84% | 0.081 |
| abi_zig_entry_wideselect_zig_per_w_set | 1972741ns | 1969170ns | 1977434ns | -0.09% | 0.000 |
| abi_zig_entry_wideselect_zig_runtime_w | 1974496ns | 1970188ns | 1978972ns | base | 0.000 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3308251ns | 3301562ns | 3311863ns | +67.55% | 0.000 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3349020ns | 3303723ns | 3437778ns | +69.61% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 181055.3 | 1977272.6 | 1979419.8 | 1 |
| abi_zig_entry_wideselect_zig_dispatch | 182357.5 | 1977005.4 | 1978101.3 | 0 |
| abi_zig_entry_wideselect_zig_null | 157882.2 | 3205.3 | 3143.8 | n/a |
| abi_zig_entry_wideselect_zig_per_w_set | 178284.7 | 1972910.7 | 1972740.8 | n/a |
| abi_zig_entry_wideselect_zig_runtime_w | 183532.9 | 1974007.0 | 1974496.4 | n/a |
| abi_zig_entry_wideselect_zig_tail_dispatch | 193641.7 | 3307458.5 | 3308250.8 | n/a |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 195337.0 | 3350487.2 | 3349019.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_zig_entry_wideselect_zig_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_null | 0.082 | 99.1% |
| abi_zig_entry_wideselect_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1982128ns | 1982128ns | +0.25% |
| abi_zig_entry_wideselect_zig_dispatch | 1980830ns | 1980830ns | +0.18% |
| abi_zig_entry_wideselect_zig_null | 5471ns | 5471ns | -99.72% |
| abi_zig_entry_wideselect_zig_per_w_set | 1975415ns | 1975415ns | -0.09% |
| abi_zig_entry_wideselect_zig_runtime_w | 1977241ns | 1977241ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3311141ns | 3311141ns | +67.46% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3351868ns | 3351868ns | +69.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_runtime_w | 1973690ns | base | --- | [1970828, 1978972] | --- | --- | --- | --- |
| abi_zig_entry_wideselect_zig_anchor | 1979509ns | +4777.6ns (+0.2%) | [+2201, +7792]ns | [1976005, 1982745] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_dispatch | 1977337ns | no significant difference | [-1634, +10681]ns | [1974938, 1982029] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_wideselect_zig_null | 3135ns | -1970544.4ns (-99.8%) | [-1975820, -1967693]ns | [3117, 3179] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_per_w_set | 1971264ns | no significant difference | [-7707, +5446]ns | [1969525, 1977434] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3308578ns | +1335907.8ns (+67.7%) | [+1328084, +1337272]ns | [3304312, 3311863] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3305415ns | +1331878.9ns (+67.5%) | [+1328106, +1463586]ns | [3303866, 3437778] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_wideselect_zig_runtime_w | abi_zig_entry_wideselect_zig_anchor | abi_zig_entry_wideselect_zig_dispatch | abi_zig_entry_wideselect_zig_null | abi_zig_entry_wideselect_zig_per_w_set | abi_zig_entry_wideselect_zig_tail_dispatch | abi_zig_entry_wideselect_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1979748ns | +0.2% | -0.1% | -99.8% | -0.5% | +66.8% | +67.0% |
| 2 | 1974872ns | +0.1% | +0.0% | -99.8% | -0.3% | +67.6% | +67.3% |
| 3 | 1970188ns | +0.3% | +0.7% | -99.8% | -0.0% | +67.9% | +80.9% |
| 4 | 1971468ns | +0.5% | +0.1% | -99.8% | +0.4% | +67.7% | +67.6% |
| 5 | 1972508ns | +0.3% | +0.4% | -99.8% | +0.2% | +67.8% | +67.5% |
| 6 | 1978196ns | +0.1% | -0.0% | -99.8% | -0.3% | +67.5% | +67.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | -0.240 | moderate- |
| abi_zig_entry_wideselect_zig_dispatch | -0.694 | HIGH- (thermal bounce) |
| abi_zig_entry_wideselect_zig_null | -0.254 | moderate- |
| abi_zig_entry_wideselect_zig_per_w_set | 0.252 | moderate+ |
| abi_zig_entry_wideselect_zig_runtime_w | 0.165 | ok |
| abi_zig_entry_wideselect_zig_tail_dispatch | -0.014 | ok |
| abi_zig_entry_wideselect_zig_tail_runtime_w | -0.247 | moderate- |

**Consistency summary:**

- **abi_zig_entry_wideselect_zig_anchor**: won 0/6, lost 5/6
- **abi_zig_entry_wideselect_zig_dispatch**: won 1/6, lost 3/6
- **abi_zig_entry_wideselect_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_wideselect_zig_per_w_set**: won 3/6, lost 2/6
- **abi_zig_entry_wideselect_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 6187347.5ns | 1979419.8ns | 312.6% | HIGH |
| abi_zig_entry_wideselect_zig_dispatch | 6178977.3ns | 1978101.3ns | 312.4% | HIGH |
| abi_zig_entry_wideselect_zig_null | 307904.3ns | 3143.8ns | 9794.2% | HIGH |
| abi_zig_entry_wideselect_zig_per_w_set | 6163064.9ns | 1972740.8ns | 312.4% | HIGH |
| abi_zig_entry_wideselect_zig_runtime_w | 6175947.8ns | 1974496.4ns | 312.8% | HIGH |
| abi_zig_entry_wideselect_zig_tail_dispatch | 10190642.0ns | 3308250.8ns | 308.0% | HIGH |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 10319788.9ns | 3349019.9ns | 308.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_wideselect_zig_anchor (n=6, range 1975263.8-1982745.5 ns)
  1975263.8 |########################################
  1975637.9 |
  1976012.0 |
  1976386.0 |########################################
  1976760.1 |
  1977134.2 |
  1977508.3 |
  1977882.4 |
  1978256.5 |########################################
  1978630.5 |
  1979004.6 |
  1979378.7 |
  1979752.8 |
  1980126.9 |
  1980501.0 |########################################
  1980875.0 |
  1981249.1 |########################################
  1981623.2 |
  1981997.3 |
  1982371.4 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_dispatch (n=6, range 1974177.1-1982028.8 ns)
  1974177.1 |########################################
  1974569.7 |
  1974962.3 |
  1975354.8 |########################################
  1975747.4 |
  1976140.0 |
  1976532.6 |
  1976925.2 |########################################
  1977317.8 |########################################
  1977710.3 |
  1978102.9 |
  1978495.5 |
  1978888.1 |
  1979280.7 |########################################
  1979673.3 |
  1980065.8 |
  1980458.4 |
  1980851.0 |
  1981243.6 |
  1981636.2 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_null (n=6, range 3107.1-3178.9 ns)
   3107.1 |####################
   3110.7 |
   3114.3 |
   3117.9 |
   3121.5 |
   3125.1 |########################################
   3128.7 |
   3132.2 |
   3135.8 |
   3139.4 |####################
   3143.0 |
   3146.6 |
   3150.2 |
   3153.8 |
   3157.4 |
   3161.0 |####################
   3164.6 |
   3168.2 |
   3171.8 |
   3175.4 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_per_w_set (n=6, range 1969170.4-1977433.6 ns)
  1969170.4 |########################################
  1969583.6 |########################################
  1969996.7 |
  1970409.9 |########################################
  1970823.0 |
  1971236.2 |
  1971649.3 |########################################
  1972062.5 |
  1972475.7 |
  1972888.8 |
  1973302.0 |
  1973715.1 |
  1974128.3 |
  1974541.4 |
  1974954.6 |
  1975367.8 |
  1975780.9 |
  1976194.1 |########################################
  1976607.2 |
  1977020.4 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_runtime_w (n=6, range 1970188.3-1978971.6 ns)
  1970188.3 |########################################
  1970627.5 |
  1971066.6 |########################################
  1971505.8 |
  1971945.0 |
  1972384.1 |########################################
  1972823.3 |
  1973262.5 |
  1973701.6 |
  1974140.8 |
  1974580.0 |########################################
  1975019.1 |
  1975458.3 |
  1975897.5 |
  1976336.6 |
  1976775.8 |
  1977215.0 |
  1977654.1 |
  1978093.3 |########################################
  1978532.5 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_dispatch (n=6, range 3301562.1-3311862.8 ns)
  3301562.1 |########################################
  3302077.1 |
  3302592.2 |
  3303107.2 |
  3303622.2 |
  3304137.3 |
  3304652.3 |
  3305167.3 |
  3305682.4 |
  3306197.4 |
  3306712.4 |########################################
  3307227.5 |
  3307742.5 |########################################
  3308257.5 |
  3308772.6 |########################################
  3309287.6 |########################################
  3309802.6 |
  3310317.7 |
  3310832.7 |
  3311347.7 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_runtime_w (n=6, range 3303722.9-3437777.9 ns)
  3303722.9 |########################################
  3310425.6 |##########
  3317128.4 |
  3323831.1 |
  3330533.9 |
  3337236.6 |
  3343939.4 |
  3350642.1 |
  3357344.9 |
  3364047.6 |
  3370750.4 |
  3377453.1 |
  3384155.9 |
  3390858.6 |
  3397561.4 |
  3404264.1 |
  3410966.9 |
  3417669.6 |
  3424372.4 |
  3431075.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_wideselect_zig_anchor**: bridge=312.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_dispatch**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_null**: bridge=9824.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_per_w_set**: bridge=312.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_runtime_w**: bridge=313.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_dispatch**: bridge=307.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: bridge=308.4% of algo (FFI overhead may distort results)
