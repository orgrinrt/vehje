# Per-branch strategy: archetype 3 (match4_blocks), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b3_table**

## Highlights

Baseline for all deltas below: **ab_b3_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Two tiers: {ab_b3_tree, ab_b3_seq, ab_b3_prof, ab_b3_table} vs {ab_b3_pred} (89% apart)

The field splits into a fast tier {ab_b3_tree, ab_b3_seq, ab_b3_prof, ab_b3_table} and a slow tier {ab_b3_pred} with a 89% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: ab_b3_tree** at 322273.8 ns median (-1.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 322273.8 ns, slowest 616869.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b3_pred | 619789ns | 619878ns | 617303ns | 619523ns | 621432ns | +88.67% |
| ab_b3_prof | 329080ns | 329558ns | 323557ns | 328601ns | 332561ns | +0.18% |
| ab_b3_seq | 328993ns | 328834ns | 323692ns | 327838ns | 333376ns | +0.15% |
| ab_b3_table | 328498ns | 329660ns | 320033ns | 328233ns | 333129ns | base |
| ab_b3_tree | 327847ns | 324862ns | 323289ns | 324391ns | 335310ns | -0.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b3_pred | 616894ns | 614292ns | 618671ns | +89.30% | 0.002 |
| ab_b3_prof | 326461ns | 320919ns | 329855ns | +0.18% | 0.003 |
| ab_b3_seq | 326515ns | 321512ns | 330883ns | +0.19% | 0.003 |
| ab_b3_table | 325886ns | 317415ns | 330479ns | base | 0.003 |
| ab_b3_tree | 325417ns | 321144ns | 332804ns | -0.14% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b3_table; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b3_pred | 0.002 | 51.5% |
| ab_b3_prof | 0.003 | 97.1% |
| ab_b3_seq | 0.003 | 97.3% |
| ab_b3_table | 0.003 | 97.0% |
| ab_b3_tree | 0.003 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b3_pred | 619789ns | 619789ns | +88.67% |
| ab_b3_prof | 329080ns | 329080ns | +0.18% |
| ab_b3_seq | 328993ns | 328993ns | +0.15% |
| ab_b3_table | 328498ns | 328498ns | base |
| ab_b3_tree | 327847ns | 327847ns | -0.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b3_table | 327244ns | base | --- | [319934, 330479] | --- | --- | --- | --- |
| ab_b3_pred | 616869ns | +291426.5ns (+89.1%) | [+286059, +295541]ns | [615143, 618671] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b3_prof | 326845ns | no significant difference | [-5271, +6932]ns | [322683, 329855] | no | 1.0000 | 1.0000 | 0 |
| ab_b3_seq | 326253ns | no significant difference | [-5684, +8314]ns | [322410, 330883] | no | 0.9167 | 0.6875 | 0 |
| ab_b3_tree | 322274ns | no significant difference | [-8716, +10235]ns | [321172, 332804] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b3_table | ab_b3_pred | ab_b3_prof | ab_b3_seq | ab_b3_tree |
|---|---|---|---|---|---|
| 1 | 317415ns | +93.5% | +3.2% | +3.6% | +4.5% |
| 2 | 329142ns | +87.2% | -1.4% | -0.2% | -2.4% |
| 3 | 326766ns | +89.7% | +1.1% | -0.9% | -1.4% |
| 4 | 327722ns | +88.4% | +0.5% | +1.6% | +1.8% |
| 5 | 331816ns | +86.0% | -1.8% | -2.6% | -2.8% |
| 6 | 322452ns | +91.2% | -0.5% | -0.3% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b3_pred | 0.086 | ok |
| ab_b3_prof | 0.038 | ok |
| ab_b3_seq | -0.230 | moderate- |
| ab_b3_table | -0.244 | moderate- |
| ab_b3_tree | -0.319 | moderate- |

**Consistency summary:**

- **ab_b3_pred**: won 0/6, lost 6/6
- **ab_b3_prof**: won 3/6, lost 3/6
- **ab_b3_seq**: won 4/6, lost 2/6
- **ab_b3_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b3_pred | 8.8ns | 616894.4ns | 0.0% |  |
| ab_b3_prof | 5.3ns | 326460.9ns | 0.0% |  |
| ab_b3_seq | 6.8ns | 326515.1ns | 0.0% |  |
| ab_b3_table | 5.1ns | 325885.8ns | 0.0% |  |
| ab_b3_tree | 4.3ns | 325416.6ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b3_pred (n=6, range 614291.7-618670.7 ns)
  614291.7 |########################################
  614510.6 |
  614729.6 |
  614948.5 |
  615167.5 |
  615386.4 |
  615605.4 |
  615824.3 |########################################
  616043.3 |
  616262.2 |
  616481.2 |########################################
  616700.1 |
  616919.1 |########################################
  617138.0 |
  617357.0 |########################################
  617575.9 |
  617794.9 |
  618013.8 |
  618232.8 |
  618451.7 |
  (0 below, 1 above range)

ab_b3_prof (n=6, range 320919.2-329854.6 ns)
  320919.2 |########################################
  321366.0 |
  321812.7 |
  322259.5 |
  322706.3 |
  323153.0 |
  323599.8 |
  324046.6 |########################################
  324493.4 |
  324940.1 |
  325386.9 |
  325833.7 |########################################
  326280.4 |
  326727.2 |
  327174.0 |
  327620.8 |########################################
  328067.5 |
  328514.3 |
  328961.1 |########################################
  329407.8 |
  (0 below, 1 above range)

ab_b3_seq (n=6, range 321512.5-330883.1 ns)
  321512.5 |####################
  321981.0 |
  322449.6 |
  322918.1 |####################
  323386.6 |
  323855.2 |####################
  324323.7 |
  324792.2 |
  325260.7 |
  325729.3 |
  326197.8 |
  326666.3 |
  327134.9 |
  327603.4 |
  328071.9 |
  328540.4 |########################################
  329009.0 |
  329477.5 |
  329946.0 |
  330414.6 |
  (0 below, 1 above range)

ab_b3_table (n=6, range 317415.4-330479.3 ns)
  317415.4 |########################################
  318068.6 |
  318721.8 |
  319375.0 |
  320028.2 |
  320681.4 |
  321334.6 |
  321987.8 |########################################
  322641.0 |
  323294.2 |
  323947.4 |
  324600.6 |
  325253.8 |
  325907.0 |
  326560.2 |########################################
  327213.4 |########################################
  327866.6 |
  328519.8 |########################################
  329173.0 |
  329826.2 |
  (0 below, 1 above range)

ab_b3_tree (n=6, range 321143.8-332803.8 ns)
  321143.8 |########################################
  321726.8 |####################
  322309.8 |####################
  322892.8 |
  323475.8 |
  324058.8 |
  324641.8 |
  325224.8 |
  325807.8 |
  326390.8 |
  326973.8 |
  327556.8 |
  328139.8 |
  328722.8 |
  329305.8 |
  329888.8 |
  330471.8 |
  331054.8 |
  331637.8 |####################
  332220.8 |
  (0 below, 1 above range)

```
