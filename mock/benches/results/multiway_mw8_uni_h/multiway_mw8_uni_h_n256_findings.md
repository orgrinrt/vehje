# Multiway branch strategies, heavy-arm, mw8_uni: 8-way, uniform key

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw8_uni**

## Key findings

- **Fastest: mw_chain_rev_h_mw8_uni** at 4755.4 ns median (-17.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.21x (fastest 4755.4 ns, slowest 5751.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 8046ns | 8559ns | 6622ns | 8168ns | 8576ns | base |
| mw_chain_h_mw8_uni | 7544ns | 7797ns | 6329ns | 7555ns | 8134ns | -6.24% |
| mw_chain_rev_h_mw8_uni | 7256ns | 7298ns | 6265ns | 6968ns | 8184ns | -9.82% |
| mw_jumptable_h_mw8_uni | 7736ns | 8096ns | 6273ns | 7927ns | 8182ns | -3.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 5403ns | 4452ns | 5754ns | base | 0.047 |
| mw_chain_h_mw8_uni | 4926ns | 4139ns | 5316ns | -8.84% | 0.052 |
| mw_chain_rev_h_mw8_uni | 4715ns | 4090ns | 5288ns | -12.73% | 0.054 |
| mw_jumptable_h_mw8_uni | 5045ns | 4092ns | 5365ns | -6.63% | 0.051 |

## Performance model

- Peak throughput: **0.063 Gops/s** (mw_chain_rev_h_mw8_uni; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw8_uni | 0.045 | 71.1% |
| mw_chain_h_mw8_uni | 0.050 | 80.5% |
| mw_chain_rev_h_mw8_uni | 0.054 | 86.0% |
| mw_jumptable_h_mw8_uni | 0.048 | 77.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw8_uni | 8046ns | 8046ns | base |
| mw_chain_h_mw8_uni | 7544ns | 7544ns | -6.24% |
| mw_chain_rev_h_mw8_uni | 7256ns | 7256ns | -9.82% |
| mw_jumptable_h_mw8_uni | 7736ns | 7736ns | -3.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 5752ns | base | --- | [4705, 5754] | --- | --- | --- | --- |
| mw_chain_h_mw8_uni | 5082ns | -436.0ns (-7.6%) | [-798, -199]ns | [4380, 5316] | YES | 0.0313 | 0.0313 | 0 |
| mw_chain_rev_h_mw8_uni | 4755ns | -472.6ns (-8.2%) | [-1180, -412]ns | [4102, 5288] | YES | 0.0313 | 0.0313 | 0 |
| mw_jumptable_h_mw8_uni | 5288ns | -392.7ns (-6.8%) | [-466, -216]ns | [4483, 5365] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw8_uni | mw_chain_h_mw8_uni | mw_chain_rev_h_mw8_uni | mw_jumptable_h_mw8_uni |
|---|---|---|---|---|
| 1 | 4957ns | -1.7% | -17.0% | -1.7% |
| 2 | 5750ns | -7.3% | -8.3% | -6.1% |
| 3 | 5753ns | -7.9% | -8.2% | -7.4% |
| 4 | 5755ns | -8.1% | -8.0% | -8.1% |
| 5 | 5754ns | -19.7% | -26.4% | -8.1% |
| 6 | 4452ns | -7.0% | -8.1% | -8.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw8_uni | -0.076 | ok |
| mw_chain_h_mw8_uni | 0.343 | moderate+ |
| mw_chain_rev_h_mw8_uni | 0.171 | ok |
| mw_jumptable_h_mw8_uni | -0.050 | ok |

**Consistency summary:**

- **mw_chain_h_mw8_uni**: won 6/6, lost 0/6
- **mw_chain_rev_h_mw8_uni**: won 6/6, lost 0/6
- **mw_jumptable_h_mw8_uni**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 4.6ns | 5403.5ns | 0.1% |  |
| mw_chain_h_mw8_uni | 5.3ns | 4926.0ns | 0.1% |  |
| mw_chain_rev_h_mw8_uni | 5.1ns | 4715.4ns | 0.1% |  |
| mw_jumptable_h_mw8_uni | 5.9ns | 5045.3ns | 0.1% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw8_uni (n=6, range 4452.5-5754.2 ns)
   4452.5 |#############
   4517.6 |
   4582.7 |
   4647.8 |
   4712.8 |
   4777.9 |
   4843.0 |
   4908.1 |#############
   4973.2 |
   5038.3 |
   5103.4 |
   5168.4 |
   5233.5 |
   5298.6 |
   5363.7 |
   5428.8 |
   5493.9 |
   5558.9 |
   5624.0 |
   5689.1 |########################################
  (0 below, 1 above range)

mw_chain_h_mw8_uni (n=6, range 4138.8-5315.6 ns)
   4138.8 |####################
   4197.6 |
   4256.5 |
   4315.3 |
   4374.2 |
   4433.0 |
   4491.8 |
   4550.7 |
   4609.5 |####################
   4668.4 |
   4727.2 |
   4786.0 |
   4844.9 |####################
   4903.7 |
   4962.6 |
   5021.4 |
   5080.2 |
   5139.1 |
   5197.9 |
   5256.8 |########################################
  (0 below, 1 above range)

mw_chain_rev_h_mw8_uni (n=6, range 4090.4-5288.4 ns)
   4090.4 |########################################
   4150.3 |
   4210.2 |####################
   4270.1 |
   4330.0 |
   4389.9 |
   4449.8 |
   4509.7 |
   4569.6 |
   4629.5 |
   4689.4 |
   4749.3 |
   4809.2 |
   4869.1 |
   4929.0 |
   4988.9 |
   5048.8 |
   5108.7 |
   5168.6 |
   5228.5 |########################################
  (0 below, 1 above range)

mw_jumptable_h_mw8_uni (n=6, range 4091.7-5364.8 ns)
   4091.7 |####################
   4155.4 |
   4219.0 |
   4282.7 |
   4346.3 |
   4410.0 |
   4473.6 |
   4537.3 |
   4600.9 |
   4664.6 |
   4728.2 |
   4791.9 |
   4855.5 |####################
   4919.2 |
   4982.8 |
   5046.5 |
   5110.1 |
   5173.8 |
   5237.4 |########################################
   5301.1 |####################
  (0 below, 1 above range)

```
