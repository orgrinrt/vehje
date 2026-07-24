# abi_native_cross (madd)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_madd_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_madd_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_madd_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_madd_native_ffi_w has the worst median (11.81 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_madd_null_entry at 2.78 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_madd_null_entry dominates: 324% faster than the next best (abi_native_cross_madd_inproc_native)

abi_native_cross_madd_null_entry (2.78 us) leads abi_native_cross_madd_inproc_native (11.78 us) by 324%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_madd_null_entry beats baseline by 77% (significant)

abi_native_cross_madd_null_entry is -9.09 us (77%) faster than baseline abi_native_cross_madd_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_madd_native_ffi_w is an outlier: 4.2x slower than the field

abi_native_cross_madd_native_ffi_w (11.81 us) is 4.2x the fastest (2.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.2x the fastest

Fastest abi_native_cross_madd_null_entry (2.78 us) to slowest abi_native_cross_madd_native_ffi_w (11.81 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_native_cross_madd_inproc_native's edge over baseline is significant but tiny (20 ns, 0.17%)

abi_native_cross_madd_inproc_native differs from baseline abi_native_cross_madd_native_ffi_w by 20 ns (0.17%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_native_cross_madd_null_entry** at 2780.4 ns median (-76.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.25x (fastest 2780.4 ns, slowest 11810.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 14120ns | 14144ns | 13501ns | 14022ns | 14577ns | -1.18% |
| abi_native_cross_madd_native_ffi_w | 14289ns | 14108ns | 14040ns | 14086ns | 14719ns | base |
| abi_native_cross_madd_null_entry | 5071ns | 5098ns | 4787ns | 5039ns | 5260ns | -64.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 11805ns | 11347ns | 12167ns | -1.60% | 0.011 |
| abi_native_cross_madd_native_ffi_w | 11997ns | 11740ns | 12419ns | base | 0.011 |
| abi_native_cross_madd_null_entry | 2776ns | 2629ns | 2883ns | -76.86% | 0.046 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 5554.9 | 11856.0 | 11804.9 | n/a |
| abi_native_cross_madd_native_ffi_w | 25414.2 | 12100.6 | 11997.2 | n/a |
| abi_native_cross_madd_null_entry | 26918.2 | 2807.6 | 2775.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_native_cross_madd_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_madd_inproc_native | 0.011 | 22.3% |
| abi_native_cross_madd_native_ffi_w | 0.011 | 22.3% |
| abi_native_cross_madd_null_entry | 0.046 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_madd_inproc_native | 14120ns | 14120ns | -1.18% |
| abi_native_cross_madd_native_ffi_w | 14289ns | 14289ns | base |
| abi_native_cross_madd_null_entry | 5071ns | 5071ns | -64.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_madd_native_ffi_w | 11811ns | base | --- | [11762, 12419] | --- | --- | --- | --- |
| abi_native_cross_madd_inproc_native | 11783ns | no significant difference | [-889, +292]ns | [11465, 12167] | no | 1.0000 | 1.0000 | 0 |
| abi_native_cross_madd_null_entry | 2780ns | -9089.6ns (-77.0%) | [-9566, -9009]ns | [2664, 2883] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_madd_native_ffi_w | abi_native_cross_madd_inproc_native | abi_native_cross_madd_null_entry |
|---|---|---|---|
| 1 | 11789ns | +1.2% | -77.7% |
| 2 | 12875ns | -11.9% | -77.5% |
| 3 | 11832ns | -2.1% | -76.7% |
| 4 | 11785ns | +1.2% | -76.2% |
| 5 | 11962ns | +3.7% | -76.1% |
| 6 | 11740ns | -0.9% | -77.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_madd_inproc_native | -0.015 | ok |
| abi_native_cross_madd_native_ffi_w | -0.289 | moderate- |
| abi_native_cross_madd_null_entry | -0.496 | moderate- |

**Consistency summary:**

- **abi_native_cross_madd_inproc_native**: won 3/6, lost 3/6
- **abi_native_cross_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 122933.3ns | 11804.9ns | 1041.4% | HIGH |
| abi_native_cross_madd_native_ffi_w | 144785.0ns | 11997.2ns | 1206.8% | HIGH |
| abi_native_cross_madd_null_entry | 119470.6ns | 2775.6ns | 4304.3% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_madd_inproc_native (n=6, range 11347.1-12166.9 ns)
  11347.1 |####################
  11388.1 |
  11429.1 |
  11470.1 |
  11511.1 |
  11552.1 |####################
  11593.0 |
  11634.0 |####################
  11675.0 |
  11716.0 |
  11757.0 |
  11798.0 |
  11839.0 |
  11880.0 |
  11921.0 |########################################
  11962.0 |
  12002.9 |
  12043.9 |
  12084.9 |
  12125.9 |
  (0 below, 1 above range)

abi_native_cross_madd_native_ffi_w (n=6, range 11740.0-12418.8 ns)
  11740.0 |####################
  11773.9 |########################################
  11807.9 |####################
  11841.8 |
  11875.8 |
  11909.7 |
  11943.6 |####################
  11977.6 |
  12011.5 |
  12045.4 |
  12079.4 |
  12113.3 |
  12147.2 |
  12181.2 |
  12215.1 |
  12249.1 |
  12283.0 |
  12316.9 |
  12350.9 |
  12384.8 |
  (0 below, 1 above range)

abi_native_cross_madd_null_entry (n=6, range 2629.2-2882.7 ns)
   2629.2 |########################################
   2641.9 |
   2654.5 |
   2667.2 |
   2679.9 |
   2692.6 |########################################
   2705.2 |
   2717.9 |
   2730.6 |
   2743.3 |########################################
   2755.9 |
   2768.6 |
   2781.3 |
   2794.0 |
   2806.6 |########################################
   2819.3 |
   2832.0 |
   2844.7 |
   2857.3 |########################################
   2870.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_madd_inproc_native**: bridge=1046.0% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_native_ffi_w**: bridge=1226.4% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_null_entry**: bridge=4325.6% of algo (FFI overhead may distort results)
