# Match lowering: if-chain vs jump-table vs decision-tree, K=8 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u8**

## Highlights

Baseline for all deltas below: **ml_jumptable_u8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_u8 is an outlier: 3.2x slower than the field

ml_tree_u8 (15.11 us) is 3.2x the fastest (4.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ml_ifchain_u8 is fastest but the noisiest (CV 6.8%)

ml_ifchain_u8 wins on median (4.75 us) yet has the highest variance (CV 6.8%), while ml_tree_u8 is the steadiest (CV 5.9%, 15.11 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ml_ifchain_u8, ml_jumptable_u8) are a dead heat (<1%)

ml_ifchain_u8 (4.75 us) and ml_jumptable_u8 (4.75 us) differ by 0.05%, inside the noise, even though the wider field spreads 217.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 3.2x the fastest

Fastest ml_ifchain_u8 (4.75 us) to slowest ml_tree_u8 (15.11 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ml_ifchain_u8's edge over baseline is significant but tiny (-2 ns, 0.05%)

ml_ifchain_u8 differs from baseline ml_jumptable_u8 by -2 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ml_ifchain_u8** at 4752.1 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 3.18x (fastest 4752.1 ns, slowest 15106.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u8 | 7186ns | 7331ns | 6135ns | 7330ns | 7496ns | +0.83% |
| ml_jumptable_u8 | 7127ns | 7338ns | 6143ns | 7179ns | 7540ns | base |
| ml_tree_u8 | 17200ns | 17628ns | 15029ns | 17394ns | 17996ns | +141.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u8 | 4668ns | 3982ns | 4886ns | +1.21% | 0.014 |
| ml_jumptable_u8 | 4612ns | 3986ns | 4865ns | base | 0.014 |
| ml_tree_u8 | 14731ns | 12872ns | 15403ns | +219.39% | 0.004 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_ifchain_u8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u8 | 0.013 | 83.8% |
| ml_jumptable_u8 | 0.013 | 83.8% |
| ml_tree_u8 | 0.004 | 26.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u8 | 7186ns | 7186ns | +0.83% |
| ml_jumptable_u8 | 7127ns | 7127ns | base |
| ml_tree_u8 | 17200ns | 17200ns | +141.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u8 | 4754ns | base | --- | [4217, 4865] | --- | --- | --- | --- |
| ml_ifchain_u8 | 4752ns | no significant difference | [-113, +283]ns | [4366, 4886] | no | 0.6875 | 0.6875 | 0 |
| ml_tree_u8 | 15107ns | +10403.4ns (+218.8%) | [+9304, +10648]ns | [13682, 15403] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u8 | ml_ifchain_u8 | ml_tree_u8 |
|---|---|---|---|
| 1 | 3986ns | -0.1% | +222.9% |
| 2 | 4448ns | +6.8% | +234.3% |
| 3 | 4754ns | +5.6% | +222.8% |
| 4 | 4755ns | -0.0% | +225.1% |
| 5 | 4770ns | -0.3% | +203.8% |
| 6 | 4960ns | -4.2% | +209.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u8 | 0.028 | ok |
| ml_jumptable_u8 | 0.293 | moderate+ |
| ml_tree_u8 | -0.009 | ok |

**Consistency summary:**

- **ml_ifchain_u8**: won 2/6, lost 2/6
- **ml_tree_u8**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u8 | 8.1ns | 4668.0ns | 0.2% |  |
| ml_jumptable_u8 | 3.2ns | 4612.2ns | 0.1% |  |
| ml_tree_u8 | 2.7ns | 14730.8ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u8 (n=6, range 3982.5-4886.2 ns)
   3982.5 |####################
   4027.7 |
   4072.9 |
   4118.1 |
   4163.2 |
   4208.4 |
   4253.6 |
   4298.8 |
   4344.0 |
   4389.2 |
   4434.4 |
   4479.6 |
   4524.8 |
   4569.9 |
   4615.1 |
   4660.3 |
   4705.5 |########################################
   4750.7 |########################################
   4795.9 |
   4841.1 |
  (0 below, 1 above range)

ml_jumptable_u8 (n=6, range 3986.2-4865.4 ns)
   3986.2 |#############
   4030.2 |
   4074.1 |
   4118.1 |
   4162.0 |
   4206.0 |
   4250.0 |
   4293.9 |
   4337.9 |
   4381.8 |
   4425.8 |#############
   4469.8 |
   4513.7 |
   4557.7 |
   4601.6 |
   4645.6 |
   4689.6 |
   4733.5 |########################################
   4777.5 |
   4821.4 |
  (0 below, 1 above range)

ml_tree_u8 (n=6, range 12872.5-15403.1 ns)
  12872.5 |####################
  12999.0 |
  13125.6 |
  13252.1 |
  13378.6 |
  13505.1 |
  13631.7 |
  13758.2 |
  13884.7 |
  14011.3 |
  14137.8 |
  14264.3 |
  14390.9 |####################
  14517.4 |
  14643.9 |
  14770.4 |####################
  14897.0 |
  15023.5 |
  15150.0 |
  15276.6 |########################################
  (0 below, 1 above range)

```
