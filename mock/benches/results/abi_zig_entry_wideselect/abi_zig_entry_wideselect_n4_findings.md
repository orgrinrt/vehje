# abi_zig_entry (wideselect)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_wideselect_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_wideselect_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_wideselect_zig_null dominates: 49358% faster than the next best (abi_zig_entry_wideselect_zig_per_w_set)

abi_zig_entry_wideselect_zig_null (3.99 us) leads abi_zig_entry_wideselect_zig_per_w_set (1.98 ms) by 49358%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_wideselect_zig_null beats baseline by 100% (significant)

abi_zig_entry_wideselect_zig_null is -1.97 ms (100%) faster than baseline abi_zig_entry_wideselect_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_wideselect_zig_tail_runtime_w is an outlier: 830.5x slower than the field

abi_zig_entry_wideselect_zig_tail_runtime_w (3.32 ms) is 830.5x the fastest (3.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_wideselect_zig_tail_dispatch shows alternating (throttle bounce) (autocorr -0.57)

abi_zig_entry_wideselect_zig_tail_dispatch's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_wideselect_zig_null} vs {abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} (49358% apart)

The field splits into a fast tier {abi_zig_entry_wideselect_zig_null} and a slow tier {abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} with a 49358% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 830.5x the fastest

Fastest abi_zig_entry_wideselect_zig_null (3.99 us) to slowest abi_zig_entry_wideselect_zig_tail_runtime_w (3.32 ms): 830.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_wideselect_zig_null** at 3993.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 830.49x (fastest 3993.8 ns, slowest 3316784.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1980106ns | 1979740ns | 1977049ns | 1979244ns | 1982928ns | +0.00% |
| abi_zig_entry_wideselect_zig_dispatch | 1979657ns | 1979072ns | 1972955ns | 1977464ns | 1986297ns | -0.02% |
| abi_zig_entry_wideselect_zig_null | 6278ns | 6282ns | 6156ns | 6247ns | 6386ns | -99.68% |
| abi_zig_entry_wideselect_zig_per_w_set | 1978853ns | 1977855ns | 1973292ns | 1977594ns | 1983522ns | -0.06% |
| abi_zig_entry_wideselect_zig_runtime_w | 1980040ns | 1980026ns | 1978085ns | 1979712ns | 1981510ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3314808ns | 3315025ns | 3305606ns | 3313221ns | 3321789ns | +67.41% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3318181ns | 3319612ns | 3308038ns | 3317610ns | 3324108ns | +67.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1977470ns | 1974531ns | 1980191ns | +0.00% | 0.000 |
| abi_zig_entry_wideselect_zig_dispatch | 1976944ns | 1970462ns | 1983486ns | -0.02% | 0.000 |
| abi_zig_entry_wideselect_zig_null | 3980ns | 3887ns | 4046ns | -99.80% | 0.001 |
| abi_zig_entry_wideselect_zig_per_w_set | 1976266ns | 1970745ns | 1980940ns | -0.06% | 0.000 |
| abi_zig_entry_wideselect_zig_runtime_w | 1977401ns | 1975534ns | 1978881ns | base | 0.000 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3312008ns | 3303105ns | 3318822ns | +67.49% | 0.000 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3315407ns | 3305321ns | 3321357ns | +67.66% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 176423.5 | 1976415.6 | 1977470.0 | 29 |
| abi_zig_entry_wideselect_zig_dispatch | 181269.2 | 1976053.8 | 1976944.0 | 6 |
| abi_zig_entry_wideselect_zig_null | 156383.4 | 4192.8 | 3979.8 | n/a |
| abi_zig_entry_wideselect_zig_per_w_set | 176831.7 | 1975765.2 | 1976266.1 | n/a |
| abi_zig_entry_wideselect_zig_runtime_w | 178421.7 | 1975761.9 | 1977400.8 | n/a |
| abi_zig_entry_wideselect_zig_tail_dispatch | 193314.0 | 3311357.7 | 3312008.1 | n/a |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 191454.6 | 3315460.7 | 3315406.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_wideselect_zig_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_null | 0.001 | 97.3% |
| abi_zig_entry_wideselect_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1980106ns | 1980106ns | +0.00% |
| abi_zig_entry_wideselect_zig_dispatch | 1979657ns | 1979657ns | -0.02% |
| abi_zig_entry_wideselect_zig_null | 6278ns | 6278ns | -99.68% |
| abi_zig_entry_wideselect_zig_per_w_set | 1978853ns | 1978853ns | -0.06% |
| abi_zig_entry_wideselect_zig_runtime_w | 1980040ns | 1980040ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3314808ns | 3314808ns | +67.41% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3318181ns | 3318181ns | +67.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_runtime_w | 1977377ns | base | --- | [1975945, 1978881] | --- | --- | --- | --- |
| abi_zig_entry_wideselect_zig_anchor | 1977070ns | no significant difference | [-2412, +3285]ns | [1975149, 1980191] | no | 0.8250 | 0.6875 | 0 |
| abi_zig_entry_wideselect_zig_dispatch | 1976311ns | no significant difference | [-7115, +5378]ns | [1971035, 1983486] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_wideselect_zig_null | 3994ns | -1973431.9ns (-99.8%) | [-1974837, -1971995]ns | [3900, 4046] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_per_w_set | 1975246ns | no significant difference | [-4255, +2060]ns | [1972612, 1980940] | no | 0.8250 | 0.6875 | 0 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3312181ns | +1334031.2ns (+67.5%) | [+1329077, +1340714]ns | [3305021, 3318822] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3316784ns | +1338596.3ns (+67.7%) | [+1330481, +1344941]ns | [3308080, 3321357] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_wideselect_zig_runtime_w | abi_zig_entry_wideselect_zig_anchor | abi_zig_entry_wideselect_zig_dispatch | abi_zig_entry_wideselect_zig_null | abi_zig_entry_wideselect_zig_per_w_set | abi_zig_entry_wideselect_zig_tail_dispatch | abi_zig_entry_wideselect_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1978919ns | -0.1% | +0.2% | -99.8% | +0.2% | +67.6% | +67.7% |
| 2 | 1978842ns | -0.2% | -0.4% | -99.8% | +0.0% | +67.4% | +67.3% |
| 3 | 1975534ns | -0.1% | +0.2% | -99.8% | -0.1% | +67.2% | +68.2% |
| 4 | 1977298ns | -0.0% | +0.3% | -99.8% | -0.1% | +67.9% | +67.9% |
| 5 | 1976355ns | +0.3% | -0.1% | -99.8% | -0.1% | +67.3% | +67.2% |
| 6 | 1977457ns | +0.1% | -0.3% | -99.8% | -0.3% | +67.5% | +67.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.291 | moderate+ |
| abi_zig_entry_wideselect_zig_dispatch | -0.281 | moderate- |
| abi_zig_entry_wideselect_zig_null | 0.332 | moderate+ |
| abi_zig_entry_wideselect_zig_per_w_set | 0.263 | moderate+ |
| abi_zig_entry_wideselect_zig_runtime_w | -0.029 | ok |
| abi_zig_entry_wideselect_zig_tail_dispatch | -0.573 | HIGH- (thermal bounce) |
| abi_zig_entry_wideselect_zig_tail_runtime_w | -0.293 | moderate- |

**Consistency summary:**

- **abi_zig_entry_wideselect_zig_anchor**: won 1/6, lost 1/6
- **abi_zig_entry_wideselect_zig_dispatch**: won 3/6, lost 3/6
- **abi_zig_entry_wideselect_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_wideselect_zig_per_w_set**: won 1/6, lost 1/6
- **abi_zig_entry_wideselect_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 6173569.2ns | 1977470.0ns | 312.2% | HIGH |
| abi_zig_entry_wideselect_zig_dispatch | 6181754.7ns | 1976944.0ns | 312.7% | HIGH |
| abi_zig_entry_wideselect_zig_null | 306924.5ns | 3979.8ns | 7712.1% | HIGH |
| abi_zig_entry_wideselect_zig_per_w_set | 6172122.3ns | 1976266.1ns | 312.3% | HIGH |
| abi_zig_entry_wideselect_zig_runtime_w | 6175716.5ns | 1977400.8ns | 312.3% | HIGH |
| abi_zig_entry_wideselect_zig_tail_dispatch | 10202359.2ns | 3312008.1ns | 308.0% | HIGH |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 10209284.8ns | 3315406.9ns | 307.9% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_wideselect_zig_anchor (n=6, range 1974530.8-1980190.9 ns)
  1974530.8 |########################################
  1974813.8 |
  1975096.8 |
  1975379.8 |
  1975662.8 |########################################
  1975945.8 |
  1976228.8 |
  1976511.8 |
  1976794.8 |########################################
  1977077.8 |########################################
  1977360.8 |
  1977643.8 |
  1977926.8 |
  1978209.8 |
  1978492.8 |########################################
  1978775.8 |
  1979058.8 |
  1979341.8 |
  1979624.8 |
  1979907.8 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_dispatch (n=6, range 1970462.5-1983486.2 ns)
  1970462.5 |########################################
  1971113.7 |########################################
  1971764.9 |
  1972416.1 |
  1973067.2 |
  1973718.4 |########################################
  1974369.6 |
  1975020.8 |
  1975672.0 |
  1976323.2 |
  1976974.4 |
  1977625.6 |
  1978276.8 |########################################
  1978927.9 |
  1979579.1 |
  1980230.3 |
  1980881.5 |
  1981532.7 |
  1982183.9 |
  1982835.1 |########################################
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_null (n=6, range 3886.7-4045.6 ns)
   3886.7 |####################
   3894.6 |
   3902.6 |
   3910.5 |####################
   3918.5 |
   3926.4 |
   3934.4 |
   3942.3 |
   3950.3 |
   3958.2 |
   3966.1 |
   3974.1 |####################
   3982.0 |
   3990.0 |
   3997.9 |
   4005.9 |########################################
   4013.8 |
   4021.8 |
   4029.7 |
   4037.7 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_per_w_set (n=6, range 1970744.6-1980940.4 ns)
  1970744.6 |####################
  1971254.4 |
  1971764.2 |
  1972274.0 |
  1972783.8 |
  1973293.6 |
  1973803.3 |
  1974313.1 |########################################
  1974822.9 |
  1975332.7 |
  1975842.5 |####################
  1976352.3 |
  1976862.1 |
  1977371.9 |
  1977881.7 |
  1978391.4 |####################
  1978901.2 |
  1979411.0 |
  1979920.8 |
  1980430.6 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_runtime_w (n=6, range 1975534.2-1978880.6 ns)
  1975534.2 |########################################
  1975701.5 |
  1975868.8 |
  1976036.2 |
  1976203.5 |########################################
  1976370.8 |
  1976538.1 |
  1976705.5 |
  1976872.8 |
  1977040.1 |
  1977207.4 |########################################
  1977374.7 |########################################
  1977542.1 |
  1977709.4 |
  1977876.7 |
  1978044.0 |
  1978211.4 |
  1978378.7 |
  1978546.0 |
  1978713.3 |########################################
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_dispatch (n=6, range 3303105.4-3318822.1 ns)
  3303105.4 |########################################
  3303891.2 |
  3304677.1 |
  3305462.9 |
  3306248.7 |########################################
  3307034.6 |
  3307820.4 |
  3308606.2 |
  3309392.1 |
  3310177.9 |
  3310963.8 |########################################
  3311749.6 |
  3312535.4 |########################################
  3313321.3 |
  3314107.1 |
  3314892.9 |
  3315678.8 |
  3316464.6 |
  3317250.4 |########################################
  3318036.3 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_runtime_w (n=6, range 3305320.8-3321357.1 ns)
  3305320.8 |########################################
  3306122.6 |
  3306924.4 |
  3307726.2 |
  3308528.1 |
  3309329.9 |
  3310131.7 |########################################
  3310933.5 |
  3311735.3 |
  3312537.1 |
  3313339.0 |
  3314140.8 |
  3314942.6 |########################################
  3315744.4 |
  3316546.2 |
  3317348.0 |########################################
  3318149.8 |
  3318951.7 |
  3319753.5 |########################################
  3320555.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_wideselect_zig_anchor**: bridge=312.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_dispatch**: bridge=312.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_null**: bridge=7696.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_per_w_set**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_runtime_w**: bridge=312.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_dispatch**: bridge=307.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: bridge=307.8% of algo (FFI overhead may distort results)
