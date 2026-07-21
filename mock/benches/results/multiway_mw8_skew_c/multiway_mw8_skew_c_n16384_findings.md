# Multiway branch strategies, cheap-arm, mw8_skew: 8-way, skewed to arm 0 (~70%)

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw8_skew**

## Key findings

- **Fastest: mw_jumptable_c_mw8_skew** at 26293.5 ns median (-29.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.66x (fastest 26293.5 ns, slowest 122625.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 39369ns | 39644ns | 35750ns | 39549ns | 40908ns | base |
| mw_chain_c_mw8_skew | 47600ns | 48780ns | 42987ns | 47260ns | 50418ns | +20.91% |
| mw_chain_rev_c_mw8_skew | 31442ns | 32341ns | 29012ns | 31585ns | 32442ns | -20.14% |
| mw_jumptable_c_mw8_skew | 28701ns | 28736ns | 25661ns | 28688ns | 30242ns | -27.10% |
| mw_predicate_all_c_mw8_skew | 125095ns | 125113ns | 120920ns | 124376ns | 128261ns | +217.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 36925ns | 33519ns | 38364ns | base | 0.444 |
| mw_chain_c_mw8_skew | 45120ns | 40742ns | 47833ns | +22.19% | 0.363 |
| mw_chain_rev_c_mw8_skew | 29065ns | 26828ns | 29989ns | -21.29% | 0.564 |
| mw_jumptable_c_mw8_skew | 26262ns | 23482ns | 27667ns | -28.88% | 0.624 |
| mw_predicate_all_c_mw8_skew | 122635ns | 118559ns | 125712ns | +232.12% | 0.134 |

## Performance model

- Peak throughput: **0.698 Gops/s** (mw_jumptable_c_mw8_skew; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw8_skew | 0.441 | 63.2% |
| mw_chain_c_mw8_skew | 0.355 | 50.9% |
| mw_chain_rev_c_mw8_skew | 0.548 | 78.6% |
| mw_jumptable_c_mw8_skew | 0.623 | 89.3% |
| mw_predicate_all_c_mw8_skew | 0.134 | 19.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw8_skew | 39369ns | 39369ns | base |
| mw_chain_c_mw8_skew | 47600ns | 47600ns | +20.91% |
| mw_chain_rev_c_mw8_skew | 31442ns | 31442ns | -20.14% |
| mw_jumptable_c_mw8_skew | 28701ns | 28701ns | -27.10% |
| mw_predicate_all_c_mw8_skew | 125095ns | 125095ns | +217.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 37182ns | base | --- | [35229, 38364] | --- | --- | --- | --- |
| mw_chain_c_mw8_skew | 46141ns | +9024.5ns (+24.3%) | [+5097, +10463]ns | [41385, 47833] | YES | 0.0313 | 0.0313 | 0 |
| mw_chain_rev_c_mw8_skew | 29894ns | -7286.2ns (-19.6%) | [-9911, -6383]ns | [27312, 29989] | YES | 0.0313 | 0.0313 | 0 |
| mw_jumptable_c_mw8_skew | 26294ns | -10501.1ns (-28.2%) | [-12070, -9417]ns | [24826, 27667] | YES | 0.0313 | 0.0313 | 0 |
| mw_predicate_all_c_mw8_skew | 122625ns | +86860.4ns (+233.6%) | [+82071, +88197]ns | [119566, 125712] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw8_skew | mw_chain_c_mw8_skew | mw_chain_rev_c_mw8_skew | mw_jumptable_c_mw8_skew | mw_predicate_all_c_mw8_skew |
|---|---|---|---|---|---|
| 1 | 37136ns | +22.6% | -19.5% | -29.5% | +227.6% |
| 2 | 36939ns | +27.0% | -19.1% | -24.0% | +234.6% |
| 3 | 37229ns | +25.6% | -19.7% | -26.8% | +238.6% |
| 4 | 38927ns | +4.7% | -31.1% | -32.8% | +204.6% |
| 5 | 33519ns | +25.4% | -17.1% | -29.9% | +259.7% |
| 6 | 37801ns | +29.0% | -20.4% | -30.1% | +231.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw8_skew | -0.556 | HIGH- (thermal bounce) |
| mw_chain_c_mw8_skew | -0.027 | ok |
| mw_chain_rev_c_mw8_skew | 0.111 | ok |
| mw_jumptable_c_mw8_skew | 0.114 | ok |
| mw_predicate_all_c_mw8_skew | -0.210 | moderate- |

**Consistency summary:**

- **mw_chain_c_mw8_skew**: won 0/6, lost 6/6
- **mw_chain_rev_c_mw8_skew**: won 6/6, lost 0/6
- **mw_jumptable_c_mw8_skew**: won 6/6, lost 0/6
- **mw_predicate_all_c_mw8_skew**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 2.6ns | 36925.1ns | 0.0% |  |
| mw_chain_c_mw8_skew | 5.0ns | 45120.0ns | 0.0% |  |
| mw_chain_rev_c_mw8_skew | 4.0ns | 29065.0ns | 0.0% |  |
| mw_jumptable_c_mw8_skew | 4.2ns | 26262.2ns | 0.0% |  |
| mw_predicate_all_c_mw8_skew | 3.8ns | 122634.5ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw8_skew (n=6, range 33518.8-38363.9 ns)
  33518.8 |####################
  33761.1 |
  34003.3 |
  34245.6 |
  34487.8 |
  34730.1 |
  34972.3 |
  35214.6 |
  35456.9 |
  35699.1 |
  35941.4 |
  36183.6 |
  36425.9 |
  36668.1 |
  36910.4 |########################################
  37152.7 |####################
  37394.9 |
  37637.2 |####################
  37879.4 |
  38121.7 |
  (0 below, 1 above range)

mw_chain_c_mw8_skew (n=6, range 40741.7-47833.2 ns)
  40741.7 |########################################
  41096.3 |
  41450.8 |
  41805.4 |########################################
  42160.0 |
  42514.6 |
  42869.1 |
  43223.7 |
  43578.3 |
  43932.9 |
  44287.4 |
  44642.0 |
  44996.6 |
  45351.1 |########################################
  45705.7 |
  46060.3 |
  46414.9 |########################################
  46769.4 |########################################
  47124.0 |
  47478.6 |
  (0 below, 1 above range)

mw_chain_rev_c_mw8_skew (n=6, range 26827.5-29989.3 ns)
  26827.5 |#############
  26985.6 |
  27143.7 |
  27301.8 |
  27459.9 |
  27618.0 |
  27776.1 |#############
  27934.1 |
  28092.2 |
  28250.3 |
  28408.4 |
  28566.5 |
  28724.6 |
  28882.7 |
  29040.8 |
  29198.9 |
  29357.0 |
  29515.1 |
  29673.2 |
  29831.3 |########################################
  (0 below, 1 above range)

mw_jumptable_c_mw8_skew (n=6, range 23482.1-27666.9 ns)
  23482.1 |####################
  23691.3 |
  23900.6 |
  24109.8 |
  24319.1 |
  24528.3 |
  24737.5 |
  24946.8 |
  25156.0 |
  25365.3 |
  25574.5 |
  25783.7 |
  25993.0 |########################################
  26202.2 |
  26411.5 |####################
  26620.7 |
  26829.9 |
  27039.2 |
  27248.4 |####################
  27457.7 |
  (0 below, 1 above range)

mw_predicate_all_c_mw8_skew (n=6, range 118559.2-125712.1 ns)
  118559.2 |########################################
  118916.8 |
  119274.5 |
  119632.1 |
  119989.8 |
  120347.4 |########################################
  120705.1 |
  121062.7 |
  121420.4 |########################################
  121778.0 |
  122135.6 |
  122493.3 |
  122850.9 |
  123208.6 |
  123566.2 |########################################
  123923.9 |
  124281.5 |
  124639.2 |
  124996.8 |
  125354.5 |########################################
  (0 below, 1 above range)

```
