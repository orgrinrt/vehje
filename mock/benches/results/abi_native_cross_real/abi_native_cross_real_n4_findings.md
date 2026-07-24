# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (13.99 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.90 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 187% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.90 us) leads abi_native_cross_real_inproc_native (11.20 us) by 187%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 72% (significant)

abi_native_cross_real_null_entry is -10.05 us (72%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 3.6x slower than the field

abi_native_cross_real_native_ffi_w (13.99 us) is 3.6x the fastest (3.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.6x the fastest

Fastest abi_native_cross_real_null_entry (3.90 us) to slowest abi_native_cross_real_native_ffi_w (13.99 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3897.9 ns median (-72.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.59x (fastest 3897.9 ns, slowest 13985.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13569ns | 13413ns | 13320ns | 13396ns | 13954ns | -17.19% |
| abi_native_cross_real_native_ffi_w | 16385ns | 16282ns | 16208ns | 16270ns | 16646ns | base |
| abi_native_cross_real_null_entry | 6169ns | 6110ns | 6047ns | 6095ns | 6342ns | -62.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11342ns | 11148ns | 11672ns | -19.46% | 0.000 |
| abi_native_cross_real_native_ffi_w | 14083ns | 13903ns | 14331ns | base | 0.000 |
| abi_native_cross_real_null_entry | 3944ns | 3858ns | 4064ns | -71.99% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5331.7 | 11382.1 | 11342.1 | n/a |
| abi_native_cross_real_native_ffi_w | 25400.1 | 14140.5 | 14082.6 | n/a |
| abi_native_cross_real_null_entry | 27206.8 | 4074.5 | 3943.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.000 | 34.4% |
| abi_native_cross_real_native_ffi_w | 0.000 | 27.6% |
| abi_native_cross_real_null_entry | 0.001 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13569ns | 13569ns | -17.19% |
| abi_native_cross_real_native_ffi_w | 16385ns | 16385ns | base |
| abi_native_cross_real_null_entry | 6169ns | 6169ns | -62.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 13985ns | base | --- | [13932, 14331] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11201ns | -2803.4ns (-20.0%) | [-3159, -2260]ns | [11153, 11672] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 3898ns | -10055.0ns (-71.9%) | [-10393, -9968]ns | [3870, 4064] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 13903ns | -15.9% | -72.0% |
| 2 | 13982ns | -19.8% | -72.4% |
| 3 | 14119ns | -20.8% | -71.0% |
| 4 | 13961ns | -16.6% | -71.1% |
| 5 | 14542ns | -23.3% | -73.3% |
| 6 | 13988ns | -20.3% | -72.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.283 | moderate- |
| abi_native_cross_real_native_ffi_w | -0.321 | moderate- |
| abi_native_cross_real_null_entry | 0.048 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 120210.2ns | 11342.1ns | 1059.9% | HIGH |
| abi_native_cross_real_native_ffi_w | 147240.6ns | 14082.6ns | 1045.5% | HIGH |
| abi_native_cross_real_null_entry | 121633.0ns | 3943.9ns | 3084.1% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11147.9-11672.1 ns)
  11147.9 |########################################
  11174.1 |####################
  11200.3 |####################
  11226.5 |
  11252.7 |
  11279.0 |
  11305.2 |
  11331.4 |
  11357.6 |
  11383.8 |
  11410.0 |
  11436.2 |
  11462.4 |
  11488.6 |
  11514.8 |
  11541.0 |
  11567.3 |
  11593.5 |
  11619.7 |
  11645.9 |####################
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 13902.9-14330.9 ns)
  13902.9 |####################
  13924.3 |
  13945.7 |####################
  13967.1 |########################################
  13988.5 |
  14009.9 |
  14031.3 |
  14052.7 |
  14074.1 |
  14095.5 |
  14116.9 |####################
  14138.3 |
  14159.7 |
  14181.1 |
  14202.5 |
  14223.9 |
  14245.3 |
  14266.7 |
  14288.1 |
  14309.5 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 3857.9-4064.2 ns)
   3857.9 |########################################
   3868.2 |
   3878.5 |########################################
   3888.8 |########################################
   3899.2 |########################################
   3909.5 |
   3919.8 |
   3930.1 |
   3940.4 |
   3950.7 |
   3961.1 |
   3971.4 |
   3981.7 |
   3992.0 |
   4002.3 |
   4012.6 |
   4022.9 |
   4033.3 |########################################
   4043.6 |
   4053.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1059.3% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1044.3% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3105.4% of algo (FFI overhead may distort results)
