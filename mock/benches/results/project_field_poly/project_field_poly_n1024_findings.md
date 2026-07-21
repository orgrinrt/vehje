# Project field access, polymorphic site: inline-cache hazard vs hash vs linear

3 variants, 6 samples per variant.
Baseline: **project_poly_hash**

## Highlights

Baseline for all deltas below: **project_poly_hash**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### project_poly_hash shows warm-up / thermal drift (autocorr +0.51)

project_poly_hash's per-pass series has lag-1 autocorrelation +0.51, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (project_poly_hash)

The baseline project_poly_hash is the fastest (2.21 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (project_poly_hash) is the fastest** at 2209.4 ns median
- 2 variants significantly slower than baseline
- Spread: 1.15x (fastest 2209.4 ns, slowest 2543.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| project_poly_hash | 4598ns | 4541ns | 4241ns | 4443ns | 5008ns | base |
| project_poly_ic | 4936ns | 4831ns | 4546ns | 4758ns | 5399ns | +7.36% |
| project_poly_linear | 4948ns | 4610ns | 4532ns | 4598ns | 5680ns | +7.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| project_poly_hash | 2224ns | 2031ns | 2422ns | base | 0.461 |
| project_poly_ic | 2588ns | 2390ns | 2814ns | +16.36% | 0.396 |
| project_poly_linear | 2583ns | 2365ns | 2934ns | +16.15% | 0.396 |

## Performance model

- Peak throughput: **0.504 Gops/s** (project_poly_hash; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| project_poly_hash | 0.463 | 91.9% |
| project_poly_ic | 0.403 | 79.8% |
| project_poly_linear | 0.422 | 83.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| project_poly_hash | 4598ns | 4598ns | base |
| project_poly_ic | 4936ns | 4936ns | +7.36% |
| project_poly_linear | 4948ns | 4948ns | +7.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| project_poly_hash | 2209ns | base | --- | [2039, 2422] | --- | --- | --- | --- |
| project_poly_ic | 2544ns | +366.1ns (+16.6%) | [+334, +391]ns | [2405, 2814] | YES | 0.0313 | 0.0313 | 0 |
| project_poly_linear | 2429ns | +389.6ns (+17.6%) | [+176, +512]ns | [2386, 2934] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | project_poly_hash | project_poly_ic | project_poly_linear |
|---|---|---|---|
| 1 | 2277ns | +14.6% | +5.7% |
| 2 | 2408ns | +15.7% | +25.8% |
| 3 | 2437ns | +16.6% | +16.5% |
| 4 | 2142ns | +15.7% | +10.4% |
| 5 | 2047ns | +18.2% | +19.4% |
| 6 | 2031ns | +17.7% | +18.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| project_poly_hash | 0.509 | HIGH+ (drift/warm-up) |
| project_poly_ic | 0.427 | moderate+ |
| project_poly_linear | 0.086 | ok |

**Consistency summary:**

- **project_poly_ic**: won 0/6, lost 6/6
- **project_poly_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| project_poly_hash | 3.8ns | 2223.6ns | 0.2% |  |
| project_poly_ic | 2.2ns | 2587.5ns | 0.1% |  |
| project_poly_linear | 3.4ns | 2582.8ns | 0.1% |  |

## Distribution (algo ns)

```
project_poly_hash (n=6, range 2030.8-2422.5 ns)
   2030.8 |########################################
   2050.4 |
   2070.0 |
   2089.6 |
   2109.1 |
   2128.7 |####################
   2148.3 |
   2167.9 |
   2187.5 |
   2207.1 |
   2226.7 |
   2246.2 |
   2265.8 |####################
   2285.4 |
   2305.0 |
   2324.6 |
   2344.2 |
   2363.7 |
   2383.3 |
   2402.9 |####################
  (0 below, 1 above range)

project_poly_ic (n=6, range 2390.4-2813.9 ns)
   2390.4 |########################################
   2411.6 |########################################
   2432.8 |
   2453.9 |
   2475.1 |########################################
   2496.3 |
   2517.5 |
   2538.6 |
   2559.8 |
   2581.0 |
   2602.2 |########################################
   2623.4 |
   2644.5 |
   2665.7 |
   2686.9 |
   2708.1 |
   2729.2 |
   2750.4 |
   2771.6 |########################################
   2792.8 |
  (0 below, 1 above range)

project_poly_linear (n=6, range 2365.4-2934.1 ns)
   2365.4 |####################
   2393.8 |########################################
   2422.3 |####################
   2450.7 |
   2479.2 |
   2507.6 |
   2536.0 |
   2564.5 |
   2592.9 |
   2621.3 |
   2649.8 |
   2678.2 |
   2706.6 |
   2735.1 |
   2763.5 |
   2792.0 |
   2820.4 |####################
   2848.8 |
   2877.3 |
   2905.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **project_poly_hash**: autocorrelation=0.51 (measurement drift or warm-up artifact)
