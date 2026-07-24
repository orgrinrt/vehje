# abi_payload_cost (tight)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_tight_scalar_payload has the worst median (9.48 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_tight_null_entry at 2.56 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_tight_soa_payload beats baseline by 73% (significant)

abi_payload_cost_tight_soa_payload is -6.88 us (73%) faster than baseline abi_payload_cost_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_tight_scalar_payload is an outlier: 3.7x slower than the field

abi_payload_cost_tight_scalar_payload (9.48 us) is 3.7x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_payload_cost_tight_null_entry (2.56 us) to slowest abi_payload_cost_tight_scalar_payload (9.48 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_tight_null_entry** at 2560.8 ns median (-73.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.70x (fastest 2560.8 ns, slowest 9478.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 4870ns | 4848ns | 4630ns | 4837ns | 5041ns | -58.75% |
| abi_payload_cost_tight_scalar_payload | 11806ns | 11817ns | 11588ns | 11773ns | 11963ns | base |
| abi_payload_cost_tight_soa_payload | 4922ns | 4916ns | 4819ns | 4889ns | 5022ns | -58.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 2574ns | 2434ns | 2667ns | -72.85% | 0.000 |
| abi_payload_cost_tight_scalar_payload | 9481ns | 9293ns | 9637ns | base | 0.000 |
| abi_payload_cost_tight_soa_payload | 2599ns | 2529ns | 2651ns | -72.59% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 20080.6 | 2754.6 | 2574.2 | n/a |
| abi_payload_cost_tight_scalar_payload | 20712.6 | 9646.1 | 9480.7 | n/a |
| abi_payload_cost_tight_soa_payload | 20577.9 | 2941.9 | 2598.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_payload_cost_tight_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_tight_null_entry | 0.000 | 95.1% |
| abi_payload_cost_tight_scalar_payload | 0.000 | 25.7% |
| abi_payload_cost_tight_soa_payload | 0.000 | 93.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_tight_null_entry | 4870ns | 4870ns | -58.75% |
| abi_payload_cost_tight_scalar_payload | 11806ns | 11806ns | base |
| abi_payload_cost_tight_soa_payload | 4922ns | 4922ns | -58.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_tight_scalar_payload | 9479ns | base | --- | [9326, 9637] | --- | --- | --- | --- |
| abi_payload_cost_tight_null_entry | 2561ns | -6867.4ns (-72.4%) | [-7093, -6760]ns | [2495, 2667] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_tight_soa_payload | 2602ns | -6875.2ns (-72.5%) | [-7035, -6736]ns | [2542, 2651] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_tight_scalar_payload | abi_payload_cost_tight_null_entry | abi_payload_cost_tight_soa_payload |
|---|---|---|---|
| 1 | 9590ns | -74.6% | -72.3% |
| 2 | 9293ns | -72.3% | -72.8% |
| 3 | 9368ns | -72.7% | -72.7% |
| 4 | 9359ns | -72.7% | -71.7% |
| 5 | 9684ns | -71.5% | -72.8% |
| 6 | 9590ns | -73.3% | -73.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_tight_null_entry | -0.112 | ok |
| abi_payload_cost_tight_scalar_payload | 0.094 | ok |
| abi_payload_cost_tight_soa_payload | -0.146 | ok |

**Consistency summary:**

- **abi_payload_cost_tight_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 106559.6ns | 2574.2ns | 4139.6% | HIGH |
| abi_payload_cost_tight_scalar_payload | 128021.4ns | 9480.7ns | 1350.3% | HIGH |
| abi_payload_cost_tight_soa_payload | 103539.0ns | 2598.6ns | 3984.4% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_tight_null_entry (n=6, range 2434.2-2666.6 ns)
   2434.2 |#############
   2445.8 |
   2457.4 |
   2469.1 |
   2480.7 |
   2492.3 |
   2503.9 |
   2515.6 |
   2527.2 |
   2538.8 |
   2550.4 |########################################
   2562.0 |
   2573.7 |#############
   2585.3 |
   2596.9 |
   2608.5 |
   2620.2 |
   2631.8 |
   2643.4 |
   2655.0 |
  (0 below, 1 above range)

abi_payload_cost_tight_scalar_payload (n=6, range 9293.3-9637.3 ns)
   9293.3 |####################
   9310.5 |
   9327.7 |
   9344.9 |####################
   9362.1 |####################
   9379.3 |
   9396.5 |
   9413.7 |
   9430.9 |
   9448.1 |
   9465.3 |
   9482.5 |
   9499.7 |
   9516.9 |
   9534.1 |
   9551.3 |
   9568.5 |
   9585.7 |########################################
   9602.9 |
   9620.1 |
  (0 below, 1 above range)

abi_payload_cost_tight_soa_payload (n=6, range 2529.2-2651.2 ns)
   2529.2 |########################################
   2535.3 |
   2541.4 |
   2547.5 |
   2553.6 |########################################
   2559.7 |
   2565.8 |########################################
   2571.9 |
   2578.0 |
   2584.1 |
   2590.2 |
   2596.3 |
   2602.4 |
   2608.5 |
   2614.6 |
   2620.7 |
   2626.8 |
   2632.9 |########################################
   2639.0 |
   2645.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_tight_null_entry**: bridge=4160.7% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_scalar_payload**: bridge=1340.4% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_soa_payload**: bridge=3966.9% of algo (FFI overhead may distort results)
