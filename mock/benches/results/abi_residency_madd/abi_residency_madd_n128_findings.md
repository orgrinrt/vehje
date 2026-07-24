# abi_residency (madd)

3 variants, 6 samples per variant.
Baseline: **abi_residency_madd_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_madd_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_residency_madd_reused_buffer) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_residency_madd_reused_buffer has the worst median (2.69 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_residency_madd_null_entry at 2.72 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_residency_madd_null_entry dominates: 98684% faster than the next best (abi_residency_madd_fresh_alloc)

abi_residency_madd_null_entry (2.72 us) leads abi_residency_madd_fresh_alloc (2.69 ms) by 98684%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_madd_null_entry beats baseline by 100% (significant)

abi_residency_madd_null_entry is -2.69 ms (100%) faster than baseline abi_residency_madd_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_madd_reused_buffer is an outlier: 988.9x slower than the field

abi_residency_madd_reused_buffer (2.69 ms) is 988.9x the fastest (2.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_madd_fresh_alloc shows alternating (throttle bounce) (autocorr -0.51)

abi_residency_madd_fresh_alloc's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 988.9x the fastest

Fastest abi_residency_madd_null_entry (2.72 us) to slowest abi_residency_madd_reused_buffer (2.69 ms): 988.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_madd_null_entry** at 2720.0 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 988.94x (fastest 2720.0 ns, slowest 2689912.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2688081ns | 2689583ns | 2679552ns | 2686536ns | 2694662ns | -0.16% |
| abi_residency_madd_null_entry | 5032ns | 5009ns | 4853ns | 4968ns | 5216ns | -99.81% |
| abi_residency_madd_reused_buffer | 2692455ns | 2692625ns | 2680865ns | 2689104ns | 2703277ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2685468ns | 2676896ns | 2692162ns | -0.16% | 0.000 |
| abi_residency_madd_null_entry | 2737ns | 2655ns | 2836ns | -99.90% | 0.047 |
| abi_residency_madd_reused_buffer | 2689706ns | 2678318ns | 2700435ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 44041.9 | 2689701.1 | 2685467.6 | 0 |
| abi_residency_madd_null_entry | 27170.5 | 2780.5 | 2737.4 | n/a |
| abi_residency_madd_reused_buffer | 43134.5 | 2688938.3 | 2689705.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_residency_madd_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_madd_fresh_alloc | 0.000 | 0.1% |
| abi_residency_madd_null_entry | 0.047 | 97.6% |
| abi_residency_madd_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2688081ns | 2688081ns | -0.16% |
| abi_residency_madd_null_entry | 5032ns | 5032ns | -99.81% |
| abi_residency_madd_reused_buffer | 2692455ns | 2692455ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_madd_reused_buffer | 2689913ns | base | --- | [2678770, 2700435] | --- | --- | --- | --- |
| abi_residency_madd_fresh_alloc | 2686936ns | no significant difference | [-16643, +9578]ns | [2677305, 2692162] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_madd_null_entry | 2720ns | -2687187.2ns (-99.9%) | [-2697773, -2675945]ns | [2656, 2836] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_madd_reused_buffer | abi_residency_madd_fresh_alloc | abi_residency_madd_null_entry |
|---|---|---|---|
| 1 | 2679222ns | +0.4% | -99.9% |
| 2 | 2689897ns | -0.5% | -99.9% |
| 3 | 2702870ns | -0.3% | -99.9% |
| 4 | 2689929ns | -0.1% | -99.9% |
| 5 | 2678318ns | +0.4% | -99.9% |
| 6 | 2698000ns | -0.8% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_madd_fresh_alloc | -0.506 | HIGH- (thermal bounce) |
| abi_residency_madd_null_entry | -0.230 | moderate- |
| abi_residency_madd_reused_buffer | -0.194 | ok |

**Consistency summary:**

- **abi_residency_madd_fresh_alloc**: won 4/6, lost 2/6
- **abi_residency_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 8112349.9ns | 2685467.6ns | 302.1% | HIGH |
| abi_residency_madd_null_entry | 119273.1ns | 2737.4ns | 4357.1% | HIGH |
| abi_residency_madd_reused_buffer | 8101633.6ns | 2689705.9ns | 301.2% | HIGH |

## Distribution (algo ns)

```
abi_residency_madd_fresh_alloc (n=6, range 2676895.8-2692162.5 ns)
  2676895.8 |########################################
  2677659.1 |########################################
  2678422.5 |
  2679185.8 |
  2679949.1 |
  2680712.5 |
  2681475.8 |
  2682239.1 |
  2683002.5 |
  2683765.8 |
  2684529.1 |
  2685292.5 |########################################
  2686055.8 |
  2686819.2 |
  2687582.5 |########################################
  2688345.8 |########################################
  2689109.2 |
  2689872.5 |
  2690635.8 |
  2691399.2 |
  (0 below, 1 above range)

abi_residency_madd_null_entry (n=6, range 2654.6-2835.9 ns)
   2654.6 |########################################
   2663.7 |####################
   2672.7 |
   2681.8 |
   2690.8 |
   2699.9 |
   2709.0 |
   2718.0 |
   2727.1 |
   2736.2 |
   2745.2 |
   2754.3 |
   2763.4 |####################
   2772.4 |
   2781.5 |
   2790.5 |####################
   2799.6 |
   2808.7 |
   2817.7 |
   2826.8 |
  (0 below, 1 above range)

abi_residency_madd_reused_buffer (n=6, range 2678318.3-2700434.8 ns)
  2678318.3 |########################################
  2679424.1 |
  2680529.9 |
  2681635.8 |
  2682741.6 |
  2683847.4 |
  2684953.2 |
  2686059.1 |
  2687164.9 |
  2688270.7 |
  2689376.5 |########################################
  2690482.4 |
  2691588.2 |
  2692694.0 |
  2693799.8 |
  2694905.7 |
  2696011.5 |
  2697117.3 |####################
  2698223.1 |
  2699329.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_madd_fresh_alloc**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_residency_madd_null_entry**: bridge=4382.2% of algo (FFI overhead may distort results)
- **abi_residency_madd_reused_buffer**: bridge=301.2% of algo (FFI overhead may distort results)
