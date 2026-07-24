# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (26.77 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_null_entry dominates: 156% faster than the next best (abi_payload_cost_real_soa_payload)

abi_payload_cost_real_null_entry (2.58 us) leads abi_payload_cost_real_soa_payload (6.60 us) by 156%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_real_null_entry beats baseline by 90% (significant)

abi_payload_cost_real_null_entry is -24.20 us (90%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 10.4x slower than the field

abi_payload_cost_real_scalar_payload (26.77 us) is 10.4x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 10.4x the fastest

Fastest abi_payload_cost_real_null_entry (2.58 us) to slowest abi_payload_cost_real_scalar_payload (26.77 us): 10.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2580.0 ns median (-90.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.38x (fastest 2580.0 ns, slowest 26774.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4872ns | 4870ns | 4652ns | 4862ns | 4997ns | -83.29% |
| abi_payload_cost_real_scalar_payload | 29159ns | 29033ns | 28917ns | 29002ns | 29517ns | base |
| abi_payload_cost_real_soa_payload | 8914ns | 8922ns | 8763ns | 8912ns | 8993ns | -69.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2582ns | 2502ns | 2630ns | -90.40% | 0.002 |
| abi_payload_cost_real_scalar_payload | 26900ns | 26683ns | 27223ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 6582ns | 6489ns | 6627ns | -75.53% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 20323.4 | 2735.9 | 2582.4 | n/a |
| abi_payload_cost_real_scalar_payload | 20388.7 | 27028.7 | 26899.5 | n/a |
| abi_payload_cost_real_soa_payload | 20883.7 | 6670.8 | 6582.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.002 | 97.0% |
| abi_payload_cost_real_scalar_payload | 0.000 | 9.3% |
| abi_payload_cost_real_soa_payload | 0.001 | 37.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4872ns | 4872ns | -83.29% |
| abi_payload_cost_real_scalar_payload | 29159ns | 29159ns | base |
| abi_payload_cost_real_soa_payload | 8914ns | 8914ns | -69.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 26775ns | base | --- | [26701, 27223] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2580ns | -24196.9ns (-90.4%) | [-24634, -24121]ns | [2537, 2630] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 6597ns | -20234.2ns (-75.6%) | [-20619, -20099]ns | [6522, 6627] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 26719ns | -90.6% | -75.1% |
| 2 | 26683ns | -90.4% | -75.4% |
| 3 | 27182ns | -90.5% | -75.7% |
| 4 | 27265ns | -90.4% | -75.8% |
| 5 | 26718ns | -90.3% | -75.3% |
| 6 | 26830ns | -90.1% | -75.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.080 | ok |
| abi_payload_cost_real_scalar_payload | 0.083 | ok |
| abi_payload_cost_real_soa_payload | -0.169 | ok |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 107525.6ns | 2582.4ns | 4163.7% | HIGH |
| abi_payload_cost_real_scalar_payload | 181498.9ns | 26899.5ns | 674.7% | HIGH |
| abi_payload_cost_real_soa_payload | 121431.2ns | 6582.2ns | 1844.8% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2502.1-2630.2 ns)
   2502.1 |########################################
   2508.5 |
   2514.9 |
   2521.3 |
   2527.7 |
   2534.1 |
   2540.5 |
   2546.9 |
   2553.3 |
   2559.7 |
   2566.1 |########################################
   2572.6 |########################################
   2579.0 |
   2585.4 |########################################
   2591.8 |
   2598.2 |
   2604.6 |########################################
   2611.0 |
   2617.4 |
   2623.8 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 26683.3-27223.3 ns)
  26683.3 |####################
  26710.3 |########################################
  26737.3 |
  26764.3 |
  26791.3 |
  26818.3 |####################
  26845.3 |
  26872.3 |
  26899.3 |
  26926.3 |
  26953.3 |
  26980.3 |
  27007.3 |
  27034.3 |
  27061.3 |
  27088.3 |
  27115.3 |
  27142.3 |
  27169.3 |####################
  27196.3 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 6488.7-6626.9 ns)
   6488.7 |####################
   6495.6 |
   6502.5 |
   6509.4 |
   6516.3 |
   6523.2 |
   6530.1 |
   6537.1 |
   6544.0 |
   6550.9 |####################
   6557.8 |
   6564.7 |
   6571.6 |
   6578.5 |
   6585.4 |
   6592.3 |########################################
   6599.2 |
   6606.1 |####################
   6613.0 |
   6619.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=4169.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=677.7% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=1834.8% of algo (FFI overhead may distort results)
