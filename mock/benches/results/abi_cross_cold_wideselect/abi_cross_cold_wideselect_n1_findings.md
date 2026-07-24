# abi_cross_cold (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_wideselect_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_wideselect_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_wideselect_warm_null dominates: 168% faster than the next best (abi_cross_cold_wideselect_cold_null)

abi_cross_cold_wideselect_warm_null (4.92 us) leads abi_cross_cold_wideselect_cold_null (13.18 us) by 168%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_wideselect_warm_scalar is an outlier: 429.7x slower than the field

abi_cross_cold_wideselect_warm_scalar (2.11 ms) is 429.7x the fastest (4.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_wideselect_warm_null)

The baseline abi_cross_cold_wideselect_warm_null is the fastest (4.92 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} vs {abi_cross_cold_wideselect_cold_scalar, abi_cross_cold_wideselect_warm_scalar} (15923% apart)

The field splits into a fast tier {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} and a slow tier {abi_cross_cold_wideselect_cold_scalar, abi_cross_cold_wideselect_warm_scalar} with a 15923% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 429.7x the fastest

Fastest abi_cross_cold_wideselect_warm_null (4.92 us) to slowest abi_cross_cold_wideselect_warm_scalar (2.11 ms): 429.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_wideselect_warm_null) is the fastest** at 4917.1 ns median
- 3 variants significantly slower than baseline
- Spread: 429.70x (fastest 4917.1 ns, slowest 2112866.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 15394ns | 15458ns | 15105ns | 15354ns | 15599ns | +112.40% |
| abi_cross_cold_wideselect_cold_scalar | 2113853ns | 2115353ns | 2103589ns | 2112578ns | 2120898ns | +29065.04% |
| abi_cross_cold_wideselect_warm_null | 7248ns | 7242ns | 7052ns | 7223ns | 7383ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2130641ns | 2116739ns | 2108474ns | 2114433ns | 2166035ns | +29296.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 13121ns | 12901ns | 13262ns | +167.58% | 0.000 |
| abi_cross_cold_wideselect_cold_scalar | 2110159ns | 2099973ns | 2117121ns | +42932.71% | 0.000 |
| abi_cross_cold_wideselect_warm_null | 4904ns | 4786ns | 4985ns | base | 0.000 |
| abi_cross_cold_wideselect_warm_scalar | 2126708ns | 2104732ns | 2161886ns | +43270.18% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 34998.6 | 19063.3 | 13120.9 | n/a |
| abi_cross_cold_wideselect_cold_scalar | 83567.3 | 2111245.3 | 2110159.2 | n/a |
| abi_cross_cold_wideselect_warm_null | 30484.5 | 5063.3 | 4903.6 | n/a |
| abi_cross_cold_wideselect_warm_scalar | 80143.9 | 2139509.9 | 2126707.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_cold_wideselect_warm_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.000 | 36.3% |
| abi_cross_cold_wideselect_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_wideselect_warm_null | 0.000 | 97.3% |
| abi_cross_cold_wideselect_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 15394ns | 15394ns | +112.40% |
| abi_cross_cold_wideselect_cold_scalar | 2113853ns | 2113853ns | +29065.04% |
| abi_cross_cold_wideselect_warm_null | 7248ns | 7248ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2130641ns | 2130641ns | +29296.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_warm_null | 4917ns | base | --- | [4809, 4985] | --- | --- | --- | --- |
| abi_cross_cold_wideselect_cold_null | 13179ns | +8232.9ns (+167.4%) | [+8113, +8306]ns | [12922, 13262] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_cold_scalar | 2111675ns | +2106782.5ns (+42846.0%) | [+2096765, +2112220]ns | [2101682, 2117121] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_warm_scalar | 2112866ns | +2108008.8ns (+42871.0%) | [+2100502, +2156901]ns | [2105371, 2161886] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_wideselect_warm_null | abi_cross_cold_wideselect_cold_null | abi_cross_cold_wideselect_cold_scalar | abi_cross_cold_wideselect_warm_scalar |
|---|---|---|---|---|
| 1 | 4999ns | +165.8% | +42163.0% | +44076.9% |
| 2 | 4970ns | +166.3% | +42563.6% | +42457.5% |
| 3 | 4905ns | +169.7% | +42779.1% | +42806.4% |
| 4 | 4786ns | +169.6% | +44000.0% | +44058.3% |
| 5 | 4929ns | +166.4% | +42506.2% | +42758.4% |
| 6 | 4832ns | +167.9% | +43642.7% | +43483.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.039 | ok |
| abi_cross_cold_wideselect_cold_scalar | -0.307 | moderate- |
| abi_cross_cold_wideselect_warm_null | 0.046 | ok |
| abi_cross_cold_wideselect_warm_scalar | 0.012 | ok |

**Consistency summary:**

- **abi_cross_cold_wideselect_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 142796.3ns | 13120.9ns | 1088.3% | HIGH |
| abi_cross_cold_wideselect_cold_scalar | 6421260.7ns | 2110159.2ns | 304.3% | HIGH |
| abi_cross_cold_wideselect_warm_null | 127376.4ns | 4903.6ns | 2597.6% | HIGH |
| abi_cross_cold_wideselect_warm_scalar | 6491589.1ns | 2126707.6ns | 305.2% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_wideselect_cold_null (n=6, range 12901.2-13261.7 ns)
  12901.2 |####################
  12919.2 |
  12937.2 |####################
  12955.3 |
  12973.3 |
  12991.3 |
  13009.3 |
  13027.4 |
  13045.4 |
  13063.4 |
  13081.4 |
  13099.4 |
  13117.5 |####################
  13135.5 |
  13153.5 |
  13171.5 |
  13189.6 |
  13207.6 |
  13225.6 |########################################
  13243.6 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_cold_scalar (n=6, range 2099972.9-2117120.9 ns)
  2099972.9 |########################################
  2100830.3 |
  2101687.7 |
  2102545.1 |########################################
  2103402.5 |
  2104259.9 |
  2105117.3 |
  2105974.7 |
  2106832.1 |
  2107689.5 |
  2108546.9 |
  2109404.3 |
  2110261.7 |########################################
  2111119.1 |
  2111976.5 |########################################
  2112833.9 |
  2113691.3 |########################################
  2114548.7 |
  2115406.1 |
  2116263.5 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_null (n=6, range 4785.8-4984.8 ns)
   4785.8 |########################################
   4795.8 |
   4805.7 |
   4815.6 |
   4825.6 |########################################
   4835.6 |
   4845.5 |
   4855.4 |
   4865.4 |
   4875.3 |
   4885.3 |
   4895.2 |
   4905.2 |########################################
   4915.1 |
   4925.1 |########################################
   4935.0 |
   4945.0 |
   4954.9 |
   4964.9 |########################################
   4974.8 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_scalar (n=6, range 2104732.1-2161885.8 ns)
  2104732.1 |########################################
  2107589.8 |
  2110447.5 |####################
  2113305.2 |########################################
  2116162.9 |
  2119020.5 |
  2121878.2 |
  2124735.9 |
  2127593.6 |
  2130451.3 |
  2133309.0 |
  2136166.7 |
  2139024.3 |
  2141882.0 |
  2144739.7 |
  2147597.4 |
  2150455.1 |
  2153312.8 |
  2156170.5 |
  2159028.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_wideselect_cold_null**: bridge=1090.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_cold_scalar**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_null**: bridge=2586.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_scalar**: bridge=303.8% of algo (FFI overhead may distort results)
