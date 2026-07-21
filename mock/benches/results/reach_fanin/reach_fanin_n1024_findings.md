# Reachability fixpoint: whole-column vs real semi-naive (fanin)

2 variants, 6 samples per variant.
Baseline: **r_fanin_whole**

## Highlights

Baseline for all deltas below: **r_fanin_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_fanin_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_fanin_whole has the worst median (23.99 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_fanin_semi at 11.33 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_fanin_semi dominates: 112% faster than the next best (r_fanin_whole)

r_fanin_semi (11.33 us) leads r_fanin_whole (23.99 us) by 112%, a clear separation rather than a photo finish. CV 94.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_fanin_semi beats baseline by 50% (significant)

r_fanin_semi is -11.97 us (50%) faster than baseline r_fanin_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_fanin_whole is inconsistent: worst-20% is 9.3x its best-20%

r_fanin_whole's best 20% of batches run at 17.70 us but its worst 20% at 165.28 us (9.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: r_fanin_semi** at 11333.5 ns median (-52.8% vs baseline)
- Spread: 2.12x (fastest 11333.5 ns, slowest 23993.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_fanin_semi | 18863ns | 13603ns | 12653ns | 13380ns | 30191ns | -74.39% |
| r_fanin_whole | 73660ns | 30386ns | 19874ns | 28533ns | 168244ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_fanin_semi | 16003ns | 10441ns | 26116ns | -77.06% | 0.064 |
| r_fanin_whole | 69763ns | 17705ns | 165282ns | base | 0.015 |

## Performance model

- Peak throughput: **0.098 Gops/s** (r_fanin_semi; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_fanin_semi | 0.090 | 92.1% |
| r_fanin_whole | 0.043 | 43.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_fanin_semi | 18863ns | 18863ns | -74.39% |
| r_fanin_whole | 73660ns | 73660ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_fanin_whole | 23993ns | base | --- | [20015, 165282] | --- | --- | --- | --- |
| r_fanin_semi | 11334ns | no significant difference | [-154722, +5407]ns | [10560, 26116] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_fanin_whole | r_fanin_semi |
|---|---|---|
| 1 | 17705ns | -34.5% |
| 2 | 292329ns | -96.4% |
| 3 | 38235ns | -72.1% |
| 4 | 22325ns | -45.2% |
| 5 | 24919ns | -55.6% |
| 6 | 23067ns | +73.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_fanin_semi | -0.037 | ok |
| r_fanin_whole | -0.216 | moderate- |

**Consistency summary:**

- **r_fanin_semi**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_fanin_semi | 318.1ns | 16003.1ns | 2.0% |  |
| r_fanin_whole | 299.3ns | 69763.2ns | 0.4% |  |

## Distribution (algo ns)

```
r_fanin_semi (n=6, range 10441.2-26115.8 ns)
  10441.2 |########################################
  11224.9 |#############
  12008.7 |#############
  12792.4 |
  13576.1 |
  14359.9 |
  15143.6 |
  15927.3 |
  16711.1 |
  17494.8 |
  18278.5 |
  19062.3 |
  19846.0 |
  20629.7 |
  21413.5 |
  22197.2 |
  22980.9 |
  23764.7 |
  24548.4 |
  25332.1 |
  (0 below, 1 above range)

r_fanin_whole (n=6, range 17704.6-165282.1 ns)
  17704.6 |########################################
  25083.5 |
  32462.3 |##########
  39841.2 |
  47220.1 |
  54599.0 |
  61977.8 |
  69356.7 |
  76735.6 |
  84114.5 |
  91493.4 |
  98872.2 |
  106251.1 |
  113630.0 |
  121008.9 |
  128387.7 |
  135766.6 |
  143145.5 |
  150524.4 |
  157903.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **r_fanin_semi**: CV=67.1% (high variance, measurements may be unstable)
- **r_fanin_whole**: CV=143.0% (high variance, measurements may be unstable)
- **r_fanin_whole**: worst_20/best_20 = 9.3x (possible bimodal distribution)
