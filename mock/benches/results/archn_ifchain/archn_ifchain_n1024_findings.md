# Per-type strategy (NATIVE tier): all ifchain

5 variants, 6 samples per variant.
Baseline: **an_ifchain_table**

## Highlights

Baseline for all deltas below: **an_ifchain_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_ifchain_pred is an outlier: 2.1x slower than the field

an_ifchain_pred (15.97 us) is 2.1x the fastest (7.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {an_ifchain_prof, an_ifchain_seq, an_ifchain_tree, an_ifchain_table} vs {an_ifchain_pred} (91% apart)

The field splits into a fast tier {an_ifchain_prof, an_ifchain_seq, an_ifchain_tree, an_ifchain_table} and a slow tier {an_ifchain_pred} with a 91% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader an_ifchain_prof vs stability leader an_ifchain_tree (+10% speed for 1.2x steadier)

an_ifchain_prof is fastest (7.61 us, CV 7.1%); an_ifchain_tree gives up 9.7% median for 1.2x lower variance (CV 6.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: an_ifchain_prof** at 7613.9 ns median (-8.9% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.10x (fastest 7613.9 ns, slowest 15968.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_ifchain_pred | 18159ns | 18467ns | 16094ns | 18163ns | 19184ns | +71.91% |
| an_ifchain_prof | 9815ns | 10037ns | 8745ns | 9690ns | 10537ns | -7.08% |
| an_ifchain_seq | 9910ns | 10374ns | 8710ns | 9953ns | 10444ns | -6.19% |
| an_ifchain_table | 10563ns | 10852ns | 9481ns | 10414ns | 11328ns | base |
| an_ifchain_tree | 10686ns | 10865ns | 9702ns | 10552ns | 11379ns | +1.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_ifchain_pred | 15739ns | 13934ns | 16620ns | +93.29% | 0.065 |
| an_ifchain_prof | 7417ns | 6584ns | 7935ns | -8.92% | 0.138 |
| an_ifchain_seq | 7466ns | 6562ns | 7874ns | -8.31% | 0.137 |
| an_ifchain_table | 8143ns | 7305ns | 8731ns | base | 0.126 |
| an_ifchain_tree | 8213ns | 7469ns | 8710ns | +0.86% | 0.125 |

## Performance model

- Peak throughput: **0.156 Gops/s** (an_ifchain_seq; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_ifchain_pred | 0.064 | 41.1% |
| an_ifchain_prof | 0.134 | 86.2% |
| an_ifchain_seq | 0.131 | 84.0% |
| an_ifchain_table | 0.123 | 78.5% |
| an_ifchain_tree | 0.123 | 78.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_ifchain_pred | 18159ns | 18159ns | +71.91% |
| an_ifchain_prof | 9815ns | 9815ns | -7.08% |
| an_ifchain_seq | 9910ns | 9910ns | -6.19% |
| an_ifchain_table | 10563ns | 10563ns | base |
| an_ifchain_tree | 10686ns | 10686ns | +1.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_ifchain_table | 8355ns | base | --- | [7343, 8731] | --- | --- | --- | --- |
| an_ifchain_pred | 15968ns | +7810.7ns (+93.5%) | [+6714, +8264]ns | [14629, 16620] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_ifchain_prof | 7614ns | -788.8ns (-9.4%) | [-1146, -244]ns | [6702, 7935] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| an_ifchain_seq | 7812ns | -727.4ns (-8.7%) | [-920, -384]ns | [6711, 7874] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_ifchain_tree | 8352ns | no significant difference | [-317, +426]ns | [7576, 8710] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_ifchain_table | an_ifchain_pred | an_ifchain_prof | an_ifchain_seq | an_ifchain_tree |
|---|---|---|---|---|---|
| 1 | 8119ns | +105.7% | -16.0% | -3.1% | +6.8% |
| 2 | 8798ns | +88.0% | -11.3% | -11.0% | -0.5% |
| 3 | 7380ns | +107.6% | +0.6% | -7.0% | +4.1% |
| 4 | 7305ns | +90.7% | -9.9% | -10.2% | +2.2% |
| 5 | 8665ns | +78.5% | -9.9% | -10.1% | -6.8% |
| 6 | 8590ns | +91.8% | -6.2% | -8.3% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_ifchain_pred | 0.264 | moderate+ |
| an_ifchain_prof | -0.175 | ok |
| an_ifchain_seq | 0.179 | ok |
| an_ifchain_table | -0.037 | ok |
| an_ifchain_tree | 0.262 | moderate+ |

**Consistency summary:**

- **an_ifchain_pred**: won 0/6, lost 6/6
- **an_ifchain_prof**: won 5/6, lost 1/6
- **an_ifchain_seq**: won 6/6, lost 0/6
- **an_ifchain_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_ifchain_pred | 5.0ns | 15739.2ns | 0.0% |  |
| an_ifchain_prof | 5.5ns | 7416.7ns | 0.1% |  |
| an_ifchain_seq | 5.7ns | 7465.8ns | 0.1% |  |
| an_ifchain_table | 6.1ns | 8142.9ns | 0.1% |  |
| an_ifchain_tree | 6.5ns | 8212.6ns | 0.1% |  |

## Distribution (algo ns)

```
an_ifchain_pred (n=6, range 13933.8-16620.4 ns)
  13933.8 |########################################
  14068.1 |
  14202.5 |
  14336.8 |
  14471.1 |
  14605.5 |
  14739.8 |
  14874.1 |
  15008.4 |
  15142.8 |
  15277.1 |########################################
  15411.4 |########################################
  15545.8 |
  15680.1 |
  15814.4 |
  15948.8 |
  16083.1 |
  16217.4 |
  16351.7 |########################################
  16486.1 |########################################
  (0 below, 1 above range)

an_ifchain_prof (n=6, range 6584.2-7934.5 ns)
   6584.2 |####################
   6651.7 |
   6719.2 |
   6786.8 |####################
   6854.3 |
   6921.8 |
   6989.3 |
   7056.8 |
   7124.3 |
   7191.9 |
   7259.4 |
   7326.9 |
   7394.4 |####################
   7461.9 |
   7529.4 |
   7597.0 |
   7664.5 |
   7732.0 |
   7799.5 |########################################
   7867.0 |
  (0 below, 1 above range)

an_ifchain_seq (n=6, range 6561.7-7874.4 ns)
   6561.7 |####################
   6627.3 |
   6693.0 |
   6758.6 |
   6824.2 |####################
   6889.9 |
   6955.5 |
   7021.1 |
   7086.8 |
   7152.4 |
   7218.0 |
   7283.7 |
   7349.3 |
   7415.0 |
   7480.6 |
   7546.2 |
   7611.9 |
   7677.5 |
   7743.1 |####################
   7808.8 |########################################
  (0 below, 1 above range)

an_ifchain_table (n=6, range 7305.4-8731.5 ns)
   7305.4 |########################################
   7376.7 |########################################
   7448.0 |
   7519.3 |
   7590.6 |
   7661.9 |
   7733.2 |
   7804.5 |
   7875.8 |
   7947.1 |
   8018.4 |
   8089.7 |########################################
   8161.0 |
   8232.3 |
   8303.6 |
   8374.9 |
   8446.2 |
   8517.5 |
   8588.8 |########################################
   8660.1 |########################################
  (0 below, 1 above range)

an_ifchain_tree (n=6, range 7469.2-8710.2 ns)
   7469.2 |########################################
   7531.2 |
   7593.3 |
   7655.4 |########################################
   7717.4 |
   7779.4 |
   7841.5 |
   7903.6 |
   7965.6 |
   8027.7 |########################################
   8089.7 |
   8151.8 |
   8213.8 |
   8275.9 |
   8337.9 |
   8400.0 |
   8462.0 |
   8524.1 |
   8586.1 |########################################
   8648.2 |########################################
  (0 below, 1 above range)

```
