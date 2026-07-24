# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (78.03 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry dominates: 1209% faster than the next best (abi_payload_cost_leaf_soa_payload)

abi_payload_cost_leaf_null_entry (2.49 us) leads abi_payload_cost_leaf_soa_payload (32.60 us) by 1209%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_leaf_null_entry beats baseline by 97% (significant)

abi_payload_cost_leaf_null_entry is -75.54 us (97%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 31.3x slower than the field

abi_payload_cost_leaf_scalar_payload (78.03 us) is 31.3x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 31.3x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.49 us) to slowest abi_payload_cost_leaf_scalar_payload (78.03 us): 31.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2490.0 ns median (-96.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 31.34x (fastest 2490.0 ns, slowest 78026.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4733ns | 4691ns | 4619ns | 4674ns | 4879ns | -94.09% |
| abi_payload_cost_leaf_scalar_payload | 80112ns | 80222ns | 79160ns | 80056ns | 80673ns | base |
| abi_payload_cost_leaf_soa_payload | 34907ns | 34876ns | 34288ns | 34846ns | 35309ns | -56.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2501ns | 2440ns | 2567ns | -96.79% | 0.006 |
| abi_payload_cost_leaf_scalar_payload | 77901ns | 76949ns | 78458ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 32631ns | 32044ns | 33005ns | -58.11% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 20170.6 | 2697.9 | 2500.7 | n/a |
| abi_payload_cost_leaf_scalar_payload | 20539.5 | 78143.9 | 77900.8 | n/a |
| abi_payload_cost_leaf_soa_payload | 20812.0 | 32616.5 | 32631.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.006 | 98.0% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 3.1% |
| abi_payload_cost_leaf_soa_payload | 0.000 | 7.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4733ns | 4733ns | -94.09% |
| abi_payload_cost_leaf_scalar_payload | 80112ns | 80112ns | base |
| abi_payload_cost_leaf_soa_payload | 34907ns | 34907ns | -56.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 78027ns | base | --- | [77218, 78458] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2490ns | -75541.2ns (-96.8%) | [-75931, -74728]ns | [2445, 2567] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 32601ns | -45354.8ns (-58.1%) | [-45829, -44625]ns | [32289, 33005] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 78082ns | -96.9% | -58.3% |
| 2 | 78737ns | -96.8% | -57.6% |
| 3 | 77487ns | -96.8% | -58.0% |
| 4 | 76949ns | -96.8% | -57.6% |
| 5 | 77972ns | -96.9% | -58.1% |
| 6 | 78178ns | -96.6% | -59.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | -0.334 | moderate- |
| abi_payload_cost_leaf_scalar_payload | 0.080 | ok |
| abi_payload_cost_leaf_soa_payload | -0.149 | ok |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 105531.1ns | 2500.7ns | 4220.1% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 332215.8ns | 77900.8ns | 426.5% | HIGH |
| abi_payload_cost_leaf_soa_payload | 184284.3ns | 32631.3ns | 564.7% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2440.4-2566.6 ns)
   2440.4 |########################################
   2446.7 |########################################
   2453.0 |
   2459.3 |
   2465.7 |
   2472.0 |
   2478.3 |
   2484.6 |########################################
   2490.9 |########################################
   2497.2 |
   2503.5 |########################################
   2509.8 |
   2516.1 |
   2522.5 |
   2528.8 |
   2535.1 |
   2541.4 |
   2547.7 |
   2554.0 |
   2560.3 |
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 76949.2-78457.5 ns)
  76949.2 |########################################
  77024.6 |
  77100.0 |
  77175.4 |
  77250.9 |
  77326.3 |
  77401.7 |
  77477.1 |########################################
  77552.5 |
  77627.9 |
  77703.4 |
  77778.8 |
  77854.2 |
  77929.6 |########################################
  78005.0 |
  78080.4 |########################################
  78155.8 |########################################
  78231.3 |
  78306.7 |
  78382.1 |
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 32043.7-33004.6 ns)
  32043.7 |####################
  32091.7 |
  32139.8 |
  32187.8 |
  32235.9 |
  32283.9 |
  32332.0 |
  32380.0 |
  32428.0 |
  32476.1 |
  32524.1 |########################################
  32572.2 |
  32620.2 |########################################
  32668.3 |
  32716.3 |
  32764.3 |
  32812.4 |
  32860.4 |
  32908.5 |
  32956.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=4232.8% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=426.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=562.8% of algo (FFI overhead may distort results)
