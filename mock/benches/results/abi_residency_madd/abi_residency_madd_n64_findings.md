# abi_residency (madd)

3 variants, 6 samples per variant.
Baseline: **abi_residency_madd_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_madd_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_madd_null_entry dominates: 104113% faster than the next best (abi_residency_madd_reused_buffer)

abi_residency_madd_null_entry (2.57 us) leads abi_residency_madd_reused_buffer (2.68 ms) by 104113%, a clear separation rather than a photo finish. CV 6.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_madd_null_entry beats baseline by 100% (significant)

abi_residency_madd_null_entry is -2.68 ms (100%) faster than baseline abi_residency_madd_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_madd_fresh_alloc is an outlier: 1042.6x slower than the field

abi_residency_madd_fresh_alloc (2.68 ms) is 1042.6x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_madd_null_entry is fastest but the noisiest (CV 6.1%)

abi_residency_madd_null_entry wins on median (2.57 us) yet has the highest variance (CV 6.1%), while abi_residency_madd_fresh_alloc is the steadiest (CV 0.3%, 2.68 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_residency_madd_fresh_alloc shows alternating (throttle bounce) (autocorr -0.55)

abi_residency_madd_fresh_alloc's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 1042.6x the fastest

Fastest abi_residency_madd_null_entry (2.57 us) to slowest abi_residency_madd_fresh_alloc (2.68 ms): 1042.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_madd_null_entry** at 2574.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1042.63x (fastest 2574.2 ns, slowest 2683948.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2687928ns | 2686620ns | 2678484ns | 2683929ns | 2698650ns | -0.53% |
| abi_residency_madd_null_entry | 4942ns | 4935ns | 4583ns | 4821ns | 5305ns | -99.82% |
| abi_residency_madd_reused_buffer | 2702362ns | 2685272ns | 2665920ns | 2683175ns | 2749363ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2685263ns | 2675690ns | 2696094ns | -0.53% | 0.000 |
| abi_residency_madd_null_entry | 2572ns | 2385ns | 2746ns | -99.90% | 0.025 |
| abi_residency_madd_reused_buffer | 2699565ns | 2663168ns | 2746247ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 42464.6 | 2687479.3 | 2685262.9 | n/a |
| abi_residency_madd_null_entry | 27999.2 | 2830.7 | 2572.2 | n/a |
| abi_residency_madd_reused_buffer | 47780.4 | 2694162.0 | 2699565.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_residency_madd_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_madd_fresh_alloc | 0.000 | 0.1% |
| abi_residency_madd_null_entry | 0.025 | 92.7% |
| abi_residency_madd_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2687928ns | 2687928ns | -0.53% |
| abi_residency_madd_null_entry | 4942ns | 4942ns | -99.82% |
| abi_residency_madd_reused_buffer | 2702362ns | 2702362ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_madd_reused_buffer | 2682638ns | base | --- | [2669810, 2746247] | --- | --- | --- | --- |
| abi_residency_madd_fresh_alloc | 2683949ns | no significant difference | [-57835, +12757]ns | [2675746, 2696094] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_madd_null_entry | 2574ns | -2680054.8ns (-99.9%) | [-2743622, -2667301]ns | [2396, 2746] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_madd_reused_buffer | abi_residency_madd_fresh_alloc | abi_residency_madd_null_entry |
|---|---|---|---|
| 1 | 2663168ns | +0.5% | -99.9% |
| 2 | 2676452ns | +0.4% | -99.9% |
| 3 | 2678175ns | +0.5% | -99.9% |
| 4 | 2688646ns | -0.5% | -99.9% |
| 5 | 2803848ns | -3.7% | -99.9% |
| 6 | 2687102ns | -0.2% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_madd_fresh_alloc | -0.555 | HIGH- (thermal bounce) |
| abi_residency_madd_null_entry | -0.194 | ok |
| abi_residency_madd_reused_buffer | -0.065 | ok |

**Consistency summary:**

- **abi_residency_madd_fresh_alloc**: won 3/6, lost 3/6
- **abi_residency_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 8105600.7ns | 2685262.9ns | 301.9% | HIGH |
| abi_residency_madd_null_entry | 113843.3ns | 2572.2ns | 4425.9% | HIGH |
| abi_residency_madd_reused_buffer | 8136967.3ns | 2699565.1ns | 301.4% | HIGH |

## Distribution (algo ns)

```
abi_residency_madd_fresh_alloc (n=6, range 2675690.4-2696093.8 ns)
  2675690.4 |########################################
  2676710.6 |
  2677730.7 |
  2678750.9 |
  2679771.1 |
  2680791.2 |####################
  2681811.4 |
  2682831.6 |
  2683851.7 |
  2684871.9 |
  2685892.1 |####################
  2686912.2 |
  2687932.4 |
  2688952.6 |
  2689972.7 |
  2690992.9 |####################
  2692013.1 |
  2693033.2 |
  2694053.4 |
  2695073.6 |
  (0 below, 1 above range)

abi_residency_madd_null_entry (n=6, range 2385.4-2746.2 ns)
   2385.4 |########################################
   2403.4 |########################################
   2421.5 |
   2439.5 |
   2457.6 |
   2475.6 |
   2493.7 |
   2511.7 |########################################
   2529.7 |
   2547.8 |
   2565.8 |
   2583.9 |
   2601.9 |
   2620.0 |########################################
   2638.0 |########################################
   2656.0 |
   2674.1 |
   2692.1 |
   2710.2 |
   2728.2 |
  (0 below, 1 above range)

abi_residency_madd_reused_buffer (n=6, range 2663168.3-2746247.0 ns)
  2663168.3 |####################
  2667322.2 |
  2671476.2 |
  2675630.1 |########################################
  2679784.0 |
  2683938.0 |####################
  2688091.9 |####################
  2692245.9 |
  2696399.8 |
  2700553.7 |
  2704707.7 |
  2708861.6 |
  2713015.5 |
  2717169.5 |
  2721323.4 |
  2725477.4 |
  2729631.3 |
  2733785.2 |
  2737939.2 |
  2742093.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_madd_fresh_alloc**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_residency_madd_null_entry**: bridge=4411.7% of algo (FFI overhead may distort results)
- **abi_residency_madd_reused_buffer**: bridge=300.7% of algo (FFI overhead may distort results)
