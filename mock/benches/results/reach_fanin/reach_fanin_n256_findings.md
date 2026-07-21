# Reachability fixpoint: whole-column vs real semi-naive (fanin)

2 variants, 6 samples per variant.
Baseline: **r_fanin_whole**

## Highlights

Baseline for all deltas below: **r_fanin_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_fanin_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_fanin_whole has the worst median (4.97 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_fanin_semi at 3.63 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_fanin_semi dominates: 37% faster than the next best (r_fanin_whole)

r_fanin_semi (3.63 us) leads r_fanin_whole (4.97 us) by 37%, a clear separation rather than a photo finish. CV 8.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_fanin_semi beats baseline by 28% (significant)

r_fanin_semi is -1.40 us (28%) faster than baseline r_fanin_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: r_fanin_semi** at 3627.3 ns median (-27.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.37x (fastest 3627.3 ns, slowest 4974.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_fanin_semi | 6077ns | 5829ns | 5672ns | 5791ns | 6710ns | -20.43% |
| r_fanin_whole | 7638ns | 7230ns | 7132ns | 7205ns | 8541ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_fanin_semi | 3776ns | 3540ns | 4154ns | -27.95% | 0.068 |
| r_fanin_whole | 5241ns | 4886ns | 5851ns | base | 0.049 |

## Performance model

- Peak throughput: **0.072 Gops/s** (r_fanin_semi; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_fanin_semi | 0.071 | 97.6% |
| r_fanin_whole | 0.051 | 71.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_fanin_semi | 6077ns | 6077ns | -20.43% |
| r_fanin_whole | 7638ns | 7638ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_fanin_whole | 4975ns | base | --- | [4898, 5851] | --- | --- | --- | --- |
| r_fanin_semi | 3627ns | -1396.9ns (-28.1%) | [-1697, -1301]ns | [3548, 4154] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_fanin_whole | r_fanin_semi |
|---|---|---|
| 1 | 4970ns | -25.9% |
| 2 | 4980ns | -28.9% |
| 3 | 6000ns | -27.8% |
| 4 | 5703ns | -30.3% |
| 5 | 4886ns | -26.9% |
| 6 | 4909ns | -27.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_fanin_semi | 0.009 | ok |
| r_fanin_whole | 0.151 | ok |

**Consistency summary:**

- **r_fanin_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_fanin_semi | 202.3ns | 3776.4ns | 5.4% | HIGH |
| r_fanin_whole | 102.2ns | 5241.2ns | 2.0% |  |

## Distribution (algo ns)

```
r_fanin_semi (n=6, range 3539.6-4154.4 ns)
   3539.6 |########################################
   3570.3 |
   3601.1 |
   3631.8 |
   3662.6 |#############
   3693.3 |
   3724.0 |
   3754.8 |
   3785.5 |
   3816.3 |
   3847.0 |
   3877.7 |
   3908.5 |
   3939.2 |
   3970.0 |#############
   4000.7 |
   4031.4 |
   4062.2 |
   4092.9 |
   4123.7 |
  (0 below, 1 above range)

r_fanin_whole (n=6, range 4885.8-5851.2 ns)
   4885.8 |########################################
   4934.1 |########################################
   4982.3 |
   5030.6 |
   5078.9 |
   5127.2 |
   5175.4 |
   5223.7 |
   5272.0 |
   5320.3 |
   5368.5 |
   5416.8 |
   5465.1 |
   5513.3 |
   5561.6 |
   5609.9 |
   5658.2 |####################
   5706.4 |
   5754.7 |
   5803.0 |
  (0 below, 1 above range)

```
