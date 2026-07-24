# abi_zig_entry (wideselect)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_wideselect_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_wideselect_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_wideselect_zig_null dominates: 57704% faster than the next best (abi_zig_entry_wideselect_zig_per_w_set)

abi_zig_entry_wideselect_zig_null (3.42 us) leads abi_zig_entry_wideselect_zig_per_w_set (1.98 ms) by 57704%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_wideselect_zig_null beats baseline by 100% (significant)

abi_zig_entry_wideselect_zig_null is -1.97 ms (100%) faster than baseline abi_zig_entry_wideselect_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_wideselect_zig_tail_runtime_w is an outlier: 970.3x slower than the field

abi_zig_entry_wideselect_zig_tail_runtime_w (3.32 ms) is 970.3x the fastest (3.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_wideselect_zig_null} vs {abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} (57704% apart)

The field splits into a fast tier {abi_zig_entry_wideselect_zig_null} and a slow tier {abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} with a 57704% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 970.3x the fastest

Fastest abi_zig_entry_wideselect_zig_null (3.42 us) to slowest abi_zig_entry_wideselect_zig_tail_runtime_w (3.32 ms): 970.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_wideselect_zig_null** at 3418.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 970.30x (fastest 3418.8 ns, slowest 3317223.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1979834ns | 1980294ns | 1974280ns | 1979842ns | 1982600ns | -0.83% |
| abi_zig_entry_wideselect_zig_dispatch | 1979441ns | 1980068ns | 1974906ns | 1978774ns | 1982709ns | -0.85% |
| abi_zig_entry_wideselect_zig_null | 5744ns | 5709ns | 5624ns | 5699ns | 5870ns | -99.71% |
| abi_zig_entry_wideselect_zig_per_w_set | 1985668ns | 1978899ns | 1971983ns | 1976739ns | 2005902ns | -0.54% |
| abi_zig_entry_wideselect_zig_runtime_w | 1996391ns | 1980694ns | 1977171ns | 1980422ns | 2029955ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3340622ns | 3315522ns | 3302250ns | 3314848ns | 3398468ns | +67.33% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3322076ns | 3320106ns | 3312636ns | 3318161ns | 3332668ns | +66.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1977125ns | 1971642ns | 1979921ns | -0.83% | 0.000 |
| abi_zig_entry_wideselect_zig_dispatch | 1976749ns | 1972059ns | 1980042ns | -0.85% | 0.000 |
| abi_zig_entry_wideselect_zig_null | 3435ns | 3374ns | 3497ns | -99.83% | 0.001 |
| abi_zig_entry_wideselect_zig_per_w_set | 1982975ns | 1969424ns | 2003125ns | -0.53% | 0.000 |
| abi_zig_entry_wideselect_zig_runtime_w | 1993628ns | 1974530ns | 2026948ns | base | 0.000 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3337693ns | 3299595ns | 3395260ns | +67.42% | 0.000 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3319209ns | 3309928ns | 3329725ns | +66.49% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 180637.1 | 1977519.2 | 1977124.5 | n/a |
| abi_zig_entry_wideselect_zig_dispatch | 181433.5 | 1976986.6 | 1976749.2 | n/a |
| abi_zig_entry_wideselect_zig_null | 158049.7 | 3604.7 | 3435.3 | n/a |
| abi_zig_entry_wideselect_zig_per_w_set | 271357.0 | 1987429.2 | 1982975.3 | 8 |
| abi_zig_entry_wideselect_zig_runtime_w | 188906.2 | 2000902.4 | 1993628.1 | n/a |
| abi_zig_entry_wideselect_zig_tail_dispatch | 201209.4 | 3347737.2 | 3337692.7 | n/a |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 194694.1 | 3314740.7 | 3319209.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_wideselect_zig_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_null | 0.001 | 98.7% |
| abi_zig_entry_wideselect_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1979834ns | 1979834ns | -0.83% |
| abi_zig_entry_wideselect_zig_dispatch | 1979441ns | 1979441ns | -0.85% |
| abi_zig_entry_wideselect_zig_null | 5744ns | 5744ns | -99.71% |
| abi_zig_entry_wideselect_zig_per_w_set | 1985668ns | 1985668ns | -0.54% |
| abi_zig_entry_wideselect_zig_runtime_w | 1996391ns | 1996391ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3340622ns | 3340622ns | +67.33% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3322076ns | 3322076ns | +66.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_runtime_w | 1978016ns | base | --- | [1975921, 2026948] | --- | --- | --- | --- |
| abi_zig_entry_wideselect_zig_anchor | 1977523ns | no significant difference | [-52292, +3232]ns | [1973929, 1979921] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_wideselect_zig_dispatch | 1977348ns | no significant difference | [-52319, +3108]ns | [1972858, 1980042] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_wideselect_zig_null | 3419ns | -1974607.3ns (-99.8%) | [-2023454, -1972518]ns | [3391, 3497] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_per_w_set | 1976166ns | no significant difference | [-28033, +4456]ns | [1969635, 2003125] | no | 1.0000 | 0.6875 | 0 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3312664ns | +1335742.7ns (+67.5%) | [+1325793, +1370658]ns | [3305155, 3395260] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3317223ns | +1335663.4ns (+67.5%) | [+1292032, +1349047]ns | [3310679, 3329725] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_wideselect_zig_runtime_w | abi_zig_entry_wideselect_zig_anchor | abi_zig_entry_wideselect_zig_dispatch | abi_zig_entry_wideselect_zig_null | abi_zig_entry_wideselect_zig_per_w_set | abi_zig_entry_wideselect_zig_tail_dispatch | abi_zig_entry_wideselect_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1977312ns | -0.1% | -0.2% | -99.8% | +0.1% | +67.5% | +67.4% |
| 2 | 1974530ns | +0.1% | +0.2% | -99.8% | +0.4% | +67.7% | +67.7% |
| 3 | 2070971ns | -4.8% | -4.5% | -99.8% | -2.3% | +67.5% | +60.4% |
| 4 | 1978232ns | +0.0% | +0.2% | -99.8% | -0.4% | +67.9% | +68.7% |
| 5 | 1977800ns | +0.2% | +0.0% | -99.8% | -0.4% | +66.8% | +67.5% |
| 6 | 1982924ns | -0.3% | -0.5% | -99.8% | -0.5% | +67.1% | +67.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | -0.016 | ok |
| abi_zig_entry_wideselect_zig_dispatch | 0.004 | ok |
| abi_zig_entry_wideselect_zig_null | -0.092 | ok |
| abi_zig_entry_wideselect_zig_per_w_set | -0.127 | ok |
| abi_zig_entry_wideselect_zig_runtime_w | -0.269 | moderate- |
| abi_zig_entry_wideselect_zig_tail_dispatch | -0.166 | ok |
| abi_zig_entry_wideselect_zig_tail_runtime_w | -0.077 | ok |

**Consistency summary:**

- **abi_zig_entry_wideselect_zig_anchor**: won 2/6, lost 2/6
- **abi_zig_entry_wideselect_zig_dispatch**: won 3/6, lost 2/6
- **abi_zig_entry_wideselect_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_wideselect_zig_per_w_set**: won 4/6, lost 1/6
- **abi_zig_entry_wideselect_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 6180194.1ns | 1977124.5ns | 312.6% | HIGH |
| abi_zig_entry_wideselect_zig_dispatch | 6182076.1ns | 1976749.2ns | 312.7% | HIGH |
| abi_zig_entry_wideselect_zig_null | 308187.7ns | 3435.3ns | 8971.1% | HIGH |
| abi_zig_entry_wideselect_zig_per_w_set | 6291806.4ns | 1982975.3ns | 317.3% | HIGH |
| abi_zig_entry_wideselect_zig_runtime_w | 6244314.0ns | 1993628.1ns | 313.2% | HIGH |
| abi_zig_entry_wideselect_zig_tail_dispatch | 10321603.9ns | 3337692.7ns | 309.2% | HIGH |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 10212845.0ns | 3319209.0ns | 307.7% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_wideselect_zig_anchor (n=6, range 1971641.7-1979921.2 ns)
  1971641.7 |########################################
  1972055.7 |
  1972469.6 |
  1972883.6 |
  1973297.6 |
  1973711.6 |
  1974125.6 |
  1974539.5 |
  1974953.5 |
  1975367.5 |
  1975781.4 |
  1976195.4 |########################################
  1976609.4 |
  1977023.4 |########################################
  1977437.3 |########################################
  1977851.3 |
  1978265.3 |########################################
  1978679.3 |
  1979093.2 |
  1979507.2 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_dispatch (n=6, range 1972059.2-1980041.6 ns)
  1972059.2 |########################################
  1972458.3 |
  1972857.4 |
  1973256.6 |
  1973655.7 |########################################
  1974054.8 |
  1974453.9 |
  1974853.1 |
  1975252.2 |
  1975651.3 |
  1976050.4 |
  1976449.5 |
  1976848.7 |########################################
  1977247.8 |########################################
  1977646.9 |
  1978046.0 |
  1978445.2 |########################################
  1978844.3 |
  1979243.4 |
  1979642.5 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_null (n=6, range 3374.2-3496.6 ns)
   3374.2 |####################
   3380.3 |
   3386.4 |
   3392.6 |
   3398.7 |
   3404.8 |########################################
   3410.9 |
   3417.1 |
   3423.2 |####################
   3429.3 |####################
   3435.4 |
   3441.5 |
   3447.7 |
   3453.8 |
   3459.9 |
   3466.0 |
   3472.2 |
   3478.3 |
   3484.4 |
   3490.5 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_per_w_set (n=6, range 1969424.2-2003124.8 ns)
  1969424.2 |########################################
  1971109.2 |
  1972794.3 |####################
  1974479.3 |
  1976164.3 |
  1977849.3 |####################
  1979534.4 |
  1981219.4 |####################
  1982904.4 |
  1984589.4 |
  1986274.5 |
  1987959.5 |
  1989644.5 |
  1991329.6 |
  1993014.6 |
  1994699.6 |
  1996384.6 |
  1998069.7 |
  1999754.7 |
  2001439.7 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_runtime_w (n=6, range 1974529.6-2026947.7 ns)
  1974529.6 |#############
  1977150.5 |########################################
  1979771.4 |
  1982392.3 |#############
  1985013.2 |
  1987634.1 |
  1990255.0 |
  1992875.9 |
  1995496.8 |
  1998117.7 |
  2000738.6 |
  2003359.6 |
  2005980.5 |
  2008601.4 |
  2011222.3 |
  2013843.2 |
  2016464.1 |
  2019085.0 |
  2021705.9 |
  2024326.8 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_dispatch (n=6, range 3299595.4-3395259.8 ns)
  3299595.4 |#############
  3304378.6 |
  3309161.8 |########################################
  3313945.1 |
  3318728.3 |#############
  3323511.5 |
  3328294.7 |
  3333077.9 |
  3337861.1 |
  3342644.4 |
  3347427.6 |
  3352210.8 |
  3356994.0 |
  3361777.2 |
  3366560.4 |
  3371343.7 |
  3376126.9 |
  3380910.1 |
  3385693.3 |
  3390476.5 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_runtime_w (n=6, range 3309928.3-3329725.0 ns)
  3309928.3 |####################
  3310918.1 |####################
  3311908.0 |####################
  3312897.8 |
  3313887.6 |
  3314877.5 |
  3315867.3 |
  3316857.1 |
  3317847.0 |
  3318836.8 |
  3319826.6 |
  3320816.5 |
  3321806.3 |########################################
  3322796.2 |
  3323786.0 |
  3324775.8 |
  3325765.7 |
  3326755.5 |
  3327745.3 |
  3328735.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_wideselect_zig_anchor**: bridge=312.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_dispatch**: bridge=312.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_null**: bridge=8944.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_per_w_set**: bridge=312.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_runtime_w**: bridge=312.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_dispatch**: bridge=308.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: bridge=307.6% of algo (FFI overhead may distort results)
