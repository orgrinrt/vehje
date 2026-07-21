# Per-branch strategy: archetype 4 (ifchain4_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b4_table**

## Highlights

Baseline for all deltas below: **ab_b4_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (ab_b4_table)

The baseline ab_b4_table is the fastest (326.29 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ab_b4_table) is the fastest** at 326287.5 ns median
- 3 variants significantly slower than baseline
- Spread: 1.17x (fastest 326287.5 ns, slowest 381905.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b4_pred | 386302ns | 384408ns | 378975ns | 383099ns | 394769ns | +18.08% |
| ab_b4_prof | 338701ns | 339180ns | 333167ns | 338504ns | 341762ns | +3.53% |
| ab_b4_seq | 337343ns | 335340ns | 333248ns | 334977ns | 342940ns | +3.12% |
| ab_b4_table | 327143ns | 328861ns | 322047ns | 326860ns | 330114ns | base |
| ab_b4_tree | 331071ns | 332897ns | 321078ns | 331631ns | 335228ns | +1.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b4_pred | 383652ns | 376503ns | 391818ns | +18.20% | 0.003 |
| ab_b4_prof | 336163ns | 330566ns | 339196ns | +3.57% | 0.003 |
| ab_b4_seq | 334741ns | 330685ns | 340363ns | +3.13% | 0.003 |
| ab_b4_table | 324577ns | 319361ns | 327656ns | base | 0.003 |
| ab_b4_tree | 328476ns | 318342ns | 332541ns | +1.20% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b4_tree; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b4_pred | 0.003 | 83.4% |
| ab_b4_prof | 0.003 | 94.6% |
| ab_b4_seq | 0.003 | 95.7% |
| ab_b4_table | 0.003 | 97.6% |
| ab_b4_tree | 0.003 | 96.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b4_pred | 386302ns | 386302ns | +18.08% |
| ab_b4_prof | 338701ns | 338701ns | +3.53% |
| ab_b4_seq | 337343ns | 337343ns | +3.12% |
| ab_b4_table | 327143ns | 327143ns | base |
| ab_b4_tree | 331071ns | 331071ns | +1.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b4_table | 326288ns | base | --- | [319787, 327656] | --- | --- | --- | --- |
| ab_b4_pred | 381906ns | +62119.0ns (+19.0%) | [+49575, +65530]ns | [377231, 391818] | YES | 0.0417 | 0.0313 | 0 |
| ab_b4_prof | 336607ns | +10283.1ns (+3.2%) | [+8128, +16347]ns | [332686, 339196] | YES | 0.0417 | 0.0313 | 0 |
| ab_b4_seq | 332679ns | +12364.8ns (+3.8%) | [+3526, +14603]ns | [331182, 340363] | YES | 0.0417 | 0.0313 | 0 |
| ab_b4_tree | 330410ns | no significant difference | [-5180, +11550]ns | [322476, 332541] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b4_table | ab_b4_pred | ab_b4_prof | ab_b4_seq | ab_b4_tree |
|---|---|---|---|---|---|
| 1 | 320212ns | +19.9% | +3.2% | +4.0% | +3.6% |
| 2 | 319361ns | +18.9% | +6.2% | +4.0% | +3.6% |
| 3 | 326337ns | +19.7% | +4.0% | +3.6% | +1.1% |
| 4 | 328138ns | +14.7% | +2.6% | +0.8% | -3.0% |
| 5 | 327174ns | +15.5% | +2.3% | +1.4% | -0.2% |
| 6 | 326238ns | +20.5% | +3.1% | +5.0% | +2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b4_pred | -0.380 | moderate- |
| ab_b4_prof | -0.125 | ok |
| ab_b4_seq | -0.280 | moderate- |
| ab_b4_table | 0.467 | moderate+ |
| ab_b4_tree | 0.049 | ok |

**Consistency summary:**

- **ab_b4_pred**: won 0/6, lost 6/6
- **ab_b4_prof**: won 0/6, lost 6/6
- **ab_b4_seq**: won 0/6, lost 6/6
- **ab_b4_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b4_pred | 6.9ns | 383651.6ns | 0.0% |  |
| ab_b4_prof | 7.7ns | 336162.9ns | 0.0% |  |
| ab_b4_seq | 8.3ns | 334741.2ns | 0.0% |  |
| ab_b4_table | 6.7ns | 324576.7ns | 0.0% |  |
| ab_b4_tree | 5.9ns | 328475.7ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b4_pred (n=6, range 376502.9-391817.7 ns)
  376502.9 |########################################
  377268.6 |########################################
  378034.4 |
  378800.1 |
  379565.9 |########################################
  380331.6 |
  381097.3 |
  381863.1 |
  382628.8 |
  383394.6 |########################################
  384160.3 |
  384926.0 |
  385691.8 |
  386457.5 |
  387223.3 |
  387989.0 |
  388754.7 |
  389520.5 |
  390286.2 |########################################
  391052.0 |
  (0 below, 1 above range)

ab_b4_prof (n=6, range 330565.8-339196.0 ns)
  330565.8 |########################################
  330997.3 |
  331428.8 |
  331860.3 |
  332291.8 |
  332723.4 |
  333154.9 |
  333586.4 |
  334017.9 |
  334449.4 |########################################
  334880.9 |
  335312.4 |
  335744.0 |
  336175.5 |########################################
  336607.0 |########################################
  337038.5 |
  337470.0 |
  337901.5 |
  338333.0 |
  338764.5 |########################################
  (0 below, 1 above range)

ab_b4_seq (n=6, range 330685.4-340362.9 ns)
  330685.4 |########################################
  331169.3 |
  331653.2 |########################################
  332137.0 |########################################
  332620.9 |
  333104.8 |########################################
  333588.7 |
  334072.5 |
  334556.4 |
  335040.3 |
  335524.2 |
  336008.1 |
  336491.9 |
  336975.8 |
  337459.7 |
  337943.6 |########################################
  338427.4 |
  338911.3 |
  339395.2 |
  339879.1 |
  (0 below, 1 above range)

ab_b4_table (n=6, range 319360.8-327656.1 ns)
  319360.8 |####################
  319775.6 |
  320190.3 |####################
  320605.1 |
  321019.8 |
  321434.6 |
  321849.4 |
  322264.1 |
  322678.9 |
  323093.7 |
  323508.4 |
  323923.2 |
  324338.0 |
  324752.7 |
  325167.5 |
  325582.2 |
  325997.0 |########################################
  326411.8 |
  326826.5 |####################
  327241.3 |
  (0 below, 1 above range)

ab_b4_tree (n=6, range 318342.1-332540.8 ns)
  318342.1 |########################################
  319052.0 |
  319762.0 |
  320471.9 |
  321181.8 |
  321891.8 |
  322601.7 |
  323311.7 |
  324021.6 |
  324731.5 |
  325441.5 |
  326151.4 |########################################
  326861.3 |
  327571.3 |
  328281.2 |
  328991.2 |
  329701.1 |########################################
  330411.0 |########################################
  331121.0 |########################################
  331830.9 |
  (0 below, 1 above range)

```
