# abi_residency (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_residency_wideselect_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_wideselect_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_wideselect_null_entry dominates: 81230% faster than the next best (abi_residency_wideselect_reused_buffer)

abi_residency_wideselect_null_entry (2.52 us) leads abi_residency_wideselect_reused_buffer (2.05 ms) by 81230%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_wideselect_null_entry beats baseline by 100% (significant)

abi_residency_wideselect_null_entry is -2.04 ms (100%) faster than baseline abi_residency_wideselect_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_wideselect_fresh_alloc is an outlier: 816.1x slower than the field

abi_residency_wideselect_fresh_alloc (2.05 ms) is 816.1x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 816.1x the fastest

Fastest abi_residency_wideselect_null_entry (2.52 us) to slowest abi_residency_wideselect_fresh_alloc (2.05 ms): 816.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_wideselect_null_entry** at 2516.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 816.07x (fastest 2516.6 ns, slowest 2053754.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2054908ns | 2056321ns | 2040498ns | 2055243ns | 2061610ns | +0.30% |
| abi_residency_wideselect_null_entry | 4798ns | 4794ns | 4610ns | 4757ns | 4953ns | -99.77% |
| abi_residency_wideselect_reused_buffer | 2048707ns | 2049532ns | 2032622ns | 2048559ns | 2056973ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2052361ns | 2037804ns | 2059111ns | +0.31% | 0.000 |
| abi_residency_wideselect_null_entry | 2519ns | 2426ns | 2596ns | -99.88% | 0.025 |
| abi_residency_wideselect_reused_buffer | 2046095ns | 2030170ns | 2054367ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 40184.6 | 2052433.7 | 2052360.9 | 0 |
| abi_residency_wideselect_null_entry | 28070.0 | 2748.4 | 2518.9 | n/a |
| abi_residency_wideselect_reused_buffer | 40853.3 | 2047096.3 | 2046095.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_residency_wideselect_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.000 | 0.1% |
| abi_residency_wideselect_null_entry | 0.025 | 96.4% |
| abi_residency_wideselect_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2054908ns | 2054908ns | +0.30% |
| abi_residency_wideselect_null_entry | 4798ns | 4798ns | -99.77% |
| abi_residency_wideselect_reused_buffer | 2048707ns | 2048707ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_wideselect_reused_buffer | 2046802ns | base | --- | [2037116, 2054367] | --- | --- | --- | --- |
| abi_residency_wideselect_fresh_alloc | 2053754ns | no significant difference | [-6927, +20899]ns | [2044217, 2059111] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_wideselect_null_entry | 2517ns | -2044334.8ns (-99.9%) | [-2051777, -2034617]ns | [2444, 2596] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_wideselect_reused_buffer | abi_residency_wideselect_fresh_alloc | abi_residency_wideselect_null_entry |
|---|---|---|---|
| 1 | 2044062ns | -0.3% | -99.9% |
| 2 | 2030170ns | +1.5% | -99.9% |
| 3 | 2047349ns | +0.3% | -99.9% |
| 4 | 2046255ns | +0.5% | -99.9% |
| 5 | 2050508ns | +0.2% | -99.9% |
| 6 | 2058227ns | -0.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | -0.324 | moderate- |
| abi_residency_wideselect_null_entry | -0.109 | ok |
| abi_residency_wideselect_reused_buffer | 0.157 | ok |

**Consistency summary:**

- **abi_residency_wideselect_fresh_alloc**: won 2/6, lost 4/6
- **abi_residency_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 6199900.2ns | 2052360.9ns | 302.1% | HIGH |
| abi_residency_wideselect_null_entry | 113766.0ns | 2518.9ns | 4516.5% | HIGH |
| abi_residency_wideselect_reused_buffer | 6184238.0ns | 2046095.1ns | 302.2% | HIGH |

## Distribution (algo ns)

```
abi_residency_wideselect_fresh_alloc (n=6, range 2037803.8-2059111.2 ns)
  2037803.8 |########################################
  2038869.2 |
  2039934.5 |
  2040999.9 |
  2042065.3 |
  2043130.7 |
  2044196.0 |
  2045261.4 |
  2046326.8 |
  2047392.2 |
  2048457.5 |
  2049522.9 |
  2050588.3 |########################################
  2051653.6 |
  2052719.0 |########################################
  2053784.4 |########################################
  2054849.8 |
  2055915.1 |
  2056980.5 |########################################
  2058045.9 |
  (0 below, 1 above range)

abi_residency_wideselect_null_entry (n=6, range 2425.8-2596.2 ns)
   2425.8 |########################################
   2434.3 |
   2442.8 |
   2451.4 |
   2459.9 |########################################
   2468.4 |########################################
   2476.9 |
   2485.5 |
   2494.0 |
   2502.5 |
   2511.0 |
   2519.5 |
   2528.1 |
   2536.6 |
   2545.1 |
   2553.6 |########################################
   2562.2 |
   2570.7 |########################################
   2579.2 |
   2587.7 |
  (0 below, 1 above range)

abi_residency_wideselect_reused_buffer (n=6, range 2030170.4-2054367.1 ns)
  2030170.4 |########################################
  2031380.2 |
  2032590.1 |
  2033799.9 |
  2035009.7 |
  2036219.6 |
  2037429.4 |
  2038639.2 |
  2039849.1 |
  2041058.9 |
  2042268.8 |
  2043478.6 |########################################
  2044688.4 |
  2045898.3 |########################################
  2047108.1 |########################################
  2048317.9 |
  2049527.8 |########################################
  2050737.6 |
  2051947.4 |
  2053157.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_wideselect_fresh_alloc**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_null_entry**: bridge=4540.5% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_reused_buffer**: bridge=302.5% of algo (FFI overhead may distort results)
