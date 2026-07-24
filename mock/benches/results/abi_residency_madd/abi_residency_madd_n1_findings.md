# abi_residency (madd)

3 variants, 6 samples per variant.
Baseline: **abi_residency_madd_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_madd_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_madd_null_entry dominates: 53970% faster than the next best (abi_residency_madd_reused_buffer)

abi_residency_madd_null_entry (4.96 us) leads abi_residency_madd_reused_buffer (2.68 ms) by 53970%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_madd_null_entry beats baseline by 100% (significant)

abi_residency_madd_null_entry is -2.68 ms (100%) faster than baseline abi_residency_madd_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_madd_fresh_alloc is an outlier: 541.2x slower than the field

abi_residency_madd_fresh_alloc (2.69 ms) is 541.2x the fastest (4.96 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_madd_null_entry shows alternating (throttle bounce) (autocorr -0.59)

abi_residency_madd_null_entry's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 541.2x the fastest

Fastest abi_residency_madd_null_entry (4.96 us) to slowest abi_residency_madd_fresh_alloc (2.69 ms): 541.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_madd_null_entry** at 4961.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 541.22x (fastest 4961.9 ns, slowest 2685454.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2697792ns | 2688148ns | 2664849ns | 2683294ns | 2736012ns | +0.15% |
| abi_residency_madd_null_entry | 7220ns | 7238ns | 6788ns | 7197ns | 7472ns | -99.73% |
| abi_residency_madd_reused_buffer | 2693752ns | 2685446ns | 2678547ns | 2684109ns | 2715819ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2695068ns | 2662376ns | 2733100ns | +0.15% | 0.000 |
| abi_residency_madd_null_entry | 4937ns | 4651ns | 5101ns | -99.82% | 0.000 |
| abi_residency_madd_reused_buffer | 2691149ns | 2675960ns | 2713142ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 46051.0 | 2741579.0 | 2695067.7 | n/a |
| abi_residency_madd_null_entry | 26938.9 | 5004.0 | 4936.9 | n/a |
| abi_residency_madd_reused_buffer | 42075.9 | 2693533.7 | 2691148.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_residency_madd_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_madd_fresh_alloc | 0.000 | 0.2% |
| abi_residency_madd_null_entry | 0.000 | 93.7% |
| abi_residency_madd_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2697792ns | 2697792ns | +0.15% |
| abi_residency_madd_null_entry | 7220ns | 7220ns | -99.73% |
| abi_residency_madd_reused_buffer | 2693752ns | 2693752ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_madd_reused_buffer | 2682877ns | base | --- | [2677428, 2713142] | --- | --- | --- | --- |
| abi_residency_madd_fresh_alloc | 2685455ns | no significant difference | [-10780, +21507]ns | [2666648, 2733100] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_madd_null_entry | 4962ns | -2677820.7ns (-99.8%) | [-2708394, -2672421]ns | [4748, 5101] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_madd_reused_buffer | abi_residency_madd_fresh_alloc | abi_residency_madd_null_entry |
|---|---|---|---|
| 1 | 2739242ns | +1.3% | -99.8% |
| 2 | 2686809ns | +0.2% | -99.8% |
| 3 | 2687041ns | -0.1% | -99.8% |
| 4 | 2678945ns | +0.3% | -99.8% |
| 5 | 2675960ns | -0.2% | -99.8% |
| 6 | 2678895ns | -0.6% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_madd_fresh_alloc | 0.104 | ok |
| abi_residency_madd_null_entry | -0.594 | HIGH- (thermal bounce) |
| abi_residency_madd_reused_buffer | 0.080 | ok |

**Consistency summary:**

- **abi_residency_madd_fresh_alloc**: won 3/6, lost 3/6
- **abi_residency_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 8193980.7ns | 2695067.7ns | 304.0% | HIGH |
| abi_residency_madd_null_entry | 124315.0ns | 4936.9ns | 2518.1% | HIGH |
| abi_residency_madd_reused_buffer | 8117367.2ns | 2691148.7ns | 301.6% | HIGH |

## Distribution (algo ns)

```
abi_residency_madd_fresh_alloc (n=6, range 2662376.2-2733100.2 ns)
  2662376.2 |####################
  2665912.4 |
  2669448.6 |####################
  2672984.8 |
  2676521.0 |
  2680057.2 |
  2683593.4 |########################################
  2687129.6 |
  2690665.8 |####################
  2694202.0 |
  2697738.2 |
  2701274.4 |
  2704810.6 |
  2708346.8 |
  2711883.0 |
  2715419.2 |
  2718955.4 |
  2722491.6 |
  2726027.8 |
  2729564.0 |
  (0 below, 1 above range)

abi_residency_madd_null_entry (n=6, range 4651.2-5101.1 ns)
   4651.2 |########################################
   4673.7 |
   4696.2 |
   4718.7 |
   4741.2 |
   4763.7 |
   4786.2 |
   4808.6 |
   4831.1 |########################################
   4853.6 |
   4876.1 |
   4898.6 |########################################
   4921.1 |
   4943.6 |
   4966.1 |
   4988.6 |
   5011.1 |########################################
   5033.6 |
   5056.1 |
   5078.6 |########################################
  (0 below, 1 above range)

abi_residency_madd_reused_buffer (n=6, range 2675959.6-2713141.9 ns)
  2675959.6 |####################
  2677818.7 |########################################
  2679677.8 |
  2681536.9 |
  2683396.1 |
  2685255.2 |########################################
  2687114.3 |
  2688973.4 |
  2690832.5 |
  2692691.6 |
  2694550.7 |
  2696409.8 |
  2698269.0 |
  2700128.1 |
  2701987.2 |
  2703846.3 |
  2705705.4 |
  2707564.5 |
  2709423.6 |
  2711282.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_madd_fresh_alloc**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_residency_madd_null_entry**: bridge=2504.9% of algo (FFI overhead may distort results)
- **abi_residency_madd_reused_buffer**: bridge=301.5% of algo (FFI overhead may distort results)
