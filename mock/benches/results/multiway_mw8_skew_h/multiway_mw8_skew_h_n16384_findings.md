# Multiway branch strategies, heavy-arm, mw8_skew: 8-way, skewed to arm 0 (~70%)

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw8_skew**

## Key findings

- **Fastest: mw_chain_h_mw8_skew** at 274802.0 ns median (-8.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.09x (fastest 274802.0 ns, slowest 300238.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 303868ns | 302642ns | 297381ns | 302332ns | 309417ns | base |
| mw_chain_h_mw8_skew | 277357ns | 277444ns | 270180ns | 276200ns | 282682ns | -8.72% |
| mw_chain_rev_h_mw8_skew | 297504ns | 296296ns | 288582ns | 294370ns | 306666ns | -2.09% |
| mw_jumptable_h_mw8_skew | 296675ns | 296675ns | 289388ns | 296319ns | 300852ns | -2.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 301255ns | 294529ns | 306996ns | base | 0.054 |
| mw_chain_h_mw8_skew | 274845ns | 267977ns | 280042ns | -8.77% | 0.060 |
| mw_chain_rev_h_mw8_skew | 294874ns | 285690ns | 304060ns | -2.12% | 0.056 |
| mw_jumptable_h_mw8_skew | 294107ns | 286816ns | 298222ns | -2.37% | 0.056 |

## Performance model

- Peak throughput: **0.061 Gops/s** (mw_chain_h_mw8_skew; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw8_skew | 0.055 | 89.3% |
| mw_chain_h_mw8_skew | 0.060 | 97.5% |
| mw_chain_rev_h_mw8_skew | 0.056 | 91.2% |
| mw_jumptable_h_mw8_skew | 0.056 | 91.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw8_skew | 303868ns | 303868ns | base |
| mw_chain_h_mw8_skew | 277357ns | 277357ns | -8.72% |
| mw_chain_rev_h_mw8_skew | 297504ns | 297504ns | -2.09% |
| mw_jumptable_h_mw8_skew | 296675ns | 296675ns | -2.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 300239ns | base | --- | [296530, 306996] | --- | --- | --- | --- |
| mw_chain_h_mw8_skew | 274802ns | -28727.5ns (-9.6%) | [-30076, -20427]ns | [269691, 280042] | YES | 0.0469 | 0.0313 | 0 |
| mw_chain_rev_h_mw8_skew | 293856ns | no significant difference | [-10606, +26]ns | [286705, 304060] | no | 0.2188 | 0.2188 | 0 |
| mw_jumptable_h_mw8_skew | 294219ns | -6020.2ns (-2.0%) | [-13929, -1495]ns | [289880, 298222] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw8_skew | mw_chain_h_mw8_skew | mw_chain_rev_h_mw8_skew | mw_jumptable_h_mw8_skew |
|---|---|---|---|---|
| 1 | 312406ns | -9.3% | -3.3% | -5.2% |
| 2 | 301002ns | -9.8% | +1.7% | -1.9% |
| 3 | 301587ns | -9.4% | -2.7% | -0.5% |
| 4 | 298532ns | -10.2% | -3.6% | -3.9% |
| 5 | 294529ns | -6.1% | -3.0% | -0.5% |
| 6 | 299475ns | -7.7% | -1.7% | -2.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw8_skew | 0.147 | ok |
| mw_chain_h_mw8_skew | -0.151 | ok |
| mw_chain_rev_h_mw8_skew | 0.455 | moderate+ |
| mw_jumptable_h_mw8_skew | -0.263 | moderate- |

**Consistency summary:**

- **mw_chain_h_mw8_skew**: won 6/6, lost 0/6
- **mw_chain_rev_h_mw8_skew**: won 5/6, lost 1/6
- **mw_jumptable_h_mw8_skew**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 9.0ns | 301255.1ns | 0.0% |  |
| mw_chain_h_mw8_skew | 14.1ns | 274844.8ns | 0.0% |  |
| mw_chain_rev_h_mw8_skew | 14.5ns | 294873.7ns | 0.0% |  |
| mw_jumptable_h_mw8_skew | 21.4ns | 294106.8ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw8_skew (n=6, range 294529.2-306996.2 ns)
  294529.2 |########################################
  295152.6 |
  295775.9 |
  296399.3 |
  297022.6 |
  297646.0 |
  298269.3 |########################################
  298892.7 |########################################
  299516.0 |
  300139.4 |
  300762.7 |########################################
  301386.1 |########################################
  302009.4 |
  302632.8 |
  303256.1 |
  303879.5 |
  304502.8 |
  305126.2 |
  305749.5 |
  306372.9 |
  (0 below, 1 above range)

mw_chain_h_mw8_skew (n=6, range 267976.7-280041.7 ns)
  267976.7 |####################
  268580.0 |
  269183.2 |
  269786.5 |
  270389.7 |
  270993.0 |####################
  271596.2 |
  272199.5 |
  272802.7 |####################
  273406.0 |
  274009.2 |
  274612.5 |
  275215.7 |
  275819.0 |
  276422.2 |########################################
  277025.5 |
  277628.7 |
  278232.0 |
  278835.2 |
  279438.5 |
  (0 below, 1 above range)

mw_chain_rev_h_mw8_skew (n=6, range 285690.4-304059.8 ns)
  285690.4 |########################################
  286608.9 |
  287527.3 |########################################
  288445.8 |
  289364.3 |
  290282.8 |
  291201.2 |
  292119.7 |
  293038.2 |########################################
  293956.6 |########################################
  294875.1 |
  295793.6 |
  296712.0 |
  297630.5 |
  298549.0 |
  299467.5 |
  300385.9 |
  301304.4 |########################################
  302222.9 |
  303141.3 |
  (0 below, 1 above range)

mw_jumptable_h_mw8_skew (n=6, range 286816.2-298222.1 ns)
  286816.2 |########################################
  287386.5 |
  287956.8 |
  288527.1 |
  289097.4 |
  289667.7 |
  290238.0 |
  290808.3 |
  291378.6 |
  291948.9 |
  292519.2 |########################################
  293089.4 |########################################
  293659.7 |
  294230.0 |
  294800.3 |########################################
  295370.6 |
  295940.9 |########################################
  296511.2 |
  297081.5 |
  297651.8 |
  (0 below, 1 above range)

```
