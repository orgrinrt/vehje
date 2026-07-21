# Multiway branch strategies, heavy-arm, mw3_skew: 3-way, skewed to arm 0 (~80%)

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw3_skew**

## Key findings

- **Baseline (mw_bintree_h_mw3_skew) is the fastest** at 5057.5 ns median
- Spread: 1.04x (fastest 5057.5 ns, slowest 5262.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 7659ns | 7651ns | 6775ns | 7487ns | 8360ns | base |
| mw_chain_h_mw3_skew | 7779ns | 7867ns | 6665ns | 7797ns | 8309ns | +1.57% |
| mw_chain_rev_h_mw3_skew | 7334ns | 7649ns | 6288ns | 7244ns | 7990ns | -4.25% |
| mw_jumptable_h_mw3_skew | 7583ns | 7655ns | 6428ns | 7654ns | 8054ns | -1.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 5053ns | 4494ns | 5490ns | base | 0.051 |
| mw_chain_h_mw3_skew | 5164ns | 4408ns | 5494ns | +2.20% | 0.050 |
| mw_chain_rev_h_mw3_skew | 4852ns | 4172ns | 5281ns | -3.99% | 0.053 |
| mw_jumptable_h_mw3_skew | 5011ns | 4257ns | 5316ns | -0.84% | 0.051 |

## Performance model

- Peak throughput: **0.061 Gops/s** (mw_chain_rev_h_mw3_skew; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw3_skew | 0.051 | 82.5% |
| mw_chain_h_mw3_skew | 0.049 | 79.3% |
| mw_chain_rev_h_mw3_skew | 0.051 | 82.4% |
| mw_jumptable_h_mw3_skew | 0.051 | 82.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw3_skew | 7659ns | 7659ns | base |
| mw_chain_h_mw3_skew | 7779ns | 7779ns | +1.57% |
| mw_chain_rev_h_mw3_skew | 7334ns | 7334ns | -4.25% |
| mw_jumptable_h_mw3_skew | 7583ns | 7583ns | -1.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 5058ns | base | --- | [4612, 5490] | --- | --- | --- | --- |
| mw_chain_h_mw3_skew | 5262ns | no significant difference | [-43, +368]ns | [4737, 5494] | no | 1.0000 | 0.3750 | **1** (17%, HIGH) |
| mw_chain_rev_h_mw3_skew | 5060ns | no significant difference | [-494, +7]ns | [4214, 5281] | no | 1.0000 | 1.0000 | 0 |
| mw_jumptable_h_mw3_skew | 5060ns | no significant difference | [-336, +202]ns | [4656, 5316] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw3_skew | mw_chain_h_mw3_skew | mw_chain_rev_h_mw3_skew | mw_jumptable_h_mw3_skew |
|---|---|---|---|---|
| 1 | 4730ns | +7.8% | -11.8% | +7.0% |
| 2 | 5057ns | +0.2% | +0.0% | +0.2% |
| 3 | 5058ns | +7.3% | +0.1% | +0.0% |
| 4 | 5490ns | +0.0% | -7.8% | -7.9% |
| 5 | 5490ns | +0.1% | +0.2% | +1.3% |
| 6 | 4494ns | -1.9% | -5.3% | -5.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw3_skew | -0.066 | ok |
| mw_chain_h_mw3_skew | -0.088 | ok |
| mw_chain_rev_h_mw3_skew | -0.222 | moderate- |
| mw_jumptable_h_mw3_skew | -0.434 | moderate- |

**Consistency summary:**

- **mw_chain_h_mw3_skew**: won 1/6, lost 4/6
- **mw_chain_rev_h_mw3_skew**: won 3/6, lost 1/6
- **mw_jumptable_h_mw3_skew**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 3.6ns | 5053.1ns | 0.1% |  |
| mw_chain_h_mw3_skew | 3.9ns | 5164.3ns | 0.1% |  |
| mw_chain_rev_h_mw3_skew | 5.5ns | 4851.5ns | 0.1% |  |
| mw_jumptable_h_mw3_skew | 5.4ns | 5010.8ns | 0.1% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw3_skew (n=6, range 4493.7-5490.0 ns)
   4493.7 |####################
   4543.5 |
   4593.3 |
   4643.1 |
   4693.0 |####################
   4742.8 |
   4792.6 |
   4842.4 |
   4892.2 |
   4942.0 |
   4991.9 |
   5041.7 |########################################
   5091.5 |
   5141.3 |
   5191.1 |
   5240.9 |
   5290.7 |
   5340.6 |
   5390.4 |
   5440.2 |
  (0 below, 2 above range)

mw_chain_h_mw3_skew (n=6, range 4407.5-5493.6 ns)
   4407.5 |####################
   4461.8 |
   4516.1 |
   4570.4 |
   4624.7 |
   4679.0 |
   4733.3 |
   4787.6 |
   4841.9 |
   4896.2 |
   4950.5 |
   5004.8 |
   5059.1 |########################################
   5113.4 |
   5167.7 |
   5222.0 |
   5276.3 |
   5330.6 |
   5384.9 |####################
   5439.2 |####################
  (0 below, 1 above range)

mw_chain_rev_h_mw3_skew (n=6, range 4171.7-5280.8 ns)
   4171.7 |####################
   4227.2 |####################
   4282.6 |
   4338.1 |
   4393.5 |
   4449.0 |
   4504.4 |
   4559.9 |
   4615.3 |
   4670.8 |
   4726.2 |
   4781.7 |
   4837.2 |
   4892.6 |
   4948.1 |
   5003.5 |####################
   5059.0 |########################################
   5114.4 |
   5169.9 |
   5225.3 |
  (0 below, 1 above range)

mw_jumptable_h_mw3_skew (n=6, range 4256.7-5315.9 ns)
   4256.7 |##########
   4309.7 |
   4362.6 |
   4415.6 |
   4468.5 |
   4521.5 |
   4574.4 |
   4627.4 |
   4680.4 |
   4733.3 |
   4786.3 |
   4839.2 |
   4892.2 |
   4945.1 |
   4998.1 |
   5051.1 |########################################
   5104.0 |
   5157.0 |
   5209.9 |
   5262.9 |
  (0 below, 1 above range)

```
