# Per-branch strategy: archetype 1 (match4_nested), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b1_table**

## Highlights

Baseline for all deltas below: **ab_b1_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Speed leader ab_b1_tree vs stability leader ab_b1_seq (+2% speed for 1.5x steadier)

ab_b1_tree is fastest (87.77 us, CV 6.9%); ab_b1_seq gives up 2.4% median for 1.5x lower variance (CV 4.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: ab_b1_tree** at 87768.1 ns median (-4.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.12x (fastest 87768.1 ns, slowest 98020.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b1_pred | 103873ns | 100301ns | 98465ns | 99880ns | 112567ns | +13.54% |
| ab_b1_prof | 90248ns | 91507ns | 81793ns | 88773ns | 96689ns | -1.35% |
| ab_b1_seq | 90840ns | 92402ns | 84012ns | 90510ns | 94750ns | -0.70% |
| ab_b1_table | 91484ns | 94473ns | 82475ns | 91004ns | 96708ns | base |
| ab_b1_tree | 89989ns | 90224ns | 82165ns | 88107ns | 96725ns | -1.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b1_pred | 101512ns | 96254ns | 110001ns | +14.02% | 0.003 |
| ab_b1_prof | 87779ns | 79561ns | 93938ns | -1.40% | 0.003 |
| ab_b1_seq | 88380ns | 81670ns | 92217ns | -0.73% | 0.003 |
| ab_b1_table | 89026ns | 80232ns | 94068ns | base | 0.003 |
| ab_b1_tree | 87508ns | 79929ns | 94016ns | -1.71% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b1_prof; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b1_pred | 0.003 | 81.2% |
| ab_b1_prof | 0.003 | 89.3% |
| ab_b1_seq | 0.003 | 88.5% |
| ab_b1_table | 0.003 | 86.5% |
| ab_b1_tree | 0.003 | 90.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b1_pred | 103873ns | 103873ns | +13.54% |
| ab_b1_prof | 90248ns | 90248ns | -1.35% |
| ab_b1_seq | 90840ns | 90840ns | -0.70% |
| ab_b1_table | 91484ns | 91484ns | base |
| ab_b1_tree | 89989ns | 89989ns | -1.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b1_table | 91978ns | base | --- | [81033, 94068] | --- | --- | --- | --- |
| ab_b1_pred | 98020ns | +15915.4ns (+17.3%) | [+5062, +16479]ns | [96515, 110001] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b1_prof | 89051ns | -530.7ns (-0.6%) | [-3119, -92]ns | [80348, 93938] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| ab_b1_seq | 89856ns | no significant difference | [-2329, +2034]ns | [83067, 92217] | no | 0.6875 | 0.6875 | 0 |
| ab_b1_tree | 87768ns | no significant difference | [-4210, +433]ns | [80741, 94016] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b1_table | ab_b1_pred | ab_b1_prof | ab_b1_seq | ab_b1_tree |
|---|---|---|---|---|---|
| 1 | 80232ns | +20.0% | -0.8% | +1.8% | -0.4% |
| 2 | 93609ns | +17.2% | +0.2% | -1.3% | -1.5% |
| 3 | 90346ns | +7.1% | -6.1% | -2.3% | -7.7% |
| 4 | 81834ns | +20.6% | -0.9% | +3.2% | -0.3% |
| 5 | 93620ns | +3.9% | -0.3% | -2.3% | +1.2% |
| 6 | 94517ns | +16.7% | -0.4% | -2.6% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b1_pred | -0.426 | moderate- |
| ab_b1_prof | -0.215 | moderate- |
| ab_b1_seq | -0.280 | moderate- |
| ab_b1_table | -0.254 | moderate- |
| ab_b1_tree | -0.144 | ok |

**Consistency summary:**

- **ab_b1_pred**: won 0/6, lost 6/6
- **ab_b1_prof**: won 5/6, lost 1/6
- **ab_b1_seq**: won 4/6, lost 2/6
- **ab_b1_tree**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b1_pred | 4.8ns | 101512.0ns | 0.0% |  |
| ab_b1_prof | 6.4ns | 87779.1ns | 0.0% |  |
| ab_b1_seq | 5.7ns | 88380.2ns | 0.0% |  |
| ab_b1_table | 5.6ns | 89026.3ns | 0.0% |  |
| ab_b1_tree | 4.4ns | 87508.3ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b1_pred (n=6, range 96254.2-110001.2 ns)
  96254.2 |########################################
  96941.6 |####################
  97628.9 |
  98316.3 |####################
  99003.6 |
  99691.0 |
  100378.3 |
  101065.7 |
  101753.0 |
  102440.4 |
  103127.7 |
  103815.1 |
  104502.4 |
  105189.8 |
  105877.1 |
  106564.5 |
  107251.8 |
  107939.2 |
  108626.5 |
  109313.9 |####################
  (0 below, 1 above range)

ab_b1_prof (n=6, range 79561.2-93938.3 ns)
  79561.2 |####################
  80280.1 |
  80998.9 |####################
  81717.8 |
  82436.6 |
  83155.5 |
  83874.3 |
  84593.2 |####################
  85312.0 |
  86030.9 |
  86749.8 |
  87468.6 |
  88187.5 |
  88906.3 |
  89625.2 |
  90344.0 |
  91062.9 |
  91781.7 |
  92500.6 |
  93219.4 |########################################
  (0 below, 1 above range)

ab_b1_seq (n=6, range 81670.4-92216.9 ns)
  81670.4 |########################################
  82197.7 |
  82725.0 |
  83252.4 |
  83779.7 |
  84307.0 |########################################
  84834.3 |
  85361.7 |
  85889.0 |
  86416.3 |
  86943.6 |
  87471.0 |
  87998.3 |########################################
  88525.6 |
  89052.9 |
  89580.3 |
  90107.6 |
  90634.9 |
  91162.2 |########################################
  91689.6 |########################################
  (0 below, 1 above range)

ab_b1_table (n=6, range 80232.1-94068.1 ns)
  80232.1 |####################
  80923.9 |
  81615.7 |####################
  82307.5 |
  82999.3 |
  83691.1 |
  84382.9 |
  85074.7 |
  85766.5 |
  86458.3 |
  87150.1 |
  87841.9 |
  88533.7 |
  89225.5 |
  89917.3 |####################
  90609.1 |
  91300.9 |
  91992.7 |
  92684.5 |
  93376.3 |########################################
  (0 below, 1 above range)

ab_b1_tree (n=6, range 79929.2-94015.6 ns)
  79929.2 |########################################
  80633.5 |
  81337.8 |########################################
  82042.2 |
  82746.5 |########################################
  83450.8 |
  84155.1 |
  84859.5 |
  85563.8 |
  86268.1 |
  86972.4 |
  87676.7 |
  88381.1 |
  89085.4 |
  89789.7 |
  90494.0 |
  91198.4 |
  91902.7 |########################################
  92607.0 |########################################
  93311.3 |
  (0 below, 1 above range)

```
