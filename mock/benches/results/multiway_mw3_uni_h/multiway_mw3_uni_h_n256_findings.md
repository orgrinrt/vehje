# Multiway branch strategies, heavy-arm, mw3_uni: 3-way, uniform key

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw3_uni**

## Key findings

- **Fastest: mw_chain_rev_h_mw3_uni** at 5015.4 ns median (-0.4% vs baseline)
- Spread: 1.06x (fastest 5015.4 ns, slowest 5308.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 7653ns | 7554ns | 6693ns | 7344ns | 8596ns | base |
| mw_chain_h_mw3_uni | 7539ns | 7897ns | 6678ns | 7536ns | 7974ns | -1.49% |
| mw_chain_rev_h_mw3_uni | 7398ns | 7432ns | 6696ns | 7201ns | 8044ns | -3.33% |
| mw_jumptable_h_mw3_uni | 7498ns | 7693ns | 6672ns | 7434ns | 8007ns | -2.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 5205ns | 4513ns | 5993ns | base | 0.049 |
| mw_chain_h_mw3_uni | 5080ns | 4498ns | 5366ns | -2.40% | 0.050 |
| mw_chain_rev_h_mw3_uni | 4992ns | 4517ns | 5444ns | -4.08% | 0.051 |
| mw_jumptable_h_mw3_uni | 5058ns | 4499ns | 5410ns | -2.81% | 0.051 |

## Performance model

- Peak throughput: **0.057 Gops/s** (mw_chain_h_mw3_uni; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw3_uni | 0.051 | 89.4% |
| mw_chain_h_mw3_uni | 0.048 | 84.7% |
| mw_chain_rev_h_mw3_uni | 0.051 | 89.7% |
| mw_jumptable_h_mw3_uni | 0.049 | 86.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw3_uni | 7653ns | 7653ns | base |
| mw_chain_h_mw3_uni | 7539ns | 7539ns | -1.49% |
| mw_chain_rev_h_mw3_uni | 7398ns | 7398ns | -3.33% |
| mw_jumptable_h_mw3_uni | 7498ns | 7498ns | -2.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 5034ns | base | --- | [4588, 5993] | --- | --- | --- | --- |
| mw_chain_h_mw3_uni | 5308ns | no significant difference | [-714, +282]ns | [4566, 5366] | no | 1.0000 | 1.0000 | 0 |
| mw_chain_rev_h_mw3_uni | 5015ns | no significant difference | [-833, +259]ns | [4518, 5444] | no | 1.0000 | 1.0000 | 0 |
| mw_jumptable_h_mw3_uni | 5184ns | no significant difference | [-714, +226]ns | [4582, 5410] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw3_uni | mw_chain_h_mw3_uni | mw_chain_rev_h_mw3_uni | mw_jumptable_h_mw3_uni |
|---|---|---|---|---|
| 1 | 5060ns | +6.2% | -10.7% | +5.9% |
| 2 | 5362ns | -0.1% | +0.4% | +1.8% |
| 3 | 4662ns | -3.5% | -3.1% | -3.5% |
| 4 | 4513ns | +2.7% | +3.0% | +3.4% |
| 5 | 5008ns | +5.0% | +7.6% | +0.0% |
| 6 | 6624ns | -19.1% | -17.0% | -19.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw3_uni | 0.043 | ok |
| mw_chain_h_mw3_uni | 0.185 | ok |
| mw_chain_rev_h_mw3_uni | -0.125 | ok |
| mw_jumptable_h_mw3_uni | 0.150 | ok |

**Consistency summary:**

- **mw_chain_h_mw3_uni**: won 2/6, lost 3/6
- **mw_chain_rev_h_mw3_uni**: won 3/6, lost 3/6
- **mw_jumptable_h_mw3_uni**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 5.1ns | 5204.7ns | 0.1% |  |
| mw_chain_h_mw3_uni | 3.9ns | 5079.9ns | 0.1% |  |
| mw_chain_rev_h_mw3_uni | 4.7ns | 4992.5ns | 0.1% |  |
| mw_jumptable_h_mw3_uni | 4.8ns | 5058.4ns | 0.1% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw3_uni (n=6, range 4512.9-5992.8 ns)
   4512.9 |########################################
   4586.9 |
   4660.9 |########################################
   4734.9 |
   4808.9 |
   4882.9 |
   4956.9 |########################################
   5030.8 |########################################
   5104.8 |
   5178.8 |
   5252.8 |
   5326.8 |########################################
   5400.8 |
   5474.8 |
   5548.8 |
   5622.8 |
   5696.8 |
   5770.8 |
   5844.8 |
   5918.8 |
  (0 below, 1 above range)

mw_chain_h_mw3_uni (n=6, range 4498.3-5366.1 ns)
   4498.3 |####################
   4541.7 |
   4585.1 |
   4628.5 |####################
   4671.9 |
   4715.2 |
   4758.6 |
   4802.0 |
   4845.4 |
   4888.8 |
   4932.2 |
   4975.6 |
   5019.0 |
   5062.3 |
   5105.7 |
   5149.1 |
   5192.5 |
   5235.9 |####################
   5279.3 |
   5322.7 |########################################
  (0 below, 1 above range)

mw_chain_rev_h_mw3_uni (n=6, range 4517.1-5444.0 ns)
   4517.1 |########################################
   4563.4 |
   4609.8 |####################
   4656.1 |
   4702.5 |
   4748.8 |
   4795.2 |
   4841.5 |
   4887.9 |
   4934.2 |
   4980.6 |
   5026.9 |
   5073.2 |
   5119.6 |
   5165.9 |
   5212.3 |
   5258.6 |
   5305.0 |
   5351.3 |########################################
   5397.7 |
  (0 below, 1 above range)

mw_jumptable_h_mw3_uni (n=6, range 4498.8-5409.6 ns)
   4498.8 |####################
   4544.3 |
   4589.9 |
   4635.4 |####################
   4681.0 |
   4726.5 |
   4772.0 |
   4817.6 |
   4863.1 |
   4908.7 |
   4954.2 |
   4999.7 |####################
   5045.3 |
   5090.8 |
   5136.4 |
   5181.9 |
   5227.4 |
   5273.0 |
   5318.5 |########################################
   5364.1 |
  (0 below, 1 above range)

```
