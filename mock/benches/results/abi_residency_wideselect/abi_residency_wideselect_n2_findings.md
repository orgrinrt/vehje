# abi_residency (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_residency_wideselect_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_wideselect_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_wideselect_null_entry dominates: 58784% faster than the next best (abi_residency_wideselect_reused_buffer)

abi_residency_wideselect_null_entry (3.50 us) leads abi_residency_wideselect_reused_buffer (2.06 ms) by 58784%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_wideselect_null_entry beats baseline by 100% (significant)

abi_residency_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_residency_wideselect_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_wideselect_fresh_alloc is an outlier: 590.3x slower than the field

abi_residency_wideselect_fresh_alloc (2.06 ms) is 590.3x the fastest (3.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 590.3x the fastest

Fastest abi_residency_wideselect_null_entry (3.50 us) to slowest abi_residency_wideselect_fresh_alloc (2.06 ms): 590.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_wideselect_null_entry** at 3495.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 590.33x (fastest 3495.2 ns, slowest 2063329.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2064437ns | 2065959ns | 2054561ns | 2064134ns | 2069828ns | +0.07% |
| abi_residency_wideselect_null_entry | 5742ns | 5783ns | 5382ns | 5697ns | 5991ns | -99.72% |
| abi_residency_wideselect_reused_buffer | 2062944ns | 2060851ns | 2054057ns | 2060430ns | 2071158ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2061868ns | 2052122ns | 2067296ns | +0.08% | 0.000 |
| abi_residency_wideselect_null_entry | 3465ns | 3256ns | 3602ns | -99.83% | 0.001 |
| abi_residency_wideselect_reused_buffer | 2060293ns | 2051664ns | 2068426ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 42231.5 | 2063163.0 | 2061867.6 | n/a |
| abi_residency_wideselect_null_entry | 27948.6 | 3541.6 | 3464.9 | n/a |
| abi_residency_wideselect_reused_buffer | 41140.6 | 2060543.5 | 2060292.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_wideselect_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.000 | 0.2% |
| abi_residency_wideselect_null_entry | 0.001 | 93.2% |
| abi_residency_wideselect_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2064437ns | 2064437ns | +0.07% |
| abi_residency_wideselect_null_entry | 5742ns | 5742ns | -99.72% |
| abi_residency_wideselect_reused_buffer | 2062944ns | 2062944ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_wideselect_reused_buffer | 2058114ns | base | --- | [2054339, 2068426] | --- | --- | --- | --- |
| abi_residency_wideselect_fresh_alloc | 2063329ns | no significant difference | [-5735, +7448]ns | [2054977, 2067296] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_wideselect_null_entry | 3495ns | -2054616.9ns (-99.8%) | [-2065128, -2050739]ns | [3298, 3602] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_wideselect_reused_buffer | abi_residency_wideselect_fresh_alloc | abi_residency_wideselect_null_entry |
|---|---|---|---|
| 1 | 2073478ns | -0.2% | -99.8% |
| 2 | 2057014ns | +0.4% | -99.8% |
| 3 | 2051664ns | +0.3% | -99.8% |
| 4 | 2058818ns | -0.3% | -99.8% |
| 5 | 2063373ns | +0.1% | -99.8% |
| 6 | 2057410ns | +0.2% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.038 | ok |
| abi_residency_wideselect_null_entry | -0.300 | moderate- |
| abi_residency_wideselect_reused_buffer | -0.056 | ok |

**Consistency summary:**

- **abi_residency_wideselect_fresh_alloc**: won 2/6, lost 4/6
- **abi_residency_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 6234003.6ns | 2061867.6ns | 302.3% | HIGH |
| abi_residency_wideselect_null_entry | 120722.5ns | 3464.9ns | 3484.1% | HIGH |
| abi_residency_wideselect_reused_buffer | 6223580.1ns | 2060292.8ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_residency_wideselect_fresh_alloc (n=6, range 2052122.1-2067296.5 ns)
  2052122.1 |########################################
  2052880.8 |
  2053639.5 |
  2054398.3 |
  2055157.0 |
  2055915.7 |
  2056674.4 |
  2057433.1 |########################################
  2058191.8 |
  2058950.6 |
  2059709.3 |
  2060468.0 |########################################
  2061226.7 |
  2061985.4 |
  2062744.1 |
  2063502.9 |
  2064261.6 |
  2065020.3 |########################################
  2065779.0 |########################################
  2066537.7 |
  (0 below, 1 above range)

abi_residency_wideselect_null_entry (n=6, range 3255.8-3601.5 ns)
   3255.8 |########################################
   3273.1 |
   3290.4 |
   3307.7 |
   3324.9 |########################################
   3342.2 |
   3359.5 |
   3376.8 |
   3394.1 |
   3411.4 |
   3428.7 |
   3445.9 |########################################
   3463.2 |
   3480.5 |
   3497.8 |
   3515.1 |########################################
   3532.4 |########################################
   3549.6 |
   3566.9 |
   3584.2 |
  (0 below, 1 above range)

abi_residency_wideselect_reused_buffer (n=6, range 2051663.8-2068425.6 ns)
  2051663.8 |####################
  2052501.9 |
  2053340.0 |
  2054178.1 |
  2055016.2 |
  2055854.2 |
  2056692.3 |########################################
  2057530.4 |
  2058368.5 |####################
  2059206.6 |
  2060044.7 |
  2060882.8 |
  2061720.9 |
  2062559.0 |####################
  2063397.1 |
  2064235.2 |
  2065073.2 |
  2065911.3 |
  2066749.4 |
  2067587.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_wideselect_fresh_alloc**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_null_entry**: bridge=3446.1% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_reused_buffer**: bridge=302.3% of algo (FFI overhead may distort results)
