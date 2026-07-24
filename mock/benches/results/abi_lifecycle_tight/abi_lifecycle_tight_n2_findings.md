# abi_lifecycle (tight)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_tight_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_tight_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_tight_null_entry dominates: 57475% faster than the next best (abi_lifecycle_tight_held_handle)

abi_lifecycle_tight_null_entry (3.51 us) leads abi_lifecycle_tight_held_handle (2.02 ms) by 57475%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_tight_null_entry beats baseline by 100% (significant)

abi_lifecycle_tight_null_entry is -2.01 ms (100%) faster than baseline abi_lifecycle_tight_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_tight_fresh_per_batch is an outlier: 1042.4x slower than the field

abi_lifecycle_tight_fresh_per_batch (3.65 ms) is 1042.4x the fastest (3.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_tight_null_entry} vs {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} (57475% apart)

The field splits into a fast tier {abi_lifecycle_tight_null_entry} and a slow tier {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} with a 57475% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1042.4x the fastest

Fastest abi_lifecycle_tight_null_entry (3.51 us) to slowest abi_lifecycle_tight_fresh_per_batch (3.65 ms): 1042.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_tight_null_entry** at 3505.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1042.44x (fastest 3505.8 ns, slowest 3654587.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 3660758ns | 3657214ns | 3646361ns | 3655937ns | 3675188ns | +80.25% |
| abi_lifecycle_tight_fresh_per_column | 2032344ns | 2031508ns | 2027517ns | 2030321ns | 2037791ns | +0.07% |
| abi_lifecycle_tight_held_handle | 2030968ns | 2021048ns | 2007884ns | 2017946ns | 2062045ns | base |
| abi_lifecycle_tight_null_entry | 5813ns | 5840ns | 5546ns | 5776ns | 6002ns | -99.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 3658079ns | 3643782ns | 3672373ns | +80.36% | 0.000 |
| abi_lifecycle_tight_fresh_per_column | 2029757ns | 2025075ns | 2035156ns | +0.08% | 0.000 |
| abi_lifecycle_tight_held_handle | 2028226ns | 2005448ns | 2058910ns | base | 0.000 |
| abi_lifecycle_tight_null_entry | 3493ns | 3355ns | 3605ns | -99.83% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 43326.4 | 3660676.2 | 3658079.4 | n/a |
| abi_lifecycle_tight_fresh_per_column | 37693.4 | 2030156.2 | 2029757.4 | 3 |
| abi_lifecycle_tight_held_handle | 42286.3 | 2034861.4 | 2028225.8 | n/a |
| abi_lifecycle_tight_null_entry | 27722.3 | 3571.5 | 3493.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_tight_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_tight_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_tight_held_handle | 0.000 | 0.2% |
| abi_lifecycle_tight_null_entry | 0.001 | 95.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 3660758ns | 3660758ns | +80.25% |
| abi_lifecycle_tight_fresh_per_column | 2032344ns | 2032344ns | +0.07% |
| abi_lifecycle_tight_held_handle | 2030968ns | 2030968ns | base |
| abi_lifecycle_tight_null_entry | 5813ns | 5813ns | -99.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_tight_held_handle | 2018451ns | base | --- | [2007317, 2058910] | --- | --- | --- | --- |
| abi_lifecycle_tight_fresh_per_batch | 3654587ns | +1634855.0ns (+81.0%) | [+1597414, +1657292]ns | [3647278, 3672373] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_tight_fresh_per_column | 2028865ns | no significant difference | [-32685, +27351]ns | [2025252, 2035156] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_tight_null_entry | 3506ns | -2014983.0ns (-99.8%) | [-2055404, -2003811]ns | [3368, 3605] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_tight_held_handle | abi_lifecycle_tight_fresh_per_batch | abi_lifecycle_tight_fresh_per_column | abi_lifecycle_tight_null_entry |
|---|---|---|---|---|
| 1 | 2009185ns | +81.4% | +1.5% | -99.8% |
| 2 | 2020580ns | +80.7% | +0.2% | -99.8% |
| 3 | 2005448ns | +82.3% | +1.3% | -99.8% |
| 4 | 2018005ns | +82.5% | +0.4% | -99.8% |
| 5 | 2018898ns | +81.0% | +0.6% | -99.8% |
| 6 | 2097239ns | +74.6% | -3.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | -0.069 | ok |
| abi_lifecycle_tight_fresh_per_column | -0.466 | moderate- |
| abi_lifecycle_tight_held_handle | 0.001 | ok |
| abi_lifecycle_tight_null_entry | -0.391 | moderate- |

**Consistency summary:**

- **abi_lifecycle_tight_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_tight_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 11018976.4ns | 3658079.4ns | 301.2% | HIGH |
| abi_lifecycle_tight_fresh_per_column | 6128459.6ns | 2029757.4ns | 301.9% | HIGH |
| abi_lifecycle_tight_held_handle | 6239263.8ns | 2028225.8ns | 307.6% | HIGH |
| abi_lifecycle_tight_null_entry | 121019.0ns | 3493.0ns | 3464.6% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_tight_fresh_per_batch (n=6, range 3643782.1-3672373.3 ns)
  3643782.1 |####################
  3645211.7 |
  3646641.2 |
  3648070.8 |
  3649500.4 |####################
  3650929.9 |
  3652359.5 |
  3653789.0 |########################################
  3655218.6 |
  3656648.2 |
  3658077.7 |
  3659507.3 |
  3660936.8 |####################
  3662366.4 |
  3663796.0 |
  3665225.5 |
  3666655.1 |
  3668084.7 |
  3669514.2 |
  3670943.8 |
  (0 below, 1 above range)

abi_lifecycle_tight_fresh_per_column (n=6, range 2025075.0-2035155.6 ns)
  2025075.0 |########################################
  2025579.0 |
  2026083.1 |
  2026587.1 |####################
  2027091.1 |
  2027595.1 |
  2028099.2 |
  2028603.2 |
  2029107.2 |
  2029611.3 |
  2030115.3 |
  2030619.3 |####################
  2031123.4 |
  2031627.4 |####################
  2032131.4 |
  2032635.5 |
  2033139.5 |
  2033643.5 |
  2034147.5 |
  2034651.6 |
  (0 below, 1 above range)

abi_lifecycle_tight_held_handle (n=6, range 2005448.3-2058909.6 ns)
  2005448.3 |####################
  2008121.4 |####################
  2010794.4 |
  2013467.5 |
  2016140.6 |####################
  2018813.6 |########################################
  2021486.7 |
  2024159.8 |
  2026832.8 |
  2029505.9 |
  2032179.0 |
  2034852.0 |
  2037525.1 |
  2040198.1 |
  2042871.2 |
  2045544.3 |
  2048217.3 |
  2050890.4 |
  2053563.5 |
  2056236.5 |
  (0 below, 1 above range)

abi_lifecycle_tight_null_entry (n=6, range 3355.4-3605.0 ns)
   3355.4 |########################################
   3367.9 |
   3380.4 |########################################
   3392.8 |
   3405.3 |
   3417.8 |
   3430.3 |
   3442.8 |
   3455.2 |
   3467.7 |
   3480.2 |########################################
   3492.7 |
   3505.2 |
   3517.6 |########################################
   3530.1 |
   3542.6 |
   3555.1 |
   3567.6 |
   3580.0 |########################################
   3592.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_tight_fresh_per_batch**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_fresh_per_column**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_held_handle**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_null_entry**: bridge=3461.2% of algo (FFI overhead may distort results)
