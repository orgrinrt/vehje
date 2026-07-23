# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (26.77 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.63 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_null_entry dominates: 147% faster than the next best (abi_payload_cost_real_soa_payload)

abi_payload_cost_real_null_entry (2.63 us) leads abi_payload_cost_real_soa_payload (6.51 us) by 147%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_real_null_entry beats baseline by 90% (significant)

abi_payload_cost_real_null_entry is -24.07 us (90%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 10.2x slower than the field

abi_payload_cost_real_scalar_payload (26.77 us) is 10.2x the fastest (2.63 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 10.2x the fastest

Fastest abi_payload_cost_real_null_entry (2.63 us) to slowest abi_payload_cost_real_scalar_payload (26.77 us): 10.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2633.8 ns median (-90.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.16x (fastest 2633.8 ns, slowest 26767.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4920ns | 4930ns | 4663ns | 4870ns | 5124ns | -83.08% |
| abi_payload_cost_real_scalar_payload | 29085ns | 29104ns | 28582ns | 28963ns | 29520ns | base |
| abi_payload_cost_real_soa_payload | 8889ns | 8830ns | 8751ns | 8813ns | 9072ns | -69.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2610ns | 2469ns | 2699ns | -90.26% | 0.002 |
| abi_payload_cost_real_scalar_payload | 26800ns | 26387ns | 27241ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 6535ns | 6439ns | 6642ns | -75.62% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 19922.1 | 2743.3 | 2610.1 | n/a |
| abi_payload_cost_real_scalar_payload | 20142.9 | 26935.8 | 26799.9 | n/a |
| abi_payload_cost_real_soa_payload | 20844.7 | 6651.1 | 6534.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.002 | 93.8% |
| abi_payload_cost_real_scalar_payload | 0.000 | 9.2% |
| abi_payload_cost_real_soa_payload | 0.001 | 37.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4920ns | 4920ns | -83.08% |
| abi_payload_cost_real_scalar_payload | 29085ns | 29085ns | base |
| abi_payload_cost_real_soa_payload | 8889ns | 8889ns | -69.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 26767ns | base | --- | [26391, 27241] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2634ns | -24068.1ns (-89.9%) | [-24608, -23894]ns | [2498, 2699] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 6507ns | -20259.6ns (-75.7%) | [-20625, -19912]ns | [6454, 6642] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 26395ns | -90.6% | -75.3% |
| 2 | 27460ns | -90.3% | -75.5% |
| 3 | 27023ns | -90.4% | -76.0% |
| 4 | 26791ns | -89.9% | -75.9% |
| 5 | 26743ns | -90.0% | -75.5% |
| 6 | 26387ns | -90.4% | -75.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | -0.173 | ok |
| abi_payload_cost_real_scalar_payload | -0.119 | ok |
| abi_payload_cost_real_soa_payload | -0.181 | ok |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 107936.0ns | 2610.1ns | 4135.3% | HIGH |
| abi_payload_cost_real_scalar_payload | 180376.1ns | 26799.9ns | 673.0% | HIGH |
| abi_payload_cost_real_soa_payload | 122223.2ns | 6534.5ns | 1870.4% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2469.2-2698.9 ns)
   2469.2 |########################################
   2480.7 |
   2492.2 |
   2503.7 |
   2515.1 |########################################
   2526.6 |
   2538.1 |
   2549.6 |
   2561.1 |
   2572.6 |
   2584.1 |
   2595.6 |########################################
   2607.0 |
   2618.5 |
   2630.0 |
   2641.5 |
   2653.0 |
   2664.5 |########################################
   2676.0 |########################################
   2687.5 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 26387.1-27241.2 ns)
  26387.1 |########################################
  26429.8 |
  26472.5 |
  26515.2 |
  26557.9 |
  26600.6 |
  26643.3 |
  26686.1 |
  26728.8 |####################
  26771.5 |####################
  26814.2 |
  26856.9 |
  26899.6 |
  26942.3 |
  26985.0 |####################
  27027.7 |
  27070.4 |
  27113.1 |
  27155.8 |
  27198.5 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 6439.2-6642.1 ns)
   6439.2 |########################################
   6449.3 |
   6459.5 |########################################
   6469.6 |
   6479.8 |
   6489.9 |########################################
   6500.1 |
   6510.2 |########################################
   6520.3 |
   6530.5 |
   6540.6 |########################################
   6550.8 |
   6560.9 |
   6571.1 |
   6581.2 |
   6591.3 |
   6601.5 |
   6611.6 |
   6621.8 |
   6631.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=4119.8% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=675.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=1877.4% of algo (FFI overhead may distort results)
