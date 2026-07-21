# IR-driven strategy showdown: interp vs native, all strategies, same mockup IR

12 variants, 6 samples per variant.
Baseline: **ir_bintree_int**

## Key findings

- **Fastest: ir_profiled_nat** at 244.4 ns median (-96.3% vs baseline)
- 7 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 80.02x (fastest 244.4 ns, slowest 19556.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ir_bintree_int | 8865ns | 9104ns | 7616ns | 8704ns | 9732ns | base |
| ir_bintree_nat | 2787ns | 2850ns | 2392ns | 2700ns | 3114ns | -68.57% |
| ir_branch_int | 9106ns | 9434ns | 7571ns | 8916ns | 10159ns | +2.72% |
| ir_branch_nat | 2775ns | 2837ns | 2375ns | 2687ns | 3109ns | -68.70% |
| ir_chain_rev_int | 9169ns | 9313ns | 7850ns | 8933ns | 10182ns | +3.42% |
| ir_chain_rev_nat | 2791ns | 2858ns | 2390ns | 2729ns | 3084ns | -68.52% |
| ir_jumptable_int | 8615ns | 8783ns | 7569ns | 8383ns | 9488ns | -2.82% |
| ir_jumptable_nat | 2770ns | 2832ns | 2366ns | 2705ns | 3069ns | -68.75% |
| ir_predicate_int | 21427ns | 22199ns | 18201ns | 21093ns | 23542ns | +141.70% |
| ir_predicate_nat | 2841ns | 2843ns | 2530ns | 2750ns | 3131ns | -67.96% |
| ir_profiled_int | 9335ns | 9490ns | 8141ns | 9054ns | 10355ns | +5.30% |
| ir_profiled_nat | 2739ns | 2742ns | 2377ns | 2710ns | 2964ns | -69.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ir_bintree_int | 6353ns | 5452ns | 6980ns | base | 0.040 |
| ir_bintree_nat | 255ns | 218ns | 280ns | -95.98% | 1.003 |
| ir_branch_int | 6588ns | 5482ns | 7320ns | +3.69% | 0.039 |
| ir_branch_nat | 243ns | 210ns | 270ns | -96.17% | 1.053 |
| ir_chain_rev_int | 6628ns | 5668ns | 7351ns | +4.33% | 0.039 |
| ir_chain_rev_nat | 257ns | 219ns | 283ns | -95.96% | 0.997 |
| ir_jumptable_int | 6158ns | 5412ns | 6785ns | -3.08% | 0.042 |
| ir_jumptable_nat | 242ns | 206ns | 267ns | -96.18% | 1.056 |
| ir_predicate_int | 18866ns | 16028ns | 20711ns | +196.96% | 0.014 |
| ir_predicate_nat | 401ns | 357ns | 448ns | -93.69% | 0.638 |
| ir_profiled_int | 6751ns | 5860ns | 7463ns | +6.26% | 0.038 |
| ir_profiled_nat | 244ns | 213ns | 269ns | -96.16% | 1.049 |

## Performance model

- Peak throughput: **1.244 Gops/s** (ir_jumptable_nat; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ir_bintree_int | 0.039 | 3.2% |
| ir_bintree_nat | 0.965 | 77.6% |
| ir_branch_int | 0.037 | 3.0% |
| ir_branch_nat | 1.027 | 82.6% |
| ir_chain_rev_int | 0.038 | 3.1% |
| ir_chain_rev_nat | 0.971 | 78.0% |
| ir_jumptable_int | 0.041 | 3.3% |
| ir_jumptable_nat | 1.023 | 82.3% |
| ir_predicate_int | 0.013 | 1.1% |
| ir_predicate_nat | 0.643 | 51.7% |
| ir_profiled_int | 0.037 | 3.0% |
| ir_profiled_nat | 1.047 | 84.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ir_bintree_int | 8865ns | 8865ns | base |
| ir_bintree_nat | 2787ns | 2787ns | -68.57% |
| ir_branch_int | 9106ns | 9106ns | +2.72% |
| ir_branch_nat | 2775ns | 2775ns | -68.70% |
| ir_chain_rev_int | 9169ns | 9169ns | +3.42% |
| ir_chain_rev_nat | 2791ns | 2791ns | -68.52% |
| ir_jumptable_int | 8615ns | 8615ns | -2.82% |
| ir_jumptable_nat | 2770ns | 2770ns | -68.75% |
| ir_predicate_int | 21427ns | 21427ns | +141.70% |
| ir_predicate_nat | 2841ns | 2841ns | -67.96% |
| ir_profiled_int | 9335ns | 9335ns | +5.30% |
| ir_profiled_nat | 2739ns | 2739ns | -69.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ir_bintree_int | 6519ns | base | --- | [5560, 6980] | --- | --- | --- | --- |
| ir_bintree_nat | 265ns | -6250.9ns (-95.9%) | [-6703, -5340]ns | [220, 280] | YES | 0.0382 | 0.0313 | 0 |
| ir_branch_int | 6851ns | +280.8ns (+4.3%) | [+32, +390]ns | [5592, 7320] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| ir_branch_nat | 249ns | -6266.7ns (-96.1%) | [-6713, -5350]ns | [210, 270] | YES | 0.0382 | 0.0313 | 0 |
| ir_chain_rev_int | 6730ns | +232.4ns (+3.6%) | [+189, +404]ns | [5804, 7351] | YES | 0.0382 | 0.0313 | 0 |
| ir_chain_rev_nat | 264ns | -6246.7ns (-95.8%) | [-6705, -5337]ns | [223, 283] | YES | 0.0382 | 0.0313 | 0 |
| ir_jumptable_int | 6270ns | -156.9ns (-2.4%) | [-404, -26]ns | [5418, 6785] | YES | 0.0382 | 0.0313 | 0 |
| ir_jumptable_nat | 250ns | -6259.6ns (-96.0%) | [-6723, -5350]ns | [211, 267] | YES | 0.0382 | 0.0313 | 0 |
| ir_predicate_int | 19556ns | +13037.0ns (+200.0%) | [+10771, +13732]ns | [16332, 20711] | YES | 0.0382 | 0.0313 | 0 |
| ir_predicate_nat | 398ns | -6071.9ns (-93.1%) | [-6585, -5199]ns | [358, 448] | YES | 0.0382 | 0.0313 | 0 |
| ir_profiled_int | 6912ns | +394.2ns (+6.0%) | [+98, +701]ns | [5878, 7463] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| ir_profiled_nat | 244ns | -6265.6ns (-96.1%) | [-6720, -5342]ns | [219, 269] | YES | 0.0382 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ir_bintree_int | ir_bintree_nat | ir_branch_int | ir_branch_nat | ir_chain_rev_int | ir_chain_rev_nat | ir_jumptable_int | ir_jumptable_nat | ir_predicate_int | ir_predicate_nat | ir_profiled_int | ir_profiled_nat |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 5669ns | -96.1% | -3.3% | -96.3% | +4.8% | -96.0% | -4.5% | -96.2% | +193.4% | -93.7% | +4.0% | -96.2% |
| 2 | 7076ns | -95.9% | +3.8% | -96.0% | +2.9% | -96.0% | -0.3% | -96.2% | +192.1% | -95.0% | +5.4% | -96.6% |
| 3 | 6512ns | -95.9% | +5.6% | -96.2% | +2.6% | -95.7% | -0.9% | -95.9% | +206.9% | -92.9% | +14.7% | -96.2% |
| 4 | 5452ns | -96.0% | +4.6% | -96.1% | +4.0% | -96.0% | -0.5% | -96.2% | +194.0% | -93.3% | +7.5% | -95.9% |
| 5 | 6883ns | -96.2% | +6.0% | -96.3% | +7.8% | -96.2% | -5.3% | -96.4% | +201.5% | -93.7% | -0.4% | -95.9% |
| 6 | 6527ns | -95.8% | +4.5% | -96.0% | +3.8% | -95.9% | -6.8% | -96.1% | +193.1% | -93.4% | +6.8% | -96.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ir_bintree_int | -0.425 | moderate- |
| ir_bintree_nat | -0.330 | moderate- |
| ir_branch_int | -0.414 | moderate- |
| ir_branch_nat | -0.350 | moderate- |
| ir_chain_rev_int | -0.449 | moderate- |
| ir_chain_rev_nat | -0.327 | moderate- |
| ir_jumptable_int | -0.430 | moderate- |
| ir_jumptable_nat | -0.322 | moderate- |
| ir_predicate_int | -0.474 | moderate- |
| ir_predicate_nat | -0.302 | moderate- |
| ir_profiled_int | -0.310 | moderate- |
| ir_profiled_nat | -0.069 | ok |

**Consistency summary:**

- **ir_bintree_nat**: won 6/6, lost 0/6
- **ir_branch_int**: won 1/6, lost 5/6
- **ir_branch_nat**: won 6/6, lost 0/6
- **ir_chain_rev_int**: won 0/6, lost 6/6
- **ir_chain_rev_nat**: won 6/6, lost 0/6
- **ir_jumptable_int**: won 6/6, lost 0/6
- **ir_jumptable_nat**: won 6/6, lost 0/6
- **ir_predicate_int**: won 0/6, lost 6/6
- **ir_predicate_nat**: won 6/6, lost 0/6
- **ir_profiled_int**: won 1/6, lost 5/6
- **ir_profiled_nat**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ir_bintree_int | 6.5ns | 6353.2ns | 0.1% |  |
| ir_bintree_nat | 3.8ns | 255.3ns | 1.5% |  |
| ir_branch_int | 5.4ns | 6587.7ns | 0.1% |  |
| ir_branch_nat | 4.2ns | 243.2ns | 1.7% |  |
| ir_chain_rev_int | 4.6ns | 6628.3ns | 0.1% |  |
| ir_chain_rev_nat | 4.1ns | 256.7ns | 1.6% |  |
| ir_jumptable_int | 4.9ns | 6157.6ns | 0.1% |  |
| ir_jumptable_nat | 3.9ns | 242.5ns | 1.6% |  |
| ir_predicate_int | 5.4ns | 18866.5ns | 0.0% |  |
| ir_predicate_nat | 29.6ns | 401.1ns | 7.4% | HIGH |
| ir_profiled_int | 4.4ns | 6750.8ns | 0.1% |  |
| ir_profiled_nat | 3.8ns | 244.0ns | 1.6% |  |

## Distribution (algo ns)

```
ir_bintree_int (n=6, range 5451.7-6979.8 ns)
   5451.7 |########################################
   5528.1 |
   5604.5 |########################################
   5680.9 |
   5757.3 |
   5833.7 |
   5910.1 |
   5986.5 |
   6062.9 |
   6139.3 |
   6215.7 |
   6292.1 |
   6368.5 |
   6444.9 |########################################
   6521.3 |########################################
   6597.7 |
   6674.1 |
   6750.5 |
   6826.9 |########################################
   6903.3 |
  (0 below, 1 above range)

ir_bintree_nat (n=6, range 217.9-280.4 ns)
    217.9 |####################
    221.0 |####################
    224.2 |
    227.3 |
    230.4 |
    233.5 |
    236.7 |
    239.8 |
    242.9 |
    246.0 |
    249.2 |
    252.3 |
    255.4 |
    258.6 |
    261.7 |
    264.8 |########################################
    267.9 |
    271.1 |####################
    274.2 |
    277.3 |
  (0 below, 1 above range)

ir_branch_int (n=6, range 5482.1-7319.6 ns)
   5482.1 |########################################
   5574.0 |
   5665.9 |########################################
   5757.7 |
   5849.6 |
   5941.5 |
   6033.4 |
   6125.2 |
   6217.1 |
   6309.0 |
   6400.9 |
   6492.7 |
   6584.6 |
   6676.5 |
   6768.4 |########################################
   6860.2 |########################################
   6952.1 |
   7044.0 |
   7135.9 |
   7227.7 |########################################
  (0 below, 1 above range)

ir_branch_nat (n=6, range 210.0-270.4 ns)
    210.0 |########################################
    213.0 |
    216.0 |
    219.1 |
    222.1 |
    225.1 |
    228.1 |
    231.1 |
    234.2 |
    237.2 |
    240.2 |
    243.2 |####################
    246.2 |
    249.3 |
    252.3 |####################
    255.3 |
    258.3 |####################
    261.3 |
    264.4 |
    267.4 |
  (0 below, 1 above range)

ir_chain_rev_int (n=6, range 5667.9-7351.2 ns)
   5667.9 |########################################
   5752.1 |
   5836.2 |
   5920.4 |########################################
   6004.6 |
   6088.7 |
   6172.9 |
   6257.1 |
   6341.2 |
   6425.4 |
   6509.5 |
   6593.7 |
   6677.9 |########################################
   6762.0 |########################################
   6846.2 |
   6930.4 |
   7014.5 |
   7098.7 |
   7182.9 |
   7267.0 |########################################
  (0 below, 1 above range)

ir_chain_rev_nat (n=6, range 219.2-283.3 ns)
    219.2 |########################################
    222.4 |
    225.6 |########################################
    228.8 |
    232.0 |
    235.2 |
    238.4 |
    241.6 |
    244.8 |
    248.0 |
    251.2 |
    254.5 |
    257.7 |
    260.9 |########################################
    264.1 |########################################
    267.3 |
    270.5 |
    273.7 |
    276.9 |
    280.1 |########################################
  (0 below, 1 above range)

ir_jumptable_int (n=6, range 5411.7-6784.8 ns)
   5411.7 |########################################
   5480.4 |
   5549.0 |
   5617.7 |
   5686.3 |
   5755.0 |
   5823.6 |
   5892.3 |
   5960.9 |
   6029.6 |####################
   6098.2 |
   6166.9 |
   6235.6 |
   6304.2 |
   6372.9 |
   6441.5 |####################
   6510.2 |####################
   6578.8 |
   6647.5 |
   6716.1 |
  (0 below, 1 above range)

ir_jumptable_nat (n=6, range 205.8-266.7 ns)
    205.8 |########################################
    208.8 |
    211.9 |
    214.9 |########################################
    218.0 |
    221.0 |
    224.1 |
    227.1 |
    230.2 |
    233.2 |
    236.2 |
    239.3 |
    242.3 |
    245.4 |########################################
    248.4 |
    251.5 |########################################
    254.5 |
    257.6 |
    260.6 |
    263.7 |
  (0 below, 2 above range)

ir_predicate_int (n=6, range 16027.5-20711.2 ns)
  16027.5 |########################################
  16261.7 |
  16495.9 |########################################
  16730.1 |
  16964.2 |
  17198.4 |
  17432.6 |
  17666.8 |
  17901.0 |
  18135.2 |
  18369.4 |
  18603.6 |
  18837.8 |
  19071.9 |########################################
  19306.1 |
  19540.3 |
  19774.5 |########################################
  20008.7 |
  20242.9 |
  20477.1 |########################################
  (0 below, 1 above range)

ir_predicate_nat (n=6, range 357.1-447.7 ns)
    357.1 |########################################
    361.6 |####################
    366.2 |
    370.7 |
    375.2 |
    379.8 |
    384.3 |
    388.8 |
    393.3 |
    397.9 |
    402.4 |
    406.9 |
    411.5 |
    416.0 |
    420.5 |
    425.1 |
    429.6 |########################################
    434.1 |
    438.6 |
    443.2 |
  (0 below, 1 above range)

ir_profiled_int (n=6, range 5860.0-7463.1 ns)
   5860.0 |########################################
   5940.2 |
   6020.3 |
   6100.5 |
   6180.6 |
   6260.8 |
   6340.9 |
   6421.1 |
   6501.2 |
   6581.4 |
   6661.6 |
   6741.7 |
   6821.9 |####################
   6902.0 |####################
   6982.2 |
   7062.3 |
   7142.5 |
   7222.6 |
   7302.8 |
   7382.9 |####################
  (0 below, 1 above range)

ir_profiled_nat (n=6, range 212.9-268.9 ns)
    212.9 |########################################
    215.7 |
    218.5 |
    221.3 |
    224.1 |########################################
    226.9 |
    229.7 |
    232.5 |
    235.3 |
    238.1 |########################################
    240.9 |
    243.7 |
    246.5 |
    249.3 |########################################
    252.1 |
    254.9 |
    257.7 |########################################
    260.5 |
    263.3 |
    266.1 |
  (0 below, 1 above range)

```
