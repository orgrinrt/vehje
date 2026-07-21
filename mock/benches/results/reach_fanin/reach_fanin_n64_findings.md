# Reachability fixpoint: whole-column vs real semi-naive (fanin)

2 variants, 6 samples per variant.
Baseline: **r_fanin_whole**

## Highlights

Baseline for all deltas below: **r_fanin_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### r_fanin_whole dominates: 25% faster than the next best (r_fanin_semi)

r_fanin_whole (2.55 us) leads r_fanin_semi (3.19 us) by 25%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (r_fanin_whole)

The baseline r_fanin_whole is the fastest (2.55 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (r_fanin_whole) is the fastest** at 2554.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.25x (fastest 2554.4 ns, slowest 3193.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_fanin_semi | 5426ns | 5453ns | 5270ns | 5445ns | 5476ns | +12.77% |
| r_fanin_whole | 4812ns | 4803ns | 4707ns | 4771ns | 4925ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_fanin_semi | 3190ns | 3110ns | 3224ns | +24.98% | 0.020 |
| r_fanin_whole | 2552ns | 2498ns | 2600ns | base | 0.025 |

## Performance model

- Peak throughput: **0.026 Gops/s** (r_fanin_whole; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_fanin_semi | 0.020 | 78.2% |
| r_fanin_whole | 0.025 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_fanin_semi | 5426ns | 5426ns | +12.77% |
| r_fanin_whole | 4812ns | 4812ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_fanin_whole | 2554ns | base | --- | [2501, 2600] | --- | --- | --- | --- |
| r_fanin_semi | 3194ns | +622.9ns (+24.4%) | [+594, +696]ns | [3150, 3224] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_fanin_whole | r_fanin_semi |
|---|---|---|
| 1 | 2527ns | +26.2% |
| 2 | 2608ns | +22.3% |
| 3 | 2582ns | +24.8% |
| 4 | 2592ns | +23.3% |
| 5 | 2498ns | +29.2% |
| 6 | 2505ns | +24.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_fanin_semi | -0.272 | moderate- |
| r_fanin_whole | 0.164 | ok |

**Consistency summary:**

- **r_fanin_semi**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_fanin_semi | 151.6ns | 3189.5ns | 4.8% |  |
| r_fanin_whole | 46.3ns | 2552.0ns | 1.8% |  |

## Distribution (algo ns)

```
r_fanin_semi (n=6, range 3110.0-3224.4 ns)
   3110.0 |####################
   3115.7 |
   3121.4 |
   3127.2 |
   3132.9 |
   3138.6 |
   3144.3 |
   3150.0 |
   3155.8 |
   3161.5 |
   3167.2 |
   3172.9 |
   3178.6 |
   3184.4 |
   3190.1 |########################################
   3195.8 |####################
   3201.5 |
   3207.2 |
   3213.0 |
   3218.7 |####################
  (0 below, 1 above range)

r_fanin_whole (n=6, range 2498.3-2600.0 ns)
   2498.3 |########################################
   2503.4 |########################################
   2508.5 |
   2513.6 |
   2518.6 |
   2523.7 |########################################
   2528.8 |
   2533.9 |
   2539.0 |
   2544.1 |
   2549.2 |
   2554.2 |
   2559.3 |
   2564.4 |
   2569.5 |
   2574.6 |
   2579.7 |########################################
   2584.7 |
   2589.8 |########################################
   2594.9 |
  (0 below, 1 above range)

```
