# Match lowering: if-chain vs jump-table vs decision-tree, K=2 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u2**

## Highlights

Baseline for all deltas below: **ml_jumptable_u2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (ml_jumptable_u2)

The baseline ml_jumptable_u2 is the fastest (4.78 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader ml_jumptable_u2 vs stability leader ml_ifchain_u2 (+2% speed for 1.1x steadier)

ml_jumptable_u2 is fastest (4.78 us, CV 12.4%); ml_ifchain_u2 gives up 1.6% median for 1.1x lower variance (CV 11.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### ml_ifchain_u2's edge over baseline is significant but tiny (-33 ns, 0.68%)

ml_ifchain_u2 differs from baseline ml_jumptable_u2 by -33 ns (0.68%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (ml_jumptable_u2) is the fastest** at 4780.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.34x (fastest 4780.0 ns, slowest 6390.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u2 | 7809ns | 7457ns | 6955ns | 7414ns | 8828ns | -0.20% |
| ml_jumptable_u2 | 7824ns | 7389ns | 6984ns | 7373ns | 8920ns | base |
| ml_tree_u2 | 9538ns | 9010ns | 7628ns | 8944ns | 11383ns | +21.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u2 | 5074ns | 4538ns | 5720ns | +0.35% | 0.013 |
| ml_jumptable_u2 | 5056ns | 4501ns | 5764ns | base | 0.013 |
| ml_tree_u2 | 6754ns | 5397ns | 8051ns | +33.57% | 0.009 |

## Performance model

- Peak throughput: **0.014 Gops/s** (ml_jumptable_u2; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u2 | 0.013 | 92.7% |
| ml_jumptable_u2 | 0.013 | 94.2% |
| ml_tree_u2 | 0.010 | 70.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u2 | 7809ns | 7809ns | -0.20% |
| ml_jumptable_u2 | 7824ns | 7824ns | base |
| ml_tree_u2 | 9538ns | 9538ns | +21.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u2 | 4780ns | base | --- | [4625, 5764] | --- | --- | --- | --- |
| ml_ifchain_u2 | 4858ns | no significant difference | [-146, +232]ns | [4645, 5720] | no | 0.6875 | 0.6875 | 0 |
| ml_tree_u2 | 6390ns | +1511.5ns (+31.6%) | [+1041, +2540]ns | [5821, 8051] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u2 | ml_ifchain_u2 | ml_tree_u2 |
|---|---|---|---|
| 1 | 6296ns | -0.1% | +48.5% |
| 2 | 5232ns | -1.5% | +29.1% |
| 3 | 4809ns | -1.2% | +29.9% |
| 4 | 4501ns | +9.5% | +45.1% |
| 5 | 4750ns | -4.5% | +31.6% |
| 6 | 4751ns | +0.8% | +13.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u2 | 0.177 | ok |
| ml_jumptable_u2 | 0.271 | moderate+ |
| ml_tree_u2 | 0.100 | ok |

**Consistency summary:**

- **ml_ifchain_u2**: won 4/6, lost 2/6
- **ml_tree_u2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u2 | 4.4ns | 5074.2ns | 0.1% |  |
| ml_jumptable_u2 | 3.0ns | 5056.4ns | 0.1% |  |
| ml_tree_u2 | 3.2ns | 6754.0ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u2 (n=6, range 4537.5-5720.4 ns)
   4537.5 |########################################
   4596.6 |
   4655.8 |
   4714.9 |########################################
   4774.1 |########################################
   4833.2 |
   4892.4 |########################################
   4951.5 |
   5010.7 |
   5069.8 |
   5128.9 |########################################
   5188.1 |
   5247.2 |
   5306.4 |
   5365.5 |
   5424.7 |
   5483.8 |
   5543.0 |
   5602.1 |
   5661.3 |
  (0 below, 1 above range)

ml_jumptable_u2 (n=6, range 4500.8-5763.9 ns)
   4500.8 |####################
   4564.0 |
   4627.1 |
   4690.3 |########################################
   4753.4 |####################
   4816.6 |
   4879.7 |
   4942.9 |
   5006.1 |
   5069.2 |
   5132.4 |
   5195.5 |####################
   5258.7 |
   5321.8 |
   5385.0 |
   5448.2 |
   5511.3 |
   5574.5 |
   5637.6 |
   5700.8 |
  (0 below, 1 above range)

ml_tree_u2 (n=6, range 5396.7-8051.1 ns)
   5396.7 |####################
   5529.4 |
   5662.1 |
   5794.9 |
   5927.6 |
   6060.3 |
   6193.0 |########################################
   6325.7 |
   6458.4 |####################
   6591.2 |
   6723.9 |####################
   6856.6 |
   6989.3 |
   7122.0 |
   7254.7 |
   7387.5 |
   7520.2 |
   7652.9 |
   7785.6 |
   7918.3 |
  (0 below, 1 above range)

```
