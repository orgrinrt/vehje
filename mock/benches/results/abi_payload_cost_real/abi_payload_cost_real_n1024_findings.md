# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (9.04 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_null_entry dominates: 143020% faster than the next best (abi_payload_cost_real_soa_payload)

abi_payload_cost_real_null_entry (2.49 us) leads abi_payload_cost_real_soa_payload (3.56 ms) by 143020%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_real_null_entry beats baseline by 100% (significant)

abi_payload_cost_real_null_entry is -9.04 ms (100%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 3632.9x slower than the field

abi_payload_cost_real_scalar_payload (9.04 ms) is 3632.9x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3632.9x the fastest

Fastest abi_payload_cost_real_null_entry (2.49 us) to slowest abi_payload_cost_real_scalar_payload (9.04 ms): 3632.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2488.1 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3632.93x (fastest 2488.1 ns, slowest 9039096.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4820ns | 4809ns | 4678ns | 4782ns | 4949ns | -99.95% |
| abi_payload_cost_real_scalar_payload | 9069295ns | 9042726ns | 9012636ns | 9036152ns | 9147339ns | base |
| abi_payload_cost_real_soa_payload | 3567021ns | 3563988ns | 3558712ns | 3562917ns | 3577330ns | -60.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2498ns | 2450ns | 2551ns | -99.97% | 0.410 |
| abi_payload_cost_real_scalar_payload | 9065649ns | 9009076ns | 9143522ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 3563942ns | 3555843ns | 3574184ns | -60.69% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 51124.5 | 2715.0 | 2497.9 | n/a |
| abi_payload_cost_real_scalar_payload | 120032.7 | 9081828.5 | 9065649.3 | n/a |
| abi_payload_cost_real_soa_payload | 90278.1 | 3564409.6 | 3563942.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.418 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.412 | 98.5% |
| abi_payload_cost_real_scalar_payload | 0.000 | 0.0% |
| abi_payload_cost_real_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4820ns | 4820ns | -99.95% |
| abi_payload_cost_real_scalar_payload | 9069295ns | 9069295ns | base |
| abi_payload_cost_real_soa_payload | 3567021ns | 3567021ns | -60.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 9039096ns | base | --- | [9014330, 9143522] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2488ns | -9036564.6ns (-100.0%) | [-9141014, -9011875]ns | [2454, 2551] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 3560978ns | -5473339.2ns (-60.6%) | [-5586858, -5444925]ns | [3556664, 3574184] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 9009076ns | -100.0% | -60.5% |
| 2 | 9242033ns | -100.0% | -61.5% |
| 3 | 9036819ns | -100.0% | -60.6% |
| 4 | 9041374ns | -100.0% | -60.5% |
| 5 | 9019583ns | -100.0% | -60.3% |
| 6 | 9045010ns | -100.0% | -60.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | -0.180 | ok |
| abi_payload_cost_real_scalar_payload | -0.321 | moderate- |
| abi_payload_cost_real_soa_payload | 0.005 | ok |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 136487.8ns | 2497.9ns | 5464.0% | HIGH |
| abi_payload_cost_real_scalar_payload | 27390387.4ns | 9065649.3ns | 302.1% | HIGH |
| abi_payload_cost_real_soa_payload | 10793550.4ns | 3563942.1ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2450.0-2551.3 ns)
   2450.0 |########################################
   2455.1 |########################################
   2460.1 |
   2465.2 |
   2470.3 |
   2475.3 |
   2480.4 |########################################
   2485.5 |
   2490.5 |########################################
   2495.6 |
   2500.7 |
   2505.7 |
   2510.8 |
   2515.8 |
   2520.9 |
   2526.0 |
   2531.0 |########################################
   2536.1 |
   2541.2 |
   2546.2 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 9009076.2-9143521.9 ns)
  9009076.2 |####################
  9015798.5 |####################
  9022520.8 |
  9029243.0 |
  9035965.3 |########################################
  9042687.6 |####################
  9049409.9 |
  9056132.2 |
  9062854.5 |
  9069576.7 |
  9076299.0 |
  9083021.3 |
  9089743.6 |
  9096465.9 |
  9103188.2 |
  9109910.4 |
  9116632.7 |
  9123355.0 |
  9130077.3 |
  9136799.6 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 3555842.9-3574184.2 ns)
  3555842.9 |########################################
  3556760.0 |########################################
  3557677.0 |
  3558594.1 |
  3559511.2 |########################################
  3560428.2 |
  3561345.3 |########################################
  3562262.4 |
  3563179.4 |
  3564096.5 |
  3565013.5 |
  3565930.6 |
  3566847.7 |
  3567764.7 |
  3568681.8 |
  3569598.9 |
  3570515.9 |
  3571433.0 |########################################
  3572350.1 |
  3573267.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=5475.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=302.9% of algo (FFI overhead may distort results)
