# Per-branch strategy: archetype 2 (match8_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b2_table**

## Highlights

Baseline for all deltas below: **ab_b2_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b2_prof, ab_b2_table) are a dead heat (<1%)

ab_b2_prof (326.28 us) and ab_b2_table (326.82 us) differ by 0.17%, inside the noise, even though the wider field spreads 41.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b2_seq shows alternating (throttle bounce) (autocorr -0.66)

ab_b2_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {ab_b2_prof, ab_b2_table, ab_b2_tree, ab_b2_seq} vs {ab_b2_pred} (39% apart)

The field splits into a fast tier {ab_b2_prof, ab_b2_table, ab_b2_tree, ab_b2_seq} and a slow tier {ab_b2_pred} with a 39% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### ab_b2_tree's edge over baseline is significant but tiny (47 ns, 0.01%)

ab_b2_tree differs from baseline ab_b2_table by 47 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ab_b2_prof** at 326275.2 ns median (-0.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.41x (fastest 326275.2 ns, slowest 460216.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b2_pred | 464629ns | 463096ns | 453688ns | 462965ns | 472596ns | +40.59% |
| ab_b2_prof | 330297ns | 328690ns | 321965ns | 327967ns | 337959ns | -0.06% |
| ab_b2_seq | 335279ns | 334799ns | 328507ns | 333593ns | 341195ns | +1.45% |
| ab_b2_table | 330486ns | 329291ns | 323098ns | 327874ns | 338097ns | base |
| ab_b2_tree | 335634ns | 330745ns | 328611ns | 330382ns | 347024ns | +1.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b2_pred | 461895ns | 450925ns | 470031ns | +40.84% | 0.002 |
| ab_b2_prof | 327851ns | 319351ns | 335567ns | -0.04% | 0.003 |
| ab_b2_seq | 332695ns | 325946ns | 338480ns | +1.44% | 0.003 |
| ab_b2_table | 327967ns | 320515ns | 335587ns | base | 0.003 |
| ab_b2_tree | 333192ns | 326370ns | 344348ns | +1.59% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b2_prof; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b2_pred | 0.002 | 69.4% |
| ab_b2_prof | 0.003 | 97.9% |
| ab_b2_seq | 0.003 | 96.1% |
| ab_b2_table | 0.003 | 97.7% |
| ab_b2_tree | 0.003 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b2_pred | 464629ns | 464629ns | +40.59% |
| ab_b2_prof | 330297ns | 330297ns | -0.06% |
| ab_b2_seq | 335279ns | 335279ns | +1.45% |
| ab_b2_table | 330486ns | 330486ns | base |
| ab_b2_tree | 335634ns | 335634ns | +1.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b2_table | 326821ns | base | --- | [321492, 335587] | --- | --- | --- | --- |
| ab_b2_pred | 460216ns | +132857.1ns (+40.7%) | [+124936, +143991]ns | [455437, 470031] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b2_prof | 326275ns | no significant difference | [-9311, +11312]ns | [321711, 335567] | no | 0.9167 | 0.6875 | 0 |
| ab_b2_seq | 332162ns | no significant difference | [-2830, +13620]ns | [327443, 338480] | no | 0.4375 | 0.2188 | 0 |
| ab_b2_tree | 328359ns | no significant difference | [-7227, +22856]ns | [326868, 344348] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b2_table | ab_b2_pred | ab_b2_prof | ab_b2_seq | ab_b2_tree |
|---|---|---|---|---|---|
| 1 | 322469ns | +42.6% | -1.0% | +1.1% | +5.2% |
| 2 | 327994ns | +37.5% | +1.6% | +4.1% | -0.2% |
| 3 | 339103ns | +37.4% | -3.3% | -2.6% | -3.3% |
| 4 | 320515ns | +43.6% | +5.4% | +4.3% | +9.0% |
| 5 | 325649ns | +45.6% | -0.5% | +1.0% | +0.2% |
| 6 | 332070ns | +38.6% | -2.3% | +1.0% | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b2_pred | -0.246 | moderate- |
| ab_b2_prof | -0.297 | moderate- |
| ab_b2_seq | -0.660 | HIGH- (thermal bounce) |
| ab_b2_table | -0.324 | moderate- |
| ab_b2_tree | -0.393 | moderate- |

**Consistency summary:**

- **ab_b2_pred**: won 0/6, lost 6/6
- **ab_b2_prof**: won 4/6, lost 2/6
- **ab_b2_seq**: won 1/6, lost 5/6
- **ab_b2_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b2_pred | 6.4ns | 461894.7ns | 0.0% |  |
| ab_b2_prof | 6.2ns | 327851.0ns | 0.0% |  |
| ab_b2_seq | 5.1ns | 332694.9ns | 0.0% |  |
| ab_b2_table | 9.2ns | 327966.7ns | 0.0% |  |
| ab_b2_tree | 4.4ns | 333191.6ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b2_pred (n=6, range 450924.6-470031.5 ns)
  450924.6 |#############
  451879.9 |
  452835.3 |
  453790.6 |
  454746.0 |
  455701.3 |
  456656.7 |
  457612.0 |
  458567.3 |
  459522.7 |########################################
  460478.0 |
  461433.4 |
  462388.7 |
  463344.1 |
  464299.4 |
  465254.7 |#############
  466210.1 |
  467165.4 |
  468120.8 |
  469076.1 |
  (0 below, 1 above range)

ab_b2_prof (n=6, range 319350.8-335567.0 ns)
  319350.8 |########################################
  320161.6 |
  320972.4 |
  321783.2 |
  322594.0 |
  323404.9 |########################################
  324215.7 |########################################
  325026.5 |
  325837.3 |
  326648.1 |
  327458.9 |########################################
  328269.7 |
  329080.5 |
  329891.4 |
  330702.2 |
  331513.0 |
  332323.8 |
  333134.6 |########################################
  333945.4 |
  334756.2 |
  (0 below, 1 above range)

ab_b2_seq (n=6, range 325946.2-338479.8 ns)
  325946.2 |########################################
  326572.9 |
  327199.6 |
  327826.2 |
  328452.9 |########################################
  329079.6 |
  329706.3 |########################################
  330332.9 |
  330959.6 |
  331586.3 |
  332213.0 |
  332839.7 |
  333466.3 |
  334093.0 |########################################
  334719.7 |
  335346.4 |########################################
  335973.0 |
  336599.7 |
  337226.4 |
  337853.1 |
  (0 below, 1 above range)

ab_b2_table (n=6, range 320515.0-335586.7 ns)
  320515.0 |########################################
  321268.6 |
  322022.2 |########################################
  322775.7 |
  323529.3 |
  324282.9 |
  325036.5 |########################################
  325790.1 |
  326543.7 |
  327297.2 |########################################
  328050.8 |
  328804.4 |
  329558.0 |
  330311.6 |
  331065.2 |
  331818.7 |########################################
  332572.3 |
  333325.9 |
  334079.5 |
  334833.1 |
  (0 below, 1 above range)

ab_b2_tree (n=6, range 326370.4-344347.7 ns)
  326370.4 |####################
  327269.3 |########################################
  328168.1 |####################
  329067.0 |
  329965.9 |
  330864.7 |
  331763.6 |
  332662.5 |
  333561.3 |
  334460.2 |
  335359.1 |
  336257.9 |
  337156.8 |
  338055.6 |
  338954.5 |####################
  339853.4 |
  340752.2 |
  341651.1 |
  342550.0 |
  343448.8 |
  (0 below, 1 above range)

```
