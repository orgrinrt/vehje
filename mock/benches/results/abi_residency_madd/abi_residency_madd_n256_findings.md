# abi_residency (madd)

3 variants, 6 samples per variant.
Baseline: **abi_residency_madd_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_madd_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_madd_null_entry dominates: 83696% faster than the next best (abi_residency_madd_reused_buffer)

abi_residency_madd_null_entry (3.22 us) leads abi_residency_madd_reused_buffer (2.70 ms) by 83696%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_madd_null_entry beats baseline by 100% (significant)

abi_residency_madd_null_entry is -2.69 ms (100%) faster than baseline abi_residency_madd_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_madd_fresh_alloc is an outlier: 839.2x slower than the field

abi_residency_madd_fresh_alloc (2.70 ms) is 839.2x the fastest (3.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 839.2x the fastest

Fastest abi_residency_madd_null_entry (3.22 us) to slowest abi_residency_madd_fresh_alloc (2.70 ms): 839.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_madd_null_entry** at 3217.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 839.24x (fastest 3217.1 ns, slowest 2699922.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2707808ns | 2702523ns | 2684740ns | 2698881ns | 2732732ns | -0.25% |
| abi_residency_madd_null_entry | 5551ns | 5520ns | 5275ns | 5486ns | 5786ns | -99.80% |
| abi_residency_madd_reused_buffer | 2714619ns | 2698622ns | 2683262ns | 2695934ns | 2758325ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2705044ns | 2682072ns | 2729638ns | -0.25% | 0.000 |
| abi_residency_madd_null_entry | 3234ns | 3079ns | 3356ns | -99.88% | 0.079 |
| abi_residency_madd_reused_buffer | 2711831ns | 2680685ns | 2755372ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 48002.5 | 2706935.4 | 2705044.4 | n/a |
| abi_residency_madd_null_entry | 27627.6 | 3276.0 | 3233.8 | n/a |
| abi_residency_madd_reused_buffer | 48869.5 | 2705809.0 | 2711831.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_residency_madd_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_madd_fresh_alloc | 0.000 | 0.1% |
| abi_residency_madd_null_entry | 0.080 | 95.7% |
| abi_residency_madd_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2707808ns | 2707808ns | -0.25% |
| abi_residency_madd_null_entry | 5551ns | 5551ns | -99.80% |
| abi_residency_madd_reused_buffer | 2714619ns | 2714619ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_madd_reused_buffer | 2695812ns | base | --- | [2684309, 2755372] | --- | --- | --- | --- |
| abi_residency_madd_fresh_alloc | 2699923ns | no significant difference | [-69741, +38362]ns | [2685572, 2729638] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_madd_null_entry | 3217ns | -2692600.4ns (-99.9%) | [-2752091, -2681100]ns | [3128, 3356] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_madd_reused_buffer | abi_residency_madd_fresh_alloc | abi_residency_madd_null_entry |
|---|---|---|---|
| 1 | 2694409ns | +0.4% | -99.9% |
| 2 | 2813409ns | -4.4% | -99.9% |
| 3 | 2680685ns | +0.7% | -99.9% |
| 4 | 2687932ns | +0.5% | -99.9% |
| 5 | 2697216ns | -0.6% | -99.9% |
| 6 | 2697335ns | +2.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_madd_fresh_alloc | -0.273 | moderate- |
| abi_residency_madd_null_entry | 0.398 | moderate+ |
| abi_residency_madd_reused_buffer | -0.288 | moderate- |

**Consistency summary:**

- **abi_residency_madd_fresh_alloc**: won 2/6, lost 4/6
- **abi_residency_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 8165768.1ns | 2705044.4ns | 301.9% | HIGH |
| abi_residency_madd_null_entry | 121278.8ns | 3233.8ns | 3750.4% | HIGH |
| abi_residency_madd_reused_buffer | 8344266.9ns | 2711831.0ns | 307.7% | HIGH |

## Distribution (algo ns)

```
abi_residency_madd_fresh_alloc (n=6, range 2682071.7-2729638.3 ns)
  2682071.7 |####################
  2684450.0 |
  2686828.4 |####################
  2689206.7 |
  2691585.0 |
  2693963.4 |
  2696341.7 |
  2698720.0 |########################################
  2701098.3 |
  2703476.7 |####################
  2705855.0 |
  2708233.3 |
  2710611.7 |
  2712990.0 |
  2715368.3 |
  2717746.6 |
  2720125.0 |
  2722503.3 |
  2724881.6 |
  2727260.0 |
  (0 below, 1 above range)

abi_residency_madd_null_entry (n=6, range 3079.2-3356.1 ns)
   3079.2 |########################################
   3093.0 |
   3106.9 |
   3120.7 |
   3134.6 |
   3148.4 |
   3162.3 |
   3176.1 |########################################
   3189.9 |########################################
   3203.8 |
   3217.6 |
   3231.5 |########################################
   3245.3 |
   3259.2 |
   3273.0 |
   3286.8 |
   3300.7 |
   3314.5 |
   3328.4 |
   3342.2 |########################################
  (0 below, 1 above range)

abi_residency_madd_reused_buffer (n=6, range 2680685.4-2755371.9 ns)
  2680685.4 |####################
  2684419.7 |####################
  2688154.0 |
  2691888.4 |####################
  2695622.7 |########################################
  2699357.0 |
  2703091.4 |
  2706825.7 |
  2710560.0 |
  2714294.3 |
  2718028.7 |
  2721763.0 |
  2725497.3 |
  2729231.6 |
  2732966.0 |
  2736700.3 |
  2740434.6 |
  2744168.9 |
  2747903.3 |
  2751637.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_madd_fresh_alloc**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_residency_madd_null_entry**: bridge=3760.9% of algo (FFI overhead may distort results)
- **abi_residency_madd_reused_buffer**: bridge=301.8% of algo (FFI overhead may distort results)
