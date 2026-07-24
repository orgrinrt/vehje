# abi_payload_cost (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_wideselect_scalar_payload has the worst median (475.08 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_wideselect_null_entry at 2.66 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_wideselect_null_entry dominates: 8083% faster than the next best (abi_payload_cost_wideselect_soa_payload)

abi_payload_cost_wideselect_null_entry (2.66 us) leads abi_payload_cost_wideselect_soa_payload (217.38 us) by 8083%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_wideselect_null_entry beats baseline by 99% (significant)

abi_payload_cost_wideselect_null_entry is -472.42 us (99%) faster than baseline abi_payload_cost_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_wideselect_scalar_payload is an outlier: 178.8x slower than the field

abi_payload_cost_wideselect_scalar_payload (475.08 us) is 178.8x the fastest (2.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 178.8x the fastest

Fastest abi_payload_cost_wideselect_null_entry (2.66 us) to slowest abi_payload_cost_wideselect_scalar_payload (475.08 us): 178.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_wideselect_null_entry** at 2656.7 ns median (-99.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 178.83x (fastest 2656.7 ns, slowest 475082.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4965ns | 5004ns | 4695ns | 4904ns | 5192ns | -98.96% |
| abi_payload_cost_wideselect_scalar_payload | 477577ns | 477472ns | 476256ns | 477071ns | 478997ns | base |
| abi_payload_cost_wideselect_soa_payload | 219699ns | 219713ns | 217789ns | 219468ns | 220999ns | -54.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 2621ns | 2478ns | 2722ns | -99.45% | 0.024 |
| abi_payload_cost_wideselect_scalar_payload | 475258ns | 473994ns | 476697ns | base | 0.000 |
| abi_payload_cost_wideselect_soa_payload | 217438ns | 215625ns | 218721ns | -54.25% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 22534.7 | 2781.6 | 2621.0 | n/a |
| abi_payload_cost_wideselect_scalar_payload | 24892.0 | 475429.2 | 475258.1 | n/a |
| abi_payload_cost_wideselect_soa_payload | 22767.0 | 217457.7 | 217437.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_payload_cost_wideselect_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | 0.024 | 93.3% |
| abi_payload_cost_wideselect_scalar_payload | 0.000 | 0.5% |
| abi_payload_cost_wideselect_soa_payload | 0.000 | 1.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4965ns | 4965ns | -98.96% |
| abi_payload_cost_wideselect_scalar_payload | 477577ns | 477577ns | base |
| abi_payload_cost_wideselect_soa_payload | 219699ns | 219699ns | -54.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_scalar_payload | 475082ns | base | --- | [473995, 476697] | --- | --- | --- | --- |
| abi_payload_cost_wideselect_null_entry | 2657ns | -472423.9ns (-99.4%) | [-474102, -471386]ns | [2484, 2722] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_wideselect_soa_payload | 217383ns | -258191.5ns (-54.3%) | [-259578, -255692]ns | [216208, 218721] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_wideselect_scalar_payload | abi_payload_cost_wideselect_null_entry | abi_payload_cost_wideselect_soa_payload |
|---|---|---|---|
| 1 | 477285ns | -99.5% | -54.4% |
| 2 | 473994ns | -99.5% | -54.5% |
| 3 | 473995ns | -99.4% | -54.0% |
| 4 | 474832ns | -99.4% | -53.8% |
| 5 | 475332ns | -99.5% | -54.3% |
| 6 | 476110ns | -99.4% | -54.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | 0.164 | ok |
| abi_payload_cost_wideselect_scalar_payload | -0.048 | ok |
| abi_payload_cost_wideselect_soa_payload | -0.008 | ok |

**Consistency summary:**

- **abi_payload_cost_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 109921.2ns | 2621.0ns | 4193.8% | HIGH |
| abi_payload_cost_wideselect_scalar_payload | 1452794.7ns | 475258.1ns | 305.7% | HIGH |
| abi_payload_cost_wideselect_soa_payload | 676284.6ns | 217437.5ns | 311.0% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_wideselect_null_entry (n=6, range 2477.9-2722.5 ns)
   2477.9 |########################################
   2490.1 |
   2502.4 |
   2514.6 |
   2526.8 |
   2539.1 |
   2551.3 |
   2563.5 |
   2575.7 |
   2588.0 |####################
   2600.2 |
   2612.4 |
   2624.7 |
   2636.9 |
   2649.1 |
   2661.3 |
   2673.6 |
   2685.8 |
   2698.0 |
   2710.3 |########################################
  (0 below, 1 above range)

abi_payload_cost_wideselect_scalar_payload (n=6, range 473994.2-476697.1 ns)
  473994.2 |########################################
  474129.3 |
  474264.5 |
  474399.6 |
  474534.8 |
  474669.9 |
  474805.1 |####################
  474940.2 |
  475075.4 |
  475210.5 |####################
  475345.7 |
  475480.8 |
  475615.9 |
  475751.1 |
  475886.2 |
  476021.4 |####################
  476156.5 |
  476291.7 |
  476426.8 |
  476562.0 |
  (0 below, 1 above range)

abi_payload_cost_wideselect_soa_payload (n=6, range 215625.0-218721.5 ns)
  215625.0 |########################################
  215779.8 |
  215934.6 |
  216089.5 |
  216244.3 |
  216399.1 |
  216553.9 |
  216708.8 |########################################
  216863.6 |
  217018.4 |
  217173.2 |########################################
  217328.0 |########################################
  217482.9 |
  217637.7 |
  217792.5 |
  217947.3 |########################################
  218102.2 |
  218257.0 |
  218411.8 |
  218566.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_wideselect_null_entry**: bridge=4168.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_scalar_payload**: bridge=306.1% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_soa_payload**: bridge=310.9% of algo (FFI overhead may distort results)
