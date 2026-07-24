# abi_lifecycle (tight)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_tight_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_tight_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_tight_null_entry dominates: 68217% faster than the next best (abi_lifecycle_tight_held_handle)

abi_lifecycle_tight_null_entry (2.94 us) leads abi_lifecycle_tight_held_handle (2.01 ms) by 68217%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_tight_null_entry beats baseline by 100% (significant)

abi_lifecycle_tight_null_entry is -2.01 ms (100%) faster than baseline abi_lifecycle_tight_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_tight_fresh_per_batch is an outlier: 822.3x slower than the field

abi_lifecycle_tight_fresh_per_batch (2.42 ms) is 822.3x the fastest (2.94 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_tight_fresh_per_batch shows alternating (throttle bounce) (autocorr -0.52)

abi_lifecycle_tight_fresh_per_batch's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_tight_null_entry} vs {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} (68217% apart)

The field splits into a fast tier {abi_lifecycle_tight_null_entry} and a slow tier {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} with a 68217% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 822.3x the fastest

Fastest abi_lifecycle_tight_null_entry (2.94 us) to slowest abi_lifecycle_tight_fresh_per_batch (2.42 ms): 822.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_tight_null_entry** at 2940.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 822.30x (fastest 2940.2 ns, slowest 2417722.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2420616ns | 2420328ns | 2414557ns | 2420213ns | 2424250ns | +20.42% |
| abi_lifecycle_tight_fresh_per_column | 2028166ns | 2027616ns | 2022175ns | 2026737ns | 2033306ns | +0.89% |
| abi_lifecycle_tight_held_handle | 2010201ns | 2011268ns | 2003767ns | 2009565ns | 2014371ns | base |
| abi_lifecycle_tight_null_entry | 5198ns | 5161ns | 5082ns | 5141ns | 5342ns | -99.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2417993ns | 2411983ns | 2421631ns | +20.44% | 0.000 |
| abi_lifecycle_tight_fresh_per_column | 2025592ns | 2019719ns | 2030584ns | +0.90% | 0.000 |
| abi_lifecycle_tight_held_handle | 2007579ns | 2001215ns | 2011602ns | base | 0.000 |
| abi_lifecycle_tight_null_entry | 2977ns | 2912ns | 3075ns | -99.85% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 39942.6 | 2416469.6 | 2417993.2 | n/a |
| abi_lifecycle_tight_fresh_per_column | 39067.6 | 2023801.1 | 2025591.9 | 0 |
| abi_lifecycle_tight_held_handle | 39334.7 | 2008261.8 | 2007578.5 | n/a |
| abi_lifecycle_tight_null_entry | 26475.1 | 3057.9 | 2977.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_lifecycle_tight_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_tight_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_tight_held_handle | 0.000 | 0.1% |
| abi_lifecycle_tight_null_entry | 0.003 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2420616ns | 2420616ns | +20.42% |
| abi_lifecycle_tight_fresh_per_column | 2028166ns | 2028166ns | +0.89% |
| abi_lifecycle_tight_held_handle | 2010201ns | 2010201ns | base |
| abi_lifecycle_tight_null_entry | 5198ns | 5198ns | -99.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_tight_held_handle | 2008665ns | base | --- | [2002468, 2011602] | --- | --- | --- | --- |
| abi_lifecycle_tight_fresh_per_batch | 2417723ns | +408862.7ns (+20.4%) | [+406121, +416260]ns | [2414625, 2421631] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_tight_fresh_per_column | 2025052ns | +18172.8ns (+0.9%) | [+12257, +23610]ns | [2021140, 2030584] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_tight_null_entry | 2940ns | -2005701.6ns (-99.9%) | [-2008685, -1999417]ns | [2917, 3075] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_tight_held_handle | abi_lifecycle_tight_fresh_per_batch | abi_lifecycle_tight_fresh_per_column | abi_lifecycle_tight_null_entry |
|---|---|---|---|---|
| 1 | 2010225ns | +20.4% | +1.2% | -99.9% |
| 2 | 2007105ns | +20.4% | +0.6% | -99.9% |
| 3 | 2001215ns | +21.1% | +1.1% | -99.8% |
| 4 | 2003721ns | +20.4% | +1.2% | -99.9% |
| 5 | 2012013ns | +20.2% | +0.7% | -99.9% |
| 6 | 2011191ns | +20.2% | +0.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | -0.522 | HIGH- (thermal bounce) |
| abi_lifecycle_tight_fresh_per_column | -0.301 | moderate- |
| abi_lifecycle_tight_held_handle | 0.265 | moderate+ |
| abi_lifecycle_tight_null_entry | 0.089 | ok |

**Consistency summary:**

- **abi_lifecycle_tight_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_tight_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 7288882.6ns | 2417993.2ns | 301.4% | HIGH |
| abi_lifecycle_tight_fresh_per_column | 6110661.0ns | 2025591.9ns | 301.7% | HIGH |
| abi_lifecycle_tight_held_handle | 6065530.9ns | 2007578.5ns | 302.1% | HIGH |
| abi_lifecycle_tight_null_entry | 118600.2ns | 2977.3ns | 3983.5% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_tight_fresh_per_batch (n=6, range 2411983.3-2421631.2 ns)
  2411983.3 |####################
  2412465.7 |
  2412948.1 |
  2413430.5 |
  2413912.9 |
  2414395.3 |
  2414877.7 |
  2415360.1 |
  2415842.5 |
  2416324.9 |
  2416807.3 |####################
  2417289.7 |########################################
  2417772.1 |
  2418254.5 |
  2418736.9 |
  2419219.3 |####################
  2419701.7 |
  2420184.1 |
  2420666.5 |
  2421148.9 |
  (0 below, 1 above range)

abi_lifecycle_tight_fresh_per_column (n=6, range 2019719.2-2030583.5 ns)
  2019719.2 |####################
  2020262.4 |
  2020805.6 |
  2021348.9 |
  2021892.1 |
  2022435.3 |####################
  2022978.5 |####################
  2023521.7 |
  2024064.9 |
  2024608.2 |
  2025151.4 |
  2025694.6 |
  2026237.8 |
  2026781.0 |########################################
  2027324.2 |
  2027867.5 |
  2028410.7 |
  2028953.9 |
  2029497.1 |
  2030040.3 |
  (0 below, 1 above range)

abi_lifecycle_tight_held_handle (n=6, range 2001215.4-2011601.9 ns)
  2001215.4 |########################################
  2001734.7 |
  2002254.0 |
  2002773.4 |
  2003292.7 |########################################
  2003812.0 |
  2004331.3 |
  2004850.7 |
  2005370.0 |
  2005889.3 |
  2006408.6 |
  2006927.9 |########################################
  2007447.3 |
  2007966.6 |
  2008485.9 |
  2009005.2 |
  2009524.6 |
  2010043.9 |########################################
  2010563.2 |
  2011082.5 |########################################
  (0 below, 1 above range)

abi_lifecycle_tight_null_entry (n=6, range 2912.5-3075.0 ns)
   2912.5 |####################
   2920.6 |########################################
   2928.8 |
   2936.9 |
   2945.0 |####################
   2953.1 |
   2961.2 |
   2969.4 |
   2977.5 |
   2985.6 |
   2993.8 |####################
   3001.9 |
   3010.0 |
   3018.1 |
   3026.2 |
   3034.4 |
   3042.5 |
   3050.6 |
   3058.8 |
   3066.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_tight_fresh_per_batch**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_fresh_per_column**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_null_entry**: bridge=4032.5% of algo (FFI overhead may distort results)
