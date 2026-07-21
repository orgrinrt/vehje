# Multiway branch strategies, heavy-arm, mw3_skew: 3-way, skewed to arm 0 (~80%)

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw3_skew**

## Key findings

- **Baseline (mw_bintree_h_mw3_skew) is the fastest** at 20225.4 ns median
- Spread: 1.00x (fastest 20225.4 ns, slowest 20240.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 22233ns | 22828ns | 19103ns | 22727ns | 23058ns | base |
| mw_chain_h_mw3_skew | 22597ns | 22840ns | 21345ns | 22839ns | 22860ns | +1.64% |
| mw_chain_rev_h_mw3_skew | 22148ns | 22839ns | 19386ns | 22456ns | 23068ns | -0.38% |
| mw_jumptable_h_mw3_skew | 22395ns | 22826ns | 19894ns | 22822ns | 23005ns | +0.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 19685ns | 16904ns | 20391ns | base | 0.052 |
| mw_chain_h_mw3_skew | 20024ns | 18920ns | 20262ns | +1.72% | 0.051 |
| mw_chain_rev_h_mw3_skew | 19604ns | 17208ns | 20416ns | -0.41% | 0.052 |
| mw_jumptable_h_mw3_skew | 19839ns | 17630ns | 20354ns | +0.78% | 0.052 |

## Performance model

- Peak throughput: **0.061 Gops/s** (mw_bintree_h_mw3_skew; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw3_skew | 0.051 | 83.6% |
| mw_chain_h_mw3_skew | 0.051 | 83.5% |
| mw_chain_rev_h_mw3_skew | 0.051 | 83.5% |
| mw_jumptable_h_mw3_skew | 0.051 | 83.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw3_skew | 22233ns | 22233ns | base |
| mw_chain_h_mw3_skew | 22597ns | 22597ns | +1.64% |
| mw_chain_rev_h_mw3_skew | 22148ns | 22148ns | -0.38% |
| mw_jumptable_h_mw3_skew | 22395ns | 22395ns | +0.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 20225ns | base | --- | [18438, 20391] | --- | --- | --- | --- |
| mw_chain_h_mw3_skew | 20234ns | no significant difference | [-814, +1818]ns | [19575, 20262] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |
| mw_chain_rev_h_mw3_skew | 20240ns | no significant difference | [-446, +188]ns | [18155, 20416] | no | 1.0000 | 0.6875 | 0 |
| mw_jumptable_h_mw3_skew | 20232ns | no significant difference | [-112, +492]ns | [18929, 20354] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw3_skew | mw_chain_h_mw3_skew | mw_chain_rev_h_mw3_skew | mw_jumptable_h_mw3_skew |
|---|---|---|---|---|
| 1 | 16904ns | +20.0% | +1.8% | +4.3% |
| 2 | 20458ns | -1.1% | +0.4% | -1.1% |
| 3 | 20230ns | +0.0% | +0.1% | -0.0% |
| 4 | 20221ns | +0.1% | +0.1% | +0.3% |
| 5 | 20324ns | -6.9% | -0.1% | +0.5% |
| 6 | 19972ns | +1.3% | -4.4% | +1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw3_skew | -0.097 | ok |
| mw_chain_h_mw3_skew | -0.231 | moderate- |
| mw_chain_rev_h_mw3_skew | -0.138 | ok |
| mw_jumptable_h_mw3_skew | -0.010 | ok |

**Consistency summary:**

- **mw_chain_h_mw3_skew**: won 2/6, lost 3/6
- **mw_chain_rev_h_mw3_skew**: won 2/6, lost 2/6
- **mw_jumptable_h_mw3_skew**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 3.5ns | 19684.8ns | 0.0% |  |
| mw_chain_h_mw3_skew | 3.8ns | 20023.7ns | 0.0% |  |
| mw_chain_rev_h_mw3_skew | 3.0ns | 19603.6ns | 0.0% |  |
| mw_jumptable_h_mw3_skew | 3.7ns | 19838.6ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw3_skew (n=6, range 16904.2-20390.8 ns)
  16904.2 |#############
  17078.5 |
  17252.9 |
  17427.2 |
  17601.5 |
  17775.9 |
  17950.2 |
  18124.5 |
  18298.9 |
  18473.2 |
  18647.5 |
  18821.9 |
  18996.2 |
  19170.5 |
  19344.9 |
  19519.2 |
  19693.5 |
  19867.9 |#############
  20042.2 |
  20216.5 |########################################
  (0 below, 1 above range)

mw_chain_h_mw3_skew (n=6, range 18920.4-20262.1 ns)
  18920.4 |##########
  18987.5 |
  19054.6 |
  19121.7 |
  19188.7 |
  19255.8 |
  19322.9 |
  19390.0 |
  19457.1 |
  19524.2 |
  19591.2 |
  19658.3 |
  19725.4 |
  19792.5 |
  19859.6 |
  19926.7 |
  19993.8 |
  20060.8 |
  20127.9 |
  20195.0 |########################################
  (0 below, 1 above range)

mw_chain_rev_h_mw3_skew (n=6, range 17207.9-20415.8 ns)
  17207.9 |####################
  17368.3 |
  17528.7 |
  17689.1 |
  17849.5 |
  18009.9 |
  18170.3 |
  18330.7 |
  18491.1 |
  18651.5 |
  18811.9 |
  18972.3 |####################
  19132.7 |
  19293.1 |
  19453.5 |
  19613.9 |
  19774.3 |
  19934.7 |
  20095.1 |########################################
  20255.5 |####################
  (0 below, 1 above range)

mw_jumptable_h_mw3_skew (n=6, range 17630.0-20354.2 ns)
  17630.0 |##########
  17766.2 |
  17902.4 |
  18038.6 |
  18174.8 |
  18311.0 |
  18447.3 |
  18583.5 |
  18719.7 |
  18855.9 |
  18992.1 |
  19128.3 |
  19264.5 |
  19400.7 |
  19536.9 |
  19673.2 |
  19809.4 |
  19945.6 |
  20081.8 |
  20218.0 |########################################
  (0 below, 1 above range)

```
