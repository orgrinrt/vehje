# Match lowering: hot-first if-chain vs the rest, K=64 arms, 90% hit one arm

4 variants, 6 samples per variant.
Baseline: **ml_jumptable_h64**

## Highlights

Baseline for all deltas below: **ml_jumptable_h64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_h64 is an outlier: 7.6x slower than the field

ml_tree_h64 (482.20 us) is 7.6x the fastest (63.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (ml_jumptable_h64)

The baseline ml_jumptable_h64 is the fastest (63.84 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ml_jumptable_h64, ml_hotfirst_h64, ml_ifchain_h64} vs {ml_tree_h64} (638% apart)

The field splits into a fast tier {ml_jumptable_h64, ml_hotfirst_h64, ml_ifchain_h64} and a slow tier {ml_tree_h64} with a 638% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.6x the fastest

Fastest ml_jumptable_h64 (63.84 us) to slowest ml_tree_h64 (482.20 us): 7.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ml_jumptable_h64) is the fastest** at 63839.2 ns median
- 1 variant significantly slower than baseline
- Spread: 7.55x (fastest 63839.2 ns, slowest 482203.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_hotfirst_h64 | 67904ns | 67355ns | 65047ns | 66891ns | 70851ns | +1.74% |
| ml_ifchain_h64 | 67365ns | 67556ns | 66091ns | 67099ns | 68400ns | +0.93% |
| ml_jumptable_h64 | 66745ns | 66025ns | 64927ns | 65977ns | 68805ns | base |
| ml_tree_h64 | 488631ns | 485147ns | 481550ns | 484651ns | 498141ns | +632.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_hotfirst_h64 | 65601ns | 62871ns | 68461ns | +1.80% | 0.016 |
| ml_ifchain_h64 | 65041ns | 63878ns | 65908ns | +0.93% | 0.016 |
| ml_jumptable_h64 | 64439ns | 62691ns | 66295ns | base | 0.016 |
| ml_tree_h64 | 486014ns | 478672ns | 495777ns | +654.23% | 0.002 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_jumptable_h64; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_hotfirst_h64 | 0.016 | 96.4% |
| ml_ifchain_h64 | 0.016 | 96.0% |
| ml_jumptable_h64 | 0.016 | 98.2% |
| ml_tree_h64 | 0.002 | 13.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_hotfirst_h64 | 67904ns | 67904ns | +1.74% |
| ml_ifchain_h64 | 67365ns | 67365ns | +0.93% |
| ml_jumptable_h64 | 66745ns | 66745ns | base |
| ml_tree_h64 | 488631ns | 488631ns | +632.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_h64 | 63839ns | base | --- | [63182, 66295] | --- | --- | --- | --- |
| ml_hotfirst_h64 | 65051ns | no significant difference | [-493, +3644]ns | [63290, 68461] | no | 0.6875 | 0.6875 | 0 |
| ml_ifchain_h64 | 65326ns | no significant difference | [-389, +2090]ns | [63889, 65908] | no | 0.6875 | 0.6875 | 0 |
| ml_tree_h64 | 482203ns | +419020.8ns (+656.4%) | [+413767, +431938]ns | [480062, 495777] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_h64 | ml_hotfirst_h64 | ml_ifchain_h64 | ml_tree_h64 |
|---|---|---|---|---|
| 1 | 63786ns | +0.6% | +1.5% | +675.5% |
| 2 | 62691ns | +5.2% | +5.1% | +668.4% |
| 3 | 66278ns | +6.1% | -0.6% | +626.4% |
| 4 | 66311ns | +0.5% | -0.6% | +621.9% |
| 5 | 63892ns | -0.3% | +0.0% | +677.7% |
| 6 | 63674ns | -1.3% | +0.3% | +658.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_hotfirst_h64 | 0.252 | moderate+ |
| ml_ifchain_h64 | 0.320 | moderate+ |
| ml_jumptable_h64 | 0.068 | ok |
| ml_tree_h64 | -0.336 | moderate- |

**Consistency summary:**

- **ml_hotfirst_h64**: won 2/6, lost 4/6
- **ml_ifchain_h64**: won 2/6, lost 3/6
- **ml_tree_h64**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_hotfirst_h64 | 3.3ns | 65600.9ns | 0.0% |  |
| ml_ifchain_h64 | 3.4ns | 65040.7ns | 0.0% |  |
| ml_jumptable_h64 | 3.4ns | 64438.7ns | 0.0% |  |
| ml_tree_h64 | 14.3ns | 486014.0ns | 0.0% |  |

## Distribution (algo ns)

```
ml_hotfirst_h64 (n=6, range 62870.8-68461.0 ns)
  62870.8 |########################################
  63150.3 |
  63429.8 |
  63709.3 |########################################
  63988.8 |########################################
  64268.4 |
  64547.9 |
  64827.4 |
  65106.9 |
  65386.4 |
  65665.9 |
  65945.4 |########################################
  66224.9 |
  66504.4 |########################################
  66783.9 |
  67063.4 |
  67343.0 |
  67622.5 |
  67902.0 |
  68181.5 |
  (0 below, 1 above range)

ml_ifchain_h64 (n=6, range 63877.5-65907.5 ns)
  63877.5 |########################################
  63979.0 |
  64080.5 |
  64182.0 |
  64283.5 |
  64385.0 |
  64486.5 |
  64588.0 |
  64689.5 |####################
  64791.0 |
  64892.5 |
  64994.0 |
  65095.5 |
  65197.0 |
  65298.5 |
  65400.0 |
  65501.5 |
  65603.0 |
  65704.5 |
  65806.0 |########################################
  (0 below, 1 above range)

ml_jumptable_h64 (n=6, range 62691.2-66294.6 ns)
  62691.2 |####################
  62871.4 |
  63051.5 |
  63231.7 |
  63411.9 |
  63592.0 |####################
  63772.2 |########################################
  63952.4 |
  64132.5 |
  64312.7 |
  64492.9 |
  64673.0 |
  64853.2 |
  65033.4 |
  65213.5 |
  65393.7 |
  65573.9 |
  65754.0 |
  65934.2 |
  66114.4 |####################
  (0 below, 1 above range)

ml_tree_h64 (n=6, range 478672.5-495776.9 ns)
  478672.5 |####################
  479527.7 |
  480382.9 |
  481238.2 |########################################
  482093.4 |####################
  482948.6 |
  483803.8 |
  484659.0 |
  485514.3 |
  486369.5 |
  487224.7 |
  488079.9 |
  488935.1 |
  489790.4 |
  490645.6 |
  491500.8 |
  492356.0 |
  493211.2 |
  494066.5 |####################
  494921.7 |
  (0 below, 1 above range)

```
