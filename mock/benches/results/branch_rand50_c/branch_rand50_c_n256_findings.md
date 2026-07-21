# Branch strategies, cheap-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_rand50**

## Highlights

Baseline for all deltas below: **br_branch_c_rand50**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### br_branch_c_rand50 dominates: 35% faster than the next best (br_lut_c_rand50)

br_branch_c_rand50 (496 ns) leads br_lut_c_rand50 (668 ns) by 35%, a clear separation rather than a photo finish. CV 7.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (br_branch_c_rand50)

The baseline br_branch_c_rand50 is the fastest (496 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {br_branch_c_rand50} vs {br_lut_c_rand50, br_predicate_c_rand50, br_mask_c_rand50} (35% apart)

The field splits into a fast tier {br_branch_c_rand50} and a slow tier {br_lut_c_rand50, br_predicate_c_rand50, br_mask_c_rand50} with a 35% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### br_lut_c_rand50 is inconsistent: worst-20% is 1.6x its best-20%

br_lut_c_rand50's best 20% of batches run at 472 ns but its worst 20% at 773 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (br_branch_c_rand50) is the fastest** at 496.0 ns median
- 3 variants significantly slower than baseline
- Spread: 1.69x (fastest 496.0 ns, slowest 840.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 3430ns | 3399ns | 3028ns | 3351ns | 3751ns | base |
| br_lut_c_rand50 | 3547ns | 3631ns | 2602ns | 3559ns | 4002ns | +3.40% |
| br_mask_c_rand50 | 3910ns | 4059ns | 2798ns | 3967ns | 4381ns | +13.98% |
| br_predicate_c_rand50 | 3592ns | 3749ns | 2596ns | 3638ns | 4020ns | +4.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_rand50 | 497ns | 442ns | 537ns | base | 0.515 |
| br_lut_c_rand50 | 665ns | 472ns | 773ns | +33.89% | 0.385 |
| br_mask_c_rand50 | 803ns | 580ns | 887ns | +61.60% | 0.319 |
| br_predicate_c_rand50 | 668ns | 487ns | 740ns | +34.47% | 0.383 |

## Performance model

- Peak throughput: **0.579 Gops/s** (br_branch_c_rand50; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_rand50 | 0.516 | 89.1% |
| br_lut_c_rand50 | 0.383 | 66.2% |
| br_mask_c_rand50 | 0.305 | 52.6% |
| br_predicate_c_rand50 | 0.366 | 63.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_rand50 | 3430ns | 3430ns | base |
| br_lut_c_rand50 | 3547ns | 3547ns | +3.40% |
| br_mask_c_rand50 | 3910ns | 3910ns | +13.98% |
| br_predicate_c_rand50 | 3592ns | 3592ns | +4.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 496ns | base | --- | [457, 537] | --- | --- | --- | --- |
| br_lut_c_rand50 | 668ns | +191.7ns (+38.6%) | [+75, +239]ns | [554, 773] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_rand50 | 840ns | +326.9ns (+65.9%) | [+201, +390]ns | [681, 887] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_rand50 | 699ns | +202.5ns (+40.8%) | [+85, +226]ns | [565, 740] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_rand50 | br_lut_c_rand50 | br_mask_c_rand50 | br_predicate_c_rand50 |
|---|---|---|---|---|
| 1 | 442ns | +6.9% | +31.3% | +10.1% |
| 2 | 518ns | +22.9% | +50.9% | +24.2% |
| 3 | 557ns | +39.1% | +59.2% | +39.9% |
| 4 | 516ns | +49.3% | +71.5% | +35.6% |
| 5 | 476ns | +46.8% | +86.4% | +46.5% |
| 6 | 471ns | +35.1% | +68.7% | +48.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_rand50 | 0.167 | ok |
| br_lut_c_rand50 | 0.263 | moderate+ |
| br_mask_c_rand50 | 0.230 | moderate+ |
| br_predicate_c_rand50 | 0.148 | ok |

**Consistency summary:**

- **br_lut_c_rand50**: won 0/6, lost 6/6
- **br_mask_c_rand50**: won 0/6, lost 6/6
- **br_predicate_c_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_rand50 | 6.0ns | 496.7ns | 1.2% |  |
| br_lut_c_rand50 | 5.5ns | 665.0ns | 0.8% |  |
| br_mask_c_rand50 | 5.7ns | 802.6ns | 0.7% |  |
| br_predicate_c_rand50 | 5.5ns | 667.9ns | 0.8% |  |

## Distribution (algo ns)

```
br_branch_c_rand50 (n=6, range 442.1-537.3 ns)
    442.1 |####################
    446.9 |
    451.6 |
    456.4 |
    461.1 |
    465.9 |
    470.7 |####################
    475.4 |####################
    480.2 |
    484.9 |
    489.7 |
    494.5 |
    499.2 |
    504.0 |
    508.7 |
    513.5 |########################################
    518.3 |
    523.0 |
    527.8 |
    532.5 |
  (0 below, 1 above range)

br_lut_c_rand50 (n=6, range 472.5-772.9 ns)
    472.5 |####################
    487.5 |
    502.5 |
    517.6 |
    532.6 |
    547.6 |
    562.6 |
    577.6 |
    592.7 |
    607.7 |
    622.7 |########################################
    637.7 |
    652.7 |
    667.8 |
    682.8 |
    697.8 |####################
    712.8 |
    727.8 |
    742.9 |
    757.9 |####################
  (0 below, 1 above range)

br_mask_c_rand50 (n=6, range 580.4-886.9 ns)
    580.4 |####################
    595.7 |
    611.0 |
    626.4 |
    641.7 |
    657.0 |
    672.4 |
    687.7 |
    703.0 |
    718.3 |
    733.7 |
    749.0 |
    764.3 |
    779.6 |####################
    795.0 |####################
    810.3 |
    825.6 |
    840.9 |
    856.3 |
    871.6 |########################################
  (0 below, 1 above range)

br_predicate_c_rand50 (n=6, range 486.7-740.4 ns)
    486.7 |#############
    499.4 |
    512.1 |
    524.8 |
    537.4 |
    550.1 |
    562.8 |
    575.5 |
    588.2 |
    600.9 |
    613.6 |
    626.2 |
    638.9 |#############
    651.6 |
    664.3 |
    677.0 |
    689.7 |########################################
    702.3 |
    715.0 |
    727.7 |
  (0 below, 1 above range)

```
