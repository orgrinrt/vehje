# Multiway branch strategies, heavy-arm, mw8_skew: 8-way, skewed to arm 0 (~70%)

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw8_skew**

## Key findings

- **Fastest: mw_chain_h_mw8_skew** at 19414.4 ns median (-11.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.13x (fastest 19414.4 ns, slowest 21929.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 24146ns | 24519ns | 23020ns | 24023ns | 24894ns | base |
| mw_chain_h_mw8_skew | 21874ns | 22015ns | 21510ns | 21880ns | 22046ns | -9.41% |
| mw_chain_rev_h_mw8_skew | 23363ns | 24015ns | 19962ns | 23996ns | 24113ns | -3.24% |
| mw_jumptable_h_mw8_skew | 23344ns | 23556ns | 22034ns | 23553ns | 23685ns | -3.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 21565ns | 20587ns | 22175ns | base | 0.047 |
| mw_chain_h_mw8_skew | 19287ns | 18945ns | 19448ns | -10.56% | 0.053 |
| mw_chain_rev_h_mw8_skew | 20817ns | 17776ns | 21467ns | -3.47% | 0.049 |
| mw_jumptable_h_mw8_skew | 20770ns | 19596ns | 21072ns | -3.69% | 0.049 |

## Performance model

- Peak throughput: **0.058 Gops/s** (mw_chain_rev_h_mw8_skew; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw8_skew | 0.047 | 81.1% |
| mw_chain_h_mw8_skew | 0.053 | 91.6% |
| mw_chain_rev_h_mw8_skew | 0.048 | 83.0% |
| mw_jumptable_h_mw8_skew | 0.049 | 84.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw8_skew | 24146ns | 24146ns | base |
| mw_chain_h_mw8_skew | 21874ns | 21874ns | -9.41% |
| mw_chain_rev_h_mw8_skew | 23363ns | 23363ns | -3.24% |
| mw_jumptable_h_mw8_skew | 23344ns | 23344ns | -3.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 21929ns | base | --- | [20590, 22175] | --- | --- | --- | --- |
| mw_chain_h_mw8_skew | 19414ns | -2514.8ns (-11.5%) | [-2981, -1338]ns | [18999, 19448] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| mw_chain_rev_h_mw8_skew | 21409ns | no significant difference | [-1814, +236]ns | [19575, 21467] | no | 0.3281 | 0.2188 | 0 |
| mw_jumptable_h_mw8_skew | 20961ns | no significant difference | [-1898, +482]ns | [20278, 21072] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw8_skew | mw_chain_h_mw8_skew | mw_chain_rev_h_mw8_skew | mw_jumptable_h_mw8_skew |
|---|---|---|---|---|
| 1 | 20593ns | -7.5% | -13.7% | +1.8% |
| 2 | 22228ns | -14.8% | -3.7% | -11.8% |
| 3 | 21869ns | -11.2% | -1.6% | -4.2% |
| 4 | 22122ns | -12.1% | -3.2% | -5.3% |
| 5 | 20587ns | -5.5% | +4.0% | +2.9% |
| 6 | 21989ns | -11.8% | -2.8% | -4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw8_skew | -0.422 | moderate- |
| mw_chain_h_mw8_skew | 0.390 | moderate+ |
| mw_chain_rev_h_mw8_skew | -0.028 | ok |
| mw_jumptable_h_mw8_skew | -0.154 | ok |

**Consistency summary:**

- **mw_chain_h_mw8_skew**: won 6/6, lost 0/6
- **mw_chain_rev_h_mw8_skew**: won 5/6, lost 1/6
- **mw_jumptable_h_mw8_skew**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 4.0ns | 21564.7ns | 0.0% |  |
| mw_chain_h_mw8_skew | 4.0ns | 19286.9ns | 0.0% |  |
| mw_chain_rev_h_mw8_skew | 3.8ns | 20817.0ns | 0.0% |  |
| mw_jumptable_h_mw8_skew | 3.2ns | 20770.0ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw8_skew (n=6, range 20587.1-22175.0 ns)
  20587.1 |########################################
  20666.5 |
  20745.9 |
  20825.3 |
  20904.7 |
  20984.1 |
  21063.5 |
  21142.9 |
  21222.3 |
  21301.7 |
  21381.0 |
  21460.4 |
  21539.8 |
  21619.2 |
  21698.6 |
  21778.0 |
  21857.4 |####################
  21936.8 |####################
  22016.2 |
  22095.6 |####################
  (0 below, 1 above range)

mw_chain_h_mw8_skew (n=6, range 18945.4-19447.7 ns)
  18945.4 |####################
  18970.5 |
  18995.6 |
  19020.7 |
  19045.9 |####################
  19071.0 |
  19096.1 |
  19121.2 |
  19146.3 |
  19171.4 |
  19196.6 |
  19221.7 |
  19246.8 |
  19271.9 |
  19297.0 |
  19322.1 |
  19347.2 |
  19372.4 |
  19397.5 |####################
  19422.6 |########################################
  (0 below, 1 above range)

mw_chain_rev_h_mw8_skew (n=6, range 17776.2-21466.7 ns)
  17776.2 |##########
  17960.7 |
  18145.2 |
  18329.8 |
  18514.3 |
  18698.8 |
  18883.3 |
  19067.9 |
  19252.4 |
  19436.9 |
  19621.4 |
  19805.9 |
  19990.5 |
  20175.0 |
  20359.5 |
  20544.0 |
  20728.6 |
  20913.1 |
  21097.6 |
  21282.1 |########################################
  (0 below, 1 above range)

mw_jumptable_h_mw8_skew (n=6, range 19595.8-21071.7 ns)
  19595.8 |##########
  19669.6 |
  19743.4 |
  19817.2 |
  19891.0 |
  19964.8 |
  20038.6 |
  20112.3 |
  20186.1 |
  20259.9 |
  20333.7 |
  20407.5 |
  20481.3 |
  20555.1 |
  20628.9 |
  20702.7 |
  20776.5 |
  20850.3 |
  20924.1 |########################################
  20997.9 |
  (0 below, 1 above range)

```
