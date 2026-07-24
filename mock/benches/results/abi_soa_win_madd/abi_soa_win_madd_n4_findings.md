# abi_soa_win (madd)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_soa_win_madd_null_entry dominates: 67560% faster than the next best (abi_soa_win_madd_scalar_payload)

abi_soa_win_madd_null_entry (4.03 us) leads abi_soa_win_madd_scalar_payload (2.72 ms) by 67560%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_madd_null_entry beats baseline by 100% (significant)

abi_soa_win_madd_null_entry is -2.72 ms (100%) faster than baseline abi_soa_win_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_madd_soa_payload is an outlier: 677.1x slower than the field

abi_soa_win_madd_soa_payload (2.73 ms) is 677.1x the fastest (4.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 677.1x the fastest

Fastest abi_soa_win_madd_null_entry (4.03 us) to slowest abi_soa_win_madd_soa_payload (2.73 ms): 677.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_madd_null_entry** at 4025.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 677.08x (fastest 4025.8 ns, slowest 2725796.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 6300ns | 6304ns | 6084ns | 6237ns | 6503ns | -99.77% |
| abi_soa_win_madd_scalar_payload | 2761021ns | 2726960ns | 2725490ns | 2726757ns | 2830181ns | base |
| abi_soa_win_madd_soa_payload | 2777426ns | 2728982ns | 2725780ns | 2728815ns | 2876165ns | +0.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 4017ns | 3878ns | 4143ns | -99.85% | 0.001 |
| abi_soa_win_madd_scalar_payload | 2757792ns | 2722541ns | 2826569ns | base | 0.000 |
| abi_soa_win_madd_soa_payload | 2774120ns | 2722480ns | 2872591ns | +0.59% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 26937.1 | 4158.3 | 4016.9 | n/a |
| abi_soa_win_madd_scalar_payload | 64073.0 | 2748960.6 | 2757792.0 | n/a |
| abi_soa_win_madd_soa_payload | 68307.7 | 2760408.8 | 2774120.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_madd_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.001 | 96.3% |
| abi_soa_win_madd_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_madd_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_madd_null_entry | 6300ns | 6300ns | -99.77% |
| abi_soa_win_madd_scalar_payload | 2761021ns | 2761021ns | base |
| abi_soa_win_madd_soa_payload | 2777426ns | 2777426ns | +0.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_madd_scalar_payload | 2723855ns | base | --- | [2722952, 2826569] | --- | --- | --- | --- |
| abi_soa_win_madd_null_entry | 4026ns | -2719974.0ns (-99.9%) | [-2822465, -2718886]ns | [3881, 4143] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_madd_soa_payload | 2725797ns | no significant difference | [-2491, +49074]ns | [2723973, 2872591] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_madd_scalar_payload | abi_soa_win_madd_null_entry | abi_soa_win_madd_soa_payload |
|---|---|---|---|
| 1 | 2729565ns | -99.9% | -0.2% |
| 2 | 2723461ns | -99.9% | +0.1% |
| 3 | 2723363ns | -99.9% | -0.0% |
| 4 | 2722541ns | -99.8% | +0.1% |
| 5 | 2923572ns | -99.9% | +3.2% |
| 6 | 2724250ns | -99.9% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_madd_null_entry | -0.114 | ok |
| abi_soa_win_madd_scalar_payload | -0.244 | moderate- |
| abi_soa_win_madd_soa_payload | -0.230 | moderate- |

**Consistency summary:**

- **abi_soa_win_madd_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_madd_soa_payload**: won 1/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 121292.1ns | 4016.9ns | 3019.6% | HIGH |
| abi_soa_win_madd_scalar_payload | 8324054.9ns | 2757792.0ns | 301.8% | HIGH |
| abi_soa_win_madd_soa_payload | 8301251.9ns | 2774120.3ns | 299.2% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_madd_null_entry (n=6, range 3878.3-4143.4 ns)
   3878.3 |########################################
   3891.6 |
   3904.8 |
   3918.1 |
   3931.3 |
   3944.6 |
   3957.8 |####################
   3971.1 |
   3984.3 |
   3997.6 |
   4010.8 |
   4024.1 |
   4037.3 |
   4050.6 |
   4063.8 |
   4077.1 |
   4090.3 |####################
   4103.6 |####################
   4116.8 |
   4130.1 |
  (0 below, 1 above range)

abi_soa_win_madd_scalar_payload (n=6, range 2722540.8-2826568.5 ns)
  2722540.8 |########################################
  2727742.2 |##########
  2732943.6 |
  2738145.0 |
  2743346.3 |
  2748547.7 |
  2753749.1 |
  2758950.5 |
  2764151.9 |
  2769353.3 |
  2774554.7 |
  2779756.1 |
  2784957.4 |
  2790158.8 |
  2795360.2 |
  2800561.6 |
  2805763.0 |
  2810964.4 |
  2816165.8 |
  2821367.2 |
  (0 below, 1 above range)

abi_soa_win_madd_soa_payload (n=6, range 2722479.6-2872590.8 ns)
  2722479.6 |########################################
  2729985.2 |
  2737490.7 |
  2744996.3 |
  2752501.8 |
  2760007.4 |
  2767513.0 |
  2775018.5 |
  2782524.1 |
  2790029.6 |
  2797535.2 |
  2805040.8 |
  2812546.3 |
  2820051.9 |
  2827557.4 |
  2835063.0 |
  2842568.6 |
  2850074.1 |
  2857579.7 |
  2865085.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_madd_null_entry**: bridge=3001.5% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_scalar_payload**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_soa_payload**: bridge=302.2% of algo (FFI overhead may distort results)
