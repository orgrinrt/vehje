# abi_lifecycle (madd)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_madd_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_madd_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_madd_null_entry dominates: 85500% faster than the next best (abi_lifecycle_madd_held_handle)

abi_lifecycle_madd_null_entry (3.17 us) leads abi_lifecycle_madd_held_handle (2.71 ms) by 85500%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_madd_null_entry beats baseline by 100% (significant)

abi_lifecycle_madd_null_entry is -2.71 ms (100%) faster than baseline abi_lifecycle_madd_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_madd_fresh_per_column is an outlier: 863.3x slower than the field

abi_lifecycle_madd_fresh_per_column (2.73 ms) is 863.3x the fastest (3.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_madd_null_entry shows alternating (throttle bounce) (autocorr -0.64)

abi_lifecycle_madd_null_entry's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_madd_null_entry} vs {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_batch, abi_lifecycle_madd_fresh_per_column} (85500% apart)

The field splits into a fast tier {abi_lifecycle_madd_null_entry} and a slow tier {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_batch, abi_lifecycle_madd_fresh_per_column} with a 85500% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 863.3x the fastest

Fastest abi_lifecycle_madd_null_entry (3.17 us) to slowest abi_lifecycle_madd_fresh_per_column (2.73 ms): 863.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_madd_null_entry** at 3167.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 863.32x (fastest 3167.7 ns, slowest 2734735.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2742883ns | 2735835ns | 2729524ns | 2734861ns | 2761595ns | +0.90% |
| abi_lifecycle_madd_fresh_per_column | 2740249ns | 2737408ns | 2732792ns | 2737120ns | 2748671ns | +0.80% |
| abi_lifecycle_madd_held_handle | 2718376ns | 2714286ns | 2711740ns | 2713515ns | 2728987ns | base |
| abi_lifecycle_madd_null_entry | 5466ns | 5444ns | 5292ns | 5416ns | 5628ns | -99.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2740001ns | 2726885ns | 2758368ns | +0.90% | 0.000 |
| abi_lifecycle_madd_fresh_per_column | 2737468ns | 2730095ns | 2745730ns | +0.80% | 0.000 |
| abi_lifecycle_madd_held_handle | 2715638ns | 2709134ns | 2726144ns | base | 0.000 |
| abi_lifecycle_madd_null_entry | 3184ns | 3088ns | 3278ns | -99.88% | 0.080 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 47629.7 | 2748269.4 | 2740000.7 | n/a |
| abi_lifecycle_madd_fresh_per_column | 46939.9 | 2743059.5 | 2737468.2 | n/a |
| abi_lifecycle_madd_held_handle | 45727.5 | 2715098.0 | 2715637.6 | n/a |
| abi_lifecycle_madd_null_entry | 26965.6 | 3206.9 | 3183.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_lifecycle_madd_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_madd_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_madd_held_handle | 0.000 | 0.1% |
| abi_lifecycle_madd_null_entry | 0.081 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2742883ns | 2742883ns | +0.90% |
| abi_lifecycle_madd_fresh_per_column | 2740249ns | 2740249ns | +0.80% |
| abi_lifecycle_madd_held_handle | 2718376ns | 2718376ns | base |
| abi_lifecycle_madd_null_entry | 5466ns | 5466ns | -99.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_madd_held_handle | 2711567ns | base | --- | [2709202, 2726144] | --- | --- | --- | --- |
| abi_lifecycle_madd_fresh_per_batch | 2733114ns | +21139.1ns (+0.8%) | [+14257, +37694]ns | [2728520, 2758368] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_madd_fresh_per_column | 2734735ns | +24658.9ns (+0.9%) | [+10851, +29981]ns | [2731939, 2745730] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_madd_null_entry | 3168ns | -2708288.4ns (-99.9%) | [-2723014, -2706059]ns | [3105, 3278] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_madd_held_handle | abi_lifecycle_madd_fresh_per_batch | abi_lifecycle_madd_fresh_per_column | abi_lifecycle_madd_null_entry |
|---|---|---|---|---|
| 1 | 2732215ns | +1.7% | +0.4% | -99.9% |
| 2 | 2712882ns | +0.8% | +1.3% | -99.9% |
| 3 | 2720073ns | +0.4% | +0.4% | -99.9% |
| 4 | 2709270ns | +0.8% | +0.9% | -99.9% |
| 5 | 2709134ns | +1.0% | +0.9% | -99.9% |
| 6 | 2710252ns | +0.6% | +0.9% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | -0.021 | ok |
| abi_lifecycle_madd_fresh_per_column | 0.161 | ok |
| abi_lifecycle_madd_held_handle | -0.023 | ok |
| abi_lifecycle_madd_null_entry | -0.642 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_lifecycle_madd_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_madd_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 8280791.3ns | 2740000.7ns | 302.2% | HIGH |
| abi_lifecycle_madd_fresh_per_column | 8267755.4ns | 2737468.2ns | 302.0% | HIGH |
| abi_lifecycle_madd_held_handle | 8183895.5ns | 2715637.6ns | 301.4% | HIGH |
| abi_lifecycle_madd_null_entry | 120530.3ns | 3183.8ns | 3785.8% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_madd_fresh_per_batch (n=6, range 2726885.4-2758368.1 ns)
  2726885.4 |########################################
  2728459.5 |
  2730033.7 |########################################
  2731607.8 |########################################
  2733181.9 |########################################
  2734756.1 |
  2736330.2 |########################################
  2737904.3 |
  2739478.5 |
  2741052.6 |
  2742626.8 |
  2744200.9 |
  2745775.0 |
  2747349.2 |
  2748923.3 |
  2750497.4 |
  2752071.6 |
  2753645.7 |
  2755219.8 |
  2756794.0 |
  (0 below, 1 above range)

abi_lifecycle_madd_fresh_per_column (n=6, range 2730095.4-2745730.0 ns)
  2730095.4 |####################
  2730877.1 |
  2731658.9 |
  2732440.6 |
  2733222.3 |########################################
  2734004.0 |
  2734785.8 |####################
  2735567.5 |
  2736349.2 |
  2737131.0 |
  2737912.7 |
  2738694.4 |
  2739476.2 |
  2740257.9 |
  2741039.6 |
  2741821.4 |
  2742603.1 |
  2743384.8 |####################
  2744166.5 |
  2744948.3 |
  (0 below, 1 above range)

abi_lifecycle_madd_held_handle (n=6, range 2709133.8-2726144.3 ns)
  2709133.8 |########################################
  2709984.3 |####################
  2710834.9 |
  2711685.4 |
  2712535.9 |####################
  2713386.4 |
  2714237.0 |
  2715087.5 |
  2715938.0 |
  2716788.5 |
  2717639.1 |
  2718489.6 |
  2719340.1 |####################
  2720190.7 |
  2721041.2 |
  2721891.7 |
  2722742.2 |
  2723592.8 |
  2724443.3 |
  2725293.8 |
  (0 below, 1 above range)

abi_lifecycle_madd_null_entry (n=6, range 3087.5-3278.3 ns)
   3087.5 |########################################
   3097.0 |
   3106.6 |
   3116.1 |########################################
   3125.7 |
   3135.2 |########################################
   3144.8 |
   3154.3 |
   3163.8 |
   3173.4 |
   3182.9 |
   3192.5 |########################################
   3202.0 |
   3211.6 |
   3221.1 |
   3230.6 |
   3240.2 |
   3249.7 |
   3259.3 |
   3268.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_madd_fresh_per_batch**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_fresh_per_column**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_held_handle**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_null_entry**: bridge=3819.4% of algo (FFI overhead may distort results)
