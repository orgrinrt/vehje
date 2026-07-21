# Reachability fixpoint: whole-column vs real semi-naive (fanin)

2 variants, 6 samples per variant.
Baseline: **r_fanin_whole**

## Highlights

Baseline for all deltas below: **r_fanin_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_fanin_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_fanin_whole has the worst median (75.56 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_fanin_semi at 44.70 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_fanin_semi dominates: 69% faster than the next best (r_fanin_whole)

r_fanin_semi (44.70 us) leads r_fanin_whole (75.56 us) by 69%, a clear separation rather than a photo finish. CV 47.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_fanin_semi beats baseline by 42% (significant)

r_fanin_semi is -31.38 us (42%) faster than baseline r_fanin_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_fanin_semi is fastest but the noisiest (CV 47.1%)

r_fanin_semi wins on median (44.70 us) yet has the highest variance (CV 47.1%), while r_fanin_whole is the steadiest (CV 10.9%, 75.56 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### r_fanin_semi is inconsistent: worst-20% is 1.8x its best-20%

r_fanin_semi's best 20% of batches run at 41.61 us but its worst 20% at 73.33 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: r_fanin_semi** at 44695.2 ns median (-40.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.69x (fastest 44695.2 ns, slowest 75558.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_fanin_semi | 55988ns | 47332ns | 43869ns | 46262ns | 76637ns | -32.07% |
| r_fanin_whole | 82418ns | 77901ns | 75979ns | 77320ns | 93285ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_fanin_semi | 53256ns | 41612ns | 73332ns | -33.40% | 0.077 |
| r_fanin_whole | 79958ns | 73706ns | 90546ns | base | 0.051 |

## Performance model

- Peak throughput: **0.098 Gops/s** (r_fanin_semi; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_fanin_semi | 0.092 | 93.1% |
| r_fanin_whole | 0.054 | 55.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_fanin_semi | 55988ns | 55988ns | -32.07% |
| r_fanin_whole | 82418ns | 82418ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_fanin_whole | 75559ns | base | --- | [73767, 90546] | --- | --- | --- | --- |
| r_fanin_semi | 44695ns | -31376.5ns (-41.5%) | [-46481, -2248]ns | [41739, 73332] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_fanin_whole | r_fanin_semi |
|---|---|---|
| 1 | 73706ns | -41.8% |
| 2 | 96197ns | -56.7% |
| 3 | 75446ns | -38.4% |
| 4 | 73829ns | -43.3% |
| 5 | 84896ns | -45.2% |
| 6 | 75672ns | +32.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_fanin_semi | 0.014 | ok |
| r_fanin_whole | -0.492 | moderate- |

**Consistency summary:**

- **r_fanin_semi**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_fanin_semi | 823.7ns | 53255.7ns | 1.5% |  |
| r_fanin_whole | 535.2ns | 79957.6ns | 0.7% |  |

## Distribution (algo ns)

```
r_fanin_semi (n=6, range 41611.7-73332.5 ns)
  41611.7 |########################################
  43197.7 |
  44783.8 |
  46369.8 |##########################
  47955.9 |
  49541.9 |
  51127.9 |
  52714.0 |
  54300.0 |
  55886.1 |
  57472.1 |
  59058.1 |
  60644.2 |
  62230.2 |
  63816.3 |
  65402.3 |
  66988.3 |
  68574.4 |
  70160.4 |
  71746.5 |
  (0 below, 1 above range)

r_fanin_whole (n=6, range 73705.8-90546.5 ns)
  73705.8 |########################################
  74547.8 |
  75389.9 |########################################
  76231.9 |
  77073.9 |
  77916.0 |
  78758.0 |
  79600.0 |
  80442.1 |
  81284.1 |
  82126.1 |
  82968.2 |
  83810.2 |
  84652.2 |####################
  85494.3 |
  86336.3 |
  87178.3 |
  88020.4 |
  88862.4 |
  89704.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **r_fanin_semi**: CV=39.6% (high variance, measurements may be unstable)
