# abi_lifecycle (tight)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_tight_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_tight_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_tight_null_entry dominates: 50988% faster than the next best (abi_lifecycle_tight_held_handle)

abi_lifecycle_tight_null_entry (3.93 us) leads abi_lifecycle_tight_held_handle (2.01 ms) by 50988%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_tight_null_entry beats baseline by 100% (significant)

abi_lifecycle_tight_null_entry is -2.00 ms (100%) faster than baseline abi_lifecycle_tight_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_tight_fresh_per_batch is an outlier: 718.6x slower than the field

abi_lifecycle_tight_fresh_per_batch (2.82 ms) is 718.6x the fastest (3.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_tight_null_entry} vs {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} (50988% apart)

The field splits into a fast tier {abi_lifecycle_tight_null_entry} and a slow tier {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} with a 50988% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 718.6x the fastest

Fastest abi_lifecycle_tight_null_entry (3.93 us) to slowest abi_lifecycle_tight_fresh_per_batch (2.82 ms): 718.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_tight_null_entry** at 3930.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 718.58x (fastest 3930.8 ns, slowest 2824634.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2827828ns | 2827231ns | 2820641ns | 2825101ns | 2835512ns | +40.67% |
| abi_lifecycle_tight_fresh_per_column | 2024571ns | 2025437ns | 2019777ns | 2024834ns | 2026573ns | +0.71% |
| abi_lifecycle_tight_held_handle | 2010276ns | 2010686ns | 2002713ns | 2010479ns | 2013753ns | base |
| abi_lifecycle_tight_null_entry | 6246ns | 6165ns | 6063ns | 6138ns | 6499ns | -99.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2825203ns | 2818244ns | 2832725ns | +40.72% | 0.000 |
| abi_lifecycle_tight_fresh_per_column | 2022013ns | 2017367ns | 2023995ns | +0.71% | 0.000 |
| abi_lifecycle_tight_held_handle | 2007723ns | 2000287ns | 2011059ns | base | 0.000 |
| abi_lifecycle_tight_null_entry | 3993ns | 3882ns | 4164ns | -99.80% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 39508.0 | 2822963.2 | 2825202.5 | n/a |
| abi_lifecycle_tight_fresh_per_column | 36678.3 | 2021985.6 | 2022013.2 | 0 |
| abi_lifecycle_tight_held_handle | 38424.2 | 2006438.8 | 2007723.5 | n/a |
| abi_lifecycle_tight_null_entry | 26842.1 | 4118.2 | 3993.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_tight_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_tight_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_tight_held_handle | 0.000 | 0.2% |
| abi_lifecycle_tight_null_entry | 0.001 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2827828ns | 2827828ns | +40.67% |
| abi_lifecycle_tight_fresh_per_column | 2024571ns | 2024571ns | +0.71% |
| abi_lifecycle_tight_held_handle | 2010276ns | 2010276ns | base |
| abi_lifecycle_tight_null_entry | 6246ns | 6246ns | -99.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_tight_held_handle | 2008180ns | base | --- | [2003932, 2011059] | --- | --- | --- | --- |
| abi_lifecycle_tight_fresh_per_batch | 2824634ns | +816790.0ns (+40.7%) | [+810880, +824767]ns | [2818249, 2832725] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_tight_fresh_per_column | 2022863ns | +13571.1ns (+0.7%) | [+9457, +19841]ns | [2019181, 2023995] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_tight_null_entry | 3931ns | -2004287.5ns (-99.8%) | [-2007134, -1999769]ns | [3886, 4164] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_tight_held_handle | abi_lifecycle_tight_fresh_per_batch | abi_lifecycle_tight_fresh_per_column | abi_lifecycle_tight_null_entry |
|---|---|---|---|---|
| 1 | 2007578ns | +41.1% | +0.7% | -99.8% |
| 2 | 2009525ns | +40.2% | +0.7% | -99.8% |
| 3 | 2000287ns | +40.9% | +1.2% | -99.8% |
| 4 | 2008022ns | +40.6% | +0.8% | -99.8% |
| 5 | 2012592ns | +40.4% | +0.5% | -99.8% |
| 6 | 2008337ns | +41.1% | +0.4% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 0.054 | ok |
| abi_lifecycle_tight_fresh_per_column | 0.102 | ok |
| abi_lifecycle_tight_held_handle | -0.138 | ok |
| abi_lifecycle_tight_null_entry | -0.325 | moderate- |

**Consistency summary:**

- **abi_lifecycle_tight_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_tight_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 8508779.3ns | 2825202.5ns | 301.2% | HIGH |
| abi_lifecycle_tight_fresh_per_column | 6105608.9ns | 2022013.2ns | 302.0% | HIGH |
| abi_lifecycle_tight_held_handle | 6061383.5ns | 2007723.5ns | 301.9% | HIGH |
| abi_lifecycle_tight_null_entry | 120991.9ns | 3993.3ns | 3029.8% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_tight_fresh_per_batch (n=6, range 2818244.2-2832724.6 ns)
  2818244.2 |########################################
  2818968.2 |
  2819692.2 |
  2820416.3 |
  2821140.3 |
  2821864.3 |
  2822588.3 |
  2823312.3 |####################
  2824036.4 |
  2824760.4 |
  2825484.4 |####################
  2826208.4 |
  2826932.4 |
  2827656.5 |
  2828380.5 |
  2829104.5 |
  2829828.5 |
  2830552.5 |
  2831276.6 |
  2832000.6 |####################
  (0 below, 1 above range)

abi_lifecycle_tight_fresh_per_column (n=6, range 2017367.1-2023995.4 ns)
  2017367.1 |########################################
  2017698.5 |
  2018029.9 |
  2018361.3 |
  2018692.8 |
  2019024.2 |
  2019355.6 |
  2019687.0 |
  2020018.4 |
  2020349.8 |
  2020681.2 |########################################
  2021012.7 |
  2021344.1 |
  2021675.5 |
  2022006.9 |
  2022338.3 |########################################
  2022669.7 |
  2023001.2 |########################################
  2023332.6 |########################################
  2023664.0 |
  (0 below, 1 above range)

abi_lifecycle_tight_held_handle (n=6, range 2000286.7-2011058.6 ns)
  2000286.7 |####################
  2000825.3 |
  2001363.9 |
  2001902.5 |
  2002441.1 |
  2002979.7 |
  2003518.3 |
  2004056.8 |
  2004595.4 |
  2005134.0 |
  2005672.6 |
  2006211.2 |
  2006749.8 |
  2007288.4 |####################
  2007827.0 |########################################
  2008365.6 |
  2008904.2 |
  2009442.8 |####################
  2009981.4 |
  2010520.0 |
  (0 below, 1 above range)

abi_lifecycle_tight_null_entry (n=6, range 3882.5-4163.5 ns)
   3882.5 |########################################
   3896.6 |####################
   3910.6 |
   3924.7 |
   3938.7 |
   3952.8 |####################
   3966.8 |
   3980.9 |
   3994.9 |
   4009.0 |
   4023.0 |
   4037.1 |
   4051.1 |
   4065.2 |####################
   4079.2 |
   4093.3 |
   4107.3 |
   4121.4 |
   4135.4 |
   4149.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_tight_fresh_per_batch**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_fresh_per_column**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_held_handle**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_null_entry**: bridge=3077.5% of algo (FFI overhead may distort results)
