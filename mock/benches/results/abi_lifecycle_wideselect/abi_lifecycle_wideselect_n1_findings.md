# abi_lifecycle (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_wideselect_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_wideselect_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_wideselect_null_entry dominates: 43073% faster than the next best (abi_lifecycle_wideselect_held_handle)

abi_lifecycle_wideselect_null_entry (4.78 us) leads abi_lifecycle_wideselect_held_handle (2.06 ms) by 43073%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_wideselect_null_entry beats baseline by 100% (significant)

abi_lifecycle_wideselect_null_entry is -2.06 ms (100%) faster than baseline abi_lifecycle_wideselect_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_wideselect_fresh_per_batch is an outlier: 1106.2x slower than the field

abi_lifecycle_wideselect_fresh_per_batch (5.29 ms) is 1106.2x the fastest (4.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_wideselect_null_entry} vs {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} (43073% apart)

The field splits into a fast tier {abi_lifecycle_wideselect_null_entry} and a slow tier {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} with a 43073% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1106.2x the fastest

Fastest abi_lifecycle_wideselect_null_entry (4.78 us) to slowest abi_lifecycle_wideselect_fresh_per_batch (5.29 ms): 1106.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_wideselect_null_entry** at 4779.1 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1106.22x (fastest 4779.1 ns, slowest 5286801.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 5501268ns | 5289636ns | 5262486ns | 5283506ns | 5947302ns | +165.58% |
| abi_lifecycle_wideselect_fresh_per_column | 2078731ns | 2080634ns | 2069702ns | 2078094ns | 2084202ns | +0.35% |
| abi_lifecycle_wideselect_held_handle | 2071405ns | 2065821ns | 2059997ns | 2064196ns | 2087921ns | base |
| abi_lifecycle_wideselect_null_entry | 7132ns | 6985ns | 6895ns | 6968ns | 7497ns | -99.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 5497713ns | 5259774ns | 5942253ns | +165.77% | 0.000 |
| abi_lifecycle_wideselect_fresh_per_column | 2076203ns | 2067300ns | 2081554ns | +0.37% | 0.000 |
| abi_lifecycle_wideselect_held_handle | 2068599ns | 2057343ns | 2084522ns | base | 0.000 |
| abi_lifecycle_wideselect_null_entry | 4859ns | 4715ns | 5058ns | -99.77% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 55242.9 | 5312088.0 | 5497713.0 | n/a |
| abi_lifecycle_wideselect_fresh_per_column | 37089.2 | 2076717.4 | 2076203.5 | 0 |
| abi_lifecycle_wideselect_held_handle | 39624.0 | 2071008.7 | 2068599.0 | n/a |
| abi_lifecycle_wideselect_null_entry | 27517.3 | 4924.0 | 4859.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_lifecycle_wideselect_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_wideselect_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_wideselect_held_handle | 0.000 | 0.2% |
| abi_lifecycle_wideselect_null_entry | 0.000 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 5501268ns | 5501268ns | +165.58% |
| abi_lifecycle_wideselect_fresh_per_column | 2078731ns | 2078731ns | +0.35% |
| abi_lifecycle_wideselect_held_handle | 2071405ns | 2071405ns | base |
| abi_lifecycle_wideselect_null_entry | 7132ns | 7132ns | -99.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_held_handle | 2063296ns | base | --- | [2057978, 2084522] | --- | --- | --- | --- |
| abi_lifecycle_wideselect_fresh_per_batch | 5286802ns | +3222437.1ns (+156.2%) | [+3206106, +3858799]ns | [5264084, 5942253] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_wideselect_fresh_per_column | 2078220ns | no significant difference | [-10460, +18862]ns | [2068837, 2081554] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_wideselect_null_entry | 4779ns | -2058520.2ns (-99.8%) | [-2079465, -2053235]ns | [4740, 5058] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_wideselect_held_handle | abi_lifecycle_wideselect_fresh_per_batch | abi_lifecycle_wideselect_fresh_per_column | abi_lifecycle_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2057343ns | +156.1% | +0.9% | -99.8% |
| 2 | 2062968ns | +157.6% | +0.8% | -99.8% |
| 3 | 2058613ns | +155.5% | +0.6% | -99.8% |
| 4 | 2065105ns | +155.6% | +0.1% | -99.7% |
| 5 | 2103940ns | +212.3% | -1.1% | -99.8% |
| 6 | 2063624ns | +156.6% | +0.9% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | -0.227 | moderate- |
| abi_lifecycle_wideselect_fresh_per_column | 0.093 | ok |
| abi_lifecycle_wideselect_held_handle | -0.094 | ok |
| abi_lifecycle_wideselect_null_entry | -0.021 | ok |

**Consistency summary:**

- **abi_lifecycle_wideselect_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 16049821.0ns | 5497713.0ns | 291.9% | HIGH |
| abi_lifecycle_wideselect_fresh_per_column | 6268088.8ns | 2076203.5ns | 301.9% | HIGH |
| abi_lifecycle_wideselect_held_handle | 6256770.7ns | 2068599.0ns | 302.5% | HIGH |
| abi_lifecycle_wideselect_null_entry | 124247.6ns | 4859.0ns | 2557.1% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_wideselect_fresh_per_batch (n=6, range 5259774.2-5942253.3 ns)
  5259774.2 |########################################
  5293898.2 |##########################
  5328022.1 |
  5362146.1 |
  5396270.0 |
  5430394.0 |
  5464517.9 |
  5498641.9 |
  5532765.9 |
  5566889.8 |
  5601013.8 |
  5635137.7 |
  5669261.7 |
  5703385.6 |
  5737509.6 |
  5771633.6 |
  5805757.5 |
  5839881.5 |
  5874005.4 |
  5908129.4 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_fresh_per_column (n=6, range 2067300.0-2081553.9 ns)
  2067300.0 |########################################
  2068012.7 |
  2068725.4 |
  2069438.1 |
  2070150.8 |########################################
  2070863.5 |
  2071576.2 |
  2072288.9 |
  2073001.6 |
  2073714.3 |
  2074427.0 |
  2075139.7 |
  2075852.4 |########################################
  2076565.1 |
  2077277.8 |
  2077990.5 |
  2078703.2 |
  2079415.9 |########################################
  2080128.6 |########################################
  2080841.3 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_held_handle (n=6, range 2057343.3-2084522.3 ns)
  2057343.3 |########################################
  2058702.2 |
  2060061.2 |
  2061420.2 |
  2062779.1 |########################################
  2064138.1 |####################
  2065497.0 |
  2066855.9 |
  2068214.9 |
  2069573.9 |
  2070932.8 |
  2072291.8 |
  2073650.7 |
  2075009.7 |
  2076368.6 |
  2077727.6 |
  2079086.5 |
  2080445.4 |
  2081804.4 |
  2083163.4 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_null_entry (n=6, range 4715.4-5057.7 ns)
   4715.4 |########################################
   4732.5 |
   4749.6 |########################################
   4766.7 |########################################
   4783.9 |########################################
   4801.0 |
   4818.1 |
   4835.2 |
   4852.3 |
   4869.4 |
   4886.6 |########################################
   4903.7 |
   4920.8 |
   4937.9 |
   4955.0 |
   4972.1 |
   4989.2 |
   5006.4 |
   5023.5 |
   5040.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_wideselect_fresh_per_batch**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_fresh_per_column**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_null_entry**: bridge=2600.0% of algo (FFI overhead may distort results)
