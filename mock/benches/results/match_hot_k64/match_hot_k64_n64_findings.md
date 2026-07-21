# Match lowering: hot-first if-chain vs the rest, K=64 arms, 90% hit one arm

4 variants, 6 samples per variant.
Baseline: **ml_jumptable_h64**

## Highlights

Baseline for all deltas below: **ml_jumptable_h64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_h64 is an outlier: 7.7x slower than the field

ml_tree_h64 (34.30 us) is 7.7x the fastest (4.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {ml_hotfirst_h64, ml_ifchain_h64, ml_jumptable_h64} vs {ml_tree_h64} (614% apart)

The field splits into a fast tier {ml_hotfirst_h64, ml_ifchain_h64, ml_jumptable_h64} and a slow tier {ml_tree_h64} with a 614% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.7x the fastest

Fastest ml_hotfirst_h64 (4.48 us) to slowest ml_tree_h64 (34.30 us): 7.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ml_hotfirst_h64** at 4480.8 ns median (-6.7% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.65x (fastest 4480.8 ns, slowest 34299.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_hotfirst_h64 | 6831ns | 6904ns | 6144ns | 6728ns | 7328ns | -5.77% |
| ml_ifchain_h64 | 6859ns | 7090ns | 6138ns | 6774ns | 7348ns | -5.37% |
| ml_jumptable_h64 | 7249ns | 7527ns | 6149ns | 7217ns | 7846ns | base |
| ml_tree_h64 | 37078ns | 36866ns | 32932ns | 35815ns | 41044ns | +411.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_hotfirst_h64 | 4430ns | 3982ns | 4753ns | -5.45% | 0.014 |
| ml_ifchain_h64 | 4441ns | 3977ns | 4752ns | -5.22% | 0.014 |
| ml_jumptable_h64 | 4685ns | 3985ns | 5089ns | base | 0.014 |
| ml_tree_h64 | 34566ns | 30720ns | 38305ns | +637.73% | 0.002 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_ifchain_h64; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_hotfirst_h64 | 0.014 | 88.8% |
| ml_ifchain_h64 | 0.014 | 86.6% |
| ml_jumptable_h64 | 0.013 | 82.8% |
| ml_tree_h64 | 0.002 | 11.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_hotfirst_h64 | 6831ns | 6831ns | -5.77% |
| ml_ifchain_h64 | 6859ns | 6859ns | -5.37% |
| ml_jumptable_h64 | 7249ns | 7249ns | base |
| ml_tree_h64 | 37078ns | 37078ns | +411.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_h64 | 4804ns | base | --- | [4164, 5089] | --- | --- | --- | --- |
| ml_hotfirst_h64 | 4481ns | -320.2ns (-6.7%) | [-386, -61]ns | [4056, 4753] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| ml_ifchain_h64 | 4592ns | -232.0ns (-4.8%) | [-494, -8]ns | [3978, 4752] | YES | 0.0469 | 0.0313 | 0 |
| ml_tree_h64 | 34300ns | +29364.2ns (+611.3%) | [+26929, +33348]ns | [31093, 38305] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_h64 | ml_hotfirst_h64 | ml_ifchain_h64 | ml_tree_h64 |
|---|---|---|---|---|
| 1 | 5159ns | -7.9% | -7.9% | +708.7% |
| 2 | 4852ns | -7.5% | -2.0% | +598.6% |
| 3 | 3985ns | +3.6% | -0.1% | +689.7% |
| 4 | 4342ns | -8.3% | -8.4% | +607.4% |
| 5 | 5018ns | -5.3% | -11.5% | +591.5% |
| 6 | 4755ns | -5.9% | -0.2% | +633.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_hotfirst_h64 | 0.013 | ok |
| ml_ifchain_h64 | 0.233 | moderate+ |
| ml_jumptable_h64 | 0.114 | ok |
| ml_tree_h64 | 0.115 | ok |

**Consistency summary:**

- **ml_hotfirst_h64**: won 5/6, lost 1/6
- **ml_ifchain_h64**: won 6/6, lost 0/6
- **ml_tree_h64**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_hotfirst_h64 | 3.0ns | 4429.9ns | 0.1% |  |
| ml_ifchain_h64 | 3.6ns | 4440.7ns | 0.1% |  |
| ml_jumptable_h64 | 3.7ns | 4685.4ns | 0.1% |  |
| ml_tree_h64 | 3.9ns | 34565.8ns | 0.0% |  |

## Distribution (algo ns)

```
ml_hotfirst_h64 (n=6, range 3982.1-4752.7 ns)
   3982.1 |########################################
   4020.6 |
   4059.2 |
   4097.7 |########################################
   4136.2 |
   4174.8 |
   4213.3 |
   4251.8 |
   4290.3 |
   4328.9 |
   4367.4 |
   4405.9 |
   4444.5 |########################################
   4483.0 |########################################
   4521.5 |
   4560.1 |
   4598.6 |
   4637.1 |
   4675.6 |
   4714.2 |########################################
  (0 below, 1 above range)

ml_ifchain_h64 (n=6, range 3977.1-4751.5 ns)
   3977.1 |########################################
   4015.8 |
   4054.5 |
   4093.3 |
   4132.0 |
   4170.7 |
   4209.4 |
   4248.1 |
   4286.9 |
   4325.6 |
   4364.3 |
   4403.0 |####################
   4441.7 |
   4480.5 |
   4519.2 |
   4557.9 |
   4596.6 |
   4635.3 |
   4674.1 |
   4712.8 |########################################
  (0 below, 1 above range)

ml_jumptable_h64 (n=6, range 3984.6-5088.8 ns)
   3984.6 |########################################
   4039.8 |
   4095.0 |
   4150.2 |
   4205.4 |
   4260.6 |
   4315.8 |########################################
   4371.1 |
   4426.3 |
   4481.5 |
   4536.7 |
   4591.9 |
   4647.1 |
   4702.3 |########################################
   4757.5 |
   4812.7 |########################################
   4867.9 |
   4923.1 |
   4978.3 |########################################
   5033.5 |
  (0 below, 1 above range)

ml_tree_h64 (n=6, range 30720.4-38304.8 ns)
  30720.4 |####################
  31099.6 |####################
  31478.8 |
  31858.1 |
  32237.3 |
  32616.5 |
  32995.7 |
  33374.9 |
  33754.2 |####################
  34133.4 |
  34512.6 |########################################
  34891.8 |
  35271.0 |
  35650.3 |
  36029.5 |
  36408.7 |
  36787.9 |
  37167.1 |
  37546.4 |
  37925.6 |
  (0 below, 1 above range)

```
