# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (11.55 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 259% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.15 us) leads abi_native_cross_real_inproc_native (11.30 us) by 259%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 73% (significant)

abi_native_cross_real_null_entry is -8.41 us (73%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 3.7x slower than the field

abi_native_cross_real_native_ffi_w (11.55 us) is 3.7x the fastest (3.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_native_cross_real_null_entry (3.15 us) to slowest abi_native_cross_real_native_ffi_w (11.55 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3148.6 ns median (-72.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.67x (fastest 3148.6 ns, slowest 11551.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13574ns | 13543ns | 13390ns | 13515ns | 13753ns | -2.71% |
| abi_native_cross_real_native_ffi_w | 13952ns | 13831ns | 13497ns | 13763ns | 14464ns | base |
| abi_native_cross_real_null_entry | 5420ns | 5404ns | 5283ns | 5386ns | 5538ns | -61.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11319ns | 11154ns | 11481ns | -2.88% | 0.023 |
| abi_native_cross_real_native_ffi_w | 11655ns | 11273ns | 12120ns | base | 0.022 |
| abi_native_cross_real_null_entry | 3151ns | 3103ns | 3196ns | -72.97% | 0.081 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5329.4 | 11348.6 | 11318.7 | n/a |
| abi_native_cross_real_native_ffi_w | 27431.5 | 11771.3 | 11654.9 | n/a |
| abi_native_cross_real_null_entry | 27458.3 | 3210.2 | 3150.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.023 | 27.5% |
| abi_native_cross_real_native_ffi_w | 0.022 | 26.9% |
| abi_native_cross_real_null_entry | 0.081 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13574ns | 13574ns | -2.71% |
| abi_native_cross_real_native_ffi_w | 13952ns | 13952ns | base |
| abi_native_cross_real_null_entry | 5420ns | 5420ns | -61.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 11551ns | base | --- | [11294, 12120] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11303ns | -199.1ns (-1.7%) | [-709, -100]ns | [11172, 11481] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 3149ns | -8411.8ns (-72.8%) | [-9003, -8098]ns | [3108, 3196] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 11501ns | -0.7% | -72.5% |
| 2 | 11601ns | -3.5% | -73.2% |
| 3 | 11315ns | -1.0% | -71.7% |
| 4 | 11273ns | -1.1% | -71.7% |
| 5 | 11688ns | -2.4% | -73.5% |
| 6 | 12551ns | -8.0% | -75.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.223 | moderate+ |
| abi_native_cross_real_native_ffi_w | 0.159 | ok |
| abi_native_cross_real_null_entry | -0.173 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 119770.7ns | 11318.7ns | 1058.2% | HIGH |
| abi_native_cross_real_native_ffi_w | 143837.5ns | 11654.9ns | 1234.1% | HIGH |
| abi_native_cross_real_null_entry | 120482.8ns | 3150.8ns | 3823.9% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11154.2-11481.0 ns)
  11154.2 |####################
  11170.5 |
  11186.9 |########################################
  11203.2 |
  11219.6 |
  11235.9 |
  11252.3 |
  11268.6 |
  11284.9 |
  11301.3 |
  11317.6 |
  11334.0 |
  11350.3 |
  11366.7 |
  11383.0 |
  11399.3 |####################
  11415.7 |####################
  11432.0 |
  11448.4 |
  11464.7 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11272.9-12119.8 ns)
  11272.9 |########################################
  11315.2 |
  11357.6 |
  11399.9 |
  11442.3 |
  11484.6 |####################
  11527.0 |
  11569.3 |####################
  11611.6 |
  11654.0 |####################
  11696.3 |
  11738.7 |
  11781.0 |
  11823.4 |
  11865.7 |
  11908.0 |
  11950.4 |
  11992.7 |
  12035.1 |
  12077.4 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 3102.9-3196.2 ns)
   3102.9 |########################################
   3107.6 |########################################
   3112.2 |
   3116.9 |
   3121.6 |
   3126.2 |########################################
   3130.9 |
   3135.6 |
   3140.2 |
   3144.9 |
   3149.6 |
   3154.2 |
   3158.9 |
   3163.6 |########################################
   3168.2 |
   3172.9 |
   3177.6 |
   3182.2 |
   3186.9 |
   3191.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1058.0% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1253.4% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3822.8% of algo (FFI overhead may distort results)
