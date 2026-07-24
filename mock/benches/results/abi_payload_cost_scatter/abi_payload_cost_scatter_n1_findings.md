# abi_payload_cost (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_scatter_scalar_payload has the worst median (9.53 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_scatter_null_entry at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_scatter_null_entry beats baseline by 73% (significant)

abi_payload_cost_scatter_null_entry is -6.99 us (73%) faster than baseline abi_payload_cost_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_scatter_scalar_payload is an outlier: 3.8x slower than the field

abi_payload_cost_scatter_scalar_payload (9.53 us) is 3.8x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_scatter_scalar_payload shows alternating (throttle bounce) (autocorr -0.52)

abi_payload_cost_scatter_scalar_payload's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.8x the fastest

Fastest abi_payload_cost_scatter_null_entry (2.51 us) to slowest abi_payload_cost_scatter_scalar_payload (9.53 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader abi_payload_cost_scatter_null_entry vs stability leader abi_payload_cost_scatter_soa_payload (+3% speed for 2.2x steadier)

abi_payload_cost_scatter_null_entry is fastest (2.51 us, CV 3.9%); abi_payload_cost_scatter_soa_payload gives up 3.2% median for 2.2x lower variance (CV 1.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: abi_payload_cost_scatter_null_entry** at 2513.5 ns median (-73.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.79x (fastest 2513.5 ns, slowest 9526.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4820ns | 4860ns | 4552ns | 4778ns | 5018ns | -59.08% |
| abi_payload_cost_scatter_scalar_payload | 11780ns | 11850ns | 11315ns | 11764ns | 12038ns | base |
| abi_payload_cost_scatter_soa_payload | 4908ns | 4917ns | 4778ns | 4879ns | 5015ns | -58.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 2540ns | 2434ns | 2663ns | -73.21% | 0.000 |
| abi_payload_cost_scatter_scalar_payload | 9479ns | 9099ns | 9678ns | base | 0.000 |
| abi_payload_cost_scatter_soa_payload | 2590ns | 2525ns | 2636ns | -72.67% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 20473.3 | 2717.1 | 2539.7 | n/a |
| abi_payload_cost_scatter_scalar_payload | 20678.0 | 9685.5 | 9478.8 | n/a |
| abi_payload_cost_scatter_soa_payload | 20184.7 | 2921.6 | 2590.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_payload_cost_scatter_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_scatter_null_entry | 0.000 | 96.8% |
| abi_payload_cost_scatter_scalar_payload | 0.000 | 25.5% |
| abi_payload_cost_scatter_soa_payload | 0.000 | 93.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4820ns | 4820ns | -59.08% |
| abi_payload_cost_scatter_scalar_payload | 11780ns | 11780ns | base |
| abi_payload_cost_scatter_soa_payload | 4908ns | 4908ns | -58.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_scalar_payload | 9527ns | base | --- | [9232, 9678] | --- | --- | --- | --- |
| abi_payload_cost_scatter_null_entry | 2514ns | -6987.3ns (-73.3%) | [-7150, -6680]ns | [2443, 2663] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_scatter_soa_payload | 2593ns | -6900.6ns (-72.4%) | [-7092, -6672]ns | [2542, 2636] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_scatter_scalar_payload | abi_payload_cost_scatter_null_entry | abi_payload_cost_scatter_soa_payload |
|---|---|---|---|
| 1 | 9430ns | -74.2% | -72.1% |
| 2 | 9364ns | -71.7% | -72.7% |
| 3 | 9705ns | -73.6% | -74.0% |
| 4 | 9099ns | -73.1% | -71.9% |
| 5 | 9623ns | -74.4% | -72.7% |
| 6 | 9651ns | -72.3% | -72.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_scatter_null_entry | -0.264 | moderate- |
| abi_payload_cost_scatter_scalar_payload | -0.521 | HIGH- (thermal bounce) |
| abi_payload_cost_scatter_soa_payload | 0.320 | moderate+ |

**Consistency summary:**

- **abi_payload_cost_scatter_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 107085.8ns | 2539.7ns | 4216.5% | HIGH |
| abi_payload_cost_scatter_scalar_payload | 128169.8ns | 9478.8ns | 1352.2% | HIGH |
| abi_payload_cost_scatter_soa_payload | 102502.6ns | 2590.3ns | 3957.2% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_scatter_null_entry (n=6, range 2433.7-2662.9 ns)
   2433.7 |########################################
   2445.2 |########################################
   2456.6 |########################################
   2468.1 |
   2479.5 |
   2491.0 |
   2502.5 |
   2513.9 |
   2525.4 |
   2536.8 |
   2548.3 |
   2559.8 |########################################
   2571.2 |
   2582.7 |
   2594.1 |
   2605.6 |
   2617.1 |
   2628.5 |
   2640.0 |
   2651.4 |########################################
  (0 below, 1 above range)

abi_payload_cost_scatter_scalar_payload (n=6, range 9099.2-9678.1 ns)
   9099.2 |########################################
   9128.1 |
   9157.1 |
   9186.0 |
   9215.0 |
   9243.9 |
   9272.9 |
   9301.8 |
   9330.8 |
   9359.7 |########################################
   9388.7 |
   9417.6 |########################################
   9446.5 |
   9475.5 |
   9504.4 |
   9533.4 |
   9562.3 |
   9591.3 |
   9620.2 |########################################
   9649.2 |########################################
  (0 below, 1 above range)

abi_payload_cost_scatter_soa_payload (n=6, range 2525.4-2636.2 ns)
   2525.4 |####################
   2530.9 |
   2536.5 |
   2542.0 |
   2547.6 |
   2553.1 |####################
   2558.7 |####################
   2564.2 |
   2569.7 |
   2575.3 |
   2580.8 |
   2586.4 |
   2591.9 |
   2597.5 |
   2603.0 |
   2608.5 |
   2614.1 |
   2619.6 |
   2625.2 |########################################
   2630.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_scatter_null_entry**: bridge=4256.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_scalar_payload**: bridge=1337.4% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_soa_payload**: bridge=3947.3% of algo (FFI overhead may distort results)
