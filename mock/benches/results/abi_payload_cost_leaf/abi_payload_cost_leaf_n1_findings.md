# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (9.50 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry beats baseline by 73% (significant)

abi_payload_cost_leaf_null_entry is -6.93 us (73%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 3.8x slower than the field

abi_payload_cost_leaf_scalar_payload (9.50 us) is 3.8x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.8x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.53 us) to slowest abi_payload_cost_leaf_scalar_payload (9.50 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2527.5 ns median (-73.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.76x (fastest 2527.5 ns, slowest 9497.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4798ns | 4780ns | 4640ns | 4739ns | 4966ns | -59.29% |
| abi_payload_cost_leaf_scalar_payload | 11785ns | 11789ns | 11492ns | 11736ns | 12005ns | base |
| abi_payload_cost_leaf_soa_payload | 4861ns | 4865ns | 4791ns | 4841ns | 4926ns | -58.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2534ns | 2439ns | 2629ns | -73.26% | 0.000 |
| abi_payload_cost_leaf_scalar_payload | 9477ns | 9183ns | 9690ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 2582ns | 2538ns | 2614ns | -72.76% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 19417.2 | 2725.3 | 2534.3 | n/a |
| abi_payload_cost_leaf_scalar_payload | 20591.2 | 9619.0 | 9477.5 | n/a |
| abi_payload_cost_leaf_soa_payload | 19750.2 | 2921.3 | 2581.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.000 | 96.5% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 25.7% |
| abi_payload_cost_leaf_soa_payload | 0.000 | 94.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4798ns | 4798ns | -59.29% |
| abi_payload_cost_leaf_scalar_payload | 11785ns | 11785ns | base |
| abi_payload_cost_leaf_soa_payload | 4861ns | 4861ns | -58.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 9497ns | base | --- | [9245, 9690] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2528ns | -6926.6ns (-72.9%) | [-7146, -6756]ns | [2446, 2629] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 2588ns | -6916.6ns (-72.8%) | [-7113, -6657]ns | [2543, 2614] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 9678ns | -74.8% | -73.7% |
| 2 | 9307ns | -73.6% | -72.4% |
| 3 | 9588ns | -72.8% | -72.6% |
| 4 | 9183ns | -72.5% | -71.6% |
| 5 | 9406ns | -73.1% | -73.0% |
| 6 | 9702ns | -72.7% | -73.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.014 | ok |
| abi_payload_cost_leaf_scalar_payload | -0.360 | moderate- |
| abi_payload_cost_leaf_soa_payload | -0.198 | ok |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 105735.3ns | 2534.3ns | 4172.1% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 128626.5ns | 9477.5ns | 1357.2% | HIGH |
| abi_payload_cost_leaf_soa_payload | 103008.8ns | 2581.8ns | 3989.7% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2439.2-2628.9 ns)
   2439.2 |########################################
   2448.7 |########################################
   2458.2 |
   2467.7 |
   2477.1 |
   2486.6 |
   2496.1 |
   2505.6 |
   2515.1 |########################################
   2524.6 |########################################
   2534.1 |
   2543.6 |
   2553.0 |
   2562.5 |
   2572.0 |
   2581.5 |
   2591.0 |
   2600.5 |########################################
   2610.0 |
   2619.5 |
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 9183.3-9690.2 ns)
   9183.3 |########################################
   9208.6 |
   9234.0 |
   9259.3 |
   9284.7 |########################################
   9310.0 |
   9335.4 |
   9360.7 |
   9386.1 |########################################
   9411.4 |
   9436.8 |
   9462.1 |
   9487.4 |
   9512.8 |
   9538.1 |
   9563.5 |########################################
   9588.8 |
   9614.2 |
   9639.5 |
   9664.9 |########################################
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 2538.3-2614.3 ns)
   2538.3 |####################
   2542.1 |
   2545.9 |####################
   2549.7 |
   2553.5 |
   2557.3 |
   2561.1 |
   2564.9 |
   2568.7 |####################
   2572.5 |
   2576.3 |
   2580.1 |
   2583.9 |
   2587.7 |
   2591.5 |
   2595.3 |
   2599.1 |
   2602.9 |########################################
   2606.7 |
   2610.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=4195.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=1350.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=3998.1% of algo (FFI overhead may distort results)
