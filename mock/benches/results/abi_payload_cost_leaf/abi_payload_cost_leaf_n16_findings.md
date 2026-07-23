# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (77.97 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.64 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry dominates: 1118% faster than the next best (abi_payload_cost_leaf_soa_payload)

abi_payload_cost_leaf_null_entry (2.64 us) leads abi_payload_cost_leaf_soa_payload (32.21 us) by 1118%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_leaf_null_entry beats baseline by 97% (significant)

abi_payload_cost_leaf_null_entry is -75.32 us (97%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 29.5x slower than the field

abi_payload_cost_leaf_scalar_payload (77.97 us) is 29.5x the fastest (2.64 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 29.5x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.64 us) to slowest abi_payload_cost_leaf_scalar_payload (77.97 us): 29.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2644.0 ns median (-96.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 29.49x (fastest 2644.0 ns, slowest 77972.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4911ns | 4998ns | 4642ns | 4902ns | 5059ns | -93.91% |
| abi_payload_cost_leaf_scalar_payload | 80594ns | 80218ns | 79793ns | 80175ns | 81624ns | base |
| abi_payload_cost_leaf_soa_payload | 34480ns | 34459ns | 34154ns | 34380ns | 34792ns | -57.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2599ns | 2469ns | 2667ns | -96.68% | 0.006 |
| abi_payload_cost_leaf_scalar_payload | 78284ns | 77478ns | 79281ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 32205ns | 31919ns | 32481ns | -58.86% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 21504.1 | 2767.8 | 2599.2 | n/a |
| abi_payload_cost_leaf_scalar_payload | 22021.5 | 78380.6 | 78283.6 | n/a |
| abi_payload_cost_leaf_soa_payload | 20994.4 | 32304.4 | 32204.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.006 | 93.4% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 3.2% |
| abi_payload_cost_leaf_soa_payload | 0.000 | 7.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4911ns | 4911ns | -93.91% |
| abi_payload_cost_leaf_scalar_payload | 80594ns | 80594ns | base |
| abi_payload_cost_leaf_soa_payload | 34480ns | 34480ns | -57.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 77973ns | base | --- | [77596, 79281] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2644ns | -75324.1ns (-96.6%) | [-76697, -75032]ns | [2487, 2667] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 32208ns | -45832.4ns (-58.8%) | [-47055, -45348]ns | [31926, 32481] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 79055ns | -96.8% | -59.1% |
| 2 | 79508ns | -96.6% | -59.6% |
| 3 | 78040ns | -96.6% | -59.1% |
| 4 | 77715ns | -96.8% | -58.5% |
| 5 | 77478ns | -96.6% | -58.8% |
| 6 | 77906ns | -96.6% | -58.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | -0.423 | moderate- |
| abi_payload_cost_leaf_scalar_payload | 0.473 | moderate+ |
| abi_payload_cost_leaf_soa_payload | -0.401 | moderate- |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 108084.9ns | 2599.2ns | 4158.4% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 333693.7ns | 78283.6ns | 426.3% | HIGH |
| abi_payload_cost_leaf_soa_payload | 182607.9ns | 32204.9ns | 567.0% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2469.2-2666.6 ns)
   2469.2 |####################
   2479.1 |
   2488.9 |
   2498.8 |####################
   2508.7 |
   2518.6 |
   2528.4 |
   2538.3 |
   2548.2 |
   2558.1 |
   2567.9 |
   2577.8 |
   2587.7 |
   2597.5 |
   2607.4 |
   2617.3 |
   2627.2 |####################
   2637.0 |
   2646.9 |
   2656.8 |########################################
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 77477.5-79281.4 ns)
  77477.5 |########################################
  77567.7 |
  77657.9 |########################################
  77748.1 |
  77838.3 |########################################
  77928.5 |
  78018.7 |########################################
  78108.9 |
  78199.1 |
  78289.3 |
  78379.5 |
  78469.7 |
  78559.9 |
  78650.1 |
  78740.3 |
  78830.5 |
  78920.7 |
  79010.9 |########################################
  79101.1 |
  79191.3 |
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 31918.7-32481.0 ns)
  31918.7 |########################################
  31946.8 |
  31974.9 |
  32003.0 |
  32031.2 |
  32059.3 |
  32087.4 |
  32115.5 |
  32143.6 |####################
  32171.7 |
  32199.8 |
  32228.0 |
  32256.1 |####################
  32284.2 |####################
  32312.3 |
  32340.4 |
  32368.5 |
  32396.7 |
  32424.8 |
  32452.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=4105.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=427.5% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=566.2% of algo (FFI overhead may distort results)
