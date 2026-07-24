# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (9.45 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.56 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_soa_payload beats baseline by 73% (significant)

abi_payload_cost_real_soa_payload is -6.86 us (73%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 3.7x slower than the field

abi_payload_cost_real_scalar_payload (9.45 us) is 3.7x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_real_scalar_payload shows alternating (throttle bounce) (autocorr -0.50)

abi_payload_cost_real_scalar_payload's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_payload_cost_real_null_entry (2.56 us) to slowest abi_payload_cost_real_scalar_payload (9.45 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2560.4 ns median (-72.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.69x (fastest 2560.4 ns, slowest 9451.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4847ns | 4842ns | 4667ns | 4834ns | 4956ns | -58.43% |
| abi_payload_cost_real_scalar_payload | 11659ns | 11758ns | 11225ns | 11631ns | 11918ns | base |
| abi_payload_cost_real_soa_payload | 4989ns | 4962ns | 4841ns | 4941ns | 5135ns | -57.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2552ns | 2468ns | 2627ns | -72.82% | 0.000 |
| abi_payload_cost_real_scalar_payload | 9390ns | 9021ns | 9614ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 2637ns | 2567ns | 2715ns | -71.91% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 19899.5 | 2716.6 | 2552.0 | 0 |
| abi_payload_cost_real_scalar_payload | 19783.3 | 9538.9 | 9390.4 | n/a |
| abi_payload_cost_real_soa_payload | 20670.1 | 2961.7 | 2637.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.000 | 96.4% |
| abi_payload_cost_real_scalar_payload | 0.000 | 26.1% |
| abi_payload_cost_real_soa_payload | 0.000 | 93.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4847ns | 4847ns | -58.43% |
| abi_payload_cost_real_scalar_payload | 11659ns | 11659ns | base |
| abi_payload_cost_real_soa_payload | 4989ns | 4989ns | -57.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 9451ns | base | --- | [9106, 9614] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2560ns | -6845.4ns (-72.4%) | [-7032, -6638]ns | [2468, 2627] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 2629ns | -6862.7ns (-72.6%) | [-6934, -6462]ns | [2568, 2715] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 9192ns | -73.1% | -72.0% |
| 2 | 9429ns | -71.7% | -72.8% |
| 3 | 9582ns | -73.0% | -71.7% |
| 4 | 9021ns | -72.6% | -69.9% |
| 5 | 9645ns | -73.3% | -72.5% |
| 6 | 9473ns | -73.2% | -72.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | -0.379 | moderate- |
| abi_payload_cost_real_scalar_payload | -0.504 | HIGH- (thermal bounce) |
| abi_payload_cost_real_soa_payload | 0.276 | moderate+ |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 106366.7ns | 2552.0ns | 4168.1% | HIGH |
| abi_payload_cost_real_scalar_payload | 127359.9ns | 9390.4ns | 1356.3% | HIGH |
| abi_payload_cost_real_soa_payload | 104245.3ns | 2637.4ns | 3952.5% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2467.9-2627.3 ns)
   2467.9 |########################################
   2475.9 |
   2483.8 |
   2491.8 |
   2499.8 |
   2507.8 |
   2515.7 |
   2523.7 |
   2531.7 |
   2539.6 |####################
   2547.6 |
   2555.6 |
   2563.5 |
   2571.5 |####################
   2579.5 |####################
   2587.5 |
   2595.4 |
   2603.4 |
   2611.4 |
   2619.3 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 9020.8-9613.5 ns)
   9020.8 |########################################
   9050.4 |
   9080.1 |
   9109.7 |
   9139.3 |
   9169.0 |########################################
   9198.6 |
   9228.3 |
   9257.9 |
   9287.5 |
   9317.2 |
   9346.8 |
   9376.4 |
   9406.1 |########################################
   9435.7 |
   9465.4 |########################################
   9495.0 |
   9524.6 |
   9554.3 |########################################
   9583.9 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 2566.7-2715.2 ns)
   2566.7 |########################################
   2574.1 |
   2581.5 |
   2589.0 |
   2596.4 |
   2603.8 |####################
   2611.2 |
   2618.7 |
   2626.1 |
   2633.5 |
   2640.9 |####################
   2648.4 |
   2655.8 |
   2663.2 |
   2670.6 |
   2678.1 |
   2685.5 |
   2692.9 |
   2700.3 |
   2707.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=4162.8% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=1344.6% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=3982.9% of algo (FFI overhead may distort results)
