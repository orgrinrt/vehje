# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (112.63 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.61 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_null_entry dominates: 1165% faster than the next best (abi_payload_cost_real_soa_payload)

abi_payload_cost_real_null_entry (2.61 us) leads abi_payload_cost_real_soa_payload (33.03 us) by 1165%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_real_null_entry beats baseline by 98% (significant)

abi_payload_cost_real_null_entry is -110.07 us (98%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 43.1x slower than the field

abi_payload_cost_real_scalar_payload (112.63 us) is 43.1x the fastest (2.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 43.1x the fastest

Fastest abi_payload_cost_real_null_entry (2.61 us) to slowest abi_payload_cost_real_scalar_payload (112.63 us): 43.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2610.2 ns median (-97.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 43.15x (fastest 2610.2 ns, slowest 112628.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4920ns | 4945ns | 4727ns | 4928ns | 5006ns | -95.73% |
| abi_payload_cost_real_scalar_payload | 115233ns | 114837ns | 114211ns | 114662ns | 116600ns | base |
| abi_payload_cost_real_soa_payload | 35334ns | 35260ns | 34812ns | 35222ns | 35764ns | -69.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2591ns | 2476ns | 2636ns | -97.71% | 0.006 |
| abi_payload_cost_real_scalar_payload | 113011ns | 112010ns | 114337ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 33064ns | 32548ns | 33414ns | -70.74% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 21291.6 | 2803.1 | 2590.5 | 0 |
| abi_payload_cost_real_scalar_payload | 20666.5 | 113020.2 | 113010.7 | n/a |
| abi_payload_cost_real_soa_payload | 21023.3 | 33140.5 | 33064.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.006 | 94.9% |
| abi_payload_cost_real_scalar_payload | 0.000 | 2.2% |
| abi_payload_cost_real_soa_payload | 0.000 | 7.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4920ns | 4920ns | -95.73% |
| abi_payload_cost_real_scalar_payload | 115233ns | 115233ns | base |
| abi_payload_cost_real_soa_payload | 35334ns | 35334ns | -69.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 112629ns | base | --- | [112066, 114337] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2610ns | -110068.1ns (-97.7%) | [-111727, -109465]ns | [2525, 2636] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 33027ns | -79682.1ns (-70.7%) | [-81072, -79085]ns | [32752, 33414] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 112418ns | -97.8% | -70.4% |
| 2 | 113455ns | -97.7% | -70.9% |
| 3 | 112840ns | -97.7% | -70.7% |
| 4 | 112010ns | -97.7% | -70.6% |
| 5 | 112122ns | -97.7% | -71.0% |
| 6 | 115219ns | -97.7% | -70.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.007 | ok |
| abi_payload_cost_real_scalar_payload | -0.171 | ok |
| abi_payload_cost_real_soa_payload | -0.351 | moderate- |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 107681.2ns | 2590.5ns | 4156.7% | HIGH |
| abi_payload_cost_real_scalar_payload | 360554.9ns | 113010.7ns | 319.0% | HIGH |
| abi_payload_cost_real_soa_payload | 186861.6ns | 33064.2ns | 565.1% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2476.2-2636.4 ns)
   2476.2 |########################################
   2484.2 |
   2492.2 |
   2500.2 |
   2508.2 |
   2516.3 |
   2524.3 |
   2532.3 |
   2540.3 |
   2548.3 |
   2556.3 |
   2564.3 |
   2572.3 |########################################
   2580.4 |
   2588.4 |
   2596.4 |
   2604.4 |########################################
   2612.4 |########################################
   2620.4 |########################################
   2628.4 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 112010.0-114337.1 ns)
  112010.0 |########################################
  112126.4 |
  112242.7 |
  112359.1 |####################
  112475.4 |
  112591.8 |
  112708.1 |
  112824.5 |####################
  112940.8 |
  113057.2 |
  113173.6 |
  113289.9 |
  113406.3 |####################
  113522.6 |
  113639.0 |
  113755.3 |
  113871.7 |
  113988.0 |
  114104.4 |
  114220.7 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 32548.3-33414.2 ns)
  32548.3 |########################################
  32591.6 |
  32634.9 |
  32678.2 |
  32721.5 |
  32764.8 |
  32808.1 |
  32851.3 |
  32894.6 |
  32937.9 |########################################
  32981.2 |########################################
  33024.5 |########################################
  33067.8 |
  33111.1 |
  33154.4 |
  33197.7 |
  33241.0 |
  33284.3 |########################################
  33327.6 |
  33370.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=4128.6% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=318.4% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=566.8% of algo (FFI overhead may distort results)
