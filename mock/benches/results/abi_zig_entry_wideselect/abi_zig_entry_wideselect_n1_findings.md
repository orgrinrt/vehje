# abi_zig_entry (wideselect)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_wideselect_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_wideselect_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_wideselect_zig_null dominates: 39354% faster than the next best (abi_zig_entry_wideselect_zig_anchor)

abi_zig_entry_wideselect_zig_null (5.00 us) leads abi_zig_entry_wideselect_zig_anchor (1.97 ms) by 39354%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_wideselect_zig_null beats baseline by 100% (significant)

abi_zig_entry_wideselect_zig_null is -1.98 ms (100%) faster than baseline abi_zig_entry_wideselect_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_wideselect_zig_tail_runtime_w is an outlier: 663.4x slower than the field

abi_zig_entry_wideselect_zig_tail_runtime_w (3.32 ms) is 663.4x the fastest (5.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_wideselect_zig_null shows alternating (throttle bounce) (autocorr -0.52)

abi_zig_entry_wideselect_zig_null's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_wideselect_zig_null} vs {abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} (39354% apart)

The field splits into a fast tier {abi_zig_entry_wideselect_zig_null} and a slow tier {abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} with a 39354% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 663.4x the fastest

Fastest abi_zig_entry_wideselect_zig_null (5.00 us) to slowest abi_zig_entry_wideselect_zig_tail_runtime_w (3.32 ms): 663.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_wideselect_zig_null** at 5005.0 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 663.38x (fastest 5005.0 ns, slowest 3320234.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1978698ns | 1977255ns | 1974424ns | 1976568ns | 1984028ns | -0.26% |
| abi_zig_entry_wideselect_zig_dispatch | 1978922ns | 1977980ns | 1974530ns | 1977530ns | 1983207ns | -0.25% |
| abi_zig_entry_wideselect_zig_null | 7317ns | 7299ns | 7188ns | 7286ns | 7427ns | -99.63% |
| abi_zig_entry_wideselect_zig_per_w_set | 2017222ns | 1979750ns | 1972988ns | 1978256ns | 2097788ns | +1.68% |
| abi_zig_entry_wideselect_zig_runtime_w | 1983899ns | 1984039ns | 1978385ns | 1982830ns | 1988259ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3319327ns | 3319003ns | 3314439ns | 3317962ns | 3323817ns | +67.31% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3323449ns | 3323154ns | 3315472ns | 3322373ns | 3329051ns | +67.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1976115ns | 1971938ns | 1981354ns | -0.26% | 0.000 |
| abi_zig_entry_wideselect_zig_dispatch | 1976283ns | 1971991ns | 1980530ns | -0.25% | 0.000 |
| abi_zig_entry_wideselect_zig_null | 5034ns | 4962ns | 5124ns | -99.75% | 0.000 |
| abi_zig_entry_wideselect_zig_per_w_set | 2014557ns | 1970535ns | 2094893ns | +1.68% | 0.000 |
| abi_zig_entry_wideselect_zig_runtime_w | 1981201ns | 1975847ns | 1985486ns | base | 0.000 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3316431ns | 3311418ns | 3320891ns | +67.39% | 0.000 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3320600ns | 3312796ns | 3326144ns | +67.61% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 177091.2 | 1975967.6 | 1976114.7 | n/a |
| abi_zig_entry_wideselect_zig_dispatch | 179840.8 | 1975753.0 | 1976282.6 | 0 |
| abi_zig_entry_wideselect_zig_null | 155896.0 | 5136.6 | 5034.4 | n/a |
| abi_zig_entry_wideselect_zig_per_w_set | 184386.6 | 1997554.0 | 2014556.9 | n/a |
| abi_zig_entry_wideselect_zig_runtime_w | 179794.6 | 1981278.1 | 1981201.3 | n/a |
| abi_zig_entry_wideselect_zig_tail_dispatch | 194460.7 | 3315299.1 | 3316431.0 | n/a |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 191181.4 | 3315795.8 | 3320599.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_zig_entry_wideselect_zig_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.000 | 0.3% |
| abi_zig_entry_wideselect_zig_dispatch | 0.000 | 0.3% |
| abi_zig_entry_wideselect_zig_null | 0.000 | 99.1% |
| abi_zig_entry_wideselect_zig_per_w_set | 0.000 | 0.3% |
| abi_zig_entry_wideselect_zig_runtime_w | 0.000 | 0.3% |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1978698ns | 1978698ns | -0.26% |
| abi_zig_entry_wideselect_zig_dispatch | 1978922ns | 1978922ns | -0.25% |
| abi_zig_entry_wideselect_zig_null | 7317ns | 7317ns | -99.63% |
| abi_zig_entry_wideselect_zig_per_w_set | 2017222ns | 2017222ns | +1.68% |
| abi_zig_entry_wideselect_zig_runtime_w | 1983899ns | 1983899ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3319327ns | 3319327ns | +67.31% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3323449ns | 3323449ns | +67.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_runtime_w | 1981297ns | base | --- | [1976821, 1985486] | --- | --- | --- | --- |
| abi_zig_entry_wideselect_zig_anchor | 1974662ns | no significant difference | [-10472, +1584]ns | [1972329, 1981354] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_wideselect_zig_dispatch | 1975323ns | no significant difference | [-10164, +433]ns | [1972995, 1980530] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_wideselect_zig_null | 5005ns | -1976296.5ns (-99.7%) | [-1980465, -1971739]ns | [4974, 5124] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_per_w_set | 1977143ns | no significant difference | [-8536, +110871]ns | [1971635, 2094893] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3316194ns | +1333276.9ns (+67.3%) | [+1328342, +1344070]ns | [3312208, 3320891] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3320235ns | +1340697.5ns (+67.7%) | [+1334115, +1343382]ns | [3315420, 3326144] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_wideselect_zig_runtime_w | abi_zig_entry_wideselect_zig_anchor | abi_zig_entry_wideselect_zig_dispatch | abi_zig_entry_wideselect_zig_null | abi_zig_entry_wideselect_zig_per_w_set | abi_zig_entry_wideselect_zig_tail_dispatch | abi_zig_entry_wideselect_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1977795ns | -0.3% | -0.3% | -99.7% | +0.0% | +68.0% | +67.8% |
| 2 | 1987280ns | -0.6% | -0.6% | -99.7% | +10.9% | +66.8% | +67.0% |
| 3 | 1981830ns | -0.3% | -0.2% | -99.7% | -0.5% | +67.4% | +67.9% |
| 4 | 1980764ns | -0.4% | +0.1% | -99.7% | +0.2% | +67.2% | +67.7% |
| 5 | 1975847ns | +0.4% | -0.1% | -99.7% | -0.3% | +67.9% | +67.7% |
| 6 | 1983693ns | -0.2% | -0.4% | -99.7% | -0.4% | +67.0% | +67.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.090 | ok |
| abi_zig_entry_wideselect_zig_dispatch | -0.019 | ok |
| abi_zig_entry_wideselect_zig_null | -0.522 | HIGH- (thermal bounce) |
| abi_zig_entry_wideselect_zig_per_w_set | -0.242 | moderate- |
| abi_zig_entry_wideselect_zig_runtime_w | -0.335 | moderate- |
| abi_zig_entry_wideselect_zig_tail_dispatch | -0.385 | moderate- |
| abi_zig_entry_wideselect_zig_tail_runtime_w | -0.336 | moderate- |

**Consistency summary:**

- **abi_zig_entry_wideselect_zig_anchor**: won 5/6, lost 1/6
- **abi_zig_entry_wideselect_zig_dispatch**: won 4/6, lost 1/6
- **abi_zig_entry_wideselect_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_wideselect_zig_per_w_set**: won 3/6, lost 2/6
- **abi_zig_entry_wideselect_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 6173715.6ns | 1976114.7ns | 312.4% | HIGH |
| abi_zig_entry_wideselect_zig_dispatch | 6178124.6ns | 1976282.6ns | 312.6% | HIGH |
| abi_zig_entry_wideselect_zig_null | 314300.3ns | 5034.4ns | 6243.0% | HIGH |
| abi_zig_entry_wideselect_zig_per_w_set | 6229686.2ns | 2014556.9ns | 309.2% | HIGH |
| abi_zig_entry_wideselect_zig_runtime_w | 6191927.1ns | 1981201.3ns | 312.5% | HIGH |
| abi_zig_entry_wideselect_zig_tail_dispatch | 10214387.2ns | 3316431.0ns | 308.0% | HIGH |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 10215014.3ns | 3320599.5ns | 307.6% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_wideselect_zig_anchor (n=6, range 1971937.9-1981353.5 ns)
  1971937.9 |########################################
  1972408.7 |########################################
  1972879.5 |
  1973350.2 |
  1973821.0 |
  1974291.8 |########################################
  1974762.6 |########################################
  1975233.4 |
  1975704.2 |
  1976174.9 |
  1976645.7 |
  1977116.5 |
  1977587.3 |
  1978058.1 |
  1978528.9 |########################################
  1978999.6 |
  1979470.4 |
  1979941.2 |
  1980412.0 |
  1980882.8 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_dispatch (n=6, range 1971990.8-1980530.0 ns)
  1971990.8 |########################################
  1972417.8 |
  1972844.7 |
  1973271.7 |
  1973698.6 |########################################
  1974125.6 |
  1974552.6 |########################################
  1974979.5 |
  1975406.5 |
  1975833.4 |########################################
  1976260.4 |
  1976687.4 |
  1977114.3 |
  1977541.3 |########################################
  1977968.2 |
  1978395.2 |
  1978822.2 |
  1979249.1 |
  1979676.1 |
  1980103.0 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_null (n=6, range 4962.1-5124.1 ns)
   4962.1 |########################################
   4970.2 |
   4978.3 |########################################
   4986.4 |
   4994.5 |########################################
   5002.6 |
   5010.7 |########################################
   5018.8 |
   5026.9 |
   5035.0 |
   5043.1 |
   5051.2 |
   5059.3 |
   5067.4 |
   5075.5 |########################################
   5083.6 |
   5091.7 |
   5099.8 |
   5107.9 |
   5116.0 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_per_w_set (n=6, range 1970535.0-2094893.1 ns)
  1970535.0 |########################################
  1976752.9 |#############
  1982970.8 |#############
  1989188.7 |
  1995406.6 |
  2001624.5 |
  2007842.4 |
  2014060.3 |
  2020278.2 |
  2026496.1 |
  2032714.0 |
  2038932.0 |
  2045149.9 |
  2051367.8 |
  2057585.7 |
  2063803.6 |
  2070021.5 |
  2076239.4 |
  2082457.3 |
  2088675.2 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_runtime_w (n=6, range 1975846.7-1985486.5 ns)
  1975846.7 |########################################
  1976328.7 |
  1976810.7 |
  1977292.7 |
  1977774.6 |########################################
  1978256.6 |
  1978738.6 |
  1979220.6 |
  1979702.6 |
  1980184.6 |
  1980666.6 |########################################
  1981148.6 |
  1981630.6 |########################################
  1982112.5 |
  1982594.5 |
  1983076.5 |
  1983558.5 |########################################
  1984040.5 |
  1984522.5 |
  1985004.5 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_dispatch (n=6, range 3311417.5-3320890.6 ns)
  3311417.5 |########################################
  3311891.2 |
  3312364.8 |
  3312838.5 |########################################
  3313312.1 |
  3313785.8 |
  3314259.4 |########################################
  3314733.1 |
  3315206.7 |
  3315680.4 |
  3316154.0 |
  3316627.7 |
  3317101.4 |
  3317575.0 |########################################
  3318048.7 |########################################
  3318522.3 |
  3318996.0 |
  3319469.6 |
  3319943.3 |
  3320416.9 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_runtime_w (n=6, range 3312796.2-3326143.5 ns)
  3312796.2 |########################################
  3313463.6 |
  3314130.9 |
  3314798.3 |
  3315465.7 |
  3316133.0 |
  3316800.4 |
  3317467.8 |########################################
  3318135.1 |########################################
  3318802.5 |
  3319469.9 |
  3320137.2 |
  3320804.6 |
  3321472.0 |########################################
  3322139.3 |
  3322806.7 |
  3323474.1 |
  3324141.4 |
  3324808.8 |########################################
  3325476.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_wideselect_zig_anchor**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_dispatch**: bridge=312.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_null**: bridge=6224.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_per_w_set**: bridge=313.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_runtime_w**: bridge=312.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_dispatch**: bridge=307.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: bridge=307.7% of algo (FFI overhead may distort results)
