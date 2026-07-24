# abi_native_cross (madd)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_madd_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_madd_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_madd_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_madd_native_ffi_w has the worst median (13.98 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_madd_null_entry at 3.92 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_madd_null_entry dominates: 198% faster than the next best (abi_native_cross_madd_inproc_native)

abi_native_cross_madd_null_entry (3.92 us) leads abi_native_cross_madd_inproc_native (11.68 us) by 198%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_madd_null_entry beats baseline by 72% (significant)

abi_native_cross_madd_null_entry is -10.06 us (72%) faster than baseline abi_native_cross_madd_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_madd_native_ffi_w is an outlier: 3.6x slower than the field

abi_native_cross_madd_native_ffi_w (13.98 us) is 3.6x the fastest (3.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.6x the fastest

Fastest abi_native_cross_madd_null_entry (3.92 us) to slowest abi_native_cross_madd_native_ffi_w (13.98 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_madd_null_entry** at 3918.1 ns median (-72.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.57x (fastest 3918.1 ns, slowest 13978.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13914ns | 14000ns | 13372ns | 13835ns | 14304ns | -15.34% |
| abi_native_cross_madd_native_ffi_w | 16434ns | 16264ns | 16182ns | 16255ns | 16830ns | base |
| abi_native_cross_madd_null_entry | 6136ns | 6121ns | 6051ns | 6109ns | 6218ns | -62.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 11602ns | 11151ns | 11939ns | -18.04% | 0.000 |
| abi_native_cross_madd_native_ffi_w | 14156ns | 13935ns | 14541ns | base | 0.000 |
| abi_native_cross_madd_null_entry | 3927ns | 3879ns | 3983ns | -72.26% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 5331.7 | 11695.6 | 11602.4 | n/a |
| abi_native_cross_madd_native_ffi_w | 25539.8 | 14218.9 | 14155.8 | n/a |
| abi_native_cross_madd_null_entry | 26409.7 | 4079.3 | 3926.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_madd_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_madd_inproc_native | 0.000 | 33.2% |
| abi_native_cross_madd_native_ffi_w | 0.000 | 27.8% |
| abi_native_cross_madd_null_entry | 0.001 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13914ns | 13914ns | -15.34% |
| abi_native_cross_madd_native_ffi_w | 16434ns | 16434ns | base |
| abi_native_cross_madd_null_entry | 6136ns | 6136ns | -62.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_madd_native_ffi_w | 13978ns | base | --- | [13949, 14541] | --- | --- | --- | --- |
| abi_native_cross_madd_inproc_native | 11683ns | -2610.0ns (-18.7%) | [-2793, -2258]ns | [11185, 11939] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_madd_null_entry | 3918ns | -10062.5ns (-72.0%) | [-10641, -9983]ns | [3879, 3983] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_madd_native_ffi_w | abi_native_cross_madd_inproc_native | abi_native_cross_madd_null_entry |
|---|---|---|---|
| 1 | 14252ns | -17.7% | -72.8% |
| 2 | 14829ns | -18.2% | -73.6% |
| 3 | 13969ns | -20.2% | -71.0% |
| 4 | 13962ns | -15.8% | -71.9% |
| 5 | 13988ns | -19.8% | -72.0% |
| 6 | 13935ns | -16.5% | -72.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_madd_inproc_native | -0.454 | moderate- |
| abi_native_cross_madd_native_ffi_w | 0.074 | ok |
| abi_native_cross_madd_null_entry | -0.055 | ok |

**Consistency summary:**

- **abi_native_cross_madd_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 121761.2ns | 11602.4ns | 1049.5% | HIGH |
| abi_native_cross_madd_native_ffi_w | 148442.4ns | 14155.8ns | 1048.6% | HIGH |
| abi_native_cross_madd_null_entry | 120486.7ns | 3926.9ns | 3068.3% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_madd_inproc_native (n=6, range 11151.2-11939.0 ns)
  11151.2 |########################################
  11190.6 |########################################
  11230.0 |
  11269.4 |
  11308.8 |
  11348.1 |
  11387.5 |
  11426.9 |
  11466.3 |
  11505.7 |
  11545.1 |
  11584.5 |
  11623.9 |########################################
  11663.2 |
  11702.6 |########################################
  11742.0 |########################################
  11781.4 |
  11820.8 |
  11860.2 |
  11899.6 |
  (0 below, 1 above range)

abi_native_cross_madd_native_ffi_w (n=6, range 13934.6-14540.6 ns)
  13934.6 |########################################
  13964.9 |########################################
  13995.2 |
  14025.5 |
  14055.8 |
  14086.1 |
  14116.4 |
  14146.7 |
  14177.0 |
  14207.3 |
  14237.6 |####################
  14267.9 |
  14298.2 |
  14328.5 |
  14358.8 |
  14389.1 |
  14419.4 |
  14449.7 |
  14480.0 |
  14510.3 |
  (0 below, 1 above range)

abi_native_cross_madd_null_entry (n=6, range 3879.2-3983.3 ns)
   3879.2 |##########################
   3884.4 |
   3889.6 |
   3894.8 |
   3900.0 |
   3905.2 |
   3910.4 |
   3915.7 |########################################
   3920.9 |
   3926.1 |
   3931.3 |
   3936.5 |
   3941.7 |
   3946.9 |
   3952.1 |
   3957.3 |
   3962.5 |
   3967.7 |
   3972.9 |
   3978.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_madd_inproc_native**: bridge=1051.7% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_native_ffi_w**: bridge=1067.0% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_null_entry**: bridge=3082.6% of algo (FFI overhead may distort results)
