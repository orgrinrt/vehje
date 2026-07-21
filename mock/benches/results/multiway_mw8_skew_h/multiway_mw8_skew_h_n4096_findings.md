# Multiway branch strategies, heavy-arm, mw8_skew: 8-way, skewed to arm 0 (~70%)

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw8_skew**

## Key findings

- **Fastest: mw_chain_h_mw8_skew** at 69971.2 ns median (-12.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.15x (fastest 69971.2 ns, slowest 80132.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 82308ns | 82449ns | 75703ns | 80647ns | 88102ns | base |
| mw_chain_h_mw8_skew | 72403ns | 72468ns | 67232ns | 70952ns | 77164ns | -12.03% |
| mw_chain_rev_h_mw8_skew | 80963ns | 83399ns | 73340ns | 80515ns | 85447ns | -1.63% |
| mw_jumptable_h_mw8_skew | 78208ns | 78283ns | 71385ns | 76510ns | 84166ns | -4.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 79879ns | 73468ns | 85496ns | base | 0.051 |
| mw_chain_h_mw8_skew | 69934ns | 64836ns | 74558ns | -12.45% | 0.059 |
| mw_chain_rev_h_mw8_skew | 78260ns | 71153ns | 82805ns | -2.03% | 0.052 |
| mw_jumptable_h_mw8_skew | 75786ns | 69242ns | 81573ns | -5.12% | 0.054 |

## Performance model

- Peak throughput: **0.063 Gops/s** (mw_chain_h_mw8_skew; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw8_skew | 0.051 | 81.0% |
| mw_chain_h_mw8_skew | 0.059 | 92.7% |
| mw_chain_rev_h_mw8_skew | 0.051 | 80.9% |
| mw_jumptable_h_mw8_skew | 0.054 | 85.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw8_skew | 82308ns | 82308ns | base |
| mw_chain_h_mw8_skew | 72403ns | 72403ns | -12.03% |
| mw_chain_rev_h_mw8_skew | 80963ns | 80963ns | -1.63% |
| mw_jumptable_h_mw8_skew | 78208ns | 78208ns | -4.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 80046ns | base | --- | [74096, 85496] | --- | --- | --- | --- |
| mw_chain_h_mw8_skew | 69971ns | -9189.7ns (-11.5%) | [-12496, -8149]ns | [65274, 74558] | YES | 0.0469 | 0.0313 | 0 |
| mw_chain_rev_h_mw8_skew | 80132ns | no significant difference | [-3115, +619]ns | [71844, 82805] | no | 0.2188 | 0.2188 | 0 |
| mw_jumptable_h_mw8_skew | 75804ns | -3954.9ns (-4.9%) | [-5031, -3295]ns | [69981, 81573] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw8_skew | mw_chain_h_mw8_skew | mw_chain_rev_h_mw8_skew | mw_jumptable_h_mw8_skew |
|---|---|---|---|---|
| 1 | 74724ns | -12.1% | +4.2% | -4.7% |
| 2 | 73468ns | -11.7% | -3.2% | -5.8% |
| 3 | 85418ns | -13.4% | -2.8% | -4.3% |
| 4 | 85574ns | -15.8% | -3.7% | -6.1% |
| 5 | 84525ns | -11.1% | -2.3% | -3.7% |
| 6 | 75567ns | -10.1% | -4.0% | -6.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw8_skew | 0.208 | moderate+ |
| mw_chain_h_mw8_skew | 0.103 | ok |
| mw_chain_rev_h_mw8_skew | -0.131 | ok |
| mw_jumptable_h_mw8_skew | 0.086 | ok |

**Consistency summary:**

- **mw_chain_h_mw8_skew**: won 6/6, lost 0/6
- **mw_chain_rev_h_mw8_skew**: won 5/6, lost 1/6
- **mw_jumptable_h_mw8_skew**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 3.8ns | 79879.4ns | 0.0% |  |
| mw_chain_h_mw8_skew | 3.7ns | 69934.4ns | 0.0% |  |
| mw_chain_rev_h_mw8_skew | 4.4ns | 78260.3ns | 0.0% |  |
| mw_jumptable_h_mw8_skew | 3.2ns | 75786.0ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw8_skew (n=6, range 73468.3-85496.0 ns)
  73468.3 |########################################
  74069.7 |
  74671.1 |########################################
  75272.5 |########################################
  75873.9 |
  76475.2 |
  77076.6 |
  77678.0 |
  78279.4 |
  78880.8 |
  79482.2 |
  80083.6 |
  80684.9 |
  81286.3 |
  81887.7 |
  82489.1 |
  83090.5 |
  83691.9 |
  84293.3 |########################################
  84894.7 |########################################
  (0 below, 1 above range)

mw_chain_h_mw8_skew (n=6, range 64836.2-74558.4 ns)
  64836.2 |########################################
  65322.3 |########################################
  65808.4 |
  66294.5 |
  66780.6 |
  67266.7 |
  67752.8 |########################################
  68239.0 |
  68725.1 |
  69211.2 |
  69697.3 |
  70183.4 |
  70669.5 |
  71155.6 |
  71641.7 |########################################
  72127.8 |
  72613.9 |
  73100.0 |
  73586.1 |########################################
  74072.2 |
  (0 below, 1 above range)

mw_chain_rev_h_mw8_skew (n=6, range 71153.3-82805.0 ns)
  71153.3 |####################
  71735.9 |
  72318.5 |####################
  72901.1 |
  73483.6 |
  74066.2 |
  74648.8 |
  75231.4 |
  75814.0 |
  76396.6 |
  76979.1 |
  77561.7 |####################
  78144.3 |
  78726.9 |
  79309.5 |
  79892.1 |
  80474.7 |
  81057.2 |
  81639.8 |
  82222.4 |########################################
  (0 below, 1 above range)

mw_jumptable_h_mw8_skew (n=6, range 69242.1-81572.9 ns)
  69242.1 |########################################
  69858.6 |
  70475.2 |########################################
  71091.7 |########################################
  71708.3 |
  72324.8 |
  72941.4 |
  73557.9 |
  74174.4 |
  74791.0 |
  75407.5 |
  76024.1 |
  76640.6 |
  77257.2 |
  77873.7 |
  78490.2 |
  79106.8 |
  79723.3 |
  80339.9 |########################################
  80956.4 |########################################
  (0 below, 1 above range)

```
