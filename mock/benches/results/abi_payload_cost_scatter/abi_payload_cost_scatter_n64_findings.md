# abi_payload_cost (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_scatter_scalar_payload has the worst median (487.16 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_scatter_null_entry at 2.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_scatter_null_entry dominates: 7998% faster than the next best (abi_payload_cost_scatter_soa_payload)

abi_payload_cost_scatter_null_entry (2.55 us) leads abi_payload_cost_scatter_soa_payload (206.25 us) by 7998%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_scatter_null_entry beats baseline by 99% (significant)

abi_payload_cost_scatter_null_entry is -484.63 us (99%) faster than baseline abi_payload_cost_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_scatter_scalar_payload is an outlier: 191.3x slower than the field

abi_payload_cost_scatter_scalar_payload (487.16 us) is 191.3x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 191.3x the fastest

Fastest abi_payload_cost_scatter_null_entry (2.55 us) to slowest abi_payload_cost_scatter_scalar_payload (487.16 us): 191.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_scatter_null_entry** at 2546.9 ns median (-99.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 191.28x (fastest 2546.9 ns, slowest 487162.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4840ns | 4817ns | 4696ns | 4797ns | 4977ns | -99.01% |
| abi_payload_cost_scatter_scalar_payload | 489133ns | 489451ns | 484115ns | 488959ns | 491904ns | base |
| abi_payload_cost_scatter_soa_payload | 209553ns | 208611ns | 206917ns | 208174ns | 212940ns | -57.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 2549ns | 2472ns | 2625ns | -99.48% | 0.025 |
| abi_payload_cost_scatter_scalar_payload | 486814ns | 481768ns | 489541ns | base | 0.000 |
| abi_payload_cost_scatter_soa_payload | 207268ns | 204740ns | 210647ns | -57.42% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 22064.6 | 2721.8 | 2548.9 | n/a |
| abi_payload_cost_scatter_scalar_payload | 24188.2 | 486891.9 | 486813.6 | n/a |
| abi_payload_cost_scatter_soa_payload | 22799.8 | 206806.0 | 207268.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_payload_cost_scatter_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_scatter_null_entry | 0.025 | 97.0% |
| abi_payload_cost_scatter_scalar_payload | 0.000 | 0.5% |
| abi_payload_cost_scatter_soa_payload | 0.000 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4840ns | 4840ns | -99.01% |
| abi_payload_cost_scatter_scalar_payload | 489133ns | 489133ns | base |
| abi_payload_cost_scatter_soa_payload | 209553ns | 209553ns | -57.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_scalar_payload | 487162ns | base | --- | [483738, 489541] | --- | --- | --- | --- |
| abi_payload_cost_scatter_null_entry | 2547ns | -484634.3ns (-99.5%) | [-486969, -481191]ns | [2475, 2625] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_scatter_soa_payload | 206254ns | -281652.7ns (-57.8%) | [-283893, -273091]ns | [204904, 210647] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_scatter_scalar_payload | abi_payload_cost_scatter_null_entry | abi_payload_cost_scatter_soa_payload |
|---|---|---|---|
| 1 | 486441ns | -99.5% | -57.8% |
| 2 | 490428ns | -99.5% | -58.3% |
| 3 | 487883ns | -99.5% | -57.8% |
| 4 | 485708ns | -99.5% | -56.2% |
| 5 | 481768ns | -99.5% | -56.7% |
| 6 | 488654ns | -99.5% | -57.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_scatter_null_entry | -0.184 | ok |
| abi_payload_cost_scatter_scalar_payload | -0.053 | ok |
| abi_payload_cost_scatter_soa_payload | 0.144 | ok |

**Consistency summary:**

- **abi_payload_cost_scatter_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 108512.1ns | 2548.9ns | 4257.2% | HIGH |
| abi_payload_cost_scatter_scalar_payload | 1485956.1ns | 486813.6ns | 305.2% | HIGH |
| abi_payload_cost_scatter_soa_payload | 644432.8ns | 207268.2ns | 310.9% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_scatter_null_entry (n=6, range 2471.7-2624.6 ns)
   2471.7 |########################################
   2479.3 |
   2487.0 |
   2494.6 |
   2502.3 |
   2509.9 |
   2517.6 |
   2525.2 |
   2532.8 |
   2540.5 |####################
   2548.1 |####################
   2555.8 |
   2563.4 |
   2571.1 |
   2578.7 |####################
   2586.3 |
   2594.0 |
   2601.6 |
   2609.3 |
   2616.9 |
  (0 below, 1 above range)

abi_payload_cost_scatter_scalar_payload (n=6, range 481768.3-489540.8 ns)
  481768.3 |########################################
  482156.9 |
  482545.6 |
  482934.2 |
  483322.8 |
  483711.4 |
  484100.1 |
  484488.7 |
  484877.3 |
  485265.9 |
  485654.6 |########################################
  486043.2 |
  486431.8 |########################################
  486820.5 |
  487209.1 |
  487597.7 |########################################
  487986.3 |
  488375.0 |########################################
  488763.6 |
  489152.2 |
  (0 below, 1 above range)

abi_payload_cost_scatter_soa_payload (n=6, range 204739.6-210647.3 ns)
  204739.6 |########################################
  205035.0 |########################################
  205330.4 |
  205625.8 |########################################
  205921.1 |
  206216.5 |
  206511.9 |########################################
  206807.3 |
  207102.7 |
  207398.1 |
  207693.5 |
  207988.8 |
  208284.2 |########################################
  208579.6 |
  208875.0 |
  209170.4 |
  209465.8 |
  209761.1 |
  210056.5 |
  210351.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_scatter_null_entry**: bridge=4262.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_scalar_payload**: bridge=305.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_soa_payload**: bridge=311.5% of algo (FFI overhead may distort results)
