# abi_zig_entry (wideselect)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_wideselect_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_wideselect_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_wideselect_zig_null dominates: 84992% faster than the next best (abi_zig_entry_wideselect_zig_per_w_set)

abi_zig_entry_wideselect_zig_null (2.32 us) leads abi_zig_entry_wideselect_zig_per_w_set (1.97 ms) by 84992%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_wideselect_zig_null beats baseline by 100% (significant)

abi_zig_entry_wideselect_zig_null is -1.97 ms (100%) faster than baseline abi_zig_entry_wideselect_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_wideselect_zig_tail_runtime_w is an outlier: 1427.5x slower than the field

abi_zig_entry_wideselect_zig_tail_runtime_w (3.31 ms) is 1427.5x the fastest (2.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_wideselect_zig_null} vs {abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} (84992% apart)

The field splits into a fast tier {abi_zig_entry_wideselect_zig_null} and a slow tier {abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} with a 84992% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1427.5x the fastest

Fastest abi_zig_entry_wideselect_zig_null (2.32 us) to slowest abi_zig_entry_wideselect_zig_tail_runtime_w (3.31 ms): 1427.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_wideselect_zig_null** at 2317.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1427.48x (fastest 2317.2 ns, slowest 3307824.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1981124ns | 1981273ns | 1977142ns | 1980914ns | 1983431ns | +0.27% |
| abi_zig_entry_wideselect_zig_dispatch | 1990713ns | 1977610ns | 1972507ns | 1976022ns | 2021853ns | +0.75% |
| abi_zig_entry_wideselect_zig_null | 4620ns | 4647ns | 4495ns | 4621ns | 4679ns | -99.77% |
| abi_zig_entry_wideselect_zig_per_w_set | 1984454ns | 1974418ns | 1973599ns | 1974242ns | 2005198ns | +0.43% |
| abi_zig_entry_wideselect_zig_runtime_w | 1975885ns | 1975129ns | 1973160ns | 1974644ns | 1979110ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3308772ns | 3307489ns | 3301214ns | 3307159ns | 3314969ns | +67.46% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3311745ns | 3310603ns | 3308702ns | 3310555ns | 3315050ns | +67.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1978527ns | 1974587ns | 1980791ns | +0.27% | 0.000 |
| abi_zig_entry_wideselect_zig_dispatch | 1988045ns | 1970053ns | 2018981ns | +0.75% | 0.000 |
| abi_zig_entry_wideselect_zig_null | 2309ns | 2255ns | 2337ns | -99.88% | 0.014 |
| abi_zig_entry_wideselect_zig_per_w_set | 1981758ns | 1971075ns | 2002384ns | +0.43% | 0.000 |
| abi_zig_entry_wideselect_zig_runtime_w | 1973269ns | 1970625ns | 1976412ns | base | 0.000 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3306046ns | 3298660ns | 3312158ns | +67.54% | 0.000 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3308918ns | 3305953ns | 3312175ns | +67.69% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 180433.0 | 1975701.0 | 1978526.7 | n/a |
| abi_zig_entry_wideselect_zig_dispatch | 186767.7 | 2164596.6 | 1988045.4 | n/a |
| abi_zig_entry_wideselect_zig_null | 157410.4 | 2608.7 | 2309.2 | n/a |
| abi_zig_entry_wideselect_zig_per_w_set | 184325.1 | 1980358.5 | 1981758.2 | n/a |
| abi_zig_entry_wideselect_zig_runtime_w | 175653.8 | 1972092.6 | 1973269.0 | n/a |
| abi_zig_entry_wideselect_zig_tail_dispatch | 187299.7 | 3306141.9 | 3306045.7 | n/a |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 192406.2 | 3306367.0 | 3308917.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_zig_entry_wideselect_zig_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_null | 0.014 | 97.3% |
| abi_zig_entry_wideselect_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1981124ns | 1981124ns | +0.27% |
| abi_zig_entry_wideselect_zig_dispatch | 1990713ns | 1990713ns | +0.75% |
| abi_zig_entry_wideselect_zig_null | 4620ns | 4620ns | -99.77% |
| abi_zig_entry_wideselect_zig_per_w_set | 1984454ns | 1984454ns | +0.43% |
| abi_zig_entry_wideselect_zig_runtime_w | 1975885ns | 1975885ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3308772ns | 3308772ns | +67.46% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3311745ns | 3311745ns | +67.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_runtime_w | 1972541ns | base | --- | [1970854, 1976412] | --- | --- | --- | --- |
| abi_zig_entry_wideselect_zig_anchor | 1978657ns | +5990.0ns (+0.3%) | [+1078, +8705]ns | [1976132, 1980791] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_wideselect_zig_dispatch | 1974982ns | no significant difference | [-1105, +43682]ns | [1970174, 2018981] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_wideselect_zig_null | 2317ns | -1970239.1ns (-99.9%) | [-1974114, -1968526]ns | [2273, 2337] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_per_w_set | 1971784ns | no significant difference | [-5025, +30269]ns | [1971106, 2002384] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3304739ns | +1333885.0ns (+67.6%) | [+1325154, +1339292]ns | [3301240, 3312158] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3307824ns | +1335579.6ns (+67.7%) | [+1334344, +1337022]ns | [3306754, 3312175] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_wideselect_zig_runtime_w | abi_zig_entry_wideselect_zig_anchor | abi_zig_entry_wideselect_zig_dispatch | abi_zig_entry_wideselect_zig_null | abi_zig_entry_wideselect_zig_per_w_set | abi_zig_entry_wideselect_zig_tail_dispatch | abi_zig_entry_wideselect_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1973800ns | +0.3% | -0.0% | -99.9% | -0.1% | +67.7% | +67.6% |
| 2 | 1971933ns | +0.3% | -0.1% | -99.9% | -0.0% | +68.1% | +67.7% |
| 3 | 1971083ns | +0.2% | +0.3% | -99.9% | +0.1% | +67.6% | +67.8% |
| 4 | 1973148ns | +0.3% | +4.2% | -99.9% | +2.9% | +67.4% | +67.6% |
| 5 | 1979024ns | -0.1% | +0.2% | -99.9% | -0.4% | +66.7% | +67.6% |
| 6 | 1970625ns | +0.6% | -0.0% | -99.9% | +0.0% | +67.7% | +67.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | -0.179 | ok |
| abi_zig_entry_wideselect_zig_dispatch | -0.109 | ok |
| abi_zig_entry_wideselect_zig_null | -0.429 | moderate- |
| abi_zig_entry_wideselect_zig_per_w_set | -0.217 | moderate- |
| abi_zig_entry_wideselect_zig_runtime_w | -0.286 | moderate- |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.281 | moderate+ |
| abi_zig_entry_wideselect_zig_tail_runtime_w | -0.417 | moderate- |

**Consistency summary:**

- **abi_zig_entry_wideselect_zig_anchor**: won 0/6, lost 5/6
- **abi_zig_entry_wideselect_zig_dispatch**: won 0/6, lost 3/6
- **abi_zig_entry_wideselect_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_wideselect_zig_per_w_set**: won 2/6, lost 2/6
- **abi_zig_entry_wideselect_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 6178673.8ns | 1978526.7ns | 312.3% | HIGH |
| abi_zig_entry_wideselect_zig_dispatch | 6397469.2ns | 1988045.4ns | 321.8% | HIGH |
| abi_zig_entry_wideselect_zig_null | 299603.0ns | 2309.2ns | 12974.6% | HIGH |
| abi_zig_entry_wideselect_zig_per_w_set | 6341340.3ns | 1981758.2ns | 320.0% | HIGH |
| abi_zig_entry_wideselect_zig_runtime_w | 6160708.0ns | 1973269.0ns | 312.2% | HIGH |
| abi_zig_entry_wideselect_zig_tail_dispatch | 10178432.8ns | 3306045.7ns | 307.9% | HIGH |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 10187516.0ns | 3308917.8ns | 307.9% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_wideselect_zig_anchor (n=6, range 1974587.1-1980791.4 ns)
  1974587.1 |########################################
  1974897.3 |
  1975207.5 |
  1975517.8 |
  1975828.0 |
  1976138.2 |
  1976448.4 |
  1976758.6 |
  1977068.8 |
  1977379.1 |########################################
  1977689.3 |########################################
  1977999.5 |
  1978309.7 |
  1978619.9 |
  1978930.1 |
  1979240.4 |########################################
  1979550.6 |########################################
  1979860.8 |
  1980171.0 |
  1980481.2 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_dispatch (n=6, range 1970053.3-2018980.8 ns)
  1970053.3 |########################################
  1972499.7 |####################
  1974946.1 |####################
  1977392.4 |
  1979838.8 |
  1982285.2 |####################
  1984731.6 |
  1987177.9 |
  1989624.3 |
  1992070.7 |
  1994517.0 |
  1996963.4 |
  1999409.8 |
  2001856.2 |
  2004302.5 |
  2006748.9 |
  2009195.3 |
  2011641.7 |
  2014088.0 |
  2016534.4 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_null (n=6, range 2255.0-2336.8 ns)
   2255.0 |########################################
   2259.1 |
   2263.2 |
   2267.3 |
   2271.4 |
   2275.5 |
   2279.6 |
   2283.6 |
   2287.7 |########################################
   2291.8 |
   2295.9 |
   2300.0 |
   2304.1 |
   2308.2 |########################################
   2312.3 |
   2316.4 |
   2320.5 |########################################
   2324.6 |
   2328.7 |########################################
   2332.8 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_per_w_set (n=6, range 1971075.4-2002384.4 ns)
  1971075.4 |########################################
  1972640.8 |##########
  1974206.3 |
  1975771.7 |
  1977337.2 |
  1978902.6 |
  1980468.1 |
  1982033.5 |
  1983599.0 |
  1985164.4 |
  1986729.9 |
  1988295.3 |
  1989860.8 |
  1991426.2 |
  1992991.7 |
  1994557.1 |
  1996122.6 |
  1997688.0 |
  1999253.5 |
  2000818.9 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_runtime_w (n=6, range 1970625.0-1976412.3 ns)
  1970625.0 |########################################
  1970914.4 |########################################
  1971203.7 |
  1971493.1 |
  1971782.5 |########################################
  1972071.8 |
  1972361.2 |
  1972650.6 |
  1972939.9 |########################################
  1973229.3 |
  1973518.6 |########################################
  1973808.0 |
  1974097.4 |
  1974386.7 |
  1974676.1 |
  1974965.5 |
  1975254.8 |
  1975544.2 |
  1975833.6 |
  1976122.9 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_dispatch (n=6, range 3298659.6-3312158.4 ns)
  3298659.6 |####################
  3299334.5 |
  3300009.5 |
  3300684.4 |
  3301359.4 |
  3302034.3 |
  3302709.2 |
  3303384.2 |########################################
  3304059.1 |
  3304734.0 |
  3305409.0 |####################
  3306083.9 |
  3306758.9 |
  3307433.8 |
  3308108.7 |
  3308783.7 |
  3309458.6 |
  3310133.5 |####################
  3310808.5 |
  3311483.4 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_runtime_w (n=6, range 3305953.3-3312175.2 ns)
  3305953.3 |####################
  3306264.4 |
  3306575.5 |
  3306886.6 |
  3307197.7 |
  3307508.8 |########################################
  3307819.9 |########################################
  3308131.0 |
  3308442.1 |
  3308753.2 |
  3309064.2 |
  3309375.3 |
  3309686.4 |
  3309997.5 |
  3310308.6 |
  3310619.7 |
  3310930.8 |
  3311241.9 |
  3311553.0 |
  3311864.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_wideselect_zig_anchor**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_dispatch**: bridge=312.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_null**: bridge=12896.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_per_w_set**: bridge=312.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_runtime_w**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_dispatch**: bridge=307.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: bridge=307.9% of algo (FFI overhead may distort results)
