# Match lowering: if-chain vs jump-table vs decision-tree, K=2 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u2**

## Highlights

Baseline for all deltas below: **ml_jumptable_u2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (ml_jumptable_u2)

The baseline ml_jumptable_u2 is the fastest (18.08 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader ml_jumptable_u2 vs stability leader ml_ifchain_u2 (+5% speed for 1.1x steadier)

ml_jumptable_u2 is fastest (18.08 us, CV 5.4%); ml_ifchain_u2 gives up 5.0% median for 1.1x lower variance (CV 5.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (ml_jumptable_u2) is the fastest** at 18075.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.33x (fastest 18075.2 ns, slowest 24025.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u2 | 21096ns | 21571ns | 18731ns | 21345ns | 21905ns | +3.70% |
| ml_jumptable_u2 | 20344ns | 20541ns | 18133ns | 20414ns | 21344ns | base |
| ml_tree_u2 | 26243ns | 26602ns | 23147ns | 26497ns | 27410ns | +29.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u2 | 18527ns | 16487ns | 19157ns | +3.46% | 0.014 |
| ml_jumptable_u2 | 17908ns | 15965ns | 18788ns | base | 0.014 |
| ml_tree_u2 | 23758ns | 20977ns | 24850ns | +32.67% | 0.011 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_jumptable_u2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u2 | 0.013 | 84.1% |
| ml_jumptable_u2 | 0.014 | 88.3% |
| ml_tree_u2 | 0.011 | 66.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u2 | 21096ns | 21096ns | +3.70% |
| ml_jumptable_u2 | 20344ns | 20344ns | base |
| ml_tree_u2 | 26243ns | 26243ns | +29.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u2 | 18075ns | base | --- | [16861, 18788] | --- | --- | --- | --- |
| ml_ifchain_u2 | 18978ns | +454.2ns (+2.5%) | [+28, +1376]ns | [17447, 19157] | YES | 0.0313 | 0.0313 | 0 |
| ml_tree_u2 | 24025ns | +5691.9ns (+31.5%) | [+5119, +6739]ns | [22398, 24850] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u2 | ml_ifchain_u2 | ml_tree_u2 |
|---|---|---|---|
| 1 | 15965ns | +3.3% | +31.4% |
| 2 | 17757ns | +6.9% | +41.1% |
| 3 | 18352ns | +0.3% | +31.1% |
| 4 | 18593ns | +2.1% | +28.1% |
| 5 | 17798ns | +8.6% | +34.8% |
| 6 | 18982ns | +0.0% | +29.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u2 | -0.054 | ok |
| ml_jumptable_u2 | 0.060 | ok |
| ml_tree_u2 | -0.285 | moderate- |

**Consistency summary:**

- **ml_ifchain_u2**: won 0/6, lost 5/6
- **ml_tree_u2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u2 | 3.0ns | 18527.3ns | 0.0% |  |
| ml_jumptable_u2 | 2.9ns | 17907.9ns | 0.0% |  |
| ml_tree_u2 | 3.5ns | 23757.8ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u2 (n=6, range 16487.1-19156.8 ns)
  16487.1 |#############
  16620.6 |
  16754.1 |
  16887.6 |
  17021.0 |
  17154.5 |
  17288.0 |
  17421.5 |
  17555.0 |
  17688.5 |
  17822.0 |
  17955.5 |
  18088.9 |
  18222.4 |
  18355.9 |#############
  18489.4 |
  18622.9 |
  18756.4 |
  18889.9 |########################################
  19023.4 |
  (0 below, 1 above range)

ml_jumptable_u2 (n=6, range 15965.4-18787.5 ns)
  15965.4 |####################
  16106.5 |
  16247.6 |
  16388.7 |
  16529.8 |
  16670.9 |
  16812.0 |
  16953.1 |
  17094.2 |
  17235.3 |
  17376.5 |
  17517.6 |
  17658.7 |########################################
  17799.8 |
  17940.9 |
  18082.0 |
  18223.1 |####################
  18364.2 |
  18505.3 |####################
  18646.4 |
  (0 below, 1 above range)

ml_tree_u2 (n=6, range 20977.1-24849.6 ns)
  20977.1 |####################
  21170.7 |
  21364.3 |
  21558.0 |
  21751.6 |
  21945.2 |
  22138.8 |
  22332.5 |
  22526.1 |
  22719.7 |
  22913.3 |
  23106.9 |
  23300.6 |
  23494.2 |
  23687.8 |####################
  23881.4 |########################################
  24075.1 |
  24268.7 |
  24462.3 |####################
  24655.9 |
  (0 below, 1 above range)

```
