# Match lowering: if-chain vs jump-table vs decision-tree, K=64 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u64**

## Highlights

Baseline for all deltas below: **ml_jumptable_u64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_u64 is an outlier: 7.5x slower than the field

ml_tree_u64 (132.53 us) is 7.5x the fastest (17.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 7.5x the fastest

Fastest ml_ifchain_u64 (17.78 us) to slowest ml_tree_u64 (132.53 us): 7.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ml_ifchain_u64** at 17776.8 ns median (-2.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.46x (fastest 17776.8 ns, slowest 132533.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u64 | 19874ns | 20207ns | 18109ns | 20192ns | 20279ns | -3.69% |
| ml_jumptable_u64 | 20635ns | 20696ns | 18946ns | 20529ns | 21638ns | base |
| ml_tree_u64 | 133778ns | 134918ns | 124271ns | 134513ns | 137431ns | +548.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u64 | 17489ns | 15912ns | 17859ns | -3.68% | 0.015 |
| ml_jumptable_u64 | 18158ns | 16729ns | 19034ns | base | 0.014 |
| ml_tree_u64 | 131265ns | 121944ns | 134735ns | +622.89% | 0.002 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_ifchain_u64; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u64 | 0.014 | 89.5% |
| ml_jumptable_u64 | 0.014 | 87.4% |
| ml_tree_u64 | 0.002 | 12.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u64 | 19874ns | 19874ns | -3.69% |
| ml_jumptable_u64 | 20635ns | 20635ns | base |
| ml_tree_u64 | 133778ns | 133778ns | +548.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u64 | 18199ns | base | --- | [17241, 19034] | --- | --- | --- | --- |
| ml_ifchain_u64 | 17777ns | -795.6ns (-4.4%) | [-1186, -26]ns | [16832, 17859] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| ml_tree_u64 | 132534ns | +114243.1ns (+627.7%) | [+109224, +115852]ns | [126525, 134735] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u64 | ml_ifchain_u64 | ml_tree_u64 |
|---|---|---|---|
| 1 | 16729ns | -4.9% | +628.9% |
| 2 | 17873ns | -0.6% | +633.6% |
| 3 | 19085ns | -6.2% | +611.0% |
| 4 | 17754ns | +0.3% | +648.3% |
| 5 | 18983ns | -6.3% | +604.7% |
| 6 | 18526ns | -4.2% | +613.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u64 | -0.001 | ok |
| ml_jumptable_u64 | -0.066 | ok |
| ml_tree_u64 | 0.122 | ok |

**Consistency summary:**

- **ml_ifchain_u64**: won 5/6, lost 1/6
- **ml_tree_u64**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u64 | 3.7ns | 17489.1ns | 0.0% |  |
| ml_jumptable_u64 | 3.7ns | 18158.2ns | 0.0% |  |
| ml_tree_u64 | 3.3ns | 131264.6ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u64 (n=6, range 15912.5-17858.8 ns)
  15912.5 |####################
  16009.8 |
  16107.1 |
  16204.4 |
  16301.8 |
  16399.1 |
  16496.4 |
  16593.7 |
  16691.0 |
  16788.3 |
  16885.6 |
  16982.9 |
  17080.2 |
  17177.6 |
  17274.9 |
  17372.2 |
  17469.5 |
  17566.8 |
  17664.1 |########################################
  17761.4 |########################################
  (0 below, 1 above range)

ml_jumptable_u64 (n=6, range 16729.2-19033.9 ns)
  16729.2 |########################################
  16844.4 |
  16959.7 |
  17074.9 |
  17190.2 |
  17305.4 |
  17420.6 |
  17535.9 |
  17651.1 |########################################
  17766.3 |########################################
  17881.6 |
  17996.8 |
  18112.0 |
  18227.3 |
  18342.5 |
  18457.8 |########################################
  18573.0 |
  18688.2 |
  18803.5 |
  18918.7 |########################################
  (0 below, 1 above range)

ml_tree_u64 (n=6, range 121943.8-134735.0 ns)
  121943.8 |########################################
  122583.4 |
  123222.9 |
  123862.5 |
  124502.0 |
  125141.6 |
  125781.2 |
  126420.7 |
  127060.3 |
  127699.8 |
  128339.4 |
  128979.0 |
  129618.5 |
  130258.1 |
  130897.6 |########################################
  131537.2 |
  132176.8 |########################################
  132816.3 |########################################
  133455.9 |########################################
  134095.4 |
  (0 below, 1 above range)

```
