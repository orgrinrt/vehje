# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (313.90 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry dominates: 7548% faster than the next best (abi_payload_cost_leaf_soa_payload)

abi_payload_cost_leaf_null_entry (2.57 us) leads abi_payload_cost_leaf_soa_payload (196.91 us) by 7548%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_leaf_null_entry beats baseline by 99% (significant)

abi_payload_cost_leaf_null_entry is -311.36 us (99%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 121.9x slower than the field

abi_payload_cost_leaf_scalar_payload (313.90 us) is 121.9x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 121.9x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.57 us) to slowest abi_payload_cost_leaf_scalar_payload (313.90 us): 121.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2574.6 ns median (-99.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 121.93x (fastest 2574.6 ns, slowest 313904.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4886ns | 4845ns | 4626ns | 4786ns | 5166ns | -98.46% |
| abi_payload_cost_leaf_scalar_payload | 317642ns | 316213ns | 315762ns | 316105ns | 320886ns | base |
| abi_payload_cost_leaf_soa_payload | 199256ns | 199138ns | 198517ns | 199039ns | 199950ns | -37.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2583ns | 2454ns | 2712ns | -99.18% | 0.025 |
| abi_payload_cost_leaf_scalar_payload | 315249ns | 313415ns | 318314ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 196982ns | 196278ns | 197622ns | -37.52% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 21967.9 | 2763.1 | 2583.1 | n/a |
| abi_payload_cost_leaf_scalar_payload | 25591.6 | 315674.4 | 315249.0 | n/a |
| abi_payload_cost_leaf_soa_payload | 23463.5 | 196886.7 | 196982.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.025 | 95.3% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 0.8% |
| abi_payload_cost_leaf_soa_payload | 0.000 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4886ns | 4886ns | -98.46% |
| abi_payload_cost_leaf_scalar_payload | 317642ns | 317642ns | base |
| abi_payload_cost_leaf_soa_payload | 199256ns | 199256ns | -37.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 313904ns | base | --- | [313529, 318314] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2575ns | -311363.5ns (-99.2%) | [-315687, -310947]ns | [2462, 2712] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 196909ns | -116886.4ns (-37.2%) | [-121609, -116305]ns | [196415, 197622] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 314134ns | -99.2% | -37.5% |
| 2 | 313642ns | -99.1% | -37.1% |
| 3 | 314212ns | -99.2% | -37.0% |
| 4 | 313415ns | -99.2% | -37.2% |
| 5 | 313675ns | -99.2% | -37.3% |
| 6 | 322415ns | -99.2% | -38.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | -0.180 | ok |
| abi_payload_cost_leaf_scalar_payload | -0.049 | ok |
| abi_payload_cost_leaf_soa_payload | -0.056 | ok |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 108937.6ns | 2583.1ns | 4217.4% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 974059.3ns | 315249.0ns | 309.0% | HIGH |
| abi_payload_cost_leaf_soa_payload | 615302.6ns | 196982.1ns | 312.4% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2453.8-2712.5 ns)
   2453.8 |########################################
   2466.7 |########################################
   2479.7 |
   2492.6 |
   2505.5 |
   2518.5 |########################################
   2531.4 |
   2544.3 |
   2557.3 |
   2570.2 |
   2583.2 |
   2596.1 |
   2609.0 |
   2622.0 |########################################
   2634.9 |
   2647.8 |
   2660.8 |
   2673.7 |
   2686.6 |########################################
   2699.6 |
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 313415.4-318313.8 ns)
  313415.4 |########################################
  313660.3 |####################
  313905.2 |####################
  314150.2 |####################
  314395.1 |
  314640.0 |
  314884.9 |
  315129.8 |
  315374.7 |
  315619.7 |
  315864.6 |
  316109.5 |
  316354.4 |
  316599.3 |
  316844.2 |
  317089.2 |
  317334.1 |
  317579.0 |
  317823.9 |
  318068.8 |
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 196277.9-197622.5 ns)
  196277.9 |########################################
  196345.1 |
  196412.4 |
  196479.6 |
  196546.8 |########################################
  196614.0 |
  196681.3 |
  196748.5 |########################################
  196815.7 |
  196882.9 |
  196950.2 |
  197017.4 |########################################
  197084.6 |########################################
  197151.9 |
  197219.1 |
  197286.3 |
  197353.5 |
  197420.8 |
  197488.0 |
  197555.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=4250.4% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=308.4% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=311.9% of algo (FFI overhead may distort results)
