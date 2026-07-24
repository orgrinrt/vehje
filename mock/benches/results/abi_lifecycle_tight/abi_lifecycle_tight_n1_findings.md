# abi_lifecycle (tight)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_tight_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_tight_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_tight_null_entry dominates: 41901% faster than the next best (abi_lifecycle_tight_held_handle)

abi_lifecycle_tight_null_entry (4.82 us) leads abi_lifecycle_tight_held_handle (2.02 ms) by 41901%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_tight_null_entry beats baseline by 100% (significant)

abi_lifecycle_tight_null_entry is -2.02 ms (100%) faster than baseline abi_lifecycle_tight_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_tight_fresh_per_batch is an outlier: 1106.8x slower than the field

abi_lifecycle_tight_fresh_per_batch (5.33 ms) is 1106.8x the fastest (4.82 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_tight_null_entry shows alternating (throttle bounce) (autocorr -0.76)

abi_lifecycle_tight_null_entry's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_tight_null_entry} vs {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} (41901% apart)

The field splits into a fast tier {abi_lifecycle_tight_null_entry} and a slow tier {abi_lifecycle_tight_held_handle, abi_lifecycle_tight_fresh_per_column, abi_lifecycle_tight_fresh_per_batch} with a 41901% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1106.8x the fastest

Fastest abi_lifecycle_tight_null_entry (4.82 us) to slowest abi_lifecycle_tight_fresh_per_batch (5.33 ms): 1106.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_tight_null_entry** at 4819.0 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1106.78x (fastest 4819.0 ns, slowest 5333507.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 5417452ns | 5336398ns | 5324792ns | 5333137ns | 5590256ns | +167.20% |
| abi_lifecycle_tight_fresh_per_column | 2044384ns | 2034252ns | 2030455ns | 2033723ns | 2067340ns | +0.83% |
| abi_lifecycle_tight_held_handle | 2027504ns | 2026654ns | 2016148ns | 2024102ns | 2038286ns | base |
| abi_lifecycle_tight_null_entry | 7091ns | 7046ns | 6921ns | 7007ns | 7302ns | -99.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 5414447ns | 5322155ns | 5586903ns | +167.40% | 0.000 |
| abi_lifecycle_tight_fresh_per_column | 2041806ns | 2027865ns | 2064698ns | +0.84% | 0.000 |
| abi_lifecycle_tight_held_handle | 2024874ns | 2013543ns | 2035585ns | base | 0.000 |
| abi_lifecycle_tight_null_entry | 4844ns | 4750ns | 4964ns | -99.76% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 55968.5 | 5405062.3 | 5414447.2 | n/a |
| abi_lifecycle_tight_fresh_per_column | 41637.3 | 2043074.2 | 2041805.9 | n/a |
| abi_lifecycle_tight_held_handle | 39145.1 | 2024313.8 | 2024874.0 | n/a |
| abi_lifecycle_tight_null_entry | 27309.0 | 4899.6 | 4844.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_lifecycle_tight_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_tight_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_tight_held_handle | 0.000 | 0.2% |
| abi_lifecycle_tight_null_entry | 0.000 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 5417452ns | 5417452ns | +167.20% |
| abi_lifecycle_tight_fresh_per_column | 2044384ns | 2044384ns | +0.83% |
| abi_lifecycle_tight_held_handle | 2027504ns | 2027504ns | base |
| abi_lifecycle_tight_null_entry | 7091ns | 7091ns | -99.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_tight_held_handle | 2024016ns | base | --- | [2015021, 2035585] | --- | --- | --- | --- |
| abi_lifecycle_tight_fresh_per_batch | 5333508ns | +3307425.0ns (+163.4%) | [+3292334, +3568961]ns | [5322931, 5586903] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_tight_fresh_per_column | 2031711ns | no significant difference | [-5542, +47754]ns | [2029009, 2064698] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_tight_null_entry | 4819ns | -2019265.2ns (-99.8%) | [-2030676, -2010147]ns | [4750, 4964] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_tight_held_handle | abi_lifecycle_tight_fresh_per_batch | abi_lifecycle_tight_fresh_per_column | abi_lifecycle_tight_null_entry |
|---|---|---|---|---|
| 1 | 2040273ns | +161.3% | -0.6% | -99.8% |
| 2 | 2013543ns | +164.3% | +0.9% | -99.8% |
| 3 | 2030897ns | +162.1% | +0.1% | -99.8% |
| 4 | 2028647ns | +163.0% | +0.1% | -99.8% |
| 5 | 2016500ns | +188.3% | +3.9% | -99.8% |
| 6 | 2019385ns | +165.4% | +0.8% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | -0.157 | ok |
| abi_lifecycle_tight_fresh_per_column | -0.181 | ok |
| abi_lifecycle_tight_held_handle | -0.398 | moderate- |
| abi_lifecycle_tight_null_entry | -0.759 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_lifecycle_tight_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_tight_fresh_per_column**: won 1/6, lost 3/6
- **abi_lifecycle_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_tight_fresh_per_batch | 16373927.1ns | 5414447.2ns | 302.4% | HIGH |
| abi_lifecycle_tight_fresh_per_column | 6174687.9ns | 2041805.9ns | 302.4% | HIGH |
| abi_lifecycle_tight_held_handle | 6114904.6ns | 2024874.0ns | 302.0% | HIGH |
| abi_lifecycle_tight_null_entry | 123470.0ns | 4844.3ns | 2548.8% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_tight_fresh_per_batch (n=6, range 5322154.6-5586902.9 ns)
  5322154.6 |########################################
  5335392.0 |
  5348629.4 |##########
  5361866.8 |
  5375104.3 |
  5388341.7 |
  5401579.1 |
  5414816.5 |
  5428053.9 |
  5441291.3 |
  5454528.8 |
  5467766.2 |
  5481003.6 |
  5494241.0 |
  5507478.4 |
  5520715.8 |
  5533953.2 |
  5547190.7 |
  5560428.1 |
  5573665.5 |
  (0 below, 1 above range)

abi_lifecycle_tight_fresh_per_column (n=6, range 2027864.6-2064697.7 ns)
  2027864.6 |####################
  2029706.3 |########################################
  2031547.9 |####################
  2033389.6 |####################
  2035231.2 |
  2037072.9 |
  2038914.5 |
  2040756.2 |
  2042597.8 |
  2044439.5 |
  2046281.2 |
  2048122.8 |
  2049964.5 |
  2051806.1 |
  2053647.8 |
  2055489.4 |
  2057331.1 |
  2059172.7 |
  2061014.4 |
  2062856.0 |
  (0 below, 1 above range)

abi_lifecycle_tight_held_handle (n=6, range 2013542.9-2035584.8 ns)
  2013542.9 |########################################
  2014645.0 |
  2015747.1 |########################################
  2016849.2 |
  2017951.3 |
  2019053.4 |########################################
  2020155.5 |
  2021257.6 |
  2022359.7 |
  2023461.8 |
  2024563.8 |
  2025665.9 |
  2026768.0 |
  2027870.1 |########################################
  2028972.2 |
  2030074.3 |########################################
  2031176.4 |
  2032278.5 |
  2033380.6 |
  2034482.7 |
  (0 below, 1 above range)

abi_lifecycle_tight_null_entry (n=6, range 4750.0-4963.8 ns)
   4750.0 |########################################
   4760.7 |
   4771.4 |
   4782.1 |
   4792.8 |
   4803.4 |
   4814.1 |
   4824.8 |
   4835.5 |
   4846.2 |
   4856.9 |
   4867.6 |
   4878.2 |#############
   4888.9 |
   4899.6 |
   4910.3 |
   4921.0 |#############
   4931.7 |
   4942.4 |
   4953.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_tight_fresh_per_batch**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_fresh_per_column**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_held_handle**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_lifecycle_tight_null_entry**: bridge=2554.2% of algo (FFI overhead may distort results)
