# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (481.87 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_null_entry dominates: 8151% faster than the next best (abi_payload_cost_real_soa_payload)

abi_payload_cost_real_null_entry (2.53 us) leads abi_payload_cost_real_soa_payload (208.69 us) by 8151%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_real_null_entry beats baseline by 99% (significant)

abi_payload_cost_real_null_entry is -479.30 us (99%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 190.5x slower than the field

abi_payload_cost_real_scalar_payload (481.87 us) is 190.5x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 190.5x the fastest

Fastest abi_payload_cost_real_null_entry (2.53 us) to slowest abi_payload_cost_real_scalar_payload (481.87 us): 190.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2529.1 ns median (-99.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 190.53x (fastest 2529.1 ns, slowest 481873.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4798ns | 4812ns | 4657ns | 4782ns | 4892ns | -99.01% |
| abi_payload_cost_real_scalar_payload | 486999ns | 484277ns | 481999ns | 483944ns | 494082ns | base |
| abi_payload_cost_real_soa_payload | 211063ns | 211124ns | 209550ns | 210833ns | 212164ns | -56.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2530ns | 2478ns | 2575ns | -99.48% | 0.025 |
| abi_payload_cost_real_scalar_payload | 484481ns | 479635ns | 491323ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 208654ns | 207324ns | 209611ns | -56.93% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 22370.1 | 2711.5 | 2530.3 | n/a |
| abi_payload_cost_real_scalar_payload | 29148.7 | 484319.9 | 484480.7 | n/a |
| abi_payload_cost_real_soa_payload | 24745.9 | 209020.6 | 208654.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.025 | 98.0% |
| abi_payload_cost_real_scalar_payload | 0.000 | 0.5% |
| abi_payload_cost_real_soa_payload | 0.000 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4798ns | 4798ns | -99.01% |
| abi_payload_cost_real_scalar_payload | 486999ns | 486999ns | base |
| abi_payload_cost_real_soa_payload | 211063ns | 211063ns | -56.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 481873ns | base | --- | [480246, 491323] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2529ns | -479298.1ns (-99.5%) | [-488836, -477716]ns | [2487, 2575] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 208689ns | -273910.6ns (-56.8%) | [-281763, -271806]ns | [207662, 209611] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 494041ns | -99.5% | -57.4% |
| 2 | 479635ns | -99.5% | -56.5% |
| 3 | 481854ns | -99.5% | -57.0% |
| 4 | 481892ns | -99.5% | -56.7% |
| 5 | 480856ns | -99.5% | -56.7% |
| 6 | 488605ns | -99.5% | -57.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.236 | moderate+ |
| abi_payload_cost_real_scalar_payload | -0.204 | moderate- |
| abi_payload_cost_real_soa_payload | 0.021 | ok |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 109156.0ns | 2530.3ns | 4313.9% | HIGH |
| abi_payload_cost_real_scalar_payload | 1483335.9ns | 484480.7ns | 306.2% | HIGH |
| abi_payload_cost_real_soa_payload | 651921.0ns | 208654.1ns | 312.4% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2477.9-2575.0 ns)
   2477.9 |########################################
   2482.8 |
   2487.6 |
   2492.5 |########################################
   2497.3 |
   2502.2 |
   2507.0 |
   2511.9 |
   2516.7 |
   2521.6 |########################################
   2526.4 |
   2531.3 |
   2536.2 |########################################
   2541.0 |
   2545.9 |
   2550.7 |
   2555.6 |
   2560.4 |
   2565.3 |########################################
   2570.1 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 479635.4-491323.3 ns)
  479635.4 |####################
  480219.8 |
  480804.2 |####################
  481388.6 |########################################
  481973.0 |
  482557.4 |
  483141.8 |
  483726.2 |
  484310.6 |
  484895.0 |
  485479.4 |
  486063.7 |
  486648.1 |
  487232.5 |
  487816.9 |
  488401.3 |####################
  488985.7 |
  489570.1 |
  490154.5 |
  490738.9 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 207323.8-209611.2 ns)
  207323.8 |########################################
  207438.2 |
  207552.5 |
  207666.9 |
  207781.3 |
  207895.7 |########################################
  208010.0 |
  208124.4 |
  208238.8 |
  208353.2 |
  208467.5 |
  208581.9 |########################################
  208696.3 |########################################
  208810.6 |########################################
  208925.0 |
  209039.4 |
  209153.8 |
  209268.1 |
  209382.5 |
  209496.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=4318.1% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=306.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=312.7% of algo (FFI overhead may distort results)
