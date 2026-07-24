# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (8.93 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_null_entry dominates: 141173% faster than the next best (abi_payload_cost_real_soa_payload)

abi_payload_cost_real_null_entry (2.49 us) leads abi_payload_cost_real_soa_payload (3.52 ms) by 141173%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_real_null_entry beats baseline by 100% (significant)

abi_payload_cost_real_null_entry is -8.93 ms (100%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 3582.1x slower than the field

abi_payload_cost_real_scalar_payload (8.93 ms) is 3582.1x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_real_null_entry shows alternating (throttle bounce) (autocorr -0.65)

abi_payload_cost_real_null_entry's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3582.1x the fastest

Fastest abi_payload_cost_real_null_entry (2.49 us) to slowest abi_payload_cost_real_scalar_payload (8.93 ms): 3582.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2494.3 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3582.06x (fastest 2494.3 ns, slowest 8934906.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4766ns | 4829ns | 4501ns | 4755ns | 4917ns | -99.95% |
| abi_payload_cost_real_scalar_payload | 8963947ns | 8938050ns | 8901657ns | 8933207ns | 9041201ns | base |
| abi_payload_cost_real_soa_payload | 3545911ns | 3526561ns | 3516222ns | 3523463ns | 3594428ns | -60.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2482ns | 2337ns | 2572ns | -99.97% | 0.412 |
| abi_payload_cost_real_scalar_payload | 8960744ns | 8898708ns | 9037820ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 3543170ns | 3513614ns | 3591621ns | -60.46% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 50058.3 | 2727.6 | 2482.5 | n/a |
| abi_payload_cost_real_scalar_payload | 100077.4 | 9184883.6 | 8960743.8 | n/a |
| abi_payload_cost_real_soa_payload | 75321.9 | 3544171.1 | 3543169.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.438 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.411 | 93.7% |
| abi_payload_cost_real_scalar_payload | 0.000 | 0.0% |
| abi_payload_cost_real_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4766ns | 4766ns | -99.95% |
| abi_payload_cost_real_scalar_payload | 8963947ns | 8963947ns | base |
| abi_payload_cost_real_soa_payload | 3545911ns | 3545911ns | -60.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 8934906ns | base | --- | [8909505, 9037820] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2494ns | -8932489.6ns (-100.0%) | [-9035344, -8906951]ns | [2381, 2572] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 3523840ns | -5405141.7ns (-60.5%) | [-5520461, -5327120]ns | [3514047, 3591621] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 8931145ns | -100.0% | -60.5% |
| 2 | 8920302ns | -100.0% | -59.1% |
| 3 | 9106724ns | -100.0% | -61.3% |
| 4 | 8968916ns | -100.0% | -60.8% |
| 5 | 8898708ns | -100.0% | -60.5% |
| 6 | 8938667ns | -100.0% | -60.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | -0.653 | HIGH- (thermal bounce) |
| abi_payload_cost_real_scalar_payload | -0.094 | ok |
| abi_payload_cost_real_soa_payload | -0.119 | ok |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 134934.7ns | 2482.5ns | 5435.5% | HIGH |
| abi_payload_cost_real_scalar_payload | 27210892.3ns | 8960743.8ns | 303.7% | HIGH |
| abi_payload_cost_real_soa_payload | 10711619.0ns | 3543169.6ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2337.1-2572.2 ns)
   2337.1 |####################
   2348.9 |
   2360.6 |
   2372.4 |
   2384.1 |
   2395.9 |
   2407.6 |
   2419.4 |####################
   2431.2 |
   2442.9 |
   2454.7 |
   2466.4 |
   2478.2 |
   2489.9 |########################################
   2501.7 |
   2513.5 |
   2525.2 |####################
   2537.0 |
   2548.7 |
   2560.5 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 8898707.5-9037820.0 ns)
  8898707.5 |########################################
  8905663.1 |
  8912618.8 |
  8919574.4 |########################################
  8926530.0 |########################################
  8933485.6 |########################################
  8940441.2 |
  8947396.9 |
  8954352.5 |
  8961308.1 |
  8968263.8 |########################################
  8975219.4 |
  8982175.0 |
  8989130.6 |
  8996086.2 |
  9003041.9 |
  9009997.5 |
  9016953.1 |
  9023908.8 |
  9030864.4 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 3513614.2-3591621.5 ns)
  3513614.2 |########################################
  3517514.6 |####################
  3521414.9 |
  3525315.3 |####################
  3529215.7 |####################
  3533116.0 |
  3537016.4 |
  3540916.7 |
  3544817.1 |
  3548717.5 |
  3552617.8 |
  3556518.2 |
  3560418.6 |
  3564318.9 |
  3568219.3 |
  3572119.6 |
  3576020.0 |
  3579920.4 |
  3583820.7 |
  3587721.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=5440.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=302.2% of algo (FFI overhead may distort results)
