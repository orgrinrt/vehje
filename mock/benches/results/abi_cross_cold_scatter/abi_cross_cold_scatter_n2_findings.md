# abi_cross_cold (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_scatter_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_scatter_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_scatter_warm_null dominates: 124% faster than the next best (abi_cross_cold_scatter_cold_null)

abi_cross_cold_scatter_warm_null (3.46 us) leads abi_cross_cold_scatter_cold_null (7.73 us) by 124%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_scatter_cold_scalar is an outlier: 651.9x slower than the field

abi_cross_cold_scatter_cold_scalar (2.25 ms) is 651.9x the fastest (3.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_scatter_warm_null)

The baseline abi_cross_cold_scatter_warm_null is the fastest (3.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} vs {abi_cross_cold_scatter_warm_scalar, abi_cross_cold_scatter_cold_scalar} (28979% apart)

The field splits into a fast tier {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} and a slow tier {abi_cross_cold_scatter_warm_scalar, abi_cross_cold_scatter_cold_scalar} with a 28979% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 651.9x the fastest

Fastest abi_cross_cold_scatter_warm_null (3.46 us) to slowest abi_cross_cold_scatter_cold_scalar (2.25 ms): 651.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_scatter_cold_null is inconsistent: worst-20% is 1.7x its best-20%

