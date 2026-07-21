# Match lowering: if-chain vs jump-table vs decision-tree, K=64 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u64**

## Highlights

Baseline for all deltas below: **ml_jumptable_u64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_u64 is an outlier: 7.6x slower than the field

ml_tree_u64 (35.93 us) is 7.6x the fastest (4.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ml_ifchain_u64, ml_jumptable_u64) are a dead heat (<1%)

ml_ifchain_u64 (4.75 us) and ml_jumptable_u64 (4.75 us) differ by 0.14%, inside the noise, even though the wider field spreads 657.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 7.6x the fastest

Fastest ml_ifchain_u64 (4.75 us) to slowest ml_tree_u64 (35.93 us): 7.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ml_ifchain_u64's edge over baseline is significant but tiny (-5 ns, 0.11%)

ml_ifchain_u64 differs from baseline ml_jumptable_u64 by -5 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ml_ifchain_u64** at 4745.6 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 7.57x (fastest 4745.6 ns, slowest 35925.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u64 | 7262ns | 7334ns | 6860ns | 7327ns | 7366ns | -0.04% |
| ml_jumptable_u64 | 7265ns | 7341ns | 6863ns | 7337ns | 7358ns | base |
| ml_tree_u64 | 38347ns | 38554ns | 36852ns | 38406ns | 39006ns | +427.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u64 | 4698ns | 4437ns | 4758ns | -0.11% | 0.014 |
| ml_jumptable_u64 | 4703ns | 4444ns | 4760ns | base | 0.014 |
| ml_tree_u64 | 35774ns | 34398ns | 36409ns | +660.70% | 0.002 |

## Performance model

- Peak throughput: **0.014 Gops/s** (ml_ifchain_u64; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u64 | 0.013 | 93.5% |
| ml_jumptable_u64 | 0.013 | 93.4% |
| ml_tree_u64 | 0.002 | 12.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u64 | 7262ns | 7262ns | -0.04% |
| ml_jumptable_u64 | 7265ns | 7265ns | base |
| ml_tree_u64 | 38347ns | 38347ns | +427.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u64 | 4752ns | base | --- | [4596, 4760] | --- | --- | --- | --- |
| ml_ifchain_u64 | 4746ns | no significant difference | [-161, +151]ns | [4589, 4758] | no | 0.2188 | 0.2188 | 0 |
| ml_tree_u64 | 35925ns | +31327.7ns (+659.2%) | [+30235, +31651]ns | [34988, 36409] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u64 | ml_ifchain_u64 | ml_tree_u64 |
|---|---|---|---|
| 1 | 4752ns | -6.6% | +662.1% |
| 2 | 4444ns | +6.8% | +702.0% |
| 3 | 4753ns | -0.1% | +623.7% |
| 4 | 4752ns | -0.2% | +648.6% |
| 5 | 4767ns | -0.0% | +666.6% |
| 6 | 4749ns | -0.1% | +663.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u64 | -0.018 | ok |
| ml_jumptable_u64 | -0.211 | moderate- |
| ml_tree_u64 | 0.211 | moderate+ |

**Consistency summary:**

- **ml_ifchain_u64**: won 3/6, lost 1/6
- **ml_tree_u64**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u64 | 3.2ns | 4697.5ns | 0.1% |  |
| ml_jumptable_u64 | 2.8ns | 4702.8ns | 0.1% |  |
| ml_tree_u64 | 3.0ns | 35774.0ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u64 (n=6, range 4436.7-4757.5 ns)
   4436.7 |##########
   4452.7 |
   4468.8 |
   4484.8 |
   4500.9 |
   4516.9 |
   4532.9 |
   4549.0 |
   4565.0 |
   4581.1 |
   4597.1 |
   4613.1 |
   4629.2 |
   4645.2 |
   4661.3 |
   4677.3 |
   4693.3 |
   4709.4 |
   4725.4 |
   4741.5 |########################################
  (0 below, 1 above range)

ml_jumptable_u64 (n=6, range 4443.8-4760.0 ns)
   4443.8 |##########
   4459.6 |
   4475.4 |
   4491.2 |
   4507.0 |
   4522.9 |
   4538.7 |
   4554.5 |
   4570.3 |
   4586.1 |
   4601.9 |
   4617.7 |
   4633.5 |
   4649.3 |
   4665.1 |
   4680.9 |
   4696.8 |
   4712.6 |
   4728.4 |
   4744.2 |########################################
  (0 below, 1 above range)

ml_tree_u64 (n=6, range 34397.5-36408.9 ns)
  34397.5 |####################
  34498.1 |
  34598.6 |
  34699.2 |
  34799.8 |
  34900.4 |
  35000.9 |
  35101.5 |
  35202.1 |
  35302.7 |
  35403.2 |
  35503.8 |####################
  35604.4 |####################
  35704.9 |
  35805.5 |
  35906.1 |
  36006.7 |
  36107.2 |
  36207.8 |########################################
  36308.4 |
  (0 below, 1 above range)

```
