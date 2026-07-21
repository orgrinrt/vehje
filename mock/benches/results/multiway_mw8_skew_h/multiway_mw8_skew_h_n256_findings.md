# Multiway branch strategies, heavy-arm, mw8_skew: 8-way, skewed to arm 0 (~70%)

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw8_skew**

## Key findings

- **Fastest: mw_chain_h_mw8_skew** at 4858.9 ns median (-11.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.13x (fastest 4858.9 ns, slowest 5507.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 7961ns | 8100ns | 7046ns | 7941ns | 8449ns | base |
| mw_chain_h_mw8_skew | 7399ns | 7462ns | 6255ns | 7459ns | 7882ns | -7.05% |
| mw_chain_rev_h_mw8_skew | 7967ns | 8043ns | 6975ns | 7856ns | 8629ns | +0.07% |
| mw_jumptable_h_mw8_skew | 7719ns | 7838ns | 6823ns | 7713ns | 8175ns | -3.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 5411ns | 4791ns | 5742ns | base | 0.047 |
| mw_chain_h_mw8_skew | 4804ns | 4075ns | 5087ns | -11.22% | 0.053 |
| mw_chain_rev_h_mw8_skew | 5388ns | 4796ns | 5818ns | -0.41% | 0.048 |
| mw_jumptable_h_mw8_skew | 5150ns | 4565ns | 5470ns | -4.81% | 0.050 |

## Performance model

- Peak throughput: **0.063 Gops/s** (mw_chain_h_mw8_skew; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw8_skew | 0.046 | 74.0% |
| mw_chain_h_mw8_skew | 0.053 | 83.9% |
| mw_chain_rev_h_mw8_skew | 0.047 | 75.1% |
| mw_jumptable_h_mw8_skew | 0.049 | 77.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw8_skew | 7961ns | 7961ns | base |
| mw_chain_h_mw8_skew | 7399ns | 7399ns | -7.05% |
| mw_chain_rev_h_mw8_skew | 7967ns | 7967ns | +0.07% |
| mw_jumptable_h_mw8_skew | 7719ns | 7719ns | -3.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 5508ns | base | --- | [4983, 5742] | --- | --- | --- | --- |
| mw_chain_h_mw8_skew | 4859ns | -556.9ns (-10.1%) | [-914, -350]ns | [4465, 5087] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| mw_chain_rev_h_mw8_skew | 5429ns | no significant difference | [-460, +308]ns | [4918, 5818] | no | 1.0000 | 1.0000 | 0 |
| mw_jumptable_h_mw8_skew | 5246ns | no significant difference | [-665, +126]ns | [4736, 5470] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw8_skew | mw_chain_h_mw8_skew | mw_chain_rev_h_mw8_skew | mw_jumptable_h_mw8_skew |
|---|---|---|---|---|
| 1 | 5175ns | -6.2% | -7.3% | +1.4% |
| 2 | 5506ns | -11.8% | -1.5% | -4.7% |
| 3 | 5509ns | -8.4% | +5.6% | -4.8% |
| 4 | 5974ns | -18.6% | -9.1% | -17.9% |
| 5 | 5511ns | -6.9% | +5.6% | +3.3% |
| 6 | 4791ns | -14.9% | +5.2% | -4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw8_skew | 0.047 | ok |
| mw_chain_h_mw8_skew | -0.269 | moderate- |
| mw_chain_rev_h_mw8_skew | -0.139 | ok |
| mw_jumptable_h_mw8_skew | -0.629 | HIGH- (thermal bounce) |

**Consistency summary:**

- **mw_chain_h_mw8_skew**: won 6/6, lost 0/6
- **mw_chain_rev_h_mw8_skew**: won 3/6, lost 3/6
- **mw_jumptable_h_mw8_skew**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 4.7ns | 5410.8ns | 0.1% |  |
| mw_chain_h_mw8_skew | 5.2ns | 4803.8ns | 0.1% |  |
| mw_chain_rev_h_mw8_skew | 5.2ns | 5388.4ns | 0.1% |  |
| mw_jumptable_h_mw8_skew | 5.3ns | 5150.5ns | 0.1% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw8_skew (n=6, range 4790.8-5742.3 ns)
   4790.8 |#############
   4838.4 |
   4885.9 |
   4933.5 |
   4981.1 |
   5028.7 |
   5076.2 |
   5123.8 |
   5171.4 |#############
   5219.0 |
   5266.6 |
   5314.1 |
   5361.7 |
   5409.3 |
   5456.9 |
   5504.4 |########################################
   5552.0 |
   5599.6 |
   5647.2 |
   5694.7 |
  (0 below, 1 above range)

mw_chain_h_mw8_skew (n=6, range 4075.4-5087.3 ns)
   4075.4 |#############
   4126.0 |
   4176.6 |
   4227.2 |
   4277.8 |
   4328.4 |
   4379.0 |
   4429.6 |
   4480.2 |
   4530.8 |
   4581.4 |
   4631.9 |
   4682.5 |
   4733.1 |
   4783.7 |
   4834.3 |########################################
   4884.9 |
   4935.5 |
   4986.1 |
   5036.7 |#############
  (0 below, 1 above range)

mw_chain_rev_h_mw8_skew (n=6, range 4795.8-5817.7 ns)
   4795.8 |####################
   4846.9 |
   4898.0 |
   4949.1 |
   5000.2 |####################
   5051.3 |
   5102.4 |
   5153.5 |
   5204.6 |
   5255.7 |
   5306.8 |
   5357.8 |
   5408.9 |########################################
   5460.0 |
   5511.1 |
   5562.2 |
   5613.3 |
   5664.4 |
   5715.5 |
   5766.6 |####################
  (0 below, 1 above range)

mw_jumptable_h_mw8_skew (n=6, range 4565.0-5470.2 ns)
   4565.0 |#############
   4610.3 |
   4655.5 |
   4700.8 |
   4746.0 |
   4791.3 |
   4836.6 |
   4881.8 |#############
   4927.1 |
   4972.3 |
   5017.6 |
   5062.9 |
   5108.1 |
   5153.4 |
   5198.6 |
   5243.9 |########################################
   5289.2 |
   5334.4 |
   5379.7 |
   5424.9 |
  (0 below, 1 above range)

```
