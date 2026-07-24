# abi_lifecycle (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_wideselect_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_wideselect_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_wideselect_null_entry dominates: 50711% faster than the next best (abi_lifecycle_wideselect_held_handle)

abi_lifecycle_wideselect_null_entry (4.05 us) leads abi_lifecycle_wideselect_held_handle (2.06 ms) by 50711%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_wideselect_null_entry beats baseline by 100% (significant)

abi_lifecycle_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_lifecycle_wideselect_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_wideselect_fresh_per_batch is an outlier: 707.5x slower than the field

abi_lifecycle_wideselect_fresh_per_batch (2.86 ms) is 707.5x the fastest (4.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_wideselect_null_entry shows alternating (throttle bounce) (autocorr -0.65)

abi_lifecycle_wideselect_null_entry's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_wideselect_null_entry} vs {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} (50711% apart)

The field splits into a fast tier {abi_lifecycle_wideselect_null_entry} and a slow tier {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} with a 50711% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 707.5x the fastest

Fastest abi_lifecycle_wideselect_null_entry (4.05 us) to slowest abi_lifecycle_wideselect_fresh_per_batch (2.86 ms): 707.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_wideselect_null_entry** at 4045.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 707.46x (fastest 4045.8 ns, slowest 2862277.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2863961ns | 2864841ns | 2856271ns | 2863828ns | 2868004ns | +39.20% |
| abi_lifecycle_wideselect_fresh_per_column | 2073498ns | 2075387ns | 2063971ns | 2073830ns | 2077764ns | +0.78% |
| abi_lifecycle_wideselect_held_handle | 2057375ns | 2058231ns | 2044020ns | 2057457ns | 2063930ns | base |
| abi_lifecycle_wideselect_null_entry | 6303ns | 6316ns | 6065ns | 6241ns | 6514ns | -99.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2861365ns | 2853690ns | 2865449ns | +39.25% | 0.000 |
| abi_lifecycle_wideselect_fresh_per_column | 2071001ns | 2061599ns | 2075236ns | +0.78% | 0.000 |
| abi_lifecycle_wideselect_held_handle | 2054907ns | 2041606ns | 2061433ns | base | 0.000 |
| abi_lifecycle_wideselect_null_entry | 4032ns | 3897ns | 4148ns | -99.80% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 39958.4 | 2862695.3 | 2861365.4 | n/a |
| abi_lifecycle_wideselect_fresh_per_column | 37479.1 | 2072262.8 | 2071000.6 | 0 |
| abi_lifecycle_wideselect_held_handle | 38336.6 | 2055296.6 | 2054907.1 | n/a |
| abi_lifecycle_wideselect_null_entry | 27412.9 | 4148.6 | 4031.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_wideselect_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_wideselect_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_wideselect_held_handle | 0.000 | 0.2% |
| abi_lifecycle_wideselect_null_entry | 0.001 | 96.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2863961ns | 2863961ns | +39.20% |
| abi_lifecycle_wideselect_fresh_per_column | 2073498ns | 2073498ns | +0.78% |
| abi_lifecycle_wideselect_held_handle | 2057375ns | 2057375ns | base |
| abi_lifecycle_wideselect_null_entry | 6303ns | 6303ns | -99.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_held_handle | 2055736ns | base | --- | [2047552, 2061433] | --- | --- | --- | --- |
| abi_lifecycle_wideselect_fresh_per_batch | 2862277ns | +805557.1ns (+39.2%) | [+797419, +816399]ns | [2856370, 2865449] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_wideselect_fresh_per_column | 2072919ns | +19000.7ns (+0.9%) | [+3414, +25866]ns | [2064847, 2075236] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_wideselect_null_entry | 4046ns | -2051763.9ns (-99.8%) | [-2057393, -2043469]ns | [3901, 4148] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_wideselect_held_handle | abi_lifecycle_wideselect_fresh_per_batch | abi_lifecycle_wideselect_fresh_per_column | abi_lifecycle_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2054948ns | +39.1% | +0.9% | -99.8% |
| 2 | 2056875ns | +39.2% | +0.5% | -99.8% |
| 3 | 2065992ns | +38.6% | -0.2% | -99.8% |
| 4 | 2053498ns | +39.3% | +1.0% | -99.8% |
| 5 | 2056525ns | +38.8% | +0.9% | -99.8% |
| 6 | 2041606ns | +40.4% | +1.5% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | -0.378 | moderate- |
| abi_lifecycle_wideselect_fresh_per_column | 0.031 | ok |
| abi_lifecycle_wideselect_held_handle | -0.057 | ok |
| abi_lifecycle_wideselect_null_entry | -0.647 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_lifecycle_wideselect_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 8626282.1ns | 2861365.4ns | 301.5% | HIGH |
| abi_lifecycle_wideselect_fresh_per_column | 6256273.7ns | 2071000.6ns | 302.1% | HIGH |
| abi_lifecycle_wideselect_held_handle | 6206101.0ns | 2054907.1ns | 302.0% | HIGH |
| abi_lifecycle_wideselect_null_entry | 122269.2ns | 4031.8ns | 3032.6% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_wideselect_fresh_per_batch (n=6, range 2853689.6-2865449.4 ns)
  2853689.6 |########################################
  2854277.6 |
  2854865.6 |
  2855453.6 |
  2856041.6 |
  2856629.5 |
  2857217.5 |
  2857805.5 |
  2858393.5 |
  2858981.5 |########################################
  2859569.5 |
  2860157.5 |
  2860745.5 |########################################
  2861333.4 |
  2861921.4 |
  2862509.4 |
  2863097.4 |########################################
  2863685.4 |########################################
  2864273.4 |
  2864861.4 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_fresh_per_column (n=6, range 2061599.2-2075235.6 ns)
  2061599.2 |########################################
  2062281.0 |
  2062962.8 |
  2063644.7 |
  2064326.5 |
  2065008.3 |
  2065690.1 |
  2066372.0 |
  2067053.8 |
  2067735.6 |########################################
  2068417.4 |
  2069099.2 |
  2069781.1 |
  2070462.9 |
  2071144.7 |
  2071826.5 |########################################
  2072508.4 |
  2073190.2 |
  2073872.0 |########################################
  2074553.8 |########################################
  (0 below, 1 above range)

abi_lifecycle_wideselect_held_handle (n=6, range 2041605.8-2061433.4 ns)
  2041605.8 |####################
  2042597.2 |
  2043588.6 |
  2044579.9 |
  2045571.3 |
  2046562.7 |
  2047554.1 |
  2048545.4 |
  2049536.8 |
  2050528.2 |
  2051519.6 |
  2052511.0 |####################
  2053502.3 |
  2054493.7 |####################
  2055485.1 |
  2056476.5 |########################################
  2057467.8 |
  2058459.2 |
  2059450.6 |
  2060442.0 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_null_entry (n=6, range 3897.1-4148.1 ns)
   3897.1 |########################################
   3909.7 |
   3922.2 |
   3934.8 |
   3947.3 |
   3959.8 |
   3972.4 |
   3985.0 |
   3997.5 |
   4010.1 |
   4022.6 |
   4035.2 |########################################
   4047.7 |
   4060.2 |
   4072.8 |
   4085.4 |
   4097.9 |
   4110.5 |####################
   4123.0 |
   4135.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_wideselect_fresh_per_batch**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_fresh_per_column**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_held_handle**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_null_entry**: bridge=3028.9% of algo (FFI overhead may distort results)
