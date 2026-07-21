# Multiway branch strategies, heavy-arm, mw3_skew: 3-way, skewed to arm 0 (~80%)

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw3_skew**

## Key findings

- **Baseline (mw_bintree_h_mw3_skew) is the fastest** at 269725.7 ns median
- Spread: 1.01x (fastest 269725.7 ns, slowest 273686.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 272981ns | 272153ns | 269428ns | 271374ns | 277168ns | base |
| mw_chain_h_mw3_skew | 273938ns | 272632ns | 270084ns | 271914ns | 278899ns | +0.35% |
| mw_chain_rev_h_mw3_skew | 276725ns | 276022ns | 271680ns | 274925ns | 281948ns | +1.37% |
| mw_jumptable_h_mw3_skew | 275605ns | 273890ns | 269965ns | 272765ns | 282684ns | +0.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 270490ns | 266968ns | 274683ns | base | 0.061 |
| mw_chain_h_mw3_skew | 271334ns | 267645ns | 276126ns | +0.31% | 0.060 |
| mw_chain_rev_h_mw3_skew | 274243ns | 269198ns | 279215ns | +1.39% | 0.060 |
| mw_jumptable_h_mw3_skew | 272982ns | 267341ns | 279903ns | +0.92% | 0.060 |

## Performance model

- Peak throughput: **0.061 Gops/s** (mw_bintree_h_mw3_skew; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw3_skew | 0.061 | 99.0% |
| mw_chain_h_mw3_skew | 0.061 | 98.8% |
| mw_chain_rev_h_mw3_skew | 0.060 | 97.5% |
| mw_jumptable_h_mw3_skew | 0.060 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw3_skew | 272981ns | 272981ns | base |
| mw_chain_h_mw3_skew | 273938ns | 273938ns | +0.35% |
| mw_chain_rev_h_mw3_skew | 276725ns | 276725ns | +1.37% |
| mw_jumptable_h_mw3_skew | 275605ns | 275605ns | +0.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 269726ns | base | --- | [267061, 274683] | --- | --- | --- | --- |
| mw_chain_h_mw3_skew | 270079ns | no significant difference | [-4419, +7480]ns | [267796, 276126] | no | 1.0000 | 1.0000 | 0 |
| mw_chain_rev_h_mw3_skew | 273687ns | no significant difference | [-2660, +9489]ns | [269829, 279215] | no | 1.0000 | 0.6875 | 0 |
| mw_jumptable_h_mw3_skew | 271425ns | no significant difference | [-1213, +7710]ns | [267619, 279903] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw3_skew | mw_chain_h_mw3_skew | mw_chain_rev_h_mw3_skew | mw_jumptable_h_mw3_skew |
|---|---|---|---|---|
| 1 | 267155ns | +4.7% | +0.8% | +0.6% |
| 2 | 270628ns | +0.0% | +4.0% | +2.8% |
| 3 | 275608ns | -2.8% | -0.7% | -0.5% |
| 4 | 268824ns | -0.4% | +3.1% | -0.3% |
| 5 | 273759ns | -0.4% | -1.2% | +2.8% |
| 6 | 266968ns | +0.9% | +2.6% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw3_skew | -0.399 | moderate- |
| mw_chain_h_mw3_skew | 0.020 | ok |
| mw_chain_rev_h_mw3_skew | -0.518 | HIGH- (thermal bounce) |
| mw_jumptable_h_mw3_skew | -0.640 | HIGH- (thermal bounce) |

**Consistency summary:**

- **mw_chain_h_mw3_skew**: won 3/6, lost 2/6
- **mw_chain_rev_h_mw3_skew**: won 2/6, lost 4/6
- **mw_jumptable_h_mw3_skew**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 5.1ns | 270490.1ns | 0.0% |  |
| mw_chain_h_mw3_skew | 4.1ns | 271333.6ns | 0.0% |  |
| mw_chain_rev_h_mw3_skew | 4.2ns | 274243.4ns | 0.0% |  |
| mw_jumptable_h_mw3_skew | 4.4ns | 272982.5ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw3_skew (n=6, range 266967.5-274683.3 ns)
  266967.5 |########################################
  267353.3 |
  267739.1 |
  268124.9 |
  268510.7 |####################
  268896.5 |
  269282.3 |
  269668.0 |
  270053.8 |
  270439.6 |####################
  270825.4 |
  271211.2 |
  271597.0 |
  271982.8 |
  272368.6 |
  272754.4 |
  273140.2 |
  273526.0 |####################
  273911.8 |
  274297.6 |
  (0 below, 1 above range)

mw_chain_h_mw3_skew (n=6, range 267645.4-276125.8 ns)
  267645.4 |########################################
  268069.4 |
  268493.4 |
  268917.5 |
  269341.5 |####################
  269765.5 |
  270189.5 |
  270613.5 |####################
  271037.6 |
  271461.6 |
  271885.6 |
  272309.6 |####################
  272733.6 |
  273157.7 |
  273581.7 |
  274005.7 |
  274429.7 |
  274853.7 |
  275277.8 |
  275701.8 |
  (0 below, 1 above range)

mw_chain_rev_h_mw3_skew (n=6, range 269197.9-279215.0 ns)
  269197.9 |########################################
  269698.8 |
  270199.6 |########################################
  270700.5 |
  271201.3 |
  271702.2 |
  272203.0 |
  272703.9 |
  273204.7 |########################################
  273705.6 |########################################
  274206.5 |
  274707.3 |
  275208.2 |
  275709.0 |
  276209.9 |
  276710.7 |########################################
  277211.6 |
  277712.4 |
  278213.3 |
  278714.1 |
  (0 below, 1 above range)

mw_jumptable_h_mw3_skew (n=6, range 267341.2-279903.3 ns)
  267341.2 |########################################
  267969.3 |
  268597.4 |####################
  269225.5 |
  269853.6 |
  270481.7 |
  271109.8 |
  271737.9 |
  272366.0 |
  272994.1 |
  273622.2 |####################
  274250.4 |
  274878.5 |
  275506.6 |
  276134.7 |
  276762.8 |
  277390.9 |
  278019.0 |####################
  278647.1 |
  279275.2 |
  (0 below, 1 above range)

```
