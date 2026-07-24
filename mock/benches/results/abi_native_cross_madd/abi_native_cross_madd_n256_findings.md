# abi_native_cross (madd)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_madd_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_madd_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_native_cross_madd_null_entry dominates: 266% faster than the next best (abi_native_cross_madd_native_ffi_w)

abi_native_cross_madd_null_entry (3.13 us) leads abi_native_cross_madd_native_ffi_w (11.45 us) by 266%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_madd_null_entry beats baseline by 73% (significant)

abi_native_cross_madd_null_entry is -8.33 us (73%) faster than baseline abi_native_cross_madd_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_madd_inproc_native is an outlier: 3.7x slower than the field

abi_native_cross_madd_inproc_native (11.49 us) is 3.7x the fastest (3.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_native_cross_madd_null_entry (3.13 us) to slowest abi_native_cross_madd_inproc_native (11.49 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_native_cross_madd_inproc_native's edge over baseline is significant but tiny (45 ns, 0.40%)

abi_native_cross_madd_inproc_native differs from baseline abi_native_cross_madd_native_ffi_w by 45 ns (0.40%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_native_cross_madd_null_entry** at 3125.6 ns median (-72.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.68x (fastest 3125.6 ns, slowest 11490.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13904ns | 13819ns | 13337ns | 13766ns | 14395ns | +0.70% |
| abi_native_cross_madd_native_ffi_w | 13807ns | 13712ns | 13350ns | 13608ns | 14335ns | base |
| abi_native_cross_madd_null_entry | 5357ns | 5337ns | 5275ns | 5322ns | 5451ns | -61.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 11561ns | 11108ns | 11956ns | +0.00% | 0.022 |
| abi_native_cross_madd_native_ffi_w | 11561ns | 11173ns | 12044ns | base | 0.022 |
| abi_native_cross_madd_null_entry | 3138ns | 3087ns | 3193ns | -72.86% | 0.082 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 5544.3 | 11655.6 | 11561.0 | 68401 |
| abi_native_cross_madd_native_ffi_w | 24924.7 | 11673.2 | 11560.8 | n/a |
| abi_native_cross_madd_null_entry | 26411.8 | 3147.3 | 3137.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_native_cross_madd_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_madd_inproc_native | 0.022 | 26.9% |
| abi_native_cross_madd_native_ffi_w | 0.022 | 27.0% |
| abi_native_cross_madd_null_entry | 0.082 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13904ns | 13904ns | +0.70% |
| abi_native_cross_madd_native_ffi_w | 13807ns | 13807ns | base |
| abi_native_cross_madd_null_entry | 5357ns | 5357ns | -61.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_madd_native_ffi_w | 11449ns | base | --- | [11190, 12044] | --- | --- | --- | --- |
| abi_native_cross_madd_inproc_native | 11490ns | no significant difference | [-598, +553]ns | [11237, 11956] | no | 1.0000 | 1.0000 | 0 |
| abi_native_cross_madd_null_entry | 3126ns | -8331.2ns (-72.8%) | [-8874, -8064]ns | [3095, 3193] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_madd_native_ffi_w | abi_native_cross_madd_inproc_native | abi_native_cross_madd_null_entry |
|---|---|---|---|
| 1 | 11173ns | +4.6% | -72.4% |
| 2 | 11265ns | -1.4% | -72.5% |
| 3 | 12404ns | -8.4% | -74.0% |
| 4 | 11206ns | +1.5% | -71.8% |
| 5 | 11633ns | +5.1% | -73.1% |
| 6 | 11683ns | -0.7% | -73.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_madd_inproc_native | -0.035 | ok |
| abi_native_cross_madd_native_ffi_w | -0.412 | moderate- |
| abi_native_cross_madd_null_entry | 0.093 | ok |

**Consistency summary:**

- **abi_native_cross_madd_inproc_native**: won 3/6, lost 3/6
- **abi_native_cross_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 121533.0ns | 11561.0ns | 1051.2% | HIGH |
| abi_native_cross_madd_native_ffi_w | 140567.6ns | 11560.8ns | 1215.9% | HIGH |
| abi_native_cross_madd_null_entry | 119710.9ns | 3137.7ns | 3815.2% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_madd_inproc_native (n=6, range 11107.9-11956.5 ns)
  11107.9 |####################
  11150.3 |
  11192.8 |
  11235.2 |
  11277.6 |
  11320.0 |
  11362.5 |########################################
  11404.9 |
  11447.3 |
  11489.7 |
  11532.2 |
  11574.6 |####################
  11617.0 |
  11659.5 |####################
  11701.9 |
  11744.3 |
  11786.7 |
  11829.2 |
  11871.6 |
  11914.0 |
  (0 below, 1 above range)

abi_native_cross_madd_native_ffi_w (n=6, range 11173.3-12043.5 ns)
  11173.3 |########################################
  11216.8 |
  11260.3 |####################
  11303.8 |
  11347.3 |
  11390.9 |
  11434.4 |
  11477.9 |
  11521.4 |
  11564.9 |
  11608.4 |####################
  11651.9 |####################
  11695.4 |
  11739.0 |
  11782.5 |
  11826.0 |
  11869.5 |
  11913.0 |
  11956.5 |
  12000.0 |
  (0 below, 1 above range)

abi_native_cross_madd_null_entry (n=6, range 3086.7-3192.9 ns)
   3086.7 |########################################
   3092.0 |
   3097.3 |########################################
   3102.6 |
   3107.9 |
   3113.2 |########################################
   3118.6 |
   3123.9 |
   3129.2 |########################################
   3134.5 |
   3139.8 |
   3145.1 |
   3150.4 |
   3155.7 |
   3161.0 |########################################
   3166.3 |
   3171.7 |
   3177.0 |
   3182.3 |
   3187.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_madd_inproc_native**: bridge=1063.8% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_native_ffi_w**: bridge=1219.8% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_null_entry**: bridge=3816.9% of algo (FFI overhead may distort results)
