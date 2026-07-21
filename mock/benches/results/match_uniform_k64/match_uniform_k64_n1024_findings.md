# Match lowering: if-chain vs jump-table vs decision-tree, K=64 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u64**

## Highlights

Baseline for all deltas below: **ml_jumptable_u64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_u64 is an outlier: 7.7x slower than the field

ml_tree_u64 (490.11 us) is 7.7x the fastest (64.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (ml_jumptable_u64)

The baseline ml_jumptable_u64 is the fastest (64.01 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 7.7x the fastest

Fastest ml_jumptable_u64 (64.01 us) to slowest ml_tree_u64 (490.11 us): 7.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader ml_jumptable_u64 vs stability leader ml_ifchain_u64 (+1% speed for 2.1x steadier)

ml_jumptable_u64 is fastest (64.01 us, CV 4.3%); ml_ifchain_u64 gives up 1.2% median for 2.1x lower variance (CV 2.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (ml_jumptable_u64) is the fastest** at 64005.4 ns median
- 1 variant significantly slower than baseline
- Spread: 7.66x (fastest 64005.4 ns, slowest 490111.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u64 | 67119ns | 66998ns | 65674ns | 66618ns | 68595ns | -0.68% |
| ml_jumptable_u64 | 67579ns | 66251ns | 65247ns | 66149ns | 70888ns | base |
| ml_tree_u64 | 514302ns | 492399ns | 487425ns | 491699ns | 561644ns | +661.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u64 | 64891ns | 63480ns | 66304ns | -0.66% | 0.016 |
| ml_jumptable_u64 | 65325ns | 63089ns | 68545ns | base | 0.016 |
| ml_tree_u64 | 511558ns | 484503ns | 558655ns | +683.10% | 0.002 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_jumptable_u64; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u64 | 0.016 | 97.4% |
| ml_jumptable_u64 | 0.016 | 98.6% |
| ml_tree_u64 | 0.002 | 12.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u64 | 67119ns | 67119ns | -0.68% |
| ml_jumptable_u64 | 67579ns | 67579ns | base |
| ml_tree_u64 | 514302ns | 514302ns | +661.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u64 | 64005ns | base | --- | [63424, 68545] | --- | --- | --- | --- |
| ml_ifchain_u64 | 64793ns | no significant difference | [-3021, +1779]ns | [63575, 66304] | no | 0.6875 | 0.6875 | 0 |
| ml_tree_u64 | 490112ns | +423833.2ns (+662.2%) | [+420828, +494038]ns | [485907, 558655] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u64 | ml_ifchain_u64 | ml_tree_u64 |
|---|---|---|---|
| 1 | 63089ns | +1.0% | +674.8% |
| 2 | 71166ns | -7.4% | +592.9% |
| 3 | 65924ns | -0.0% | +639.2% |
| 4 | 63758ns | +4.6% | +879.0% |
| 5 | 63778ns | -0.2% | +670.5% |
| 6 | 64233ns | -1.2% | +654.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u64 | 0.119 | ok |
| ml_jumptable_u64 | -0.140 | ok |
| ml_tree_u64 | -0.235 | moderate- |

**Consistency summary:**

- **ml_ifchain_u64**: won 3/6, lost 2/6
- **ml_tree_u64**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u64 | 3.8ns | 64891.0ns | 0.0% |  |
| ml_jumptable_u64 | 4.0ns | 65324.7ns | 0.0% |  |
| ml_tree_u64 | 17.7ns | 511557.8ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u64 (n=6, range 63480.0-66304.1 ns)
  63480.0 |####################
  63621.2 |########################################
  63762.4 |
  63903.6 |
  64044.8 |
  64186.0 |
  64327.2 |
  64468.5 |
  64609.7 |
  64750.9 |
  64892.1 |
  65033.3 |
  65174.5 |
  65315.7 |
  65456.9 |
  65598.1 |
  65739.3 |####################
  65880.5 |####################
  66021.7 |
  66162.9 |
  (0 below, 1 above range)

ml_jumptable_u64 (n=6, range 63089.2-68545.0 ns)
  63089.2 |####################
  63362.0 |
  63634.8 |########################################
  63907.6 |
  64180.4 |####################
  64453.1 |
  64725.9 |
  64998.7 |
  65271.5 |
  65544.3 |
  65817.1 |####################
  66089.9 |
  66362.7 |
  66635.5 |
  66908.3 |
  67181.1 |
  67453.8 |
  67726.6 |
  67999.4 |
  68272.2 |
  (0 below, 1 above range)

ml_tree_u64 (n=6, range 484502.9-558654.8 ns)
  484502.9 |########################################
  488210.5 |########################################
  491918.1 |####################
  495625.7 |
  499333.3 |
  503040.9 |
  506748.5 |
  510456.1 |
  514163.7 |
  517871.3 |
  521578.9 |
  525286.4 |
  528994.0 |
  532701.6 |
  536409.2 |
  540116.8 |
  543824.4 |
  547532.0 |
  551239.6 |
  554947.2 |
  (0 below, 1 above range)

```
