# abi_native_cross (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_leaf_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_leaf_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_leaf_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_leaf_native_ffi_w has the worst median (13.42 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_leaf_null_entry at 3.02 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_leaf_null_entry dominates: 278% faster than the next best (abi_native_cross_leaf_inproc_native)

abi_native_cross_leaf_null_entry (3.02 us) leads abi_native_cross_leaf_inproc_native (11.40 us) by 278%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_leaf_null_entry beats baseline by 77% (significant)

abi_native_cross_leaf_null_entry is -10.27 us (77%) faster than baseline abi_native_cross_leaf_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_leaf_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_leaf_native_ffi_w (13.42 us) is 4.4x the fastest (3.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_leaf_null_entry (3.02 us) to slowest abi_native_cross_leaf_native_ffi_w (13.42 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_leaf_null_entry** at 3019.8 ns median (-77.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.44x (fastest 3019.8 ns, slowest 13420.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13697ns | 13727ns | 13368ns | 13621ns | 13976ns | -12.40% |
| abi_native_cross_leaf_native_ffi_w | 15635ns | 15698ns | 15437ns | 15632ns | 15740ns | base |
| abi_native_cross_leaf_null_entry | 5296ns | 5246ns | 5144ns | 5228ns | 5473ns | -66.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 11421ns | 11141ns | 11699ns | -14.52% | 0.001 |
| abi_native_cross_leaf_native_ffi_w | 13361ns | 13174ns | 13474ns | base | 0.001 |
| abi_native_cross_leaf_null_entry | 3050ns | 2980ns | 3148ns | -77.18% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 5347.9 | 11461.3 | 11420.9 | n/a |
| abi_native_cross_leaf_native_ffi_w | 25446.6 | 13834.5 | 13361.2 | n/a |
| abi_native_cross_leaf_null_entry | 27234.6 | 3094.3 | 3049.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_native_cross_leaf_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_leaf_inproc_native | 0.001 | 26.1% |
| abi_native_cross_leaf_native_ffi_w | 0.001 | 22.2% |
| abi_native_cross_leaf_null_entry | 0.003 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13697ns | 13697ns | -12.40% |
| abi_native_cross_leaf_native_ffi_w | 15635ns | 15635ns | base |
| abi_native_cross_leaf_null_entry | 5296ns | 5296ns | -66.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_leaf_native_ffi_w | 13420ns | base | --- | [13189, 13474] | --- | --- | --- | --- |
| abi_native_cross_leaf_inproc_native | 11404ns | -1939.4ns (-14.5%) | [-2245, -1637]ns | [11160, 11699] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_leaf_null_entry | 3020ns | -10271.9ns (-76.5%) | [-10481, -10182]ns | [2981, 3148] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_leaf_native_ffi_w | abi_native_cross_leaf_inproc_native | abi_native_cross_leaf_null_entry |
|---|---|---|---|
| 1 | 13205ns | -11.1% | -77.4% |
| 2 | 13482ns | -17.1% | -77.7% |
| 3 | 13431ns | -16.3% | -76.8% |
| 4 | 13409ns | -13.8% | -76.3% |
| 5 | 13174ns | -15.4% | -77.0% |
| 6 | 13466ns | -13.4% | -77.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_leaf_inproc_native | -0.481 | moderate- |
| abi_native_cross_leaf_native_ffi_w | -0.387 | moderate- |
| abi_native_cross_leaf_null_entry | 0.234 | moderate+ |

**Consistency summary:**

- **abi_native_cross_leaf_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 120840.7ns | 11420.9ns | 1058.1% | HIGH |
| abi_native_cross_leaf_native_ffi_w | 144217.0ns | 13361.2ns | 1079.4% | HIGH |
| abi_native_cross_leaf_null_entry | 119663.0ns | 3049.6ns | 3923.9% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_leaf_inproc_native (n=6, range 11141.2-11699.0 ns)
  11141.2 |########################################
  11169.1 |########################################
  11197.0 |
  11224.9 |########################################
  11252.8 |
  11280.6 |
  11308.5 |
  11336.4 |
  11364.3 |
  11392.2 |
  11420.1 |
  11448.0 |
  11475.9 |
  11503.7 |
  11531.6 |
  11559.5 |########################################
  11587.4 |
  11615.3 |
  11643.2 |########################################
  11671.1 |
  (0 below, 1 above range)

abi_native_cross_leaf_native_ffi_w (n=6, range 13173.8-13474.0 ns)
  13173.8 |########################################
  13188.8 |
  13203.8 |########################################
  13218.8 |
  13233.8 |
  13248.8 |
  13263.8 |
  13278.9 |
  13293.9 |
  13308.9 |
  13323.9 |
  13338.9 |
  13353.9 |
  13368.9 |
  13383.9 |
  13398.9 |########################################
  13413.9 |
  13428.9 |########################################
  13443.9 |
  13458.9 |########################################
  (0 below, 1 above range)

abi_native_cross_leaf_null_entry (n=6, range 2980.4-3148.3 ns)
   2980.4 |########################################
   2988.8 |
   2997.2 |####################
   3005.6 |
   3014.0 |
   3022.4 |
   3030.8 |####################
   3039.2 |
   3047.6 |
   3056.0 |
   3064.4 |
   3072.8 |
   3081.2 |
   3089.6 |
   3098.0 |
   3106.4 |####################
   3114.8 |
   3123.2 |
   3131.6 |
   3140.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_leaf_inproc_native**: bridge=1059.5% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_native_ffi_w**: bridge=1084.9% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_null_entry**: bridge=3961.6% of algo (FFI overhead may distort results)
