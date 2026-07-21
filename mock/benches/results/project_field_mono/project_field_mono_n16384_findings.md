# Project field access, monomorphic site: direct offset vs inline cache vs hash vs linear

4 variants, 6 samples per variant.
Baseline: **project_mono_direct**

## Highlights

Baseline for all deltas below: **project_mono_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### project_mono_direct dominates: 19% faster than the next best (project_mono_ic)

project_mono_direct (11.69 us) leads project_mono_ic (13.89 us) by 19%, a clear separation rather than a photo finish. CV 5.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### project_mono_linear is an outlier: 3.6x slower than the field

project_mono_linear (41.74 us) is 3.6x the fastest (11.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (project_mono_direct)

The baseline project_mono_direct is the fastest (11.69 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {project_mono_direct, project_mono_ic} vs {project_mono_hash, project_mono_linear} (156% apart)

The field splits into a fast tier {project_mono_direct, project_mono_ic} and a slow tier {project_mono_hash, project_mono_linear} with a 156% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.6x the fastest

Fastest project_mono_direct (11.69 us) to slowest project_mono_linear (41.74 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (project_mono_direct) is the fastest** at 11692.5 ns median
- 3 variants significantly slower than baseline
- Spread: 3.57x (fastest 11692.5 ns, slowest 41742.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| project_mono_direct | 13961ns | 14024ns | 13062ns | 13744ns | 14737ns | base |
| project_mono_hash | 37405ns | 38020ns | 34077ns | 37017ns | 39651ns | +167.92% |
| project_mono_ic | 16244ns | 16333ns | 14653ns | 15958ns | 17467ns | +16.35% |
| project_mono_linear | 43369ns | 44236ns | 39396ns | 43121ns | 45728ns | +210.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| project_mono_direct | 11649ns | 10899ns | 12312ns | base | 1.406 |
| project_mono_hash | 34979ns | 31863ns | 37081ns | +200.27% | 0.468 |
| project_mono_ic | 13759ns | 12457ns | 14767ns | +18.11% | 1.191 |
| project_mono_linear | 40893ns | 37178ns | 43095ns | +251.03% | 0.401 |

## Performance model

- Peak throughput: **1.503 Gops/s** (project_mono_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| project_mono_direct | 1.401 | 93.2% |
| project_mono_hash | 0.461 | 30.6% |
| project_mono_ic | 1.180 | 78.5% |
| project_mono_linear | 0.393 | 26.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| project_mono_direct | 13961ns | 13961ns | base |
| project_mono_hash | 37405ns | 37405ns | +167.92% |
| project_mono_ic | 16244ns | 16244ns | +16.35% |
| project_mono_linear | 43369ns | 43369ns | +210.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| project_mono_direct | 11692ns | base | --- | [10944, 12312] | --- | --- | --- | --- |
| project_mono_hash | 35561ns | +23731.3ns (+203.0%) | [+21229, +25028]ns | [32296, 37081] | YES | 0.0313 | 0.0313 | 0 |
| project_mono_ic | 13888ns | +2033.1ns (+17.4%) | [+1448, +2847]ns | [12622, 14767] | YES | 0.0313 | 0.0313 | 0 |
| project_mono_linear | 41742ns | +29607.8ns (+253.2%) | [+26775, +31348]ns | [37841, 43095] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | project_mono_direct | project_mono_hash | project_mono_ic | project_mono_linear |
|---|---|---|---|---|
| 1 | 10899ns | +226.8% | +27.8% | +285.9% |
| 2 | 12241ns | +190.0% | +18.5% | +238.4% |
| 3 | 12268ns | +197.3% | +12.9% | +257.0% |
| 4 | 12356ns | +205.0% | +21.6% | +243.1% |
| 5 | 10989ns | +197.8% | +16.4% | +250.4% |
| 6 | 11144ns | +185.9% | +11.8% | +233.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| project_mono_direct | 0.091 | ok |
| project_mono_hash | 0.242 | moderate+ |
| project_mono_ic | 0.071 | ok |
| project_mono_linear | 0.371 | moderate+ |

**Consistency summary:**

- **project_mono_hash**: won 0/6, lost 6/6
- **project_mono_ic**: won 0/6, lost 6/6
- **project_mono_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| project_mono_direct | 3.8ns | 11649.4ns | 0.0% |  |
| project_mono_hash | 3.7ns | 34979.0ns | 0.0% |  |
| project_mono_ic | 5.1ns | 13758.9ns | 0.0% |  |
| project_mono_linear | 3.8ns | 40892.7ns | 0.0% |  |

## Distribution (algo ns)

```
project_mono_direct (n=6, range 10899.2-12311.6 ns)
  10899.2 |########################################
  10969.8 |########################################
  11040.4 |
  11111.1 |########################################
  11181.7 |
  11252.3 |
  11322.9 |
  11393.6 |
  11464.2 |
  11534.8 |
  11605.4 |
  11676.0 |
  11746.7 |
  11817.3 |
  11887.9 |
  11958.5 |
  12029.2 |
  12099.8 |
  12170.4 |########################################
  12241.0 |########################################
  (0 below, 1 above range)

project_mono_hash (n=6, range 31863.3-37080.6 ns)
  31863.3 |########################################
  32124.2 |
  32385.0 |
  32645.9 |########################################
  32906.8 |
  33167.6 |
  33428.5 |
  33689.4 |
  33950.2 |
  34211.1 |
  34472.0 |
  34732.8 |
  34993.7 |
  35254.6 |########################################
  35515.4 |########################################
  35776.3 |
  36037.2 |
  36298.0 |########################################
  36558.9 |
  36819.8 |
  (0 below, 1 above range)

project_mono_ic (n=6, range 12457.1-14767.2 ns)
  12457.1 |####################
  12572.6 |
  12688.1 |####################
  12803.6 |
  12919.1 |
  13034.6 |
  13150.1 |
  13265.7 |
  13381.2 |
  13496.7 |
  13612.2 |
  13727.7 |
  13843.2 |########################################
  13958.7 |
  14074.2 |
  14189.7 |
  14305.2 |
  14420.7 |####################
  14536.2 |
  14651.7 |
  (0 below, 1 above range)

project_mono_linear (n=6, range 37177.9-43094.6 ns)
  37177.9 |########################################
  37473.7 |
  37769.6 |
  38065.4 |
  38361.2 |########################################
  38657.1 |
  38952.9 |
  39248.7 |
  39544.6 |
  39840.4 |
  40136.2 |
  40432.1 |
  40727.9 |
  41023.8 |
  41319.6 |########################################
  41615.4 |
  41911.3 |########################################
  42207.1 |########################################
  42502.9 |
  42798.8 |
  (0 below, 1 above range)

```
