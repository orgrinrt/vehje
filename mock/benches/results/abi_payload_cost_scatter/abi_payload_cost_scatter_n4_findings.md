# abi_payload_cost (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_scatter_scalar_payload has the worst median (27.01 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_scatter_null_entry at 2.62 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_scatter_null_entry dominates: 149% faster than the next best (abi_payload_cost_scatter_soa_payload)

abi_payload_cost_scatter_null_entry (2.62 us) leads abi_payload_cost_scatter_soa_payload (6.53 us) by 149%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_scatter_null_entry beats baseline by 90% (significant)

abi_payload_cost_scatter_null_entry is -24.39 us (90%) faster than baseline abi_payload_cost_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_scatter_scalar_payload is an outlier: 10.3x slower than the field

abi_payload_cost_scatter_scalar_payload (27.01 us) is 10.3x the fastest (2.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 10.3x the fastest

Fastest abi_payload_cost_scatter_null_entry (2.62 us) to slowest abi_payload_cost_scatter_scalar_payload (27.01 us): 10.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_scatter_null_entry** at 2620.2 ns median (-90.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.31x (fastest 2620.2 ns, slowest 27006.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4912ns | 4923ns | 4616ns | 4879ns | 5109ns | -83.21% |
| abi_payload_cost_scatter_scalar_payload | 29248ns | 29275ns | 27780ns | 29119ns | 30175ns | base |
| abi_payload_cost_scatter_soa_payload | 8817ns | 8836ns | 8685ns | 8788ns | 8927ns | -69.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 2626ns | 2462ns | 2747ns | -90.26% | 0.002 |
| abi_payload_cost_scatter_scalar_payload | 26959ns | 25620ns | 27799ns | base | 0.000 |
| abi_payload_cost_scatter_soa_payload | 6517ns | 6420ns | 6594ns | -75.83% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 19824.9 | 2780.1 | 2625.8 | n/a |
| abi_payload_cost_scatter_scalar_payload | 20278.5 | 27101.2 | 26958.8 | n/a |
| abi_payload_cost_scatter_soa_payload | 19935.6 | 6596.5 | 6516.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_payload_cost_scatter_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_scatter_null_entry | 0.002 | 94.0% |
| abi_payload_cost_scatter_scalar_payload | 0.000 | 9.1% |
| abi_payload_cost_scatter_soa_payload | 0.001 | 37.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4912ns | 4912ns | -83.21% |
| abi_payload_cost_scatter_scalar_payload | 29248ns | 29248ns | base |
| abi_payload_cost_scatter_soa_payload | 8817ns | 8817ns | -69.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_scalar_payload | 27006ns | base | --- | [26071, 27799] | --- | --- | --- | --- |
| abi_payload_cost_scatter_null_entry | 2620ns | -24386.1ns (-90.3%) | [-25160, -23453]ns | [2510, 2747] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_scatter_soa_payload | 6527ns | -20523.8ns (-76.0%) | [-21234, -19568]ns | [6429, 6594] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_scatter_scalar_payload | abi_payload_cost_scatter_null_entry | abi_payload_cost_scatter_soa_payload |
|---|---|---|---|
| 1 | 26521ns | -90.7% | -75.8% |
| 2 | 28210ns | -90.9% | -76.9% |
| 3 | 27040ns | -90.4% | -76.2% |
| 4 | 27389ns | -90.1% | -75.9% |
| 5 | 25620ns | -89.2% | -74.3% |
| 6 | 26972ns | -90.2% | -75.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_scatter_null_entry | 0.406 | moderate+ |
| abi_payload_cost_scatter_scalar_payload | -0.269 | moderate- |
| abi_payload_cost_scatter_soa_payload | -0.075 | ok |

**Consistency summary:**

- **abi_payload_cost_scatter_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 108252.3ns | 2625.8ns | 4122.6% | HIGH |
| abi_payload_cost_scatter_scalar_payload | 179812.7ns | 26958.8ns | 667.0% | HIGH |
| abi_payload_cost_scatter_soa_payload | 120756.1ns | 6516.8ns | 1853.0% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_scatter_null_entry (n=6, range 2461.7-2747.1 ns)
   2461.7 |########################################
   2476.0 |
   2490.2 |
   2504.5 |
   2518.8 |
   2533.0 |
   2547.3 |########################################
   2561.6 |
   2575.9 |
   2590.1 |
   2604.4 |########################################
   2618.7 |########################################
   2632.9 |
   2647.2 |
   2661.5 |
   2675.8 |
   2690.0 |
   2704.3 |
   2718.6 |########################################
   2732.8 |
  (0 below, 1 above range)

abi_payload_cost_scatter_scalar_payload (n=6, range 25620.4-27799.4 ns)
  25620.4 |########################################
  25729.4 |
  25838.3 |
  25947.2 |
  26056.2 |
  26165.2 |
  26274.1 |
  26383.1 |
  26492.0 |########################################
  26601.0 |
  26709.9 |
  26818.9 |
  26927.8 |########################################
  27036.8 |########################################
  27145.7 |
  27254.7 |
  27363.6 |########################################
  27472.6 |
  27581.5 |
  27690.5 |
  (0 below, 1 above range)

abi_payload_cost_scatter_soa_payload (n=6, range 6420.4-6593.9 ns)
   6420.4 |####################
   6429.1 |
   6437.8 |####################
   6446.4 |
   6455.1 |
   6463.8 |
   6472.5 |
   6481.1 |
   6489.8 |
   6498.5 |
   6507.2 |
   6515.9 |
   6524.5 |########################################
   6533.2 |
   6541.9 |
   6550.6 |
   6559.2 |
   6567.9 |
   6576.6 |####################
   6585.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_scatter_null_entry**: bridge=4131.4% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_scalar_payload**: bridge=668.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_soa_payload**: bridge=1850.0% of algo (FFI overhead may distort results)
