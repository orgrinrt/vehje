# abi_cross_cold (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_tight_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_tight_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_tight_warm_null dominates: 29% faster than the next best (abi_cross_cold_tight_cold_null)

abi_cross_cold_tight_warm_null (4.03 us) leads abi_cross_cold_tight_cold_null (5.20 us) by 29%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_tight_cold_scalar is an outlier: 522.4x slower than the field

abi_cross_cold_tight_cold_scalar (2.11 ms) is 522.4x the fastest (4.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_tight_cold_scalar shows alternating (throttle bounce) (autocorr -0.61)

abi_cross_cold_tight_cold_scalar's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_tight_warm_null)

The baseline abi_cross_cold_tight_warm_null is the fastest (4.03 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} vs {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} (40153% apart)

The field splits into a fast tier {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} and a slow tier {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} with a 40153% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 522.4x the fastest

Fastest abi_cross_cold_tight_warm_null (4.03 us) to slowest abi_cross_cold_tight_cold_scalar (2.11 ms): 522.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_tight_warm_null) is the fastest** at 4033.1 ns median
- 3 variants significantly slower than baseline
- Spread: 522.45x (fastest 4033.1 ns, slowest 2107085.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 7498ns | 7515ns | 7262ns | 7490ns | 7627ns | +16.45% |
| abi_cross_cold_tight_cold_scalar | 2135027ns | 2111032ns | 2053182ns | 2105135ns | 2220788ns | +33058.78% |
| abi_cross_cold_tight_warm_null | 6439ns | 6337ns | 6215ns | 6296ns | 6765ns | base |
| abi_cross_cold_tight_warm_scalar | 2111459ns | 2098422ns | 2052391ns | 2085020ns | 2180652ns | +32692.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 5192ns | 5031ns | 5281ns | +26.88% | 0.001 |
| abi_cross_cold_tight_cold_scalar | 2130948ns | 2049501ns | 2216340ns | +51974.25% | 0.000 |
| abi_cross_cold_tight_warm_null | 4092ns | 3949ns | 4280ns | base | 0.001 |
| abi_cross_cold_tight_warm_scalar | 2107466ns | 2048683ns | 2176159ns | +51400.41% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 33323.8 | 7153.1 | 5192.3 | n/a |
| abi_cross_cold_tight_cold_scalar | 84893.5 | 2130673.8 | 2130947.6 | n/a |
| abi_cross_cold_tight_warm_null | 30064.5 | 4186.5 | 4092.1 | n/a |
| abi_cross_cold_tight_warm_scalar | 77904.9 | 2103032.6 | 2107465.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_tight_warm_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.001 | 75.9% |
| abi_cross_cold_tight_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_tight_warm_null | 0.001 | 97.9% |
| abi_cross_cold_tight_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_tight_cold_null | 7498ns | 7498ns | +16.45% |
| abi_cross_cold_tight_cold_scalar | 2135027ns | 2135027ns | +33058.78% |
| abi_cross_cold_tight_warm_null | 6439ns | 6439ns | base |
| abi_cross_cold_tight_warm_scalar | 2111459ns | 2111459ns | +32692.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_tight_warm_null | 4033ns | base | --- | [3963, 4280] | --- | --- | --- | --- |
| abi_cross_cold_tight_cold_null | 5203ns | +1151.7ns (+28.6%) | [+931, +1217]ns | [5092, 5281] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_tight_cold_scalar | 2107085ns | +2103099.8ns (+52146.0%) | [+2065174, +2212292]ns | [2069417, 2216340] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_tight_warm_scalar | 2094498ns | +2090497.5ns (+51833.5%) | [+2047740, +2171883]ns | [2051740, 2176159] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_tight_warm_null | abi_cross_cold_tight_cold_null | abi_cross_cold_tight_cold_scalar | abi_cross_cold_tight_warm_scalar |
|---|---|---|---|---|
| 1 | 3949ns | +27.4% | +53008.8% | +52017.2% |
| 2 | 4507ns | +17.3% | +46256.5% | +48763.0% |
| 3 | 4053ns | +27.8% | +55975.6% | +52480.8% |
| 4 | 3978ns | +29.6% | +51422.2% | +51401.6% |
| 5 | 4044ns | +29.2% | +53316.2% | +53069.4% |
| 6 | 4022ns | +31.1% | +52530.5% | +50982.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_tight_cold_null | -0.327 | moderate- |
| abi_cross_cold_tight_cold_scalar | -0.614 | HIGH- (thermal bounce) |
| abi_cross_cold_tight_warm_null | -0.291 | moderate- |
| abi_cross_cold_tight_warm_scalar | -0.428 | moderate- |

**Consistency summary:**

- **abi_cross_cold_tight_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_tight_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_tight_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 116072.9ns | 5192.3ns | 2235.5% | HIGH |
| abi_cross_cold_tight_cold_scalar | 6467772.9ns | 2130947.6ns | 303.5% | HIGH |
| abi_cross_cold_tight_warm_null | 125565.6ns | 4092.1ns | 3068.5% | HIGH |
| abi_cross_cold_tight_warm_scalar | 6405766.9ns | 2107465.6ns | 304.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_tight_cold_null (n=6, range 5030.8-5281.2 ns)
   5030.8 |########################################
   5043.3 |
   5055.8 |
   5068.4 |
   5080.9 |
   5093.4 |
   5105.9 |
   5118.5 |
   5131.0 |
   5143.5 |########################################
   5156.0 |
   5168.5 |########################################
   5181.1 |
   5193.6 |
   5206.1 |
   5218.6 |########################################
   5231.2 |
   5243.7 |
   5256.2 |
   5268.7 |########################################
  (0 below, 1 above range)

abi_cross_cold_tight_cold_scalar (n=6, range 2049501.2-2216340.5 ns)
  2049501.2 |########################################
  2057843.2 |
  2066185.1 |
  2074527.1 |
  2082869.1 |########################################
  2091211.0 |########################################
  2099553.0 |
  2107894.9 |
  2116236.9 |########################################
  2124578.9 |
  2132920.8 |
  2141262.8 |
  2149604.8 |
  2157946.7 |########################################
  2166288.7 |
  2174630.6 |
  2182972.6 |
  2191314.6 |
  2199656.5 |
  2207998.5 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_null (n=6, range 3948.7-4280.0 ns)
   3948.7 |########################################
   3965.3 |########################################
   3981.8 |
   3998.4 |
   4015.0 |########################################
   4031.5 |########################################
   4048.1 |########################################
   4064.7 |
   4081.2 |
   4097.8 |
   4114.4 |
   4130.9 |
   4147.5 |
   4164.0 |
   4180.6 |
   4197.2 |
   4213.7 |
   4230.3 |
   4246.9 |
   4263.4 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_scalar (n=6, range 2048683.3-2176158.5 ns)
  2048683.3 |########################################
  2055057.1 |####################
  2061430.8 |
  2067804.6 |
  2074178.4 |
  2080552.1 |
  2086925.9 |
  2093299.6 |
  2099673.4 |
  2106047.2 |
  2112420.9 |
  2118794.7 |
  2125168.4 |####################
  2131542.2 |
  2137916.0 |
  2144289.7 |####################
  2150663.5 |
  2157037.3 |
  2163411.0 |
  2169784.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_tight_cold_null**: bridge=2230.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_cold_scalar**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_null**: bridge=3051.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_scalar**: bridge=304.5% of algo (FFI overhead may distort results)
