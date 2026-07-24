# abi_soa_win (madd)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_madd_scalar_payload has the worst median (2.73 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_madd_null_entry at 3.10 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_madd_null_entry dominates: 34918% faster than the next best (abi_soa_win_madd_soa_payload)

abi_soa_win_madd_null_entry (3.10 us) leads abi_soa_win_madd_soa_payload (1.09 ms) by 34918%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_madd_null_entry beats baseline by 100% (significant)

abi_soa_win_madd_null_entry is -2.72 ms (100%) faster than baseline abi_soa_win_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_madd_scalar_payload is an outlier: 878.7x slower than the field

abi_soa_win_madd_scalar_payload (2.73 ms) is 878.7x the fastest (3.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 878.7x the fastest

Fastest abi_soa_win_madd_null_entry (3.10 us) to slowest abi_soa_win_madd_scalar_payload (2.73 ms): 878.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_madd_null_entry** at 3102.9 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 878.74x (fastest 3102.9 ns, slowest 2726693.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 5583ns | 5315ns | 5250ns | 5301ns | 6171ns | -99.80% |
| abi_soa_win_madd_scalar_payload | 2739440ns | 2730315ns | 2714519ns | 2727891ns | 2769224ns | base |
| abi_soa_win_madd_soa_payload | 1090660ns | 1089299ns | 1084148ns | 1087833ns | 1098156ns | -60.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 3186ns | 3075ns | 3372ns | -99.88% | 0.080 |
| abi_soa_win_madd_scalar_payload | 2736139ns | 2711887ns | 2765751ns | base | 0.000 |
| abi_soa_win_madd_soa_payload | 1087951ns | 1081618ns | 1095265ns | -60.24% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 28166.2 | 3199.2 | 3185.6 | n/a |
| abi_soa_win_madd_scalar_payload | 61925.8 | 2733177.1 | 2736138.6 | n/a |
| abi_soa_win_madd_soa_payload | 44091.3 | 1088123.3 | 1087951.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_soa_win_madd_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.083 | 99.1% |
| abi_soa_win_madd_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_madd_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_madd_null_entry | 5583ns | 5583ns | -99.80% |
| abi_soa_win_madd_scalar_payload | 2739440ns | 2739440ns | base |
| abi_soa_win_madd_soa_payload | 1090660ns | 1090660ns | -60.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_madd_scalar_payload | 2726694ns | base | --- | [2715972, 2765751] | --- | --- | --- | --- |
| abi_soa_win_madd_null_entry | 3103ns | -2723612.3ns (-99.9%) | [-2762378, -2712869]ns | [3081, 3372] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_madd_soa_payload | 1086585ns | -1640108.8ns (-60.2%) | [-1674160, -1630294]ns | [1082004, 1095265] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_madd_scalar_payload | abi_soa_win_madd_null_entry | abi_soa_win_madd_soa_payload |
|---|---|---|---|
| 1 | 2723482ns | -99.9% | -60.1% |
| 2 | 2711887ns | -99.9% | -60.1% |
| 3 | 2729905ns | -99.9% | -60.2% |
| 4 | 2766883ns | -99.9% | -60.9% |
| 5 | 2764618ns | -99.9% | -60.2% |
| 6 | 2720056ns | -99.9% | -60.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.029 | ok |
| abi_soa_win_madd_scalar_payload | 0.244 | moderate+ |
| abi_soa_win_madd_soa_payload | -0.214 | moderate- |

**Consistency summary:**

- **abi_soa_win_madd_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 122089.1ns | 3185.6ns | 3832.6% | HIGH |
| abi_soa_win_madd_scalar_payload | 8262543.7ns | 2736138.6ns | 302.0% | HIGH |
| abi_soa_win_madd_soa_payload | 3309779.2ns | 1087951.2ns | 304.2% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_madd_null_entry (n=6, range 3075.4-3372.5 ns)
   3075.4 |########################################
   3090.3 |
   3105.1 |#############
   3120.0 |
   3134.8 |
   3149.7 |
   3164.5 |
   3179.4 |
   3194.2 |
   3209.1 |
   3223.9 |
   3238.8 |
   3253.7 |
   3268.5 |#############
   3283.4 |
   3298.2 |
   3313.1 |
   3327.9 |
   3342.8 |
   3357.6 |
  (0 below, 1 above range)

abi_soa_win_madd_scalar_payload (n=6, range 2711887.1-2765750.6 ns)
  2711887.1 |########################################
  2714580.3 |
  2717273.5 |
  2719966.6 |########################################
  2722659.8 |########################################
  2725353.0 |
  2728046.1 |########################################
  2730739.3 |
  2733432.5 |
  2736125.7 |
  2738818.8 |
  2741512.0 |
  2744205.2 |
  2746898.4 |
  2749591.5 |
  2752284.7 |
  2754977.9 |
  2757671.1 |
  2760364.2 |
  2763057.4 |########################################
  (0 below, 1 above range)

abi_soa_win_madd_soa_payload (n=6, range 1081618.3-1095265.0 ns)
  1081618.3 |########################################
  1082300.6 |########################################
  1082983.0 |
  1083665.3 |
  1084347.6 |
  1085030.0 |########################################
  1085712.3 |
  1086394.6 |
  1087077.0 |
  1087759.3 |########################################
  1088441.6 |########################################
  1089124.0 |
  1089806.3 |
  1090488.7 |
  1091171.0 |
  1091853.3 |
  1092535.7 |
  1093218.0 |
  1093900.3 |
  1094582.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_madd_null_entry**: bridge=3840.5% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_scalar_payload**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_soa_payload**: bridge=304.2% of algo (FFI overhead may distort results)
