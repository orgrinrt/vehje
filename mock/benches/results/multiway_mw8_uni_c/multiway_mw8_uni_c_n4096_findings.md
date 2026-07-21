# Multiway branch strategies, cheap-arm, mw8_uni: 8-way, uniform key

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw8_uni**

## Key findings

- **Fastest: mw_chain_c_mw8_uni** at 5331.6 ns median (-37.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.13x (fastest 5331.6 ns, slowest 32704.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 11572ns | 11092ns | 10525ns | 10942ns | 13039ns | base |
| mw_chain_c_mw8_uni | 7852ns | 7858ns | 7609ns | 7780ns | 8081ns | -32.15% |
| mw_chain_rev_c_mw8_uni | 7937ns | 8089ns | 7538ns | 7920ns | 8162ns | -31.41% |
| mw_jumptable_c_mw8_uni | 7971ns | 8072ns | 7030ns | 8071ns | 8291ns | -31.12% |
| mw_predicate_all_c_mw8_uni | 34845ns | 35203ns | 30630ns | 35016ns | 36695ns | +201.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 8944ns | 8106ns | 10120ns | base | 0.458 |
| mw_chain_c_mw8_uni | 5328ns | 5166ns | 5481ns | -40.43% | 0.769 |
| mw_chain_rev_c_mw8_uni | 5369ns | 5118ns | 5499ns | -39.97% | 0.763 |
| mw_jumptable_c_mw8_uni | 5407ns | 4767ns | 5626ns | -39.54% | 0.758 |
| mw_predicate_all_c_mw8_uni | 32360ns | 28405ns | 34081ns | +261.81% | 0.127 |

## Performance model

- Peak throughput: **0.859 Gops/s** (mw_jumptable_c_mw8_uni; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw8_uni | 0.479 | 55.7% |
| mw_chain_c_mw8_uni | 0.768 | 89.4% |
| mw_chain_rev_c_mw8_uni | 0.748 | 87.1% |
| mw_jumptable_c_mw8_uni | 0.748 | 87.1% |
| mw_predicate_all_c_mw8_uni | 0.125 | 14.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw8_uni | 11572ns | 11572ns | base |
| mw_chain_c_mw8_uni | 7852ns | 7852ns | -32.15% |
| mw_chain_rev_c_mw8_uni | 7937ns | 7937ns | -31.41% |
| mw_jumptable_c_mw8_uni | 7971ns | 7971ns | -31.12% |
| mw_predicate_all_c_mw8_uni | 34845ns | 34845ns | +201.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 8552ns | base | --- | [8160, 10120] | --- | --- | --- | --- |
| mw_chain_c_mw8_uni | 5332ns | -3372.5ns (-39.4%) | [-4642, -2834]ns | [5170, 5481] | YES | 0.0313 | 0.0313 | 0 |
| mw_chain_rev_c_mw8_uni | 5475ns | -3306.9ns (-38.7%) | [-4639, -2777]ns | [5134, 5499] | YES | 0.0313 | 0.0313 | 0 |
| mw_jumptable_c_mw8_uni | 5476ns | -3076.0ns (-36.0%) | [-4860, -2675]ns | [5120, 5626] | YES | 0.0313 | 0.0313 | 0 |
| mw_predicate_all_c_mw8_uni | 32705ns | +24317.8ns (+284.4%) | [+20648, +25283]ns | [30295, 34081] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw8_uni | mw_chain_c_mw8_uni | mw_chain_rev_c_mw8_uni | mw_jumptable_c_mw8_uni | mw_predicate_all_c_mw8_uni |
|---|---|---|---|---|---|
| 1 | 11079ns | -50.5% | -50.6% | -57.0% | +156.4% |
| 2 | 8435ns | -38.8% | -35.1% | -35.1% | +303.9% |
| 3 | 8215ns | -37.0% | -37.7% | -33.4% | +291.8% |
| 4 | 8668ns | -40.1% | -40.6% | -36.9% | +279.6% |
| 5 | 9160ns | -40.3% | -40.1% | -37.2% | +272.2% |
| 6 | 8106ns | -32.4% | -32.0% | -32.2% | +301.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw8_uni | -0.122 | ok |
| mw_chain_c_mw8_uni | 0.164 | ok |
| mw_chain_rev_c_mw8_uni | 0.191 | ok |
| mw_jumptable_c_mw8_uni | 0.032 | ok |
| mw_predicate_all_c_mw8_uni | -0.272 | moderate- |

**Consistency summary:**

- **mw_chain_c_mw8_uni**: won 6/6, lost 0/6
- **mw_chain_rev_c_mw8_uni**: won 6/6, lost 0/6
- **mw_jumptable_c_mw8_uni**: won 6/6, lost 0/6
- **mw_predicate_all_c_mw8_uni**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 4.5ns | 8943.9ns | 0.1% |  |
| mw_chain_c_mw8_uni | 4.8ns | 5327.8ns | 0.1% |  |
| mw_chain_rev_c_mw8_uni | 4.6ns | 5369.4ns | 0.1% |  |
| mw_jumptable_c_mw8_uni | 4.0ns | 5407.1ns | 0.1% |  |
| mw_predicate_all_c_mw8_uni | 4.5ns | 32360.2ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw8_uni (n=6, range 8105.8-10119.8 ns)
   8105.8 |########################################
   8206.5 |########################################
   8307.2 |
   8407.9 |########################################
   8508.6 |
   8609.3 |########################################
   8710.0 |
   8810.7 |
   8911.4 |
   9012.1 |
   9112.8 |########################################
   9213.5 |
   9314.2 |
   9414.9 |
   9515.6 |
   9616.3 |
   9717.0 |
   9817.7 |
   9918.4 |
  10019.1 |
  (0 below, 1 above range)

mw_chain_c_mw8_uni (n=6, range 5165.8-5481.5 ns)
   5165.8 |########################################
   5181.6 |####################
   5197.4 |
   5213.1 |
   5228.9 |
   5244.7 |
   5260.5 |
   5276.3 |
   5292.1 |
   5307.8 |
   5323.6 |
   5339.4 |
   5355.2 |
   5371.0 |
   5386.8 |
   5402.5 |
   5418.3 |
   5434.1 |
   5449.9 |
   5465.7 |########################################
  (0 below, 1 above range)

mw_chain_rev_c_mw8_uni (n=6, range 5117.9-5499.4 ns)
   5117.9 |####################
   5137.0 |####################
   5156.0 |
   5175.1 |
   5194.2 |
   5213.3 |
   5232.3 |
   5251.4 |
   5270.5 |
   5289.6 |
   5308.6 |
   5327.7 |
   5346.8 |
   5365.9 |
   5384.9 |
   5404.0 |
   5423.1 |
   5442.2 |
   5461.2 |########################################
   5480.3 |####################
  (0 below, 1 above range)

mw_jumptable_c_mw8_uni (n=6, range 4766.7-5626.1 ns)
   4766.7 |#############
   4809.7 |
   4852.6 |
   4895.6 |
   4938.6 |
   4981.5 |
   5024.5 |
   5067.5 |
   5110.4 |
   5153.4 |
   5196.4 |
   5239.3 |
   5282.3 |
   5325.3 |
   5368.2 |
   5411.2 |
   5454.2 |########################################
   5497.1 |#############
   5540.1 |
   5583.1 |
  (0 below, 1 above range)

mw_predicate_all_c_mw8_uni (n=6, range 28405.0-34080.8 ns)
  28405.0 |########################################
  28688.8 |
  28972.6 |
  29256.4 |
  29540.2 |
  29824.0 |
  30107.7 |
  30391.5 |
  30675.3 |
  30959.1 |
  31242.9 |
  31526.7 |
  31810.5 |
  32094.3 |########################################
  32378.1 |########################################
  32661.9 |########################################
  32945.6 |
  33229.4 |
  33513.2 |
  33797.0 |########################################
  (0 below, 1 above range)

```
