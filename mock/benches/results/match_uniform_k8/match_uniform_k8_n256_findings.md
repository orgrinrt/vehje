# Match lowering: if-chain vs jump-table vs decision-tree, K=8 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u8**

## Highlights

Baseline for all deltas below: **ml_jumptable_u8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_u8 is an outlier: 3.4x slower than the field

ml_tree_u8 (54.49 us) is 3.4x the fastest (16.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (ml_jumptable_u8)

The baseline ml_jumptable_u8 is the fastest (16.02 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest ml_jumptable_u8 (16.02 us) to slowest ml_tree_u8 (54.49 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ml_ifchain_u8's edge over baseline is significant but tiny (17 ns, 0.10%)

ml_ifchain_u8 differs from baseline ml_jumptable_u8 by 17 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (ml_jumptable_u8) is the fastest** at 16020.2 ns median
- 1 variant significantly slower than baseline
- Spread: 3.40x (fastest 16020.2 ns, slowest 54493.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u8 | 19089ns | 19169ns | 17782ns | 18813ns | 20158ns | +2.77% |
| ml_jumptable_u8 | 18576ns | 18195ns | 17772ns | 18163ns | 19596ns | base |
| ml_tree_u8 | 57189ns | 56844ns | 53742ns | 55864ns | 60901ns | +207.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u8 | 16810ns | 15655ns | 17757ns | +2.77% | 0.015 |
| ml_jumptable_u8 | 16357ns | 15655ns | 17249ns | base | 0.016 |
| ml_tree_u8 | 54772ns | 51446ns | 58334ns | +234.85% | 0.005 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_ifchain_u8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u8 | 0.015 | 92.8% |
| ml_jumptable_u8 | 0.016 | 97.7% |
| ml_tree_u8 | 0.005 | 28.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u8 | 19089ns | 19089ns | +2.77% |
| ml_jumptable_u8 | 18576ns | 18576ns | base |
| ml_tree_u8 | 57189ns | 57189ns | +207.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u8 | 16020ns | base | --- | [15803, 17249] | --- | --- | --- | --- |
| ml_ifchain_u8 | 16876ns | no significant difference | [-218, +1560]ns | [15797, 17757] | no | 1.0000 | 1.0000 | 0 |
| ml_tree_u8 | 54493ns | +37633.4ns (+234.9%) | [+35474, +42137]ns | [51489, 58334] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u8 | ml_ifchain_u8 | ml_tree_u8 |
|---|---|---|---|
| 1 | 15655ns | +13.4% | +275.1% |
| 2 | 15961ns | +0.2% | +227.5% |
| 3 | 15951ns | -0.1% | +222.5% |
| 4 | 16079ns | -2.6% | +220.5% |
| 5 | 17758ns | -0.0% | +219.4% |
| 6 | 16739ns | +6.1% | +246.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u8 | 0.138 | ok |
| ml_jumptable_u8 | 0.232 | moderate+ |
| ml_tree_u8 | 0.159 | ok |

**Consistency summary:**

- **ml_ifchain_u8**: won 1/6, lost 3/6
- **ml_tree_u8**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u8 | 2.0ns | 16810.3ns | 0.0% |  |
| ml_jumptable_u8 | 2.5ns | 16357.4ns | 0.0% |  |
| ml_tree_u8 | 3.3ns | 54772.2ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u8 (n=6, range 15655.4-17757.3 ns)
  15655.4 |####################
  15760.5 |
  15865.6 |####################
  15970.7 |####################
  16075.8 |
  16180.9 |
  16286.0 |
  16391.1 |
  16496.2 |
  16601.3 |
  16706.4 |
  16811.4 |
  16916.5 |
  17021.6 |
  17126.7 |
  17231.8 |
  17336.9 |
  17442.0 |
  17547.1 |
  17652.2 |########################################
  (0 below, 1 above range)

ml_jumptable_u8 (n=6, range 15655.4-17248.5 ns)
  15655.4 |####################
  15735.1 |
  15814.7 |
  15894.4 |########################################
  15974.0 |
  16053.7 |####################
  16133.3 |
  16213.0 |
  16292.7 |
  16372.3 |
  16452.0 |
  16531.6 |
  16611.3 |
  16690.9 |####################
  16770.6 |
  16850.3 |
  16929.9 |
  17009.6 |
  17089.2 |
  17168.9 |
  (0 below, 1 above range)

ml_tree_u8 (n=6, range 51445.8-58334.4 ns)
  51445.8 |########################################
  51790.2 |
  52134.7 |####################
  52479.1 |
  52823.5 |
  53167.9 |
  53512.4 |
  53856.8 |
  54201.2 |
  54545.7 |
  54890.1 |
  55234.5 |
  55579.0 |
  55923.4 |
  56267.8 |
  56612.2 |####################
  56956.7 |
  57301.1 |
  57645.5 |####################
  57990.0 |
  (0 below, 1 above range)

```
