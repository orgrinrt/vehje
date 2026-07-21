# Match lowering: if-chain vs jump-table vs decision-tree, K=8 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u8**

## Highlights

Baseline for all deltas below: **ml_jumptable_u8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_u8 is an outlier: 3.2x slower than the field

ml_tree_u8 (820.14 us) is 3.2x the fastest (256.34 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (ml_jumptable_u8)

The baseline ml_jumptable_u8 is the fastest (256.34 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.2x the fastest

Fastest ml_jumptable_u8 (256.34 us) to slowest ml_tree_u8 (820.14 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ml_jumptable_u8) is the fastest** at 256342.5 ns median
- 1 variant significantly slower than baseline
- Spread: 3.20x (fastest 256342.5 ns, slowest 820137.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u8 | 260965ns | 262206ns | 255978ns | 260550ns | 264081ns | +1.07% |
| ml_jumptable_u8 | 258204ns | 258577ns | 254089ns | 257155ns | 261834ns | base |
| ml_tree_u8 | 822792ns | 823211ns | 816695ns | 821276ns | 828114ns | +218.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u8 | 258436ns | 253618ns | 261523ns | +1.03% | 0.016 |
| ml_jumptable_u8 | 255810ns | 251620ns | 259271ns | base | 0.016 |
| ml_tree_u8 | 819992ns | 813782ns | 825779ns | +220.55% | 0.005 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_jumptable_u8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u8 | 0.016 | 97.0% |
| ml_jumptable_u8 | 0.016 | 98.2% |
| ml_tree_u8 | 0.005 | 30.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u8 | 260965ns | 260965ns | +1.07% |
| ml_jumptable_u8 | 258204ns | 258204ns | base |
| ml_tree_u8 | 822792ns | 822792ns | +218.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u8 | 256342ns | base | --- | [251816, 259271] | --- | --- | --- | --- |
| ml_ifchain_u8 | 259488ns | no significant difference | [-2047, +7837]ns | [254296, 261523] | no | 1.0000 | 1.0000 | 0 |
| ml_tree_u8 | 820137ns | +562531.4ns (+219.4%) | [+558929, +571087]ns | [814061, 825779] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u8 | ml_ifchain_u8 | ml_tree_u8 |
|---|---|---|---|
| 1 | 257370ns | -0.9% | +221.5% |
| 2 | 260015ns | -0.1% | +216.2% |
| 3 | 255315ns | -0.7% | +220.5% |
| 4 | 258527ns | +1.7% | +215.0% |
| 5 | 251620ns | +3.4% | +223.4% |
| 6 | 252013ns | +2.8% | +227.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u8 | -0.397 | moderate- |
| ml_jumptable_u8 | 0.128 | ok |
| ml_tree_u8 | 0.209 | moderate+ |

**Consistency summary:**

- **ml_ifchain_u8**: won 2/6, lost 3/6
- **ml_tree_u8**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u8 | 3.8ns | 258435.6ns | 0.0% |  |
| ml_jumptable_u8 | 5.5ns | 255809.9ns | 0.0% |  |
| ml_tree_u8 | 25.4ns | 819992.2ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u8 (n=6, range 253617.5-261523.2 ns)
  253617.5 |########################################
  254012.8 |
  254408.1 |
  254803.3 |########################################
  255198.6 |
  255593.9 |
  255989.2 |
  256384.5 |
  256779.8 |
  257175.0 |
  257570.3 |
  257965.6 |
  258360.9 |
  258756.2 |
  259151.5 |########################################
  259546.7 |########################################
  259942.0 |########################################
  260337.3 |
  260732.6 |
  261127.9 |
  (0 below, 1 above range)

ml_jumptable_u8 (n=6, range 251620.0-259270.7 ns)
  251620.0 |########################################
  252002.5 |########################################
  252385.1 |
  252767.6 |
  253150.1 |
  253532.7 |
  253915.2 |
  254297.7 |
  254680.3 |
  255062.8 |########################################
  255445.3 |
  255827.9 |
  256210.4 |
  256592.9 |
  256975.5 |
  257358.0 |########################################
  257740.5 |
  258123.1 |
  258505.6 |########################################
  258888.1 |
  (0 below, 1 above range)

ml_tree_u8 (n=6, range 813782.5-825778.8 ns)
  813782.5 |########################################
  814382.3 |
  814982.1 |
  815581.9 |
  816181.8 |
  816781.6 |
  817381.4 |
  817981.2 |####################
  818581.0 |
  819180.8 |
  819780.6 |
  820380.4 |
  820980.2 |
  821580.1 |####################
  822179.9 |
  822779.7 |
  823379.5 |
  823979.3 |####################
  824579.1 |
  825178.9 |
  (0 below, 1 above range)

```
