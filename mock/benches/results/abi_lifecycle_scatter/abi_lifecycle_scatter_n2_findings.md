# abi_lifecycle (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_scatter_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_scatter_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_scatter_null_entry dominates: 60613% faster than the next best (abi_lifecycle_scatter_held_handle)

abi_lifecycle_scatter_null_entry (3.54 us) leads abi_lifecycle_scatter_held_handle (2.15 ms) by 60613%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_scatter_null_entry beats baseline by 100% (significant)

abi_lifecycle_scatter_null_entry is -2.14 ms (100%) faster than baseline abi_lifecycle_scatter_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_scatter_fresh_per_batch is an outlier: 1046.5x slower than the field

abi_lifecycle_scatter_fresh_per_batch (3.70 ms) is 1046.5x the fastest (3.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_scatter_fresh_per_column shows alternating (throttle bounce) (autocorr -0.65)

abi_lifecycle_scatter_fresh_per_column's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_scatter_null_entry} vs {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} (60613% apart)

The field splits into a fast tier {abi_lifecycle_scatter_null_entry} and a slow tier {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} with a 60613% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1046.5x the fastest

Fastest abi_lifecycle_scatter_null_entry (3.54 us) to slowest abi_lifecycle_scatter_fresh_per_batch (3.70 ms): 1046.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_scatter_null_entry** at 3537.7 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1046.46x (fastest 3537.7 ns, slowest 3702062.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 3704095ns | 3704620ns | 3694248ns | 3702871ns | 3710853ns | +72.23% |
| abi_lifecycle_scatter_fresh_per_column | 2167858ns | 2167669ns | 2162150ns | 2166394ns | 2172908ns | +0.80% |
| abi_lifecycle_scatter_held_handle | 2150632ns | 2150403ns | 2141003ns | 2148982ns | 2157921ns | base |
| abi_lifecycle_scatter_null_entry | 5882ns | 5858ns | 5775ns | 5831ns | 6013ns | -99.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 3701469ns | 3691675ns | 3708064ns | +72.32% | 0.000 |
| abi_lifecycle_scatter_fresh_per_column | 2165266ns | 2159770ns | 2170185ns | +0.80% | 0.000 |
| abi_lifecycle_scatter_held_handle | 2148052ns | 2138566ns | 2155213ns | base | 0.000 |
| abi_lifecycle_scatter_null_entry | 3554ns | 3495ns | 3628ns | -99.83% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 42091.0 | 3701595.6 | 3701469.0 | n/a |
| abi_lifecycle_scatter_fresh_per_column | 38660.4 | 2162408.6 | 2165265.7 | n/a |
| abi_lifecycle_scatter_held_handle | 37243.2 | 2148230.8 | 2148052.5 | n/a |
| abi_lifecycle_scatter_null_entry | 26600.9 | 3635.6 | 3554.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_scatter_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_scatter_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_scatter_held_handle | 0.000 | 0.2% |
| abi_lifecycle_scatter_null_entry | 0.001 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 3704095ns | 3704095ns | +72.23% |
| abi_lifecycle_scatter_fresh_per_column | 2167858ns | 2167858ns | +0.80% |
| abi_lifecycle_scatter_held_handle | 2150632ns | 2150632ns | base |
| abi_lifecycle_scatter_null_entry | 5882ns | 5882ns | -99.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_held_handle | 2147845ns | base | --- | [2141099, 2155213] | --- | --- | --- | --- |
| abi_lifecycle_scatter_fresh_per_batch | 3702063ns | +1556186.5ns (+72.5%) | [+1545729, +1558334]ns | [3694281, 3708064] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 2165091ns | +17823.8ns (+0.8%) | [+6703, +27113]ns | [2160521, 2170185] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_scatter_null_entry | 3538ns | -2144297.3ns (-99.8%) | [-2151596, -2137601]ns | [3498, 3628] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_scatter_held_handle | abi_lifecycle_scatter_fresh_per_batch | abi_lifecycle_scatter_fresh_per_column | abi_lifecycle_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2155705ns | +71.8% | +0.3% | -99.8% |
| 2 | 2143632ns | +72.7% | +1.1% | -99.8% |
| 3 | 2138566ns | +72.9% | +1.4% | -99.8% |
| 4 | 2147550ns | +71.9% | +0.6% | -99.8% |
| 5 | 2148140ns | +72.4% | +1.1% | -99.8% |
| 6 | 2154721ns | +72.3% | +0.4% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.191 | ok |
| abi_lifecycle_scatter_fresh_per_column | -0.649 | HIGH- (thermal bounce) |
| abi_lifecycle_scatter_held_handle | 0.063 | ok |
| abi_lifecycle_scatter_null_entry | -0.340 | moderate- |

**Consistency summary:**

- **abi_lifecycle_scatter_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 11146606.2ns | 3701469.0ns | 301.1% | HIGH |
| abi_lifecycle_scatter_fresh_per_column | 6528745.5ns | 2165265.7ns | 301.5% | HIGH |
| abi_lifecycle_scatter_held_handle | 6485304.6ns | 2148052.5ns | 301.9% | HIGH |
| abi_lifecycle_scatter_null_entry | 119980.7ns | 3554.3ns | 3375.7% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_scatter_fresh_per_batch (n=6, range 3691674.6-3708063.5 ns)
  3691674.6 |####################
  3692494.0 |
  3693313.5 |
  3694132.9 |
  3694952.4 |
  3695771.8 |
  3696591.3 |####################
  3697410.7 |
  3698230.2 |
  3699049.6 |
  3699869.1 |
  3700688.5 |####################
  3701508.0 |
  3702327.4 |########################################
  3703146.9 |
  3703966.3 |
  3704785.8 |
  3705605.2 |
  3706424.7 |
  3707244.1 |
  (0 below, 1 above range)

abi_lifecycle_scatter_fresh_per_column (n=6, range 2159770.4-2170185.0 ns)
  2159770.4 |########################################
  2160291.1 |
  2160811.9 |########################################
  2161332.6 |
  2161853.3 |
  2162374.0 |########################################
  2162894.8 |
  2163415.5 |
  2163936.2 |
  2164457.0 |
  2164977.7 |
  2165498.4 |
  2166019.2 |
  2166539.9 |
  2167060.6 |
  2167581.4 |########################################
  2168102.1 |
  2168622.8 |########################################
  2169143.5 |
  2169664.3 |
  (0 below, 1 above range)

abi_lifecycle_scatter_held_handle (n=6, range 2138565.8-2155213.3 ns)
  2138565.8 |########################################
  2139398.2 |
  2140230.5 |
  2141062.9 |
  2141895.3 |
  2142727.7 |
  2143560.0 |########################################
  2144392.4 |
  2145224.8 |
  2146057.2 |
  2146889.5 |########################################
  2147721.9 |########################################
  2148554.3 |
  2149386.7 |
  2150219.0 |
  2151051.4 |
  2151883.8 |
  2152716.2 |
  2153548.5 |
  2154380.9 |########################################
  (0 below, 1 above range)

abi_lifecycle_scatter_null_entry (n=6, range 3495.4-3627.5 ns)
   3495.4 |########################################
   3502.0 |####################
   3508.6 |
   3515.2 |
   3521.8 |
   3528.4 |
   3535.0 |
   3541.6 |
   3548.2 |
   3554.8 |
   3561.4 |
   3568.1 |####################
   3574.7 |
   3581.3 |
   3587.9 |####################
   3594.5 |
   3601.1 |
   3607.7 |
   3614.3 |
   3620.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_scatter_fresh_per_batch**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_fresh_per_column**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_held_handle**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_null_entry**: bridge=3396.8% of algo (FFI overhead may distort results)
