# IR-driven strategy showdown: interp vs native, all strategies, same mockup IR

12 variants, 6 samples per variant.
Baseline: **ir_bintree_int**

## Key findings

- **Fastest: ir_jumptable_nat** at 3406.7 ns median (-96.5% vs baseline)
- 6 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 78.08x (fastest 3406.7 ns, slowest 266001.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ir_bintree_int | 98708ns | 100135ns | 88972ns | 99320ns | 102659ns | base |
| ir_bintree_nat | 6256ns | 6291ns | 5765ns | 6159ns | 6645ns | -93.66% |
| ir_branch_int | 98743ns | 97122ns | 93661ns | 96711ns | 104331ns | +0.04% |
| ir_branch_nat | 5828ns | 5839ns | 5282ns | 5654ns | 6361ns | -94.10% |
| ir_chain_rev_int | 103415ns | 105255ns | 96080ns | 104021ns | 106175ns | +4.77% |
| ir_chain_rev_nat | 6103ns | 6179ns | 5407ns | 6091ns | 6468ns | -93.82% |
| ir_jumptable_int | 97814ns | 99539ns | 88373ns | 97591ns | 102868ns | -0.91% |
| ir_jumptable_nat | 5843ns | 5920ns | 5138ns | 5858ns | 6172ns | -94.08% |
| ir_predicate_int | 269088ns | 268551ns | 265690ns | 268008ns | 272405ns | +172.61% |
| ir_predicate_nat | 12228ns | 12250ns | 11530ns | 12221ns | 12587ns | -87.61% |
| ir_profiled_int | 103145ns | 104820ns | 92474ns | 104268ns | 106795ns | +4.49% |
| ir_profiled_nat | 5902ns | 5983ns | 5417ns | 5811ns | 6282ns | -94.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ir_bintree_int | 96194ns | 86802ns | 100036ns | base | 0.043 |
| ir_bintree_nat | 3797ns | 3450ns | 4058ns | -96.05% | 1.079 |
| ir_branch_int | 96343ns | 91435ns | 101787ns | +0.15% | 0.043 |
| ir_branch_nat | 3432ns | 3097ns | 3772ns | -96.43% | 1.193 |
| ir_chain_rev_int | 100801ns | 93832ns | 103411ns | +4.79% | 0.041 |
| ir_chain_rev_nat | 3627ns | 3235ns | 3845ns | -96.23% | 1.129 |
| ir_jumptable_int | 95243ns | 86147ns | 100131ns | -0.99% | 0.043 |
| ir_jumptable_nat | 3358ns | 2929ns | 3550ns | -96.51% | 1.220 |
| ir_predicate_int | 266406ns | 262921ns | 269714ns | +176.95% | 0.015 |
| ir_predicate_nat | 9679ns | 9123ns | 9971ns | -89.94% | 0.423 |
| ir_profiled_int | 100603ns | 90243ns | 104180ns | +4.58% | 0.041 |
| ir_profiled_nat | 3457ns | 3166ns | 3699ns | -96.41% | 1.185 |

## Performance model

- Peak throughput: **1.399 Gops/s** (ir_jumptable_nat; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ir_bintree_int | 0.042 | 3.0% |
| ir_bintree_nat | 1.082 | 77.4% |
| ir_branch_int | 0.043 | 3.1% |
| ir_branch_nat | 1.197 | 85.6% |
| ir_chain_rev_int | 0.040 | 2.9% |
| ir_chain_rev_nat | 1.117 | 79.8% |
| ir_jumptable_int | 0.042 | 3.0% |
| ir_jumptable_nat | 1.202 | 86.0% |
| ir_predicate_int | 0.015 | 1.1% |
| ir_predicate_nat | 0.421 | 30.1% |
| ir_profiled_int | 0.040 | 2.9% |
| ir_profiled_nat | 1.170 | 83.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ir_bintree_int | 98708ns | 98708ns | base |
| ir_bintree_nat | 6256ns | 6256ns | -93.66% |
| ir_branch_int | 98743ns | 98743ns | +0.04% |
| ir_branch_nat | 5828ns | 5828ns | -94.10% |
| ir_chain_rev_int | 103415ns | 103415ns | +4.77% |
| ir_chain_rev_nat | 6103ns | 6103ns | -93.82% |
| ir_jumptable_int | 97814ns | 97814ns | -0.91% |
| ir_jumptable_nat | 5843ns | 5843ns | -94.08% |
| ir_predicate_int | 269088ns | 269088ns | +172.61% |
| ir_predicate_nat | 12228ns | 12228ns | -87.61% |
| ir_profiled_int | 103145ns | 103145ns | +4.49% |
| ir_profiled_nat | 5902ns | 5902ns | -94.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ir_bintree_int | 97550ns | base | --- | [90995, 100036] | --- | --- | --- | --- |
| ir_bintree_nat | 3786ns | -93771.6ns (-96.1%) | [-96166, -87254]ns | [3547, 4058] | YES | 0.0491 | 0.0313 | 0 |
| ir_branch_int | 94738ns | no significant difference | [-5657, +8285]ns | [92502, 101787] | no | 0.6875 | 0.6875 | 0 |
| ir_branch_nat | 3422ns | -94109.1ns (-96.5%) | [-96604, -87571]ns | [3102, 3772] | YES | 0.0491 | 0.0313 | 0 |
| ir_chain_rev_int | 102675ns | +5701.4ns (+5.8%) | [+848, +7274]ns | [96319, 103411] | YES (adj: no) | 0.2674 | 0.2188 | 0 |
| ir_chain_rev_nat | 3668ns | -93712.1ns (-96.1%) | [-96386, -87603]ns | [3367, 3845] | YES | 0.0491 | 0.0313 | 0 |
| ir_jumptable_int | 96896ns | no significant difference | [-6091, +4990]ns | [88701, 100131] | no | 0.6875 | 0.6875 | 0 |
| ir_jumptable_nat | 3407ns | -94333.5ns (-96.7%) | [-96486, -87686]ns | [3119, 3550] | YES | 0.0491 | 0.0313 | 0 |
| ir_predicate_int | 266002ns | +169084.6ns (+173.3%) | [+165975, +175578]ns | [263504, 269714] | YES | 0.0491 | 0.0313 | 0 |
| ir_predicate_nat | 9723ns | -87780.8ns (-90.0%) | [-90692, -81071]ns | [9344, 9971] | YES | 0.0491 | 0.0313 | 0 |
| ir_profiled_int | 102163ns | no significant difference | [-3114, +11420]ns | [95466, 104180] | no | 0.2674 | 0.2188 | 0 |
| ir_profiled_nat | 3502ns | -94173.3ns (-96.5%) | [-96468, -87567]ns | [3171, 3699] | YES | 0.0491 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ir_bintree_int | ir_bintree_nat | ir_branch_int | ir_branch_nat | ir_chain_rev_int | ir_chain_rev_nat | ir_jumptable_int | ir_jumptable_nat | ir_predicate_int | ir_predicate_nat | ir_profiled_int | ir_profiled_nat |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 95683ns | -96.2% | -4.4% | -96.8% | +7.3% | -95.9% | -10.0% | -96.9% | +174.8% | -89.9% | +5.2% | -96.7% |
| 2 | 99436ns | -96.3% | -4.0% | -96.2% | +3.5% | -96.5% | -2.4% | -96.5% | +171.9% | -90.8% | -9.2% | -96.4% |
| 3 | 100635ns | -95.9% | -7.0% | -96.6% | -1.8% | -96.2% | -1.1% | -96.4% | +163.8% | -90.5% | +4.8% | -96.3% |
| 4 | 99418ns | -96.1% | -0.4% | -96.6% | +4.5% | -96.2% | -2.7% | -96.5% | +168.1% | -90.1% | +3.0% | -96.6% |
| 5 | 86802ns | -96.0% | +8.3% | -96.4% | +8.1% | -96.3% | +5.1% | -96.2% | +204.2% | -88.4% | +18.5% | -96.3% |
| 6 | 95187ns | -95.8% | +9.8% | -96.1% | +7.9% | -96.3% | +5.8% | -96.5% | +182.7% | -89.8% | +7.1% | -96.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ir_bintree_int | 0.048 | ok |
| ir_bintree_nat | -0.342 | moderate- |
| ir_branch_int | -0.229 | moderate- |
| ir_branch_nat | -0.484 | moderate- |
| ir_chain_rev_int | -0.557 | HIGH- (thermal bounce) |
| ir_chain_rev_nat | -0.200 | moderate- |
| ir_jumptable_int | -0.195 | ok |
| ir_jumptable_nat | -0.003 | ok |
| ir_predicate_int | -0.584 | HIGH- (thermal bounce) |
| ir_predicate_nat | 0.246 | moderate+ |
| ir_profiled_int | -0.252 | moderate- |
| ir_profiled_nat | -0.214 | moderate- |

**Consistency summary:**

- **ir_bintree_nat**: won 6/6, lost 0/6
- **ir_branch_int**: won 4/6, lost 2/6
- **ir_branch_nat**: won 6/6, lost 0/6
- **ir_chain_rev_int**: won 1/6, lost 5/6
- **ir_chain_rev_nat**: won 6/6, lost 0/6
- **ir_jumptable_int**: won 4/6, lost 2/6
- **ir_jumptable_nat**: won 6/6, lost 0/6
- **ir_predicate_int**: won 0/6, lost 6/6
- **ir_predicate_nat**: won 6/6, lost 0/6
- **ir_profiled_int**: won 1/6, lost 5/6
- **ir_profiled_nat**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ir_bintree_int | 4.4ns | 96193.6ns | 0.0% |  |
| ir_bintree_nat | 3.0ns | 3796.7ns | 0.1% |  |
| ir_branch_int | 3.9ns | 96342.6ns | 0.0% |  |
| ir_branch_nat | 2.8ns | 3432.1ns | 0.1% |  |
| ir_chain_rev_int | 4.3ns | 100801.4ns | 0.0% |  |
| ir_chain_rev_nat | 3.3ns | 3626.7ns | 0.1% |  |
| ir_jumptable_int | 4.2ns | 95242.6ns | 0.0% |  |
| ir_jumptable_nat | 2.8ns | 3358.3ns | 0.1% |  |
| ir_predicate_int | 6.5ns | 266406.2ns | 0.0% |  |
| ir_predicate_nat | 26.8ns | 9679.2ns | 0.3% |  |
| ir_profiled_int | 3.8ns | 100603.1ns | 0.0% |  |
| ir_profiled_nat | 2.4ns | 3457.4ns | 0.1% |  |

## Distribution (algo ns)

```
ir_bintree_int (n=6, range 86802.1-100035.8 ns)
  86802.1 |####################
  87463.8 |
  88125.5 |
  88787.2 |
  89448.8 |
  90110.5 |
  90772.2 |
  91433.9 |
  92095.6 |
  92757.3 |
  93418.9 |
  94080.6 |
  94742.3 |####################
  95404.0 |####################
  96065.7 |
  96727.4 |
  97389.1 |
  98050.7 |
  98712.4 |
  99374.1 |########################################
  (0 below, 1 above range)

ir_bintree_nat (n=6, range 3449.6-4057.7 ns)
   3449.6 |####################
   3480.0 |
   3510.4 |
   3540.8 |
   3571.2 |
   3601.6 |
   3632.0 |########################################
   3662.4 |
   3692.8 |
   3723.2 |
   3753.6 |
   3784.1 |
   3814.5 |
   3844.9 |
   3875.3 |
   3905.7 |####################
   3936.1 |
   3966.5 |
   3996.9 |
   4027.3 |####################
  (0 below, 1 above range)

ir_branch_int (n=6, range 91434.6-101787.2 ns)
  91434.6 |########################################
  91952.2 |
  92469.9 |
  92987.5 |
  93505.1 |########################################
  94022.8 |########################################
  94540.4 |
  95058.0 |########################################
  95575.7 |
  96093.3 |
  96610.9 |
  97128.6 |
  97646.2 |
  98163.8 |
  98681.5 |########################################
  99199.1 |
  99716.7 |
  100234.4 |
  100752.0 |
  101269.6 |
  (0 below, 1 above range)

ir_branch_nat (n=6, range 3097.1-3772.1 ns)
   3097.1 |########################################
   3130.8 |
   3164.6 |
   3198.3 |
   3232.1 |
   3265.8 |
   3299.6 |
   3333.3 |
   3367.1 |
   3400.8 |########################################
   3434.6 |
   3468.3 |
   3502.1 |
   3535.8 |
   3569.6 |
   3603.3 |
   3637.1 |
   3670.8 |
   3704.6 |
   3738.3 |####################
  (0 below, 1 above range)

ir_chain_rev_int (n=6, range 93831.7-103411.2 ns)
  93831.7 |####################
  94310.7 |
  94789.7 |
  95268.6 |
  95747.6 |
  96226.6 |
  96705.6 |
  97184.5 |
  97663.5 |
  98142.5 |
  98621.5 |####################
  99100.5 |
  99579.4 |
  100058.4 |
  100537.4 |
  101016.4 |
  101495.3 |
  101974.3 |
  102453.3 |########################################
  102932.3 |####################
  (0 below, 1 above range)

ir_chain_rev_nat (n=6, range 3235.4-3845.2 ns)
   3235.4 |####################
   3265.9 |
   3296.4 |
   3326.9 |
   3357.4 |
   3387.8 |
   3418.3 |
   3448.8 |
   3479.3 |####################
   3509.8 |
   3540.3 |####################
   3570.8 |
   3601.3 |
   3631.8 |
   3662.3 |
   3692.8 |
   3723.2 |
   3753.7 |
   3784.2 |########################################
   3814.7 |
  (0 below, 1 above range)

ir_jumptable_int (n=6, range 86146.7-100130.9 ns)
  86146.7 |####################
  86845.9 |
  87545.1 |
  88244.3 |
  88943.5 |
  89642.7 |
  90341.9 |
  91041.2 |####################
  91740.4 |
  92439.6 |
  93138.8 |
  93838.0 |
  94537.2 |
  95236.4 |
  95935.6 |
  96634.8 |########################################
  97334.0 |
  98033.2 |
  98732.4 |
  99431.6 |####################
  (0 below, 1 above range)

ir_jumptable_nat (n=6, range 2928.8-3549.8 ns)
   2928.8 |####################
   2959.9 |
   2990.9 |
   3022.0 |
   3053.0 |
   3084.1 |
   3115.1 |
   3146.2 |
   3177.2 |
   3208.2 |
   3239.3 |
   3270.4 |
   3301.4 |########################################
   3332.5 |
   3363.5 |
   3394.6 |
   3425.6 |
   3456.7 |
   3487.7 |########################################
   3518.8 |
  (0 below, 1 above range)

ir_predicate_int (n=6, range 262920.8-269713.8 ns)
  262920.8 |########################################
  263260.4 |
  263600.1 |
  263939.7 |########################################
  264279.4 |
  264619.0 |
  264958.7 |
  265298.3 |########################################
  265638.0 |
  265977.6 |
  266317.3 |########################################
  266656.9 |
  266996.6 |
  267336.2 |
  267675.9 |
  268015.5 |
  268355.2 |
  268694.8 |
  269034.5 |########################################
  269374.1 |
  (0 below, 1 above range)

ir_predicate_nat (n=6, range 9123.3-9970.6 ns)
   9123.3 |########################################
   9165.7 |
   9208.0 |
   9250.4 |
   9292.8 |
   9335.1 |
   9377.5 |
   9419.9 |
   9462.2 |
   9504.6 |
   9546.9 |########################################
   9589.3 |
   9631.7 |
   9674.0 |########################################
   9716.4 |########################################
   9758.8 |
   9801.1 |########################################
   9843.5 |
   9885.9 |
   9928.2 |
  (0 below, 1 above range)

ir_profiled_int (n=6, range 90242.9-104180.2 ns)
  90242.9 |########################################
  90939.8 |
  91636.6 |
  92333.5 |
  93030.4 |
  93727.2 |
  94424.1 |
  95121.0 |
  95817.8 |
  96514.7 |
  97211.6 |
  97908.4 |
  98605.3 |
  99302.1 |
  99999.0 |########################################
  100695.9 |
  101392.7 |########################################
  102089.6 |########################################
  102786.5 |########################################
  103483.3 |
  (0 below, 1 above range)

ir_profiled_nat (n=6, range 3166.2-3699.4 ns)
   3166.2 |########################################
   3192.9 |
   3219.5 |
   3246.2 |
   3272.8 |
   3299.5 |
   3326.2 |
   3352.8 |
   3379.5 |####################
   3406.1 |
   3432.8 |
   3459.5 |
   3486.1 |
   3512.8 |
   3539.4 |
   3566.1 |
   3592.8 |####################
   3619.4 |
   3646.1 |
   3672.7 |####################
  (0 below, 1 above range)

```
