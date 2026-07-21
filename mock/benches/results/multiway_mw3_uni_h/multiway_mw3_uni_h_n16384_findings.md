# Multiway branch strategies, heavy-arm, mw3_uni: 3-way, uniform key

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw3_uni**

## Key findings

- **Fastest: mw_chain_rev_h_mw3_uni** at 291936.7 ns median (-1.2% vs baseline)
- Spread: 1.01x (fastest 291936.7 ns, slowest 295381.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 296176ns | 297991ns | 286095ns | 295364ns | 302435ns | base |
| mw_chain_h_mw3_uni | 296488ns | 296987ns | 291315ns | 296234ns | 299455ns | +0.11% |
| mw_chain_rev_h_mw3_uni | 293835ns | 294538ns | 286641ns | 293505ns | 297928ns | -0.79% |
| mw_jumptable_h_mw3_uni | 294712ns | 294638ns | 287635ns | 293675ns | 299808ns | -0.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 293579ns | 283502ns | 299759ns | base | 0.056 |
| mw_chain_h_mw3_uni | 293954ns | 289063ns | 296807ns | +0.13% | 0.056 |
| mw_chain_rev_h_mw3_uni | 291256ns | 284058ns | 295191ns | -0.79% | 0.056 |
| mw_jumptable_h_mw3_uni | 292183ns | 285402ns | 297043ns | -0.48% | 0.056 |

## Performance model

- Peak throughput: **0.058 Gops/s** (mw_bintree_h_mw3_uni; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw3_uni | 0.055 | 96.0% |
| mw_chain_h_mw3_uni | 0.056 | 96.3% |
| mw_chain_rev_h_mw3_uni | 0.056 | 97.1% |
| mw_jumptable_h_mw3_uni | 0.056 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw3_uni | 296176ns | 296176ns | base |
| mw_chain_h_mw3_uni | 296488ns | 296488ns | +0.11% |
| mw_chain_rev_h_mw3_uni | 293835ns | 293835ns | -0.79% |
| mw_jumptable_h_mw3_uni | 294712ns | 294712ns | -0.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 295381ns | base | --- | [285597, 299759] | --- | --- | --- | --- |
| mw_chain_h_mw3_uni | 294430ns | no significant difference | [-4696, +7008]ns | [290626, 296807] | no | 1.0000 | 0.6875 | 0 |
| mw_chain_rev_h_mw3_uni | 291937ns | no significant difference | [-7136, +1042]ns | [286639, 295191] | no | 1.0000 | 1.0000 | 0 |
| mw_jumptable_h_mw3_uni | 292125ns | no significant difference | [-7399, +2436]ns | [287381, 297043] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw3_uni | mw_chain_h_mw3_uni | mw_chain_rev_h_mw3_uni | mw_jumptable_h_mw3_uni |
|---|---|---|---|---|
| 1 | 283502ns | +4.5% | +0.2% | +0.7% |
| 2 | 287691ns | +0.5% | +0.5% | +0.6% |
| 3 | 296396ns | -0.6% | +0.1% | -0.0% |
| 4 | 294835ns | -0.2% | -0.7% | +1.0% |
| 5 | 303122ns | -1.9% | -3.9% | -4.1% |
| 6 | 295927ns | -1.3% | -0.8% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw3_uni | 0.333 | moderate+ |
| mw_chain_h_mw3_uni | -0.417 | moderate- |
| mw_chain_rev_h_mw3_uni | 0.118 | ok |
| mw_jumptable_h_mw3_uni | 0.187 | ok |

**Consistency summary:**

- **mw_chain_h_mw3_uni**: won 4/6, lost 2/6
- **mw_chain_rev_h_mw3_uni**: won 3/6, lost 3/6
- **mw_jumptable_h_mw3_uni**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 6.3ns | 293579.0ns | 0.0% |  |
| mw_chain_h_mw3_uni | 6.1ns | 293954.4ns | 0.0% |  |
| mw_chain_rev_h_mw3_uni | 7.8ns | 291255.6ns | 0.0% |  |
| mw_jumptable_h_mw3_uni | 7.3ns | 292183.2ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw3_uni (n=6, range 283502.5-299759.3 ns)
  283502.5 |####################
  284315.3 |
  285128.2 |
  285941.0 |
  286753.9 |
  287566.7 |####################
  288379.6 |
  289192.4 |
  290005.2 |
  290818.1 |
  291630.9 |
  292443.8 |
  293256.6 |
  294069.5 |####################
  294882.3 |
  295695.1 |########################################
  296508.0 |
  297320.8 |
  298133.7 |
  298946.5 |
  (0 below, 1 above range)

mw_chain_h_mw3_uni (n=6, range 289063.3-296807.1 ns)
  289063.3 |########################################
  289450.5 |
  289837.7 |
  290224.9 |
  290612.1 |
  290999.2 |
  291386.4 |
  291773.6 |
  292160.8 |########################################
  292548.0 |
  292935.2 |
  293322.4 |
  293709.6 |
  294096.8 |########################################
  294484.0 |########################################
  294871.1 |
  295258.3 |
  295645.5 |
  296032.7 |########################################
  296419.9 |
  (0 below, 1 above range)

mw_chain_rev_h_mw3_uni (n=6, range 284057.5-295191.0 ns)
  284057.5 |########################################
  284614.2 |
  285170.9 |
  285727.5 |
  286284.2 |
  286840.9 |
  287397.6 |
  287954.2 |
  288510.9 |
  289067.6 |########################################
  289624.3 |
  290181.0 |
  290737.6 |########################################
  291294.3 |
  291851.0 |
  292407.7 |########################################
  292964.3 |
  293521.0 |########################################
  294077.7 |
  294634.4 |
  (0 below, 1 above range)

mw_jumptable_h_mw3_uni (n=6, range 285402.1-297043.1 ns)
  285402.1 |########################################
  285984.1 |
  286566.2 |
  287148.2 |
  287730.3 |
  288312.3 |
  288894.4 |########################################
  289476.4 |
  290058.5 |
  290640.5 |########################################
  291222.6 |
  291804.6 |
  292386.7 |
  292968.8 |
  293550.8 |########################################
  294132.8 |
  294714.9 |
  295296.9 |
  295879.0 |########################################
  296461.0 |
  (0 below, 1 above range)

```
