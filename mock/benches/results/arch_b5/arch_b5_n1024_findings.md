# Per-branch strategy: archetype 5 (ifchain4_nested), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b5_table**

## Highlights

Baseline for all deltas below: **ab_b5_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b5_table, ab_b5_tree) are a dead heat (<1%)

ab_b5_table (328.26 us) and ab_b5_tree (331.15 us) differ by 0.88%, inside the noise, even though the wider field spreads 17.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b5_pred shows warm-up / thermal drift (autocorr +0.51)

ab_b5_pred's per-pass series has lag-1 autocorrelation +0.51, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_b5_table)

The baseline ab_b5_table is the fastest (328.26 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ab_b5_table) is the fastest** at 328255.6 ns median
- 3 variants significantly slower than baseline
- Spread: 1.18x (fastest 328255.6 ns, slowest 386730.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b5_pred | 387074ns | 389403ns | 377065ns | 385953ns | 393759ns | +17.55% |
| ab_b5_prof | 346762ns | 347485ns | 340226ns | 347093ns | 349534ns | +5.30% |
| ab_b5_seq | 338741ns | 337977ns | 334868ns | 337586ns | 342411ns | +2.87% |
| ab_b5_table | 329296ns | 330520ns | 322576ns | 328208ns | 334288ns | base |
| ab_b5_tree | 334028ns | 333847ns | 327750ns | 331972ns | 340251ns | +1.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b5_pred | 384424ns | 374167ns | 391084ns | +17.62% | 0.003 |
| ab_b5_prof | 344260ns | 337899ns | 346914ns | +5.33% | 0.003 |
| ab_b5_seq | 336253ns | 332426ns | 339911ns | +2.88% | 0.003 |
| ab_b5_table | 326847ns | 319914ns | 331664ns | base | 0.003 |
| ab_b5_tree | 331452ns | 325417ns | 337773ns | +1.41% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b5_table; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b5_pred | 0.003 | 82.7% |
| ab_b5_prof | 0.003 | 92.7% |
| ab_b5_seq | 0.003 | 95.3% |
| ab_b5_table | 0.003 | 97.5% |
| ab_b5_tree | 0.003 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b5_pred | 387074ns | 387074ns | +17.55% |
| ab_b5_prof | 346762ns | 346762ns | +5.30% |
| ab_b5_seq | 338741ns | 338741ns | +2.87% |
| ab_b5_table | 329296ns | 329296ns | base |
| ab_b5_tree | 334028ns | 334028ns | +1.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b5_table | 328256ns | base | --- | [320622, 331664] | --- | --- | --- | --- |
| ab_b5_pred | 386730ns | +57298.1ns (+17.5%) | [+50511, +64922]ns | [375460, 391084] | YES | 0.0417 | 0.0313 | 0 |
| ab_b5_prof | 345066ns | +17145.7ns (+5.2%) | [+9926, +25166]ns | [340799, 346914] | YES | 0.0417 | 0.0313 | 0 |
| ab_b5_seq | 335576ns | +10507.9ns (+3.2%) | [+4850, +12859]ns | [333272, 339911] | YES | 0.0417 | 0.0313 | 0 |
| ab_b5_tree | 331147ns | no significant difference | [-3609, +12907]ns | [325437, 337773] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b5_table | ab_b5_pred | ab_b5_prof | ab_b5_seq | ab_b5_tree |
|---|---|---|---|---|---|
| 1 | 319914ns | +17.0% | +7.9% | +3.9% | +2.5% |
| 2 | 329983ns | +14.2% | +4.5% | +3.9% | -1.4% |
| 3 | 328109ns | +17.1% | +5.9% | +2.1% | -0.8% |
| 4 | 321331ns | +21.5% | +7.8% | +4.0% | +5.4% |
| 5 | 328402ns | +18.5% | +2.9% | +2.6% | +2.6% |
| 6 | 333344ns | +17.5% | +3.1% | +0.8% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b5_pred | 0.513 | HIGH+ (drift/warm-up) |
| ab_b5_prof | -0.001 | ok |
| ab_b5_seq | -0.494 | moderate- |
| ab_b5_table | -0.173 | ok |
| ab_b5_tree | 0.390 | moderate+ |

**Consistency summary:**

- **ab_b5_pred**: won 0/6, lost 6/6
- **ab_b5_prof**: won 0/6, lost 6/6
- **ab_b5_seq**: won 0/6, lost 6/6
- **ab_b5_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b5_pred | 8.8ns | 384424.5ns | 0.0% |  |
| ab_b5_prof | 5.5ns | 344259.6ns | 0.0% |  |
| ab_b5_seq | 6.5ns | 336252.8ns | 0.0% |  |
| ab_b5_table | 4.5ns | 326847.2ns | 0.0% |  |
| ab_b5_tree | 10.0ns | 331452.4ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b5_pred (n=6, range 374167.1-391083.5 ns)
  374167.1 |########################################
  375012.9 |
  375858.7 |
  376704.6 |########################################
  377550.4 |
  378396.2 |
  379242.0 |
  380087.9 |
  380933.7 |
  381779.5 |
  382625.3 |
  383471.1 |
  384317.0 |########################################
  385162.8 |
  386008.6 |
  386854.4 |
  387700.3 |
  388546.1 |########################################
  389391.9 |
  390237.7 |########################################
  (0 below, 1 above range)

ab_b5_prof (n=6, range 337898.7-346913.8 ns)
  337898.7 |########################################
  338349.5 |
  338800.2 |
  339251.0 |
  339701.7 |
  340152.5 |
  340603.2 |
  341054.0 |
  341504.7 |
  341955.5 |
  342406.2 |
  342857.0 |
  343307.7 |########################################
  343758.5 |
  344209.2 |
  344660.0 |########################################
  345110.7 |########################################
  345561.5 |
  346012.2 |########################################
  346463.0 |
  (0 below, 1 above range)

ab_b5_seq (n=6, range 332425.8-339910.8 ns)
  332425.8 |########################################
  332800.0 |
  333174.3 |
  333548.5 |
  333922.8 |########################################
  334297.0 |
  334671.3 |
  335045.5 |########################################
  335419.8 |
  335794.0 |########################################
  336168.3 |
  336542.6 |########################################
  336916.8 |
  337291.1 |
  337665.3 |
  338039.6 |
  338413.8 |
  338788.1 |
  339162.3 |
  339536.6 |
  (0 below, 1 above range)

ab_b5_table (n=6, range 319913.8-331663.8 ns)
  319913.8 |########################################
  320501.3 |
  321088.8 |########################################
  321676.3 |
  322263.8 |
  322851.3 |
  323438.8 |
  324026.3 |
  324613.8 |
  325201.3 |
  325788.8 |
  326376.3 |
  326963.8 |
  327551.3 |########################################
  328138.8 |########################################
  328726.3 |
  329313.8 |
  329901.3 |########################################
  330488.8 |
  331076.3 |
  (0 below, 1 above range)

ab_b5_tree (n=6, range 325416.7-337773.3 ns)
  325416.7 |########################################
  326034.5 |
  326652.4 |
  327270.2 |
  327888.0 |####################
  328505.9 |
  329123.7 |
  329741.5 |
  330359.3 |
  330977.2 |
  331595.0 |
  332212.8 |
  332830.7 |
  333448.5 |
  334066.3 |####################
  334684.2 |
  335302.0 |
  335919.8 |
  336537.6 |####################
  337155.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **ab_b5_pred**: autocorrelation=0.51 (measurement drift or warm-up artifact)
