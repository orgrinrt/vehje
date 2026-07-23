# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (113.90 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_null_entry dominates: 1172% faster than the next best (abi_payload_cost_real_soa_payload)

abi_payload_cost_real_null_entry (2.58 us) leads abi_payload_cost_real_soa_payload (32.88 us) by 1172%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_real_null_entry beats baseline by 98% (significant)

abi_payload_cost_real_null_entry is -111.30 us (98%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 44.1x slower than the field

abi_payload_cost_real_scalar_payload (113.90 us) is 44.1x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_real_scalar_payload shows alternating (throttle bounce) (autocorr -0.72)

abi_payload_cost_real_scalar_payload's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 44.1x the fastest

Fastest abi_payload_cost_real_null_entry (2.58 us) to slowest abi_payload_cost_real_scalar_payload (113.90 us): 44.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2583.8 ns median (-97.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 44.08x (fastest 2583.8 ns, slowest 113899.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4939ns | 4966ns | 4690ns | 4906ns | 5113ns | -95.75% |
| abi_payload_cost_real_scalar_payload | 116213ns | 116158ns | 115262ns | 116099ns | 116859ns | base |
| abi_payload_cost_real_soa_payload | 35235ns | 35180ns | 34475ns | 35047ns | 35895ns | -69.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2571ns | 2458ns | 2646ns | -97.74% | 0.006 |
| abi_payload_cost_real_scalar_payload | 113925ns | 112996ns | 114498ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 32966ns | 32285ns | 33592ns | -71.06% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 21385.1 | 2781.8 | 2571.5 | n/a |
| abi_payload_cost_real_scalar_payload | 22888.6 | 113834.9 | 113924.6 | n/a |
| abi_payload_cost_real_soa_payload | 21461.7 | 33062.3 | 32966.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.006 | 95.1% |
| abi_payload_cost_real_scalar_payload | 0.000 | 2.2% |
| abi_payload_cost_real_soa_payload | 0.000 | 7.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4939ns | 4939ns | -95.75% |
| abi_payload_cost_real_scalar_payload | 116213ns | 116213ns | base |
| abi_payload_cost_real_soa_payload | 35235ns | 35235ns | -69.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 113899ns | base | --- | [113377, 114498] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2584ns | -111304.2ns (-97.7%) | [-112013, -110743]ns | [2485, 2646] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 32878ns | -81471.0ns (-71.5%) | [-81620, -79785]ns | [32428, 33592] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 114022ns | -97.8% | -71.4% |
| 2 | 113776ns | -97.6% | -71.6% |
| 3 | 113758ns | -97.7% | -70.3% |
| 4 | 114687ns | -97.8% | -71.2% |
| 5 | 112996ns | -97.7% | -70.4% |
| 6 | 114309ns | -97.8% | -71.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | -0.214 | moderate- |
| abi_payload_cost_real_scalar_payload | -0.716 | HIGH- (thermal bounce) |
| abi_payload_cost_real_soa_payload | -0.217 | moderate- |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 107439.2ns | 2571.5ns | 4178.2% | HIGH |
| abi_payload_cost_real_scalar_payload | 365115.9ns | 113924.6ns | 320.5% | HIGH |
| abi_payload_cost_real_soa_payload | 187015.3ns | 32966.1ns | 567.3% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2458.3-2645.6 ns)
   2458.3 |####################
   2467.7 |
   2477.0 |
   2486.4 |
   2495.8 |
   2505.1 |####################
   2514.5 |
   2523.9 |
   2533.2 |
   2542.6 |
   2551.9 |####################
   2561.3 |
   2570.7 |
   2580.0 |
   2589.4 |
   2598.8 |
   2608.1 |########################################
   2617.5 |
   2626.9 |
   2636.2 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 112995.8-114497.7 ns)
  112995.8 |####################
  113070.9 |
  113146.0 |
  113221.1 |
  113296.2 |
  113371.3 |
  113446.4 |
  113521.5 |
  113596.6 |
  113671.7 |
  113746.8 |########################################
  113821.8 |
  113896.9 |
  113972.0 |####################
  114047.1 |
  114122.2 |
  114197.3 |
  114272.4 |####################
  114347.5 |
  114422.6 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 32285.0-33591.9 ns)
  32285.0 |########################################
  32350.3 |
  32415.7 |
  32481.0 |
  32546.4 |########################################
  32611.7 |
  32677.1 |
  32742.4 |########################################
  32807.8 |
  32873.1 |
  32938.4 |
  33003.8 |########################################
  33069.1 |
  33134.5 |
  33199.8 |
  33265.2 |
  33330.5 |
  33395.9 |########################################
  33461.2 |
  33526.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=4171.6% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=319.7% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=567.0% of algo (FFI overhead may distort results)
