# abi_payload_cost (madd)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_madd_scalar_payload has the worst median (26.71 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_madd_null_entry at 2.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_madd_null_entry dominates: 155% faster than the next best (abi_payload_cost_madd_soa_payload)

abi_payload_cost_madd_null_entry (2.57 us) leads abi_payload_cost_madd_soa_payload (6.56 us) by 155%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_madd_null_entry beats baseline by 91% (significant)

abi_payload_cost_madd_null_entry is -24.19 us (91%) faster than baseline abi_payload_cost_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_madd_scalar_payload is an outlier: 10.4x slower than the field

abi_payload_cost_madd_scalar_payload (26.71 us) is 10.4x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 10.4x the fastest

Fastest abi_payload_cost_madd_null_entry (2.57 us) to slowest abi_payload_cost_madd_scalar_payload (26.71 us): 10.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_madd_null_entry** at 2572.3 ns median (-90.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.38x (fastest 2572.3 ns, slowest 26712.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4847ns | 4850ns | 4610ns | 4825ns | 5000ns | -83.31% |
| abi_payload_cost_madd_scalar_payload | 29047ns | 28975ns | 28811ns | 28934ns | 29334ns | base |
| abi_payload_cost_madd_soa_payload | 8834ns | 8858ns | 8478ns | 8782ns | 9090ns | -69.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 2572ns | 2462ns | 2665ns | -90.40% | 0.002 |
| abi_payload_cost_madd_scalar_payload | 26781ns | 26552ns | 27060ns | base | 0.000 |
| abi_payload_cost_madd_soa_payload | 6525ns | 6258ns | 6692ns | -75.63% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 19645.1 | 2736.1 | 2571.5 | n/a |
| abi_payload_cost_madd_scalar_payload | 20519.4 | 26889.2 | 26781.2 | n/a |
| abi_payload_cost_madd_soa_payload | 20601.3 | 6620.4 | 6525.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_payload_cost_madd_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_madd_null_entry | 0.002 | 95.7% |
| abi_payload_cost_madd_scalar_payload | 0.000 | 9.2% |
| abi_payload_cost_madd_soa_payload | 0.001 | 37.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4847ns | 4847ns | -83.31% |
| abi_payload_cost_madd_scalar_payload | 29047ns | 29047ns | base |
| abi_payload_cost_madd_soa_payload | 8834ns | 8834ns | -69.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_madd_scalar_payload | 26712ns | base | --- | [26572, 27060] | --- | --- | --- | --- |
| abi_payload_cost_madd_null_entry | 2572ns | -24192.5ns (-90.6%) | [-24459, -23978]ns | [2477, 2665] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_madd_soa_payload | 6560ns | -20321.4ns (-76.1%) | [-20474, -19972]ns | [6324, 6692] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_madd_scalar_payload | abi_payload_cost_madd_null_entry | abi_payload_cost_madd_soa_payload |
|---|---|---|---|
| 1 | 26552ns | -90.6% | -74.7% |
| 2 | 26897ns | -90.5% | -75.3% |
| 3 | 26787ns | -90.8% | -76.1% |
| 4 | 27222ns | -90.3% | -75.5% |
| 5 | 26591ns | -89.9% | -75.6% |
| 6 | 26638ns | -90.3% | -76.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_madd_null_entry | 0.113 | ok |
| abi_payload_cost_madd_scalar_payload | -0.253 | moderate- |
| abi_payload_cost_madd_soa_payload | -0.058 | ok |

**Consistency summary:**

- **abi_payload_cost_madd_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 106911.3ns | 2571.5ns | 4157.5% | HIGH |
| abi_payload_cost_madd_scalar_payload | 181256.0ns | 26781.2ns | 676.8% | HIGH |
| abi_payload_cost_madd_soa_payload | 121494.7ns | 6525.4ns | 1861.9% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_madd_null_entry (n=6, range 2461.7-2665.2 ns)
   2461.7 |########################################
   2471.9 |
   2482.0 |
   2492.2 |########################################
   2502.4 |
   2512.6 |
   2522.8 |
   2532.9 |
   2543.1 |
   2553.3 |########################################
   2563.4 |
   2573.6 |
   2583.8 |########################################
   2594.0 |
   2604.1 |
   2614.3 |
   2624.5 |
   2634.7 |########################################
   2644.8 |
   2655.0 |
  (0 below, 1 above range)

abi_payload_cost_madd_scalar_payload (n=6, range 26552.5-27059.8 ns)
  26552.5 |########################################
  26577.9 |########################################
  26603.2 |
  26628.6 |########################################
  26654.0 |
  26679.3 |
  26704.7 |
  26730.1 |
  26755.4 |
  26780.8 |########################################
  26806.2 |
  26831.5 |
  26856.9 |
  26882.2 |########################################
  26907.6 |
  26933.0 |
  26958.3 |
  26983.7 |
  27009.1 |
  27034.4 |
  (0 below, 1 above range)

abi_payload_cost_madd_soa_payload (n=6, range 6257.9-6691.9 ns)
   6257.9 |########################################
   6279.6 |
   6301.3 |
   6323.0 |
   6344.7 |
   6366.4 |
   6388.1 |########################################
   6409.8 |
   6431.5 |
   6453.2 |
   6474.9 |########################################
   6496.6 |
   6518.3 |
   6540.0 |
   6561.7 |
   6583.4 |
   6605.1 |
   6626.8 |########################################
   6648.5 |
   6670.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_madd_null_entry**: bridge=4167.5% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_scalar_payload**: bridge=678.4% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_soa_payload**: bridge=1847.5% of algo (FFI overhead may distort results)
