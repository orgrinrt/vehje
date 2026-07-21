# Multiway branch strategies, heavy-arm, mw8_uni: 8-way, uniform key

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw8_uni**

## Key findings

- **Fastest: mw_chain_rev_h_mw8_uni** at 65867.7 ns median (-7.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.08x (fastest 65867.7 ns, slowest 71311.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 81466ns | 73496ns | 72265ns | 73235ns | 98413ns | base |
| mw_chain_h_mw8_uni | 69798ns | 68874ns | 67245ns | 68546ns | 72951ns | -14.32% |
| mw_chain_rev_h_mw8_uni | 76421ns | 68116ns | 66925ns | 67758ns | 94165ns | -6.19% |
| mw_jumptable_h_mw8_uni | 72019ns | 68378ns | 66452ns | 68258ns | 80443ns | -11.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 79069ns | 70060ns | 95645ns | base | 0.052 |
| mw_chain_h_mw8_uni | 67462ns | 64977ns | 70502ns | -14.68% | 0.061 |
| mw_chain_rev_h_mw8_uni | 73972ns | 64666ns | 91294ns | -6.45% | 0.055 |
| mw_jumptable_h_mw8_uni | 69668ns | 64224ns | 77945ns | -11.89% | 0.059 |

## Performance model

- Peak throughput: **0.064 Gops/s** (mw_jumptable_h_mw8_uni; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw8_uni | 0.057 | 90.1% |
| mw_chain_h_mw8_uni | 0.061 | 96.4% |
| mw_chain_rev_h_mw8_uni | 0.062 | 97.5% |
| mw_jumptable_h_mw8_uni | 0.062 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw8_uni | 81466ns | 81466ns | base |
| mw_chain_h_mw8_uni | 69798ns | 69798ns | -14.32% |
| mw_chain_rev_h_mw8_uni | 76421ns | 76421ns | -6.19% |
| mw_jumptable_h_mw8_uni | 72019ns | 72019ns | -11.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 71311ns | base | --- | [70250, 95645] | --- | --- | --- | --- |
| mw_chain_h_mw8_uni | 66608ns | -5693.5ns (-8.0%) | [-25143, -3985]ns | [65276, 70502] | YES | 0.0469 | 0.0313 | 0 |
| mw_chain_rev_h_mw8_uni | 65868ns | no significant difference | [-11213, +895]ns | [64754, 91294] | no | 0.2188 | 0.2188 | 0 |
| mw_jumptable_h_mw8_uni | 66128ns | -6067.3ns (-8.5%) | [-17700, -4435]ns | [64931, 77945] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw8_uni | mw_chain_h_mw8_uni | mw_chain_rev_h_mw8_uni | mw_jumptable_h_mw8_uni |
|---|---|---|---|---|
| 1 | 109350ns | -37.6% | +5.3% | -19.5% |
| 2 | 71942ns | -8.2% | -6.3% | -7.9% |
| 3 | 70680ns | -7.2% | -8.3% | -9.1% |
| 4 | 70060ns | -4.1% | -7.7% | -5.8% |
| 5 | 70440ns | -7.8% | -5.8% | -6.8% |
| 6 | 81940ns | -11.2% | -20.2% | -17.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw8_uni | -0.023 | ok |
| mw_chain_h_mw8_uni | -0.256 | moderate- |
| mw_chain_rev_h_mw8_uni | 0.005 | ok |
| mw_jumptable_h_mw8_uni | -0.005 | ok |

**Consistency summary:**

- **mw_chain_h_mw8_uni**: won 6/6, lost 0/6
- **mw_chain_rev_h_mw8_uni**: won 5/6, lost 1/6
- **mw_jumptable_h_mw8_uni**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 8.9ns | 79068.8ns | 0.0% |  |
| mw_chain_h_mw8_uni | 3.9ns | 67461.7ns | 0.0% |  |
| mw_chain_rev_h_mw8_uni | 10.0ns | 73971.9ns | 0.0% |  |
| mw_jumptable_h_mw8_uni | 5.2ns | 69667.8ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw8_uni (n=6, range 70060.4-95645.0 ns)
  70060.4 |########################################
  71339.6 |#############
  72618.9 |
  73898.1 |
  75177.3 |
  76456.5 |
  77735.8 |
  79015.0 |
  80294.2 |
  81573.5 |#############
  82852.7 |
  84131.9 |
  85411.2 |
  86690.4 |
  87969.6 |
  89248.9 |
  90528.1 |
  91807.3 |
  93086.5 |
  94365.8 |
  (0 below, 1 above range)

mw_chain_h_mw8_uni (n=6, range 64977.1-70501.9 ns)
  64977.1 |########################################
  65253.3 |
  65529.6 |########################################
  65805.8 |########################################
  66082.1 |
  66358.3 |
  66634.5 |
  66910.8 |
  67187.0 |########################################
  67463.2 |
  67739.5 |
  68015.7 |########################################
  68291.9 |
  68568.2 |
  68844.4 |
  69120.7 |
  69396.9 |
  69673.1 |
  69949.4 |
  70225.6 |
  (0 below, 1 above range)

mw_chain_rev_h_mw8_uni (n=6, range 64666.2-91294.0 ns)
  64666.2 |########################################
  65997.6 |#############
  67329.0 |#############
  68660.4 |
  69991.8 |
  71323.1 |
  72654.5 |
  73985.9 |
  75317.3 |
  76648.7 |
  77980.1 |
  79311.5 |
  80642.9 |
  81974.2 |
  83305.6 |
  84637.0 |
  85968.4 |
  87299.8 |
  88631.2 |
  89962.6 |
  (0 below, 1 above range)

mw_jumptable_h_mw8_uni (n=6, range 64224.2-77944.5 ns)
  64224.2 |#############
  64910.2 |
  65596.2 |########################################
  66282.3 |
  66968.3 |
  67654.3 |#############
  68340.3 |
  69026.3 |
  69712.3 |
  70398.4 |
  71084.4 |
  71770.4 |
  72456.4 |
  73142.4 |
  73828.4 |
  74514.5 |
  75200.5 |
  75886.5 |
  76572.5 |
  77258.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **mw_chain_rev_h_mw8_uni**: CV=25.0% (high variance, measurements may be unstable)
