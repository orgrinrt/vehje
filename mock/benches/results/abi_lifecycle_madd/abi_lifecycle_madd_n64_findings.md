# abi_lifecycle (madd)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_madd_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_madd_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_madd_null_entry dominates: 104788% faster than the next best (abi_lifecycle_madd_held_handle)

abi_lifecycle_madd_null_entry (2.60 us) leads abi_lifecycle_madd_held_handle (2.73 ms) by 104788%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_madd_null_entry beats baseline by 100% (significant)

abi_lifecycle_madd_null_entry is -2.72 ms (100%) faster than baseline abi_lifecycle_madd_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_madd_fresh_per_batch is an outlier: 1069.8x slower than the field

abi_lifecycle_madd_fresh_per_batch (2.78 ms) is 1069.8x the fastest (2.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_madd_fresh_per_batch shows alternating (throttle bounce) (autocorr -0.69)

abi_lifecycle_madd_fresh_per_batch's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_madd_null_entry} vs {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} (104788% apart)

The field splits into a fast tier {abi_lifecycle_madd_null_entry} and a slow tier {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} with a 104788% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1069.8x the fastest

Fastest abi_lifecycle_madd_null_entry (2.60 us) to slowest abi_lifecycle_madd_fresh_per_batch (2.78 ms): 1069.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_madd_null_entry** at 2598.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1069.75x (fastest 2598.6 ns, slowest 2779802.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2783087ns | 2783143ns | 2771805ns | 2779721ns | 2793776ns | +1.21% |
| abi_lifecycle_madd_fresh_per_column | 2748086ns | 2746557ns | 2734204ns | 2744490ns | 2760421ns | -0.07% |
| abi_lifecycle_madd_held_handle | 2749948ns | 2728768ns | 2715056ns | 2724712ns | 2805248ns | base |
| abi_lifecycle_madd_null_entry | 4913ns | 4976ns | 4676ns | 4890ns | 5065ns | -99.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2779912ns | 2768838ns | 2790520ns | +1.21% | 0.000 |
| abi_lifecycle_madd_fresh_per_column | 2744944ns | 2731234ns | 2757039ns | -0.06% | 0.000 |
| abi_lifecycle_madd_held_handle | 2746662ns | 2711908ns | 2801622ns | base | 0.000 |
| abi_lifecycle_madd_null_entry | 2561ns | 2435ns | 2628ns | -99.91% | 0.025 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 62999.0 | 2786755.6 | 2779911.9 | n/a |
| abi_lifecycle_madd_fresh_per_column | 65120.3 | 2749326.1 | 2744943.8 | 1 |
| abi_lifecycle_madd_held_handle | 62865.3 | 2740451.8 | 2746661.7 | n/a |
| abi_lifecycle_madd_null_entry | 27460.3 | 2802.3 | 2561.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_lifecycle_madd_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_madd_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_madd_held_handle | 0.000 | 0.1% |
| abi_lifecycle_madd_null_entry | 0.025 | 93.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2783087ns | 2783087ns | +1.21% |
| abi_lifecycle_madd_fresh_per_column | 2748086ns | 2748086ns | -0.07% |
| abi_lifecycle_madd_held_handle | 2749948ns | 2749948ns | base |
| abi_lifecycle_madd_null_entry | 4913ns | 4913ns | -99.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_madd_held_handle | 2725562ns | base | --- | [2712801, 2801622] | --- | --- | --- | --- |
| abi_lifecycle_madd_fresh_per_batch | 2779803ns | no significant difference | [-27450, +70988]ns | [2769413, 2790520] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_madd_fresh_per_column | 2743485ns | no significant difference | [-62741, +38527]ns | [2734307, 2757039] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_madd_null_entry | 2599ns | -2723041.0ns (-99.9%) | [-2799088, -2710173]ns | [2456, 2628] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_madd_held_handle | abi_lifecycle_madd_fresh_per_batch | abi_lifecycle_madd_fresh_per_column | abi_lifecycle_madd_null_entry |
|---|---|---|---|---|
| 1 | 2713694ns | +2.1% | +1.1% | -99.9% |
| 2 | 2723126ns | +2.6% | +0.3% | -99.9% |
| 3 | 2870971ns | -3.6% | -4.7% | -99.9% |
| 4 | 2732273ns | +2.1% | +0.9% | -99.9% |
| 5 | 2711908ns | +2.7% | +1.7% | -99.9% |
| 6 | 2727998ns | +1.7% | +0.5% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | -0.693 | HIGH- (thermal bounce) |
| abi_lifecycle_madd_fresh_per_column | 0.251 | moderate+ |
| abi_lifecycle_madd_held_handle | -0.148 | ok |
| abi_lifecycle_madd_null_entry | -0.260 | moderate- |

**Consistency summary:**

- **abi_lifecycle_madd_fresh_per_batch**: won 1/6, lost 5/6
- **abi_lifecycle_madd_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 8416401.1ns | 2779911.9ns | 302.8% | HIGH |
| abi_lifecycle_madd_fresh_per_column | 8313066.6ns | 2744943.8ns | 302.9% | HIGH |
| abi_lifecycle_madd_held_handle | 8294075.8ns | 2746661.7ns | 302.0% | HIGH |
| abi_lifecycle_madd_null_entry | 113540.1ns | 2561.1ns | 4433.3% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_madd_fresh_per_batch (n=6, range 2768838.3-2790519.8 ns)
  2768838.3 |########################################
  2769922.4 |########################################
  2771006.4 |
  2772090.5 |
  2773174.6 |
  2774258.7 |########################################
  2775342.8 |
  2776426.8 |
  2777510.9 |
  2778595.0 |
  2779679.0 |
  2780763.1 |
  2781847.2 |
  2782931.3 |
  2784015.3 |########################################
  2785099.4 |
  2786183.5 |
  2787267.6 |
  2788351.6 |########################################
  2789435.7 |
  (0 below, 1 above range)

abi_lifecycle_madd_fresh_per_column (n=6, range 2731234.2-2757039.0 ns)
  2731234.2 |########################################
  2732524.4 |
  2733814.7 |
  2735104.9 |
  2736395.2 |########################################
  2737685.4 |
  2738975.6 |
  2740265.9 |
  2741556.1 |########################################
  2742846.3 |
  2744136.6 |########################################
  2745426.8 |
  2746717.1 |
  2748007.3 |
  2749297.5 |
  2750587.8 |
  2751878.0 |
  2753168.2 |
  2754458.5 |
  2755748.7 |########################################
  (0 below, 1 above range)

abi_lifecycle_madd_held_handle (n=6, range 2711907.9-2801622.0 ns)
  2711907.9 |########################################
  2716393.6 |
  2720879.3 |####################
  2725365.0 |####################
  2729850.7 |####################
  2734336.4 |
  2738822.1 |
  2743307.9 |
  2747793.6 |
  2752279.3 |
  2756765.0 |
  2761250.7 |
  2765736.4 |
  2770222.1 |
  2774707.8 |
  2779193.5 |
  2783679.2 |
  2788164.9 |
  2792650.6 |
  2797136.3 |
  (0 below, 1 above range)

abi_lifecycle_madd_null_entry (n=6, range 2434.6-2628.3 ns)
   2434.6 |########################################
   2444.3 |
   2454.0 |
   2463.7 |
   2473.3 |########################################
   2483.0 |
   2492.7 |
   2502.4 |
   2512.1 |
   2521.8 |
   2531.5 |
   2541.2 |
   2550.8 |
   2560.5 |
   2570.2 |
   2579.9 |
   2589.6 |########################################
   2599.3 |########################################
   2609.0 |########################################
   2618.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_madd_fresh_per_batch**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_fresh_per_column**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_held_handle**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_null_entry**: bridge=4367.5% of algo (FFI overhead may distort results)
