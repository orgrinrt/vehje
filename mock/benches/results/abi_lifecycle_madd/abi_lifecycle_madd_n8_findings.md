# abi_lifecycle (madd)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_madd_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_madd_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_madd_null_entry dominates: 91849% faster than the next best (abi_lifecycle_madd_held_handle)

abi_lifecycle_madd_null_entry (2.97 us) leads abi_lifecycle_madd_held_handle (2.73 ms) by 91849%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_madd_null_entry beats baseline by 100% (significant)

abi_lifecycle_madd_null_entry is -2.73 ms (100%) faster than baseline abi_lifecycle_madd_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_madd_fresh_per_batch is an outlier: 1057.1x slower than the field

abi_lifecycle_madd_fresh_per_batch (3.14 ms) is 1057.1x the fastest (2.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_madd_null_entry shows alternating (throttle bounce) (autocorr -0.55)

abi_lifecycle_madd_null_entry's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_madd_null_entry} vs {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} (91849% apart)

The field splits into a fast tier {abi_lifecycle_madd_null_entry} and a slow tier {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} with a 91849% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1057.1x the fastest

Fastest abi_lifecycle_madd_null_entry (2.97 us) to slowest abi_lifecycle_madd_fresh_per_batch (3.14 ms): 1057.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_madd_null_entry** at 2968.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1057.13x (fastest 2968.3 ns, slowest 3137930.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 3145224ns | 3141249ns | 3136554ns | 3139989ns | 3157412ns | +15.05% |
| abi_lifecycle_madd_fresh_per_column | 2767236ns | 2744342ns | 2740986ns | 2744190ns | 2814931ns | +1.23% |
| abi_lifecycle_madd_held_handle | 2733717ns | 2732729ns | 2728741ns | 2731809ns | 2739066ns | base |
| abi_lifecycle_madd_null_entry | 5237ns | 5238ns | 5100ns | 5208ns | 5349ns | -99.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 3141949ns | 3133309ns | 3154111ns | +15.07% | 0.000 |
| abi_lifecycle_madd_fresh_per_column | 2764130ns | 2737990ns | 2811449ns | +1.24% | 0.000 |
| abi_lifecycle_madd_held_handle | 2730360ns | 2725289ns | 2735578ns | base | 0.000 |
| abi_lifecycle_madd_null_entry | 2982ns | 2925ns | 3052ns | -99.89% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 63187.4 | 3147280.2 | 3141949.0 | 0 |
| abi_lifecycle_madd_fresh_per_column | 62169.8 | 2775036.2 | 2764130.1 | 0 |
| abi_lifecycle_madd_held_handle | 63343.4 | 2728718.3 | 2730360.4 | n/a |
| abi_lifecycle_madd_null_entry | 26175.5 | 3062.3 | 2982.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_lifecycle_madd_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_madd_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_madd_held_handle | 0.000 | 0.1% |
| abi_lifecycle_madd_null_entry | 0.003 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 3145224ns | 3145224ns | +15.05% |
| abi_lifecycle_madd_fresh_per_column | 2767236ns | 2767236ns | +1.23% |
| abi_lifecycle_madd_held_handle | 2733717ns | 2733717ns | base |
| abi_lifecycle_madd_null_entry | 5237ns | 5237ns | -99.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_madd_held_handle | 2729365ns | base | --- | [2726138, 2735578] | --- | --- | --- | --- |
| abi_lifecycle_madd_fresh_per_batch | 3137930ns | +408459.4ns (+15.0%) | [+405010, +421297]ns | [3133806, 3154111] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_madd_fresh_per_column | 2741453ns | +15087.9ns (+0.6%) | [+5978, +80243]ns | [2739489, 2811449] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_madd_null_entry | 2968ns | -2726413.1ns (-99.9%) | [-2732576, -2723145]ns | [2927, 3052] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_madd_held_handle | abi_lifecycle_madd_fresh_per_batch | abi_lifecycle_madd_fresh_per_column | abi_lifecycle_madd_null_entry |
|---|---|---|---|---|
| 1 | 2729340ns | +15.0% | +0.3% | -99.9% |
| 2 | 2733022ns | +15.8% | +5.2% | -99.9% |
| 3 | 2726988ns | +15.0% | +0.5% | -99.9% |
| 4 | 2725289ns | +15.0% | +0.6% | -99.9% |
| 5 | 2738134ns | +14.8% | +0.1% | -99.9% |
| 6 | 2729390ns | +14.8% | +0.7% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | -0.218 | moderate- |
| abi_lifecycle_madd_fresh_per_column | -0.270 | moderate- |
| abi_lifecycle_madd_held_handle | -0.390 | moderate- |
| abi_lifecycle_madd_null_entry | -0.553 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_lifecycle_madd_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_madd_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 9497300.1ns | 3141949.0ns | 302.3% | HIGH |
| abi_lifecycle_madd_fresh_per_column | 8391705.8ns | 2764130.1ns | 303.6% | HIGH |
| abi_lifecycle_madd_held_handle | 8243131.2ns | 2730360.4ns | 301.9% | HIGH |
| abi_lifecycle_madd_null_entry | 118752.5ns | 2982.5ns | 3981.7% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_madd_fresh_per_batch (n=6, range 3133308.8-3154111.2 ns)
  3133308.8 |########################################
  3134348.9 |
  3135389.0 |####################
  3136429.2 |
  3137469.3 |
  3138509.4 |
  3139549.5 |####################
  3140589.7 |
  3141629.8 |
  3142669.9 |####################
  3143710.0 |
  3144750.1 |
  3145790.3 |
  3146830.4 |
  3147870.5 |
  3148910.6 |
  3149950.8 |
  3150990.9 |
  3152031.0 |
  3153071.1 |
  (0 below, 1 above range)

abi_lifecycle_madd_fresh_per_column (n=6, range 2737989.6-2811448.8 ns)
  2737989.6 |########################################
  2741662.6 |
  2745335.5 |##########
  2749008.5 |
  2752681.4 |
  2756354.4 |
  2760027.3 |
  2763700.3 |
  2767373.3 |
  2771046.2 |
  2774719.2 |
  2778392.1 |
  2782065.1 |
  2785738.0 |
  2789411.0 |
  2793084.0 |
  2796756.9 |
  2800429.9 |
  2804102.8 |
  2807775.8 |
  (0 below, 1 above range)

abi_lifecycle_madd_held_handle (n=6, range 2725288.8-2735577.8 ns)
  2725288.8 |####################
  2725803.2 |
  2726317.7 |
  2726832.1 |####################
  2727346.6 |
  2727861.0 |
  2728375.5 |
  2728889.9 |########################################
  2729404.4 |
  2729918.8 |
  2730433.3 |
  2730947.7 |
  2731462.2 |
  2731976.6 |
  2732491.1 |
  2733005.5 |####################
  2733520.0 |
  2734034.4 |
  2734548.9 |
  2735063.3 |
  (0 below, 1 above range)

abi_lifecycle_madd_null_entry (n=6, range 2924.6-3051.7 ns)
   2924.6 |########################################
   2931.0 |
   2937.3 |
   2943.7 |
   2950.0 |
   2956.4 |####################
   2962.7 |
   2969.1 |####################
   2975.4 |
   2981.8 |
   2988.1 |
   2994.5 |
   3000.9 |
   3007.2 |
   3013.6 |
   3019.9 |####################
   3026.3 |
   3032.6 |
   3039.0 |
   3045.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_madd_fresh_per_batch**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_fresh_per_column**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_held_handle**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_null_entry**: bridge=4005.6% of algo (FFI overhead may distort results)
