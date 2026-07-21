# IR-driven strategy showdown: interp vs native, all strategies, same mockup IR

12 variants, 6 samples per variant.
Baseline: **ir_bintree_int**

## Key findings

- **Fastest: ir_jumptable_nat** at 844.4 ns median (-96.8% vs baseline)
- 6 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 86.30x (fastest 844.4 ns, slowest 72867.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ir_bintree_int | 27941ns | 29032ns | 24297ns | 27623ns | 30239ns | base |
| ir_bintree_nat | 3347ns | 3449ns | 2997ns | 3300ns | 3594ns | -88.02% |
| ir_branch_int | 27713ns | 28613ns | 24685ns | 27495ns | 29554ns | -0.81% |
| ir_branch_nat | 3316ns | 3327ns | 3048ns | 3245ns | 3556ns | -88.13% |
| ir_chain_rev_int | 28896ns | 29826ns | 25810ns | 29158ns | 30046ns | +3.42% |
| ir_chain_rev_nat | 3449ns | 3546ns | 3000ns | 3394ns | 3755ns | -87.66% |
| ir_jumptable_int | 26123ns | 25909ns | 23868ns | 25262ns | 28542ns | -6.51% |
| ir_jumptable_nat | 3243ns | 3269ns | 2888ns | 3186ns | 3505ns | -88.39% |
| ir_predicate_int | 73831ns | 75479ns | 68864ns | 73553ns | 76732ns | +164.24% |
| ir_predicate_nat | 4263ns | 4250ns | 3986ns | 4241ns | 4435ns | -84.74% |
| ir_profiled_int | 28038ns | 28797ns | 24844ns | 27553ns | 30362ns | +0.35% |
| ir_profiled_nat | 3350ns | 3502ns | 2976ns | 3350ns | 3538ns | -88.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ir_bintree_int | 25370ns | 22053ns | 27449ns | base | 0.040 |
| ir_bintree_nat | 922ns | 826ns | 991ns | -96.37% | 1.111 |
| ir_branch_int | 25296ns | 22517ns | 26974ns | -0.29% | 0.040 |
| ir_branch_nat | 869ns | 799ns | 935ns | -96.57% | 1.178 |
| ir_chain_rev_int | 26335ns | 23562ns | 27410ns | +3.80% | 0.039 |
| ir_chain_rev_nat | 926ns | 814ns | 1006ns | -96.35% | 1.106 |
| ir_jumptable_int | 23735ns | 21693ns | 25946ns | -6.44% | 0.043 |
| ir_jumptable_nat | 838ns | 753ns | 905ns | -96.70% | 1.221 |
| ir_predicate_int | 71329ns | 66599ns | 74146ns | +181.16% | 0.014 |
| ir_predicate_nat | 1657ns | 1526ns | 1740ns | -93.47% | 0.618 |
| ir_profiled_int | 25607ns | 22667ns | 27719ns | +0.93% | 0.040 |
| ir_profiled_nat | 889ns | 781ns | 945ns | -96.49% | 1.151 |

## Performance model

- Peak throughput: **1.359 Gops/s** (ir_jumptable_nat; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ir_bintree_int | 0.039 | 2.9% |
| ir_bintree_nat | 1.082 | 79.6% |
| ir_branch_int | 0.039 | 2.9% |
| ir_branch_nat | 1.177 | 86.6% |
| ir_chain_rev_int | 0.038 | 2.8% |
| ir_chain_rev_nat | 1.088 | 80.0% |
| ir_jumptable_int | 0.043 | 3.2% |
| ir_jumptable_nat | 1.213 | 89.2% |
| ir_predicate_int | 0.014 | 1.0% |
| ir_predicate_nat | 0.620 | 45.6% |
| ir_profiled_int | 0.039 | 2.9% |
| ir_profiled_nat | 1.105 | 81.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ir_bintree_int | 27941ns | 27941ns | base |
| ir_bintree_nat | 3347ns | 3347ns | -88.02% |
| ir_branch_int | 27713ns | 27713ns | -0.81% |
| ir_branch_nat | 3316ns | 3316ns | -88.13% |
| ir_chain_rev_int | 28896ns | 28896ns | +3.42% |
| ir_chain_rev_nat | 3449ns | 3449ns | -87.66% |
| ir_jumptable_int | 26123ns | 26123ns | -6.51% |
| ir_jumptable_nat | 3243ns | 3243ns | -88.39% |
| ir_predicate_int | 73831ns | 73831ns | +164.24% |
| ir_predicate_nat | 4263ns | 4263ns | -84.74% |
| ir_profiled_int | 28038ns | 28038ns | +0.35% |
| ir_profiled_nat | 3350ns | 3350ns | -88.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ir_bintree_int | 26357ns | base | --- | [22303, 27449] | --- | --- | --- | --- |
| ir_bintree_nat | 947ns | -25482.3ns (-96.7%) | [-26458, -21404]ns | [828, 991] | YES | 0.0491 | 0.0313 | 0 |
| ir_branch_int | 26110ns | no significant difference | [-1352, +1034]ns | [22805, 26974] | no | 1.0000 | 1.0000 | 0 |
| ir_branch_nat | 870ns | -25484.8ns (-96.7%) | [-26578, -21438]ns | [803, 935] | YES | 0.0491 | 0.0313 | 0 |
| ir_chain_rev_int | 27128ns | no significant difference | [-1068, +3230]ns | [24467, 27410] | no | 0.7563 | 0.6875 | 0 |
| ir_chain_rev_nat | 941ns | -25450.7ns (-96.6%) | [-26507, -21373]ns | [831, 1006] | YES | 0.0491 | 0.0313 | 0 |
| ir_jumptable_int | 23548ns | no significant difference | [-4090, +1342]ns | [21711, 25946] | no | 0.3008 | 0.2188 | 0 |
| ir_jumptable_nat | 844ns | -25486.2ns (-96.7%) | [-26619, -21488]ns | [766, 905] | YES | 0.0491 | 0.0313 | 0 |
| ir_predicate_int | 72867ns | +45473.1ns (+172.5%) | [+42321, +50084]ns | [66974, 74146] | YES | 0.0491 | 0.0313 | 0 |
| ir_predicate_nat | 1652ns | -24714.8ns (-93.8%) | [-25784, -20639]ns | [1579, 1740] | YES | 0.0491 | 0.0313 | 0 |
| ir_profiled_int | 26282ns | no significant difference | [-3303, +3542]ns | [22818, 27719] | no | 0.7563 | 0.6875 | 0 |
| ir_profiled_nat | 927ns | -25422.3ns (-96.5%) | [-26511, -21507]ns | [796, 945] | YES | 0.0491 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ir_bintree_int | ir_bintree_nat | ir_branch_int | ir_branch_nat | ir_chain_rev_int | ir_chain_rev_nat | ir_jumptable_int | ir_jumptable_nat | ir_predicate_int | ir_predicate_nat | ir_profiled_int | ir_profiled_nat |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 26752ns | -96.3% | -0.0% | -97.0% | +2.1% | -96.6% | -14.2% | -97.2% | +151.7% | -94.3% | +3.1% | -96.5% |
| 2 | 26072ns | -96.8% | +3.9% | -96.4% | +3.5% | -96.3% | -16.8% | -96.5% | +183.6% | -93.7% | +4.9% | -96.4% |
| 3 | 22053ns | -96.3% | +4.7% | -95.8% | +24.7% | -95.3% | -1.5% | -96.1% | +237.1% | -92.4% | +26.3% | -96.3% |
| 4 | 28146ns | -96.5% | -9.5% | -96.7% | -3.1% | -96.6% | -6.5% | -96.8% | +162.6% | -93.6% | -18.4% | -96.7% |
| 5 | 26642ns | -96.5% | +0.8% | -97.0% | -4.8% | -96.8% | -9.4% | -96.9% | +169.6% | -93.9% | -5.4% | -96.5% |
| 6 | 22552ns | -95.7% | -0.2% | -96.5% | +4.5% | -96.4% | +13.3% | -96.5% | +195.3% | -92.7% | +0.5% | -96.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ir_bintree_int | -0.346 | moderate- |
| ir_bintree_nat | -0.141 | ok |
| ir_branch_int | -0.284 | moderate- |
| ir_branch_nat | 0.175 | ok |
| ir_chain_rev_int | 0.347 | moderate+ |
| ir_chain_rev_nat | 0.408 | moderate+ |
| ir_jumptable_int | 0.121 | ok |
| ir_jumptable_nat | -0.196 | ok |
| ir_predicate_int | 0.068 | ok |
| ir_predicate_nat | -0.007 | ok |
| ir_profiled_int | 0.131 | ok |
| ir_profiled_nat | -0.255 | moderate- |

**Consistency summary:**

- **ir_bintree_nat**: won 6/6, lost 0/6
- **ir_branch_int**: won 2/6, lost 3/6
- **ir_branch_nat**: won 6/6, lost 0/6
- **ir_chain_rev_int**: won 2/6, lost 4/6
- **ir_chain_rev_nat**: won 6/6, lost 0/6
- **ir_jumptable_int**: won 5/6, lost 1/6
- **ir_jumptable_nat**: won 6/6, lost 0/6
- **ir_predicate_int**: won 0/6, lost 6/6
- **ir_predicate_nat**: won 6/6, lost 0/6
- **ir_profiled_int**: won 2/6, lost 4/6
- **ir_profiled_nat**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ir_bintree_int | 4.6ns | 25369.7ns | 0.0% |  |
| ir_bintree_nat | 2.7ns | 921.6ns | 0.3% |  |
| ir_branch_int | 3.1ns | 25296.5ns | 0.0% |  |
| ir_branch_nat | 2.6ns | 869.4ns | 0.3% |  |
| ir_chain_rev_int | 4.3ns | 26334.9ns | 0.0% |  |
| ir_chain_rev_nat | 2.6ns | 926.1ns | 0.3% |  |
| ir_jumptable_int | 3.3ns | 23735.0ns | 0.0% |  |
| ir_jumptable_nat | 2.7ns | 838.4ns | 0.3% |  |
| ir_predicate_int | 4.1ns | 71329.0ns | 0.0% |  |
| ir_predicate_nat | 28.5ns | 1657.3ns | 1.7% |  |
| ir_profiled_int | 3.1ns | 25606.6ns | 0.0% |  |
| ir_profiled_nat | 3.2ns | 889.4ns | 0.4% |  |

## Distribution (algo ns)

```
ir_bintree_int (n=6, range 22053.3-27449.2 ns)
  22053.3 |####################
  22323.1 |####################
  22592.9 |
  22862.7 |
  23132.5 |
  23402.3 |
  23672.1 |
  23941.8 |
  24211.6 |
  24481.4 |
  24751.2 |
  25021.0 |
  25290.8 |
  25560.6 |
  25830.4 |####################
  26100.2 |
  26370.0 |
  26639.8 |########################################
  26909.6 |
  27179.4 |
  (0 below, 1 above range)

ir_bintree_nat (n=6, range 826.2-990.8 ns)
    826.2 |########################################
    834.4 |
    842.7 |
    850.9 |
    859.1 |
    867.4 |
    875.6 |
    883.8 |
    892.0 |
    900.3 |
    908.5 |
    916.7 |####################
    925.0 |
    933.2 |
    941.4 |
    949.6 |
    957.9 |
    966.1 |####################
    974.3 |
    982.6 |####################
  (0 below, 1 above range)

ir_branch_int (n=6, range 22517.1-26973.5 ns)
  22517.1 |########################################
  22739.9 |
  22962.7 |########################################
  23185.6 |
  23408.4 |
  23631.2 |
  23854.0 |
  24076.8 |
  24299.7 |
  24522.5 |
  24745.3 |
  24968.1 |
  25190.9 |
  25413.8 |########################################
  25636.6 |
  25859.4 |
  26082.2 |
  26305.0 |
  26527.9 |########################################
  26750.7 |########################################
  (0 below, 1 above range)

ir_branch_nat (n=6, range 799.2-935.2 ns)
    799.2 |####################
    806.0 |########################################
    812.8 |
    819.6 |
    826.4 |
    833.2 |
    840.0 |
    846.8 |
    853.6 |
    860.4 |
    867.2 |
    874.0 |
    880.8 |
    887.6 |
    894.4 |
    901.2 |
    908.0 |
    914.8 |
    921.6 |
    928.4 |########################################
  (0 below, 1 above range)

ir_chain_rev_int (n=6, range 23562.1-27410.0 ns)
  23562.1 |####################
  23754.5 |
  23946.9 |
  24139.3 |
  24331.7 |
  24524.1 |
  24716.5 |
  24908.9 |
  25101.3 |
  25293.7 |####################
  25486.0 |
  25678.4 |
  25870.8 |
  26063.2 |
  26255.6 |
  26448.0 |
  26640.4 |
  26832.8 |####################
  27025.2 |
  27217.6 |########################################
  (0 below, 1 above range)

ir_chain_rev_nat (n=6, range 813.8-1006.2 ns)
    813.8 |####################
    823.4 |
    833.0 |
    842.7 |####################
    852.3 |
    861.9 |
    871.5 |
    881.1 |
    890.8 |
    900.4 |
    910.0 |####################
    919.6 |
    929.2 |
    938.9 |
    948.5 |
    958.1 |########################################
    967.7 |
    977.3 |
    987.0 |
    996.6 |
  (0 below, 1 above range)

ir_jumptable_int (n=6, range 21692.9-25945.8 ns)
  21692.9 |########################################
  21905.5 |
  22118.2 |
  22330.8 |
  22543.5 |
  22756.1 |####################
  22968.8 |
  23181.4 |
  23394.1 |
  23606.7 |
  23819.4 |
  24032.0 |####################
  24244.7 |
  24457.3 |
  24670.0 |
  24882.6 |
  25095.3 |
  25307.9 |
  25520.6 |####################
  25733.2 |
  (0 below, 1 above range)

ir_jumptable_nat (n=6, range 753.3-905.2 ns)
    753.3 |########################################
    760.9 |
    768.5 |
    776.1 |########################################
    783.7 |
    791.3 |
    798.9 |
    806.5 |
    814.1 |
    821.7 |
    829.2 |
    836.8 |########################################
    844.4 |########################################
    852.0 |
    859.6 |
    867.2 |
    874.8 |
    882.4 |
    890.0 |
    897.6 |########################################
  (0 below, 1 above range)

ir_predicate_int (n=6, range 66599.2-74146.4 ns)
  66599.2 |####################
  66976.6 |####################
  67353.9 |
  67731.3 |
  68108.6 |
  68486.0 |
  68863.4 |
  69240.7 |
  69618.1 |
  69995.5 |
  70372.8 |
  70750.2 |
  71127.6 |
  71504.9 |####################
  71882.3 |
  72259.6 |
  72637.0 |
  73014.4 |
  73391.7 |
  73769.1 |########################################
  (0 below, 1 above range)

ir_predicate_nat (n=6, range 1526.2-1740.2 ns)
   1526.2 |####################
   1536.9 |
   1547.6 |
   1558.3 |
   1569.0 |
   1579.7 |
   1590.4 |
   1601.1 |
   1611.8 |
   1622.5 |####################
   1633.2 |
   1643.9 |########################################
   1654.6 |
   1665.3 |####################
   1676.0 |
   1686.7 |
   1697.4 |
   1708.1 |
   1718.8 |
   1729.5 |
  (0 below, 1 above range)

ir_profiled_int (n=6, range 22666.7-27719.4 ns)
  22666.7 |########################################
  22919.3 |########################################
  23172.0 |
  23424.6 |
  23677.2 |
  23929.9 |
  24182.5 |
  24435.1 |
  24687.8 |
  24940.4 |
  25193.1 |########################################
  25445.7 |
  25698.3 |
  25951.0 |
  26203.6 |
  26456.2 |
  26708.9 |
  26961.5 |
  27214.1 |########################################
  27466.8 |########################################
  (0 below, 1 above range)

ir_profiled_nat (n=6, range 780.8-945.5 ns)
    780.8 |########################################
    789.0 |
    797.3 |
    805.5 |########################################
    813.7 |
    822.0 |
    830.2 |
    838.4 |
    846.7 |
    854.9 |
    863.1 |
    871.4 |
    879.6 |
    887.8 |
    896.1 |
    904.3 |
    912.5 |
    920.8 |########################################
    929.0 |########################################
    937.2 |########################################
  (0 below, 1 above range)

```
