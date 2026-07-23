# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (9.69 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_soa_payload beats baseline by 73% (significant)

abi_payload_cost_real_soa_payload is -7.07 us (73%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 3.8x slower than the field

abi_payload_cost_real_scalar_payload (9.69 us) is 3.8x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_real_soa_payload shows alternating (throttle bounce) (autocorr -0.79)

abi_payload_cost_real_soa_payload's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.8x the fastest

Fastest abi_payload_cost_real_null_entry (2.58 us) to slowest abi_payload_cost_real_scalar_payload (9.69 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader abi_payload_cost_real_null_entry vs stability leader abi_payload_cost_real_soa_payload (+1% speed for 2.4x steadier)

abi_payload_cost_real_null_entry is fastest (2.58 us, CV 3.4%); abi_payload_cost_real_soa_payload gives up 1.3% median for 2.4x lower variance (CV 1.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2579.6 ns median (-73.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.76x (fastest 2579.6 ns, slowest 9691.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4872ns | 4938ns | 4650ns | 4854ns | 5011ns | -59.20% |
| abi_payload_cost_real_scalar_payload | 11940ns | 12112ns | 11538ns | 11939ns | 12142ns | base |
| abi_payload_cost_real_soa_payload | 4924ns | 4938ns | 4824ns | 4908ns | 4999ns | -58.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2557ns | 2436ns | 2644ns | -73.30% | 0.000 |
| abi_payload_cost_real_scalar_payload | 9579ns | 9211ns | 9773ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 2608ns | 2550ns | 2644ns | -72.78% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 20077.4 | 2760.7 | 2557.4 | n/a |
| abi_payload_cost_real_scalar_payload | 20872.8 | 9747.6 | 9579.2 | n/a |
| abi_payload_cost_real_soa_payload | 19961.3 | 2951.0 | 2607.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.000 | 94.4% |
| abi_payload_cost_real_scalar_payload | 0.000 | 25.1% |
| abi_payload_cost_real_soa_payload | 0.000 | 93.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4872ns | 4872ns | -59.20% |
| abi_payload_cost_real_scalar_payload | 11940ns | 11940ns | base |
| abi_payload_cost_real_soa_payload | 4924ns | 4924ns | -58.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 9691ns | base | --- | [9274, 9773] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2580ns | -7047.1ns (-72.7%) | [-7289, -6729]ns | [2449, 2644] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 2614ns | -7066.7ns (-72.9%) | [-7145, -6703]ns | [2565, 2644] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 9744ns | -75.0% | -73.0% |
| 2 | 9337ns | -71.8% | -72.2% |
| 3 | 9670ns | -72.5% | -72.6% |
| 4 | 9211ns | -73.3% | -72.3% |
| 5 | 9801ns | -74.2% | -73.0% |
| 6 | 9712ns | -72.9% | -73.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | -0.233 | moderate- |
| abi_payload_cost_real_scalar_payload | -0.499 | moderate- |
| abi_payload_cost_real_soa_payload | -0.794 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 106249.8ns | 2557.4ns | 4154.6% | HIGH |
| abi_payload_cost_real_scalar_payload | 129087.3ns | 9579.2ns | 1347.6% | HIGH |
| abi_payload_cost_real_soa_payload | 102662.6ns | 2607.9ns | 3936.6% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2436.2-2643.9 ns)
   2436.2 |####################
   2446.6 |
   2457.0 |####################
   2467.4 |
   2477.8 |
   2488.1 |
   2498.5 |
   2508.9 |
   2519.3 |
   2529.7 |####################
   2540.1 |
   2550.5 |
   2560.8 |
   2571.2 |
   2581.6 |
   2592.0 |
   2602.4 |
   2612.8 |
   2623.2 |########################################
   2633.6 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 9211.2-9772.7 ns)
   9211.2 |########################################
   9239.3 |
   9267.4 |
   9295.4 |
   9323.5 |########################################
   9351.6 |
   9379.7 |
   9407.7 |
   9435.8 |
   9463.9 |
   9492.0 |
   9520.0 |
   9548.1 |
   9576.2 |
   9604.2 |
   9632.3 |
   9660.4 |########################################
   9688.5 |########################################
   9716.6 |########################################
   9744.6 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 2549.6-2644.4 ns)
   2549.6 |########################################
   2554.3 |
   2559.1 |
   2563.8 |
   2568.6 |
   2573.3 |
   2578.0 |########################################
   2582.8 |
   2587.5 |
   2592.2 |########################################
   2597.0 |
   2601.7 |
   2606.5 |
   2611.2 |
   2615.9 |
   2620.7 |
   2625.4 |
   2630.1 |
   2634.9 |########################################
   2639.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=4123.8% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=1329.4% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=3929.9% of algo (FFI overhead may distort results)
