# abi_payload_cost (madd)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_madd_scalar_payload has the worst median (9.50 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_madd_null_entry at 2.56 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_madd_null_entry beats baseline by 73% (significant)

abi_payload_cost_madd_null_entry is -6.90 us (73%) faster than baseline abi_payload_cost_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_madd_scalar_payload is an outlier: 3.7x slower than the field

abi_payload_cost_madd_scalar_payload (9.50 us) is 3.7x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_madd_scalar_payload shows alternating (throttle bounce) (autocorr -0.68)

abi_payload_cost_madd_scalar_payload's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_payload_cost_madd_null_entry (2.56 us) to slowest abi_payload_cost_madd_scalar_payload (9.50 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_madd_null_entry** at 2557.5 ns median (-73.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.72x (fastest 2557.5 ns, slowest 9502.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4891ns | 4906ns | 4619ns | 4884ns | 5036ns | -58.66% |
| abi_payload_cost_madd_scalar_payload | 11831ns | 11767ns | 11658ns | 11731ns | 12068ns | base |
| abi_payload_cost_madd_soa_payload | 4965ns | 4965ns | 4915ns | 4958ns | 4999ns | -58.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 2566ns | 2452ns | 2655ns | -73.09% | 0.000 |
| abi_payload_cost_madd_scalar_payload | 9535ns | 9378ns | 9715ns | base | 0.000 |
| abi_payload_cost_madd_soa_payload | 2630ns | 2609ns | 2652ns | -72.42% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 20055.4 | 2779.1 | 2565.6 | n/a |
| abi_payload_cost_madd_scalar_payload | 20557.0 | 9704.1 | 9534.9 | n/a |
| abi_payload_cost_madd_soa_payload | 20551.3 | 2958.9 | 2630.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_payload_cost_madd_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_madd_null_entry | 0.000 | 95.9% |
| abi_payload_cost_madd_scalar_payload | 0.000 | 25.8% |
| abi_payload_cost_madd_soa_payload | 0.000 | 93.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4891ns | 4891ns | -58.66% |
| abi_payload_cost_madd_scalar_payload | 11831ns | 11831ns | base |
| abi_payload_cost_madd_soa_payload | 4965ns | 4965ns | -58.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_madd_scalar_payload | 9503ns | base | --- | [9387, 9715] | --- | --- | --- | --- |
| abi_payload_cost_madd_null_entry | 2558ns | -6897.7ns (-72.6%) | [-7219, -6791]ns | [2484, 2655] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_madd_soa_payload | 2627ns | -6858.9ns (-72.2%) | [-7093, -6762]ns | [2611, 2652] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_madd_scalar_payload | abi_payload_cost_madd_null_entry | abi_payload_cost_madd_soa_payload |
|---|---|---|---|
| 1 | 9709ns | -74.7% | -73.1% |
| 2 | 9378ns | -73.2% | -72.1% |
| 3 | 9570ns | -72.4% | -72.1% |
| 4 | 9396ns | -72.6% | -71.9% |
| 5 | 9720ns | -73.9% | -72.9% |
| 6 | 9435ns | -71.7% | -72.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_madd_null_entry | -0.012 | ok |
| abi_payload_cost_madd_scalar_payload | -0.685 | HIGH- (thermal bounce) |
| abi_payload_cost_madd_soa_payload | -0.018 | ok |

**Consistency summary:**

- **abi_payload_cost_madd_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 106506.7ns | 2565.6ns | 4151.3% | HIGH |
| abi_payload_cost_madd_scalar_payload | 127985.5ns | 9534.9ns | 1342.3% | HIGH |
| abi_payload_cost_madd_soa_payload | 103072.7ns | 2630.0ns | 3919.1% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_madd_null_entry (n=6, range 2452.5-2655.2 ns)
   2452.5 |########################################
   2462.6 |
   2472.8 |
   2482.9 |
   2493.0 |
   2503.2 |
   2513.3 |########################################
   2523.4 |
   2533.6 |########################################
   2543.7 |
   2553.8 |
   2564.0 |
   2574.1 |########################################
   2584.3 |
   2594.4 |
   2604.5 |
   2614.7 |
   2624.8 |
   2634.9 |########################################
   2645.1 |
  (0 below, 1 above range)

abi_payload_cost_madd_scalar_payload (n=6, range 9377.9-9714.8 ns)
   9377.9 |########################################
   9394.7 |########################################
   9411.6 |
   9428.4 |########################################
   9445.3 |
   9462.1 |
   9479.0 |
   9495.8 |
   9512.7 |
   9529.5 |
   9546.3 |
   9563.2 |########################################
   9580.0 |
   9596.9 |
   9613.7 |
   9630.6 |
   9647.4 |
   9664.3 |
   9681.1 |
   9698.0 |########################################
  (0 below, 1 above range)

abi_payload_cost_madd_soa_payload (n=6, range 2609.2-2652.3 ns)
   2609.2 |########################################
   2611.4 |########################################
   2613.5 |
   2615.7 |
   2617.8 |
   2620.0 |########################################
   2622.1 |
   2624.3 |
   2626.4 |
   2628.6 |
   2630.8 |
   2632.9 |########################################
   2635.1 |########################################
   2637.2 |
   2639.4 |
   2641.5 |
   2643.7 |
   2645.8 |
   2648.0 |
   2650.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_madd_null_entry**: bridge=4163.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_scalar_payload**: bridge=1342.7% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_soa_payload**: bridge=3927.1% of algo (FFI overhead may distort results)
