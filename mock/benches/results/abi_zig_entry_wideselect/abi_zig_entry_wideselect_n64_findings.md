# abi_zig_entry (wideselect)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_wideselect_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_wideselect_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_wideselect_zig_null dominates: 80092% faster than the next best (abi_zig_entry_wideselect_zig_runtime_w)

abi_zig_entry_wideselect_zig_null (2.46 us) leads abi_zig_entry_wideselect_zig_runtime_w (1.97 ms) by 80092%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_wideselect_zig_null beats baseline by 100% (significant)

abi_zig_entry_wideselect_zig_null is -1.97 ms (100%) faster than baseline abi_zig_entry_wideselect_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_wideselect_zig_tail_dispatch is an outlier: 1347.5x slower than the field

abi_zig_entry_wideselect_zig_tail_dispatch (3.31 ms) is 1347.5x the fastest (2.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_wideselect_zig_null shows alternating (throttle bounce) (autocorr -0.80)

abi_zig_entry_wideselect_zig_null's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_wideselect_zig_null} vs {abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch} (80092% apart)

The field splits into a fast tier {abi_zig_entry_wideselect_zig_null} and a slow tier {abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch} with a 80092% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1347.5x the fastest

Fastest abi_zig_entry_wideselect_zig_null (2.46 us) to slowest abi_zig_entry_wideselect_zig_tail_dispatch (3.31 ms): 1347.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_wideselect_zig_null** at 2459.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1347.50x (fastest 2459.2 ns, slowest 3313699.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 2061219ns | 1982186ns | 1977394ns | 1981941ns | 2222049ns | +4.36% |
| abi_zig_entry_wideselect_zig_dispatch | 2014082ns | 1980039ns | 1973629ns | 1978870ns | 2087127ns | +1.97% |
| abi_zig_entry_wideselect_zig_null | 4759ns | 4754ns | 4668ns | 4743ns | 4829ns | -99.76% |
| abi_zig_entry_wideselect_zig_per_w_set | 1977686ns | 1976918ns | 1971808ns | 1975856ns | 1983372ns | +0.13% |
| abi_zig_entry_wideselect_zig_runtime_w | 1975198ns | 1974552ns | 1973484ns | 1974350ns | 1977325ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3338273ns | 3316598ns | 3307616ns | 3314013ns | 3389990ns | +69.01% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3309038ns | 3306968ns | 3304911ns | 3306451ns | 3314984ns | +67.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 2057492ns | 1974893ns | 2216209ns | +4.30% | 0.000 |
| abi_zig_entry_wideselect_zig_dispatch | 2011090ns | 1970923ns | 2083601ns | +1.95% | 0.000 |
| abi_zig_entry_wideselect_zig_null | 2458ns | 2430ns | 2483ns | -99.88% | 0.026 |
| abi_zig_entry_wideselect_zig_per_w_set | 1975013ns | 1969323ns | 1980606ns | +0.12% | 0.000 |
| abi_zig_entry_wideselect_zig_runtime_w | 1972591ns | 1970854ns | 1974631ns | base | 0.000 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3334016ns | 3304852ns | 3382887ns | +69.02% | 0.000 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3306204ns | 3302295ns | 3312008ns | +67.61% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 207898.1 | 2024337.0 | 2057491.6 | n/a |
| abi_zig_entry_wideselect_zig_dispatch | 201762.9 | 2013418.5 | 2011089.5 | n/a |
| abi_zig_entry_wideselect_zig_null | 156932.6 | 2752.6 | 2457.9 | n/a |
| abi_zig_entry_wideselect_zig_per_w_set | 179784.2 | 1974398.0 | 1975013.0 | n/a |
| abi_zig_entry_wideselect_zig_runtime_w | 176101.4 | 1972804.9 | 1972591.2 | n/a |
| abi_zig_entry_wideselect_zig_tail_dispatch | 207323.7 | 3482342.1 | 3334015.9 | n/a |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 193234.5 | 3306467.2 | 3306204.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_zig_entry_wideselect_zig_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_null | 0.026 | 98.8% |
| abi_zig_entry_wideselect_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 2061219ns | 2061219ns | +4.36% |
| abi_zig_entry_wideselect_zig_dispatch | 2014082ns | 2014082ns | +1.97% |
| abi_zig_entry_wideselect_zig_null | 4759ns | 4759ns | -99.76% |
| abi_zig_entry_wideselect_zig_per_w_set | 1977686ns | 1977686ns | +0.13% |
| abi_zig_entry_wideselect_zig_runtime_w | 1975198ns | 1975198ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3338273ns | 3338273ns | +69.01% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3309038ns | 3309038ns | +67.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_runtime_w | 1972032ns | base | --- | [1971110, 1974631] | --- | --- | --- | --- |
| abi_zig_entry_wideselect_zig_anchor | 1979465ns | +7716.6ns (+0.4%) | [+4750, +242235]ns | [1976802, 2216209] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_dispatch | 1977304ns | no significant difference | [-327, +109627]ns | [1972363, 2083601] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_wideselect_zig_null | 2459ns | -1969571.2ns (-99.9%) | [-1972174, -1968655]ns | [2432, 2483] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_per_w_set | 1974186ns | no significant difference | [-3698, +8809]ns | [1970247, 1980606] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3313700ns | +1341009.8ns (+68.0%) | [+1334351, +1408913]ns | [3305461, 3382887] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3304172ns | +1333062.0ns (+67.6%) | [+1328459, +1339318]ns | [3302433, 3312008] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_wideselect_zig_runtime_w | abi_zig_entry_wideselect_zig_anchor | abi_zig_entry_wideselect_zig_dispatch | abi_zig_entry_wideselect_zig_null | abi_zig_entry_wideselect_zig_per_w_set | abi_zig_entry_wideselect_zig_tail_dispatch | abi_zig_entry_wideselect_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1976524ns | +22.9% | +6.9% | -99.9% | -0.4% | +71.9% | +67.1% |
| 2 | 1971366ns | +0.4% | +0.1% | -99.9% | -0.0% | +67.7% | +67.6% |
| 3 | 1970854ns | +0.4% | +0.5% | -99.9% | +0.4% | +67.7% | +67.6% |
| 4 | 1972738ns | +0.1% | -0.1% | -99.9% | +0.5% | +68.1% | +67.9% |
| 5 | 1971423ns | +1.6% | +4.2% | -99.9% | +0.2% | +70.8% | +67.5% |
| 6 | 1972642ns | +0.4% | +0.1% | -99.9% | +0.0% | +67.9% | +67.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | -0.047 | ok |
| abi_zig_entry_wideselect_zig_dispatch | -0.268 | moderate- |
| abi_zig_entry_wideselect_zig_null | -0.799 | HIGH- (thermal bounce) |
| abi_zig_entry_wideselect_zig_per_w_set | 0.296 | moderate+ |
| abi_zig_entry_wideselect_zig_runtime_w | -0.149 | ok |
| abi_zig_entry_wideselect_zig_tail_dispatch | -0.236 | moderate- |
| abi_zig_entry_wideselect_zig_tail_runtime_w | -0.443 | moderate- |

**Consistency summary:**

- **abi_zig_entry_wideselect_zig_anchor**: won 0/6, lost 6/6
- **abi_zig_entry_wideselect_zig_dispatch**: won 0/6, lost 4/6
- **abi_zig_entry_wideselect_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_wideselect_zig_per_w_set**: won 1/6, lost 3/6
- **abi_zig_entry_wideselect_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 6552737.1ns | 2057491.6ns | 318.5% | HIGH |
| abi_zig_entry_wideselect_zig_dispatch | 6369491.9ns | 2011089.5ns | 316.7% | HIGH |
| abi_zig_entry_wideselect_zig_null | 299072.4ns | 2457.9ns | 12167.8% | HIGH |
| abi_zig_entry_wideselect_zig_per_w_set | 6170739.0ns | 1975013.0ns | 312.4% | HIGH |
| abi_zig_entry_wideselect_zig_runtime_w | 6163551.2ns | 1972591.2ns | 312.5% | HIGH |
| abi_zig_entry_wideselect_zig_tail_dispatch | 10492835.3ns | 3334015.9ns | 314.7% | HIGH |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 10185822.5ns | 3306204.5ns | 308.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_wideselect_zig_anchor (n=6, range 1974893.3-2216208.8 ns)
  1974893.3 |########################################
  1986959.1 |
  1999024.8 |##########
  2011090.6 |
  2023156.4 |
  2035222.2 |
  2047287.9 |
  2059353.7 |
  2071419.5 |
  2083485.3 |
  2095551.0 |
  2107616.8 |
  2119682.6 |
  2131748.3 |
  2143814.1 |
  2155879.9 |
  2167945.7 |
  2180011.4 |
  2192077.2 |
  2204143.0 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_dispatch (n=6, range 1970922.9-2083600.9 ns)
  1970922.9 |########################################
  1976556.8 |#############
  1982190.7 |
  1987824.6 |
  1993458.5 |
  1999092.4 |
  2004726.3 |
  2010360.2 |
  2015994.1 |
  2021628.0 |
  2027261.9 |
  2032895.8 |
  2038529.7 |
  2044163.6 |
  2049797.5 |#############
  2055431.4 |
  2061065.3 |
  2066699.2 |
  2072333.1 |
  2077967.0 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_null (n=6, range 2430.0-2482.9 ns)
   2430.0 |####################
   2432.6 |####################
   2435.3 |####################
   2437.9 |
   2440.6 |
   2443.2 |
   2445.9 |
   2448.5 |
   2451.2 |
   2453.8 |
   2456.4 |
   2459.1 |
   2461.7 |
   2464.4 |
   2467.0 |
   2469.7 |
   2472.3 |
   2475.0 |
   2477.6 |
   2480.3 |########################################
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_per_w_set (n=6, range 1969322.9-1980605.6 ns)
  1969322.9 |########################################
  1969887.0 |
  1970451.2 |
  1971015.3 |########################################
  1971579.4 |
  1972143.6 |
  1972707.7 |
  1973271.8 |########################################
  1973836.0 |
  1974400.1 |
  1974964.2 |########################################
  1975528.4 |
  1976092.5 |
  1976656.7 |
  1977220.8 |
  1977784.9 |
  1978349.1 |########################################
  1978913.2 |
  1979477.3 |
  1980041.5 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_runtime_w (n=6, range 1970854.2-1974631.2 ns)
  1970854.2 |####################
  1971043.1 |
  1971231.9 |####################
  1971420.8 |####################
  1971609.6 |
  1971798.5 |
  1971987.3 |
  1972176.2 |
  1972365.0 |
  1972553.9 |########################################
  1972742.7 |
  1972931.6 |
  1973120.4 |
  1973309.3 |
  1973498.1 |
  1973687.0 |
  1973875.8 |
  1974064.7 |
  1974253.5 |
  1974442.4 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_dispatch (n=6, range 3304852.5-3382886.9 ns)
  3304852.5 |########################################
  3308754.2 |####################
  3312655.9 |####################
  3316557.7 |
  3320459.4 |
  3324361.1 |
  3328262.8 |
  3332164.5 |
  3336066.2 |
  3339968.0 |
  3343869.7 |
  3347771.4 |
  3351673.1 |
  3355574.8 |
  3359476.5 |
  3363378.3 |
  3367280.0 |####################
  3371181.7 |
  3375083.4 |
  3378985.1 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_runtime_w (n=6, range 3302295.4-3312008.3 ns)
  3302295.4 |########################################
  3302781.0 |
  3303266.7 |
  3303752.3 |####################
  3304238.0 |####################
  3304723.6 |
  3305209.3 |
  3305694.9 |
  3306180.6 |
  3306666.2 |
  3307151.8 |
  3307637.5 |
  3308123.1 |
  3308608.8 |
  3309094.4 |
  3309580.1 |
  3310065.7 |
  3310551.4 |
  3311037.0 |####################
  3311522.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_wideselect_zig_anchor**: bridge=312.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_dispatch**: bridge=312.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_null**: bridge=12084.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_per_w_set**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_runtime_w**: bridge=312.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_dispatch**: bridge=307.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: bridge=308.1% of algo (FFI overhead may distort results)
