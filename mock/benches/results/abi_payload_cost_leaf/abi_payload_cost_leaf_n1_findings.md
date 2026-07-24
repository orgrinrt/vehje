# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (9.54 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry beats baseline by 73% (significant)

abi_payload_cost_leaf_null_entry is -6.92 us (73%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 3.7x slower than the field

abi_payload_cost_leaf_scalar_payload (9.54 us) is 3.7x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_leaf_scalar_payload shows alternating (throttle bounce) (autocorr -0.73)

abi_payload_cost_leaf_scalar_payload's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.58 us) to slowest abi_payload_cost_leaf_scalar_payload (9.54 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2577.2 ns median (-73.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.70x (fastest 2577.2 ns, slowest 9536.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4865ns | 4897ns | 4612ns | 4877ns | 4974ns | -58.61% |
| abi_payload_cost_leaf_scalar_payload | 11754ns | 11818ns | 11536ns | 11744ns | 11879ns | base |
| abi_payload_cost_leaf_soa_payload | 4941ns | 4944ns | 4894ns | 4941ns | 4965ns | -57.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2578ns | 2462ns | 2648ns | -72.78% | 0.000 |
| abi_payload_cost_leaf_scalar_payload | 9470ns | 9265ns | 9579ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 2614ns | 2515ns | 2649ns | -72.40% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 20633.2 | 2725.4 | 2578.0 | n/a |
| abi_payload_cost_leaf_scalar_payload | 21477.9 | 9628.2 | 9469.6 | n/a |
| abi_payload_cost_leaf_soa_payload | 20981.6 | 2977.1 | 2613.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.000 | 95.5% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 25.8% |
| abi_payload_cost_leaf_soa_payload | 0.000 | 93.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4865ns | 4865ns | -58.61% |
| abi_payload_cost_leaf_scalar_payload | 11754ns | 11754ns | base |
| abi_payload_cost_leaf_soa_payload | 4941ns | 4941ns | -57.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 9537ns | base | --- | [9293, 9579] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2577ns | -6920.9ns (-72.6%) | [-7068, -6686]ns | [2509, 2648] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 2637ns | -6900.0ns (-72.4%) | [-6959, -6709]ns | [2555, 2649] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 9564ns | -74.3% | -72.9% |
| 2 | 9265ns | -71.3% | -72.9% |
| 3 | 9590ns | -73.3% | -72.5% |
| 4 | 9321ns | -72.5% | -71.5% |
| 5 | 9568ns | -72.9% | -72.5% |
| 6 | 9510ns | -72.2% | -72.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | -0.388 | moderate- |
| abi_payload_cost_leaf_scalar_payload | -0.734 | HIGH- (thermal bounce) |
| abi_payload_cost_leaf_soa_payload | 0.127 | ok |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 107393.2ns | 2578.0ns | 4165.8% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 128547.1ns | 9469.6ns | 1357.5% | HIGH |
| abi_payload_cost_leaf_soa_payload | 104316.2ns | 2613.6ns | 3991.3% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2461.7-2647.7 ns)
   2461.7 |####################
   2471.0 |
   2480.3 |
   2489.6 |
   2498.9 |
   2508.2 |
   2517.5 |
   2526.8 |
   2536.1 |
   2545.4 |
   2554.7 |########################################
   2564.0 |
   2573.3 |
   2582.6 |
   2591.9 |####################
   2601.2 |
   2610.5 |
   2619.8 |
   2629.1 |
   2638.4 |####################
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 9265.4-9578.5 ns)
   9265.4 |####################
   9281.1 |
   9296.7 |
   9312.4 |####################
   9328.0 |
   9343.7 |
   9359.3 |
   9375.0 |
   9390.7 |
   9406.3 |
   9422.0 |
   9437.6 |
   9453.3 |
   9468.9 |
   9484.6 |
   9500.3 |####################
   9515.9 |
   9531.6 |
   9547.2 |
   9562.9 |########################################
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 2515.0-2648.6 ns)
   2515.0 |########################################
   2521.7 |
   2528.4 |
   2535.0 |
   2541.7 |
   2548.4 |
   2555.1 |
   2561.7 |
   2568.4 |
   2575.1 |
   2581.8 |
   2588.5 |
   2595.1 |########################################
   2601.8 |
   2608.5 |
   2615.2 |
   2621.8 |
   2628.5 |########################################
   2635.2 |########################################
   2641.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=4175.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=1349.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=3945.4% of algo (FFI overhead may distort results)
