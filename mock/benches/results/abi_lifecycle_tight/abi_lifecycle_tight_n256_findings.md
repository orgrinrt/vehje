# abi_lifecycle (tight)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_tight_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_tight_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_tight_null_entry dominates: 60909% faster than the next best (abi_lifecycle_tight_held_handle)

abi_lifecycle_tight_null_entry (3.28 us) leads abi_lifecycle_tight_held_handle (2.00 ms) by 60909%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_tight_null_entry beats baseline by 100% (significant)

abi_lifecycle_tight_null_entry is -2.00 ms (100%) faster than baseline abi_lifecycle_tight_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_tight_fresh_per_column is an outlier: 614.7x slower than the field

abi_lifecycle_tight_fresh_per_column (2.02 ms) is 614.7x the fastest (3.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_tight_null_entry shows alternating (throttle bounce) (autocorr -0.61)

abi_lifecycle_tight_null_entry's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_tight_null_entry} vs {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_batch, abi_lifecycle_tight_fresh_per_column} (60909% apart)

The field splits into a fast tier {abi_lifecycle_tight_null_entry} and a slow tier {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_batch, abi_lifecycle_tight_fresh_per_column} with a 60909% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 614.7x the fastest

Fastest abi_lifecycle_tight_null_entry (3.28 us) to slowest abi_lifecycle_tight_fresh_per_column (2.02 ms): 614.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_tight_null_entry** at 3279.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 614.68x (fastest 3279.6 ns, slowest 2015879.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2017071ns | 2016792ns | 2011016ns | 2015971ns | 2021749ns | +0.84% |
| abi_lifecycle_tight_fresh_per_column | 2021079ns | 2018435ns | 2013759ns | 2017662ns | 2029863ns | +1.04% |
| abi_lifecycle_tight_held_handle | 2000185ns | 2003412ns | 1984032ns | 2000858ns | 2007251ns | base |
| abi_lifecycle_tight_null_entry | 5579ns | 5601ns | 5403ns | 5542ns | 5721ns | -99.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2014578ns | 2008572ns | 2019223ns | +0.85% | 0.000 |
| abi_lifecycle_tight_fresh_per_column | 2018505ns | 2011165ns | 2027269ns | +1.05% | 0.000 |
| abi_lifecycle_tight_held_handle | 1997627ns | 1981653ns | 2004696ns | base | 0.000 |
| abi_lifecycle_tight_null_entry | 3258ns | 3148ns | 3331ns | -99.84% | 0.079 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 37309.6 | 2014379.7 | 2014578.4 | 0 |
| abi_lifecycle_tight_fresh_per_column | 37658.5 | 2016433.2 | 2018505.5 | 0 |
| abi_lifecycle_tight_held_handle | 38406.8 | 1996784.6 | 1997626.8 | n/a |
| abi_lifecycle_tight_null_entry | 28115.1 | 3267.2 | 3257.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.081 Gops/s** (abi_lifecycle_tight_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 0.000 | 0.2% |
| abi_lifecycle_tight_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_tight_held_handle | 0.000 | 0.2% |
| abi_lifecycle_tight_null_entry | 0.078 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 2017071ns | 2017071ns | +0.84% |
| abi_lifecycle_tight_fresh_per_column | 2021079ns | 2021079ns | +1.04% |
| abi_lifecycle_tight_held_handle | 2000185ns | 2000185ns | base |
| abi_lifecycle_tight_null_entry | 5579ns | 5579ns | -99.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_tight_held_handle | 2000835ns | base | --- | [1987350, 2004696] | --- | --- | --- | --- |
| abi_lifecycle_tight_fresh_per_batch | 2014295ns | +13462.1ns (+0.7%) | [+8194, +29199]ns | [2010218, 2019223] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_tight_fresh_per_column | 2015879ns | +19004.6ns (+0.9%) | [+13186, +30446]ns | [2012368, 2027269] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_tight_null_entry | 3280ns | -1997672.1ns (-99.8%) | [-2001365, -1984070]ns | [3163, 3331] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_tight_held_handle | abi_lifecycle_tight_fresh_per_batch | abi_lifecycle_tight_fresh_per_column | abi_lifecycle_tight_null_entry |
|---|---|---|---|---|
| 1 | 2004070ns | +0.4% | +0.5% | -99.8% |
| 2 | 1993047ns | +1.3% | +0.9% | -99.8% |
| 3 | 1999977ns | +0.4% | +0.8% | -99.8% |
| 4 | 1981653ns | +1.6% | +1.7% | -99.8% |
| 5 | 2001693ns | +0.6% | +1.0% | -99.8% |
| 6 | 2005321ns | +0.7% | +1.4% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | -0.412 | moderate- |
| abi_lifecycle_tight_fresh_per_column | 0.281 | moderate+ |
| abi_lifecycle_tight_held_handle | -0.279 | moderate- |
| abi_lifecycle_tight_null_entry | -0.607 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_lifecycle_tight_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_tight_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 6082194.2ns | 2014578.4ns | 301.9% | HIGH |
| abi_lifecycle_tight_fresh_per_column | 6087982.8ns | 2018505.5ns | 301.6% | HIGH |
| abi_lifecycle_tight_held_handle | 6031101.4ns | 1997626.8ns | 301.9% | HIGH |
| abi_lifecycle_tight_null_entry | 121755.5ns | 3257.8ns | 3737.4% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_tight_fresh_per_batch (n=6, range 2008572.5-2019222.7 ns)
  2008572.5 |########################################
  2009105.0 |
  2009637.5 |
  2010170.0 |
  2010702.5 |
  2011235.1 |
  2011767.6 |########################################
  2012300.1 |
  2012832.6 |
  2013365.1 |
  2013897.6 |########################################
  2014430.1 |########################################
  2014962.6 |
  2015495.1 |
  2016027.6 |
  2016560.2 |
  2017092.7 |
  2017625.2 |
  2018157.7 |
  2018690.2 |########################################
  (0 below, 1 above range)

abi_lifecycle_tight_fresh_per_column (n=6, range 2011165.4-2027269.1 ns)
  2011165.4 |########################################
  2011970.6 |
  2012775.8 |########################################
  2013581.0 |
  2014386.1 |########################################
  2015191.3 |
  2015996.5 |
  2016801.7 |########################################
  2017606.9 |
  2018412.1 |
  2019217.3 |
  2020022.5 |
  2020827.6 |########################################
  2021632.8 |
  2022438.0 |
  2023243.2 |
  2024048.4 |
  2024853.6 |
  2025658.8 |
  2026464.0 |
  (0 below, 1 above range)

abi_lifecycle_tight_held_handle (n=6, range 1981652.9-2004695.6 ns)
  1981652.9 |########################################
  1982805.0 |
  1983957.2 |
  1985109.3 |
  1986261.4 |
  1987413.6 |
  1988565.7 |
  1989717.8 |
  1990870.0 |
  1992022.1 |########################################
  1993174.2 |
  1994326.4 |
  1995478.5 |
  1996630.7 |
  1997782.8 |
  1998934.9 |########################################
  2000087.1 |
  2001239.2 |########################################
  2002391.3 |
  2003543.5 |########################################
  (0 below, 1 above range)

abi_lifecycle_tight_null_entry (n=6, range 3148.3-3330.8 ns)
   3148.3 |####################
   3157.4 |
   3166.6 |
   3175.7 |####################
   3184.8 |
   3193.9 |
   3203.1 |
   3212.2 |
   3221.3 |
   3230.4 |
   3239.6 |####################
   3248.7 |
   3257.8 |
   3267.0 |
   3276.1 |
   3285.2 |
   3294.3 |
   3303.5 |
   3312.6 |########################################
   3321.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_tight_fresh_per_batch**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_fresh_per_column**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_held_handle**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_null_entry**: bridge=3714.8% of algo (FFI overhead may distort results)
