# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_null dominates: 507% faster than the next best (abi_cross_cold_real_cold_null)

abi_cross_cold_real_warm_null (5.47 us) leads abi_cross_cold_real_cold_null (33.19 us) by 507%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_real_cold_scalar is an outlier: 404.2x slower than the field

abi_cross_cold_real_cold_scalar (2.21 ms) is 404.2x the fastest (5.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (5.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} (6494% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} with a 6494% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 404.2x the fastest

Fastest abi_cross_cold_real_warm_null (5.47 us) to slowest abi_cross_cold_real_cold_scalar (2.21 ms): 404.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 5469.1 ns median
- 3 variants significantly slower than baseline
- Spread: 404.21x (fastest 5469.1 ns, slowest 2210698.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 35757ns | 35511ns | 34512ns | 35287ns | 37083ns | +356.44% |
| abi_cross_cold_real_cold_scalar | 2212072ns | 2213979ns | 2197504ns | 2210339ns | 2221955ns | +28137.65% |
| abi_cross_cold_real_warm_null | 7834ns | 7902ns | 7508ns | 7820ns | 8017ns | base |
| abi_cross_cold_real_warm_scalar | 2202536ns | 2191859ns | 2180768ns | 2190678ns | 2231206ns | +28015.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 33451ns | 32272ns | 34742ns | +511.30% | 0.000 |
| abi_cross_cold_real_cold_scalar | 2208658ns | 2194328ns | 2218155ns | +40262.42% | 0.000 |
| abi_cross_cold_real_warm_null | 5472ns | 5332ns | 5609ns | base | 0.000 |
| abi_cross_cold_real_warm_scalar | 2199094ns | 2177422ns | 2227584ns | +40087.63% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 34500.2 | 37567.5 | 33450.9 | n/a |
| abi_cross_cold_real_cold_scalar | 73719.3 | 2211730.6 | 2208658.3 | n/a |
| abi_cross_cold_real_warm_null | 28508.0 | 5541.2 | 5472.1 | n/a |
| abi_cross_cold_real_warm_scalar | 70267.8 | 2199807.5 | 2199093.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.000 | 16.1% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_real_warm_null | 0.000 | 97.5% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 35757ns | 35757ns | +356.44% |
| abi_cross_cold_real_cold_scalar | 2212072ns | 2212072ns | +28137.65% |
| abi_cross_cold_real_warm_null | 7834ns | 7834ns | base |
| abi_cross_cold_real_warm_scalar | 2202536ns | 2202536ns | +28015.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 5469ns | base | --- | [5338, 5609] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 33190ns | +27851.9ns (+509.3%) | [+26906, +29178]ns | [32421, 34742] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2210698ns | +2205183.2ns (+40320.4%) | [+2191783, +2212592]ns | [2197121, 2218155] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2188506ns | +2183168.0ns (+39917.9%) | [+2175677, +2222020]ns | [2181192, 2227584] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 5332ns | +516.1% | +41053.2% | +40912.2% |
| 2 | 5643ns | +477.2% | +39048.0% | +38620.6% |
| 3 | 5551ns | +522.7% | +39866.0% | +39440.1% |
| 4 | 5576ns | +526.2% | +39676.8% | +40439.0% |
| 5 | 5388ns | +499.0% | +40963.8% | +40316.2% |
| 6 | 5343ns | +527.5% | +41071.5% | +40889.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.108 | ok |
| abi_cross_cold_real_cold_scalar | 0.189 | ok |
| abi_cross_cold_real_warm_null | -0.002 | ok |
| abi_cross_cold_real_warm_scalar | -0.249 | moderate- |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 214968.7ns | 33450.9ns | 642.6% | HIGH |
| abi_cross_cold_real_cold_scalar | 6711454.6ns | 2208658.3ns | 303.9% | HIGH |
| abi_cross_cold_real_warm_null | 127695.1ns | 5472.1ns | 2333.6% | HIGH |
| abi_cross_cold_real_warm_scalar | 6664148.0ns | 2199093.9ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 32272.1-34741.7 ns)
  32272.1 |########################################
  32395.6 |
  32519.1 |########################################
  32642.5 |
  32766.0 |########################################
  32889.5 |
  33013.0 |
  33136.5 |
  33259.9 |
  33383.4 |
  33506.9 |########################################
  33630.4 |
  33753.9 |
  33877.3 |
  34000.8 |
  34124.3 |
  34247.8 |
  34371.3 |
  34494.7 |########################################
  34618.2 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2194327.5-2218155.4 ns)
  2194327.5 |########################################
  2195518.9 |
  2196710.3 |
  2197901.7 |
  2199093.1 |########################################
  2200284.5 |
  2201475.9 |
  2202667.3 |
  2203858.7 |
  2205050.1 |
  2206241.5 |
  2207432.8 |
  2208624.2 |########################################
  2209815.6 |
  2211007.0 |
  2212198.4 |########################################
  2213389.8 |
  2214581.2 |
  2215772.6 |
  2216964.0 |########################################
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 5332.1-5609.4 ns)
   5332.1 |########################################
   5346.0 |
   5359.8 |
   5373.7 |####################
   5387.6 |
   5401.4 |
   5415.3 |
   5429.1 |
   5443.0 |
   5456.9 |
   5470.7 |
   5484.6 |
   5498.5 |
   5512.3 |
   5526.2 |
   5540.0 |####################
   5553.9 |
   5567.8 |####################
   5581.6 |
   5595.5 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2177421.7-2227583.5 ns)
  2177421.7 |####################
  2179929.8 |
  2182437.9 |
  2184946.0 |########################################
  2187454.1 |
  2189962.2 |####################
  2192470.3 |####################
  2194978.3 |
  2197486.4 |
  2199994.5 |
  2202502.6 |
  2205010.7 |
  2207518.8 |
  2210026.9 |
  2212535.0 |
  2215043.1 |
  2217551.2 |
  2220059.3 |
  2222567.4 |
  2225075.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=646.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=2339.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=303.5% of algo (FFI overhead may distort results)
