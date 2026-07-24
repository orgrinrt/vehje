# abi_native_cross (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_leaf_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_leaf_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_leaf_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_leaf_native_ffi_w has the worst median (11.85 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_leaf_null_entry at 2.70 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_leaf_null_entry dominates: 320% faster than the next best (abi_native_cross_leaf_inproc_native)

abi_native_cross_leaf_null_entry (2.70 us) leads abi_native_cross_leaf_inproc_native (11.34 us) by 320%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_leaf_null_entry beats baseline by 78% (significant)

abi_native_cross_leaf_null_entry is -9.18 us (78%) faster than baseline abi_native_cross_leaf_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_leaf_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_leaf_native_ffi_w (11.85 us) is 4.4x the fastest (2.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_leaf_null_entry (2.70 us) to slowest abi_native_cross_leaf_native_ffi_w (11.85 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_leaf_null_entry** at 2698.1 ns median (-77.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.39x (fastest 2698.1 ns, slowest 11846.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13861ns | 13641ns | 13557ns | 13615ns | 14381ns | -1.63% |
| abi_native_cross_leaf_native_ffi_w | 14091ns | 14153ns | 13678ns | 14137ns | 14228ns | base |
| abi_native_cross_leaf_null_entry | 4952ns | 5003ns | 4815ns | 4943ns | 5035ns | -64.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 11542ns | 11320ns | 11962ns | -2.16% | 0.011 |
| abi_native_cross_leaf_native_ffi_w | 11796ns | 11435ns | 11913ns | base | 0.011 |
| abi_native_cross_leaf_null_entry | 2698ns | 2618ns | 2764ns | -77.13% | 0.047 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 5364.9 | 11452.1 | 11541.9 | n/a |
| abi_native_cross_leaf_native_ffi_w | 26268.8 | 11907.2 | 11796.4 | n/a |
| abi_native_cross_leaf_null_entry | 26965.4 | 2733.3 | 2697.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_native_cross_leaf_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_leaf_inproc_native | 0.011 | 23.1% |
| abi_native_cross_leaf_native_ffi_w | 0.011 | 22.1% |
| abi_native_cross_leaf_null_entry | 0.047 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13861ns | 13861ns | -1.63% |
| abi_native_cross_leaf_native_ffi_w | 14091ns | 14091ns | base |
| abi_native_cross_leaf_null_entry | 4952ns | 4952ns | -64.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_leaf_native_ffi_w | 11846ns | base | --- | [11630, 11913] | --- | --- | --- | --- |
| abi_native_cross_leaf_inproc_native | 11344ns | no significant difference | [-559, +102]ns | [11320, 11962] | no | 0.6875 | 0.6875 | 0 |
| abi_native_cross_leaf_null_entry | 2698ns | -9184.1ns (-77.5%) | [-9245, -8866]ns | [2631, 2764] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_leaf_native_ffi_w | abi_native_cross_leaf_inproc_native | abi_native_cross_leaf_null_entry |
|---|---|---|---|
| 1 | 11893ns | +1.4% | -78.0% |
| 2 | 11860ns | -4.2% | -77.4% |
| 3 | 11435ns | -1.0% | -75.7% |
| 4 | 11826ns | +0.3% | -76.7% |
| 5 | 11833ns | -4.3% | -77.7% |
| 6 | 11932ns | -5.1% | -77.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_leaf_inproc_native | -0.271 | moderate- |
| abi_native_cross_leaf_native_ffi_w | -0.130 | ok |
| abi_native_cross_leaf_null_entry | 0.020 | ok |

**Consistency summary:**

- **abi_native_cross_leaf_inproc_native**: won 4/6, lost 2/6
- **abi_native_cross_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 121423.8ns | 11541.9ns | 1052.0% | HIGH |
| abi_native_cross_leaf_native_ffi_w | 145446.7ns | 11796.4ns | 1233.0% | HIGH |
| abi_native_cross_leaf_null_entry | 119009.4ns | 2697.8ns | 4411.3% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_leaf_inproc_native (n=6, range 11319.6-11961.6 ns)
  11319.6 |########################################
  11351.7 |#############
  11383.8 |
  11415.9 |
  11448.0 |
  11480.1 |
  11512.2 |
  11544.3 |
  11576.4 |
  11608.5 |
  11640.6 |
  11672.7 |
  11704.8 |
  11736.9 |
  11769.0 |
  11801.1 |
  11833.2 |#############
  11865.3 |
  11897.4 |
  11929.5 |
  (0 below, 1 above range)

abi_native_cross_leaf_native_ffi_w (n=6, range 11434.6-11912.7 ns)
  11434.6 |####################
  11458.5 |
  11482.4 |
  11506.3 |
  11530.2 |
  11554.1 |
  11578.0 |
  11601.9 |
  11625.8 |
  11649.7 |
  11673.7 |
  11697.6 |
  11721.5 |
  11745.4 |
  11769.3 |
  11793.2 |
  11817.1 |########################################
  11841.0 |####################
  11864.9 |
  11888.8 |####################
  (0 below, 1 above range)

abi_native_cross_leaf_null_entry (n=6, range 2618.3-2763.9 ns)
   2618.3 |########################################
   2625.6 |
   2632.9 |
   2640.1 |########################################
   2647.4 |
   2654.7 |
   2662.0 |
   2669.3 |
   2676.6 |########################################
   2683.8 |
   2691.1 |
   2698.4 |
   2705.7 |
   2713.0 |########################################
   2720.3 |
   2727.5 |
   2734.8 |
   2742.1 |
   2749.4 |########################################
   2756.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_leaf_inproc_native**: bridge=1057.7% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_native_ffi_w**: bridge=1230.8% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_null_entry**: bridge=4405.7% of algo (FFI overhead may distort results)