abi_cross_cold_scatter_cold_null's best 20% of batches run at 7.61 us but its worst 20% at 12.91 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (abi_cross_cold_scatter_warm_null) is the fastest** at 3457.2 ns median
- 3 variants significantly slower than baseline
- Spread: 651.94x (fastest 3457.2 ns, slowest 2253911.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 11959ns | 10143ns | 9861ns | 10081ns | 15826ns | +108.21% |
| abi_cross_cold_scatter_cold_scalar | 2280485ns | 2258122ns | 2242550ns | 2254559ns | 2338341ns | +39603.30% |
| abi_cross_cold_scatter_warm_null | 5744ns | 5761ns | 5610ns | 5749ns | 5803ns | base |
| abi_cross_cold_scatter_warm_scalar | 2249685ns | 2253156ns | 2239077ns | 2249114ns | 2255846ns | +39067.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 9419ns | 7610ns | 12906ns | +171.85% | 0.000 |
| abi_cross_cold_scatter_cold_scalar | 2276252ns | 2238494ns | 2333919ns | +65599.64% | 0.000 |
| abi_cross_cold_scatter_warm_null | 3465ns | 3388ns | 3519ns | base | 0.001 |
| abi_cross_cold_scatter_warm_scalar | 2245450ns | 2234722ns | 2251574ns | +64710.62% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 40316.1 | 12342.6 | 9418.7 | n/a |
| abi_cross_cold_scatter_cold_scalar | 98816.6 | 2266262.0 | 2276251.6 | n/a |
| abi_cross_cold_scatter_warm_null | 28651.4 | 3556.7 | 3464.6 | n/a |
| abi_cross_cold_scatter_warm_scalar | 92485.6 | 2239244.6 | 2245450.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_scatter_warm_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.000 | 43.8% |
| abi_cross_cold_scatter_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_scatter_warm_null | 0.001 | 98.0% |
| abi_cross_cold_scatter_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 11959ns | 11959ns | +108.21% |
| abi_cross_cold_scatter_cold_scalar | 2280485ns | 2280485ns | +39603.30% |
| abi_cross_cold_scatter_warm_null | 5744ns | 5744ns | base |
| abi_cross_cold_scatter_warm_scalar | 2249685ns | 2249685ns | +39067.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_warm_null | 3457ns | base | --- | [3418, 3519] | --- | --- | --- | --- |
| abi_cross_cold_scatter_cold_null | 7734ns | +4277.1ns (+123.7%) | [+4162, +9423]ns | [7616, 12906] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_scatter_cold_scalar | 2253911ns | +2250421.7ns (+65092.8%) | [+2237507, +2330432]ns | [2240925, 2333919] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_scatter_warm_scalar | 2249003ns | +2245509.4ns (+64950.7%) | [+2232354, +2248094]ns | [2235774, 2251574] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_scatter_warm_null | abi_cross_cold_scatter_cold_null | abi_cross_cold_scatter_cold_scalar | abi_cross_cold_scatter_warm_scalar |
|---|---|---|---|---|
| 1 | 3453ns | +128.4% | +65220.6% | +64673.6% |
| 2 | 3388ns | +125.6% | +65965.4% | +65854.1% |
| 3 | 3526ns | +122.0% | +63775.0% | +63690.1% |
| 4 | 3461ns | +120.2% | +65864.1% | +64874.5% |
| 5 | 3512ns | +410.4% | +67807.0% | +64046.3% |
| 6 | 3448ns | +120.7% | +64972.0% | +65179.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_scatter_cold_null | -0.256 | moderate- |
| abi_cross_cold_scatter_cold_scalar | -0.085 | ok |
| abi_cross_cold_scatter_warm_null | -0.408 | moderate- |
| abi_cross_cold_scatter_warm_scalar | 0.435 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_scatter_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 129043.5ns | 9418.7ns | 1370.1% | HIGH |
| abi_cross_cold_scatter_cold_scalar | 6906915.8ns | 2276251.6ns | 303.4% | HIGH |
| abi_cross_cold_scatter_warm_null | 121563.8ns | 3464.6ns | 3508.7% | HIGH |
| abi_cross_cold_scatter_warm_scalar | 6828473.2ns | 2245450.3ns | 304.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_scatter_cold_null (n=6, range 7609.6-12905.9 ns)
   7609.6 |########################################
   7874.4 |##########
   8139.2 |
   8404.0 |
   8668.9 |
   8933.7 |
   9198.5 |
   9463.3 |
   9728.1 |
   9992.9 |
  10257.7 |
  10522.5 |
  10787.4 |
  11052.2 |
  11317.0 |
  11581.8 |
  11846.6 |
  12111.4 |
  12376.2 |
  12641.0 |
  (0 below, 1 above range)

abi_cross_cold_scatter_cold_scalar (n=6, range 2238494.2-2333918.5 ns)
  2238494.2 |########################################
  2243265.4 |########################################
  2248036.6 |########################################
  2252807.9 |########################################
  2257579.1 |
  2262350.3 |
  2267121.5 |
  2271892.7 |
  2276663.9 |
  2281435.2 |########################################
  2286206.4 |
  2290977.6 |
  2295748.8 |
  2300520.0 |
  2305291.2 |
  2310062.5 |
  2314833.7 |
  2319604.9 |
  2324376.1 |
  2329147.3 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_null (n=6, range 3388.3-3518.8 ns)
   3388.3 |####################
   3394.8 |
   3401.3 |
   3407.9 |
   3414.4 |
   3420.9 |
   3427.4 |
   3434.0 |
   3440.5 |
   3447.0 |########################################
   3453.5 |
   3460.0 |####################
   3466.6 |
   3473.1 |
   3479.6 |
   3486.1 |
   3492.7 |
   3499.2 |
   3505.7 |####################
   3512.2 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_scalar (n=6, range 2234722.1-2251573.5 ns)
  2234722.1 |########################################
  2235564.7 |
  2236407.2 |########################################
  2237249.8 |
  2238092.4 |
  2238935.0 |
  2239777.5 |
  2240620.1 |
  2241462.7 |
  2242305.3 |
  2243147.8 |
  2243990.4 |
  2244833.0 |
  2245675.5 |
  2246518.1 |
  2247360.7 |
  2248203.3 |########################################
  2249045.8 |########################################
  2249888.4 |########################################
  2250731.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_scatter_cold_null**: CV=40.4% (high variance, measurements may be unstable)
- **abi_cross_cold_scatter_cold_null**: bridge=1505.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_cold_scalar**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_null**: bridge=3525.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_scalar**: bridge=303.3% of algo (FFI overhead may distort results)
