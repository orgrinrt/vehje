# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (1.45 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.46 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry dominates: 33166% faster than the next best (abi_payload_cost_leaf_soa_payload)

abi_payload_cost_leaf_null_entry (2.46 us) leads abi_payload_cost_leaf_soa_payload (818.70 us) by 33166%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_leaf_null_entry beats baseline by 100% (significant)

abi_payload_cost_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 590.7x slower than the field

abi_payload_cost_leaf_scalar_payload (1.45 ms) is 590.7x the fastest (2.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 590.7x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.46 us) to slowest abi_payload_cost_leaf_scalar_payload (1.45 ms): 590.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2461.1 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 590.67x (fastest 2461.1 ns, slowest 1453667.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4779ns | 4713ns | 4660ns | 4708ns | 4944ns | -99.67% |
| abi_payload_cost_leaf_scalar_payload | 1462422ns | 1456137ns | 1437585ns | 1452764ns | 1489327ns | base |
| abi_payload_cost_leaf_soa_payload | 819334ns | 821254ns | 806086ns | 817029ns | 829416ns | -43.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2498ns | 2436ns | 2597ns | -99.83% | 0.102 |
| abi_payload_cost_leaf_scalar_payload | 1459809ns | 1435137ns | 1486408ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 816884ns | 803832ns | 826856ns | -44.04% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 27377.6 | 2727.3 | 2498.4 | n/a |
| abi_payload_cost_leaf_scalar_payload | 41218.4 | 1461683.1 | 1459808.9 | n/a |
| abi_payload_cost_leaf_soa_payload | 33980.5 | 816509.8 | 816883.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.105 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.104 | 99.0% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 0.2% |
| abi_payload_cost_leaf_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4779ns | 4779ns | -99.67% |
| abi_payload_cost_leaf_scalar_payload | 1462422ns | 1462422ns | base |
| abi_payload_cost_leaf_soa_payload | 819334ns | 819334ns | -43.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 1453667ns | base | --- | [1439352, 1486408] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2461ns | -1451206.2ns (-99.8%) | [-1483902, -1436823]ns | [2437, 2597] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 818705ns | -645392.3ns (-44.4%) | [-662736, -620647]ns | [805090, 826856] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 1475280ns | -99.8% | -43.7% |
| 2 | 1497536ns | -99.8% | -45.0% |
| 3 | 1452230ns | -99.8% | -44.5% |
| 4 | 1443567ns | -99.8% | -43.4% |
| 5 | 1455104ns | -99.8% | -44.8% |
| 6 | 1435137ns | -99.8% | -42.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | -0.293 | moderate- |
| abi_payload_cost_leaf_scalar_payload | 0.235 | moderate+ |
| abi_payload_cost_leaf_soa_payload | -0.053 | ok |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 112931.8ns | 2498.4ns | 4520.2% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 4447783.3ns | 1459808.9ns | 304.7% | HIGH |
| abi_payload_cost_leaf_soa_payload | 2487488.3ns | 816883.7ns | 304.5% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2436.2-2596.9 ns)
   2436.2 |########################################
   2444.2 |
   2452.3 |####################
   2460.3 |
   2468.3 |####################
   2476.4 |
   2484.4 |
   2492.4 |
   2500.5 |
   2508.5 |
   2516.5 |
   2524.6 |
   2532.6 |
   2540.7 |
   2548.7 |
   2556.7 |
   2564.8 |####################
   2572.8 |
   2580.8 |
   2588.9 |
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 1435136.7-1486407.7 ns)
  1435136.7 |########################################
  1437700.2 |
  1440263.8 |
  1442827.4 |########################################
  1445390.9 |
  1447954.4 |
  1450518.0 |########################################
  1453081.6 |########################################
  1455645.1 |
  1458208.7 |
  1460772.2 |
  1463335.8 |
  1465899.3 |
  1468462.9 |
  1471026.4 |
  1473590.0 |########################################
  1476153.5 |
  1478717.1 |
  1481280.6 |
  1483844.2 |
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 803831.7-826856.2 ns)
  803831.7 |########################################
  804982.9 |
  806134.1 |########################################
  807285.4 |
  808436.6 |
  809587.8 |
  810739.0 |
  811890.3 |
  813041.5 |
  814192.7 |
  815343.9 |
  816495.2 |########################################
  817646.4 |
  818797.6 |
  819948.8 |########################################
  821100.1 |
  822251.3 |########################################
  823402.5 |
  824553.8 |
  825705.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=4568.5% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=304.3% of algo (FFI overhead may distort results)
