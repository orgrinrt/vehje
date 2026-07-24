# abi_payload_cost (tight)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_tight_scalar_payload has the worst median (27.15 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_tight_null_entry at 2.65 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_tight_null_entry dominates: 149% faster than the next best (abi_payload_cost_tight_soa_payload)

abi_payload_cost_tight_null_entry (2.65 us) leads abi_payload_cost_tight_soa_payload (6.60 us) by 149%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_tight_null_entry beats baseline by 90% (significant)

abi_payload_cost_tight_null_entry is -24.56 us (90%) faster than baseline abi_payload_cost_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_tight_scalar_payload is an outlier: 10.3x slower than the field

abi_payload_cost_tight_scalar_payload (27.15 us) is 10.3x the fastest (2.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_tight_scalar_payload shows alternating (throttle bounce) (autocorr -0.55)

abi_payload_cost_tight_scalar_payload's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.3x the fastest

Fastest abi_payload_cost_tight_null_entry (2.65 us) to slowest abi_payload_cost_tight_scalar_payload (27.15 us): 10.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_tight_null_entry** at 2646.7 ns median (-90.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.26x (fastest 2646.7 ns, slowest 27145.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 4933ns | 4963ns | 4640ns | 4888ns | 5148ns | -83.20% |
| abi_payload_cost_tight_scalar_payload | 29361ns | 29406ns | 28744ns | 29258ns | 29823ns | base |
| abi_payload_cost_tight_soa_payload | 8925ns | 8921ns | 8816ns | 8911ns | 9000ns | -69.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 2615ns | 2475ns | 2711ns | -90.35% | 0.002 |
| abi_payload_cost_tight_scalar_payload | 27087ns | 26512ns | 27505ns | base | 0.000 |
| abi_payload_cost_tight_soa_payload | 6599ns | 6540ns | 6637ns | -75.64% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 20594.4 | 2763.5 | 2615.2 | 0 |
| abi_payload_cost_tight_scalar_payload | 20463.2 | 27290.6 | 27087.1 | n/a |
| abi_payload_cost_tight_soa_payload | 21090.8 | 6696.2 | 6598.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_payload_cost_tight_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_tight_null_entry | 0.002 | 93.5% |
| abi_payload_cost_tight_scalar_payload | 0.000 | 9.1% |
| abi_payload_cost_tight_soa_payload | 0.001 | 37.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_tight_null_entry | 4933ns | 4933ns | -83.20% |
| abi_payload_cost_tight_scalar_payload | 29361ns | 29361ns | base |
| abi_payload_cost_tight_soa_payload | 8925ns | 8925ns | -69.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_tight_scalar_payload | 27145ns | base | --- | [26611, 27505] | --- | --- | --- | --- |
| abi_payload_cost_tight_null_entry | 2647ns | -24558.9ns (-90.5%) | [-24855, -24002]ns | [2488, 2711] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_tight_soa_payload | 6598ns | -20533.3ns (-75.6%) | [-20882, -20050]ns | [6561, 6637] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_tight_scalar_payload | abi_payload_cost_tight_null_entry | abi_payload_cost_tight_soa_payload |
|---|---|---|---|
| 1 | 26850ns | -90.8% | -75.4% |
| 2 | 27486ns | -90.6% | -75.8% |
| 3 | 26512ns | -89.8% | -75.3% |
| 4 | 27525ns | -90.2% | -76.0% |
| 5 | 27441ns | -90.2% | -75.9% |
| 6 | 26710ns | -90.6% | -75.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_tight_null_entry | 0.136 | ok |
| abi_payload_cost_tight_scalar_payload | -0.551 | HIGH- (thermal bounce) |
| abi_payload_cost_tight_soa_payload | -0.442 | moderate- |

**Consistency summary:**

- **abi_payload_cost_tight_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 108062.6ns | 2615.2ns | 4132.1% | HIGH |
| abi_payload_cost_tight_scalar_payload | 181257.6ns | 27087.1ns | 669.2% | HIGH |
| abi_payload_cost_tight_soa_payload | 120685.6ns | 6598.9ns | 1828.9% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_tight_null_entry (n=6, range 2474.6-2711.2 ns)
   2474.6 |########################################
   2486.4 |
   2498.3 |########################################
   2510.1 |
   2521.9 |
   2533.8 |
   2545.6 |
   2557.4 |
   2569.3 |
   2581.1 |
   2592.9 |########################################
   2604.8 |
   2616.6 |
   2628.4 |
   2640.3 |
   2652.1 |
   2663.9 |
   2675.8 |
   2687.6 |########################################
   2699.4 |########################################
  (0 below, 1 above range)

abi_payload_cost_tight_scalar_payload (n=6, range 26512.5-27505.2 ns)
  26512.5 |########################################
  26562.1 |
  26611.8 |
  26661.4 |########################################
  26711.0 |
  26760.7 |
  26810.3 |########################################
  26859.9 |
  26909.6 |
  26959.2 |
  27008.8 |
  27058.5 |
  27108.1 |
  27157.8 |
  27207.4 |
  27257.0 |
  27306.7 |
  27356.3 |
  27405.9 |########################################
  27455.6 |########################################
  (0 below, 1 above range)

abi_payload_cost_tight_soa_payload (n=6, range 6540.0-6636.9 ns)
   6540.0 |########################################
   6544.8 |
   6549.7 |
   6554.5 |
   6559.4 |
   6564.2 |
   6569.1 |
   6573.9 |
   6578.7 |########################################
   6583.6 |
   6588.4 |
   6593.3 |########################################
   6598.1 |########################################
   6603.0 |
   6607.8 |
   6612.6 |
   6617.5 |########################################
   6622.3 |
   6627.2 |
   6632.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_tight_null_entry**: bridge=4103.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_scalar_payload**: bridge=667.4% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_soa_payload**: bridge=1819.2% of algo (FFI overhead may distort results)
