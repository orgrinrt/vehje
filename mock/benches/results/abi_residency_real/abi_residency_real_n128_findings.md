# abi_residency (real)

3 variants, 6 samples per variant.
Baseline: **abi_residency_real_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_real_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_real_null_entry dominates: 76935% faster than the next best (abi_residency_real_reused_buffer)

abi_residency_real_null_entry (2.77 us) leads abi_residency_real_reused_buffer (2.14 ms) by 76935%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_real_null_entry beats baseline by 100% (significant)

abi_residency_real_null_entry is -2.13 ms (100%) faster than baseline abi_residency_real_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_real_fresh_alloc is an outlier: 770.9x slower than the field

abi_residency_real_fresh_alloc (2.14 ms) is 770.9x the fastest (2.77 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 770.9x the fastest

Fastest abi_residency_real_null_entry (2.77 us) to slowest abi_residency_real_fresh_alloc (2.14 ms): 770.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_real_null_entry** at 2772.5 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 770.85x (fastest 2772.5 ns, slowest 2137183.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2142602ns | 2139799ns | 2135104ns | 2139317ns | 2151278ns | +0.22% |
| abi_residency_real_null_entry | 5056ns | 5081ns | 4833ns | 5015ns | 5228ns | -99.76% |
| abi_residency_real_reused_buffer | 2137867ns | 2138340ns | 2131450ns | 2136456ns | 2143191ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2139998ns | 2132356ns | 2148704ns | +0.22% | 0.000 |
| abi_residency_real_null_entry | 2752ns | 2628ns | 2829ns | -99.87% | 0.047 |
| abi_residency_real_reused_buffer | 2135314ns | 2128898ns | 2140605ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 41603.3 | 2140287.6 | 2139997.8 | n/a |
| abi_residency_real_null_entry | 27815.7 | 2778.4 | 2751.9 | n/a |
| abi_residency_real_reused_buffer | 41125.4 | 2137335.7 | 2135313.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_residency_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.000 | 0.1% |
| abi_residency_real_null_entry | 0.046 | 94.8% |
| abi_residency_real_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_real_fresh_alloc | 2142602ns | 2142602ns | +0.22% |
| abi_residency_real_null_entry | 5056ns | 5056ns | -99.76% |
| abi_residency_real_reused_buffer | 2137867ns | 2137867ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_real_reused_buffer | 2135807ns | base | --- | [2129529, 2140605] | --- | --- | --- | --- |
| abi_residency_real_fresh_alloc | 2137184ns | no significant difference | [-3935, +15485]ns | [2134106, 2148704] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_real_null_entry | 2772ns | -2132977.8ns (-99.9%) | [-2137951, -2126756]ns | [2654, 2829] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_real_reused_buffer | abi_residency_real_fresh_alloc | abi_residency_real_null_entry |
|---|---|---|---|
| 1 | 2137540ns | +0.9% | -99.9% |
| 2 | 2138299ns | -0.1% | -99.9% |
| 3 | 2142911ns | -0.3% | -99.9% |
| 4 | 2130159ns | +0.3% | -99.9% |
| 5 | 2134075ns | -0.1% | -99.9% |
| 6 | 2128898ns | +0.5% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_real_fresh_alloc | -0.083 | ok |
| abi_residency_real_null_entry | 0.260 | moderate+ |
| abi_residency_real_reused_buffer | 0.032 | ok |

**Consistency summary:**

- **abi_residency_real_fresh_alloc**: won 2/6, lost 3/6
- **abi_residency_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 6467174.0ns | 2139997.8ns | 302.2% | HIGH |
| abi_residency_real_null_entry | 120162.1ns | 2751.9ns | 4366.5% | HIGH |
| abi_residency_real_reused_buffer | 6455081.3ns | 2135313.6ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_residency_real_fresh_alloc (n=6, range 2132356.2-2148704.1 ns)
  2132356.2 |########################################
  2133173.6 |
  2133991.0 |
  2134808.4 |
  2135625.8 |########################################
  2136443.2 |########################################
  2137260.6 |########################################
  2138078.0 |
  2138895.4 |
  2139712.8 |########################################
  2140530.2 |
  2141347.6 |
  2142165.0 |
  2142982.4 |
  2143799.8 |
  2144617.2 |
  2145434.6 |
  2146252.0 |
  2147069.4 |
  2147886.8 |
  (0 below, 1 above range)

abi_residency_real_null_entry (n=6, range 2628.3-2829.3 ns)
   2628.3 |########################################
   2638.4 |
   2648.4 |
   2658.5 |
   2668.5 |
   2678.6 |########################################
   2688.6 |
   2698.7 |
   2708.7 |
   2718.8 |
   2728.8 |
   2738.9 |########################################
   2748.9 |
   2759.0 |
   2769.0 |
   2779.1 |
   2789.1 |########################################
   2799.2 |
   2809.2 |########################################
   2819.3 |
  (0 below, 1 above range)

abi_residency_real_reused_buffer (n=6, range 2128898.3-2140605.0 ns)
  2128898.3 |########################################
  2129483.6 |
  2130069.0 |########################################
  2130654.3 |
  2131239.6 |
  2131825.0 |
  2132410.3 |
  2132995.6 |
  2133581.0 |########################################
  2134166.3 |
  2134751.6 |
  2135337.0 |
  2135922.3 |
  2136507.7 |
  2137093.0 |########################################
  2137678.3 |
  2138263.7 |########################################
  2138849.0 |
  2139434.3 |
  2140019.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_real_fresh_alloc**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_residency_real_null_entry**: bridge=4327.8% of algo (FFI overhead may distort results)
- **abi_residency_real_reused_buffer**: bridge=302.0% of algo (FFI overhead may distort results)
