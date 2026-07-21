# Reachability fixpoint: whole-column vs real semi-naive (chain128)

2 variants, 6 samples per variant.
Baseline: **r_chain128_whole**

## Highlights

Baseline for all deltas below: **r_chain128_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain128_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain128_whole has the worst median (47.90 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain128_semi at 19.79 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain128_semi dominates: 142% faster than the next best (r_chain128_whole)

r_chain128_semi (19.79 us) leads r_chain128_whole (47.90 us) by 142%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain128_semi beats baseline by 59% (significant)

r_chain128_semi is -28.46 us (59%) faster than baseline r_chain128_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: r_chain128_semi** at 19786.7 ns median (-58.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.42x (fastest 19786.7 ns, slowest 47898.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain128_semi | 21924ns | 22049ns | 21241ns | 21876ns | 22336ns | -56.33% |
| r_chain128_whole | 50200ns | 50118ns | 48928ns | 49767ns | 51485ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain128_semi | 19675ns | 19077ns | 20031ns | -58.98% | 0.003 |
| r_chain128_whole | 47964ns | 46748ns | 49176ns | base | 0.001 |

## Performance model

- Peak throughput: **0.003 Gops/s** (r_chain128_semi; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain128_semi | 0.003 | 96.4% |
| r_chain128_whole | 0.001 | 39.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain128_semi | 21924ns | 21924ns | -56.33% |
| r_chain128_whole | 50200ns | 50200ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain128_whole | 47898ns | base | --- | [46818, 49176] | --- | --- | --- | --- |
| r_chain128_semi | 19787ns | -28462.3ns (-59.4%) | [-29145, -27261]ns | [19206, 20031] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain128_whole | r_chain128_semi |
|---|---|---|
| 1 | 46748ns | -58.6% |
| 2 | 49262ns | -59.8% |
| 3 | 49091ns | -58.7% |
| 4 | 46887ns | -57.8% |
| 5 | 47413ns | -59.8% |
| 6 | 48383ns | -59.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain128_semi | -0.049 | ok |
| r_chain128_whole | -0.159 | ok |

**Consistency summary:**

- **r_chain128_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain128_semi | 173.2ns | 19674.5ns | 0.9% |  |
| r_chain128_whole | 55.7ns | 47964.1ns | 0.1% |  |

## Distribution (algo ns)

```
r_chain128_semi (n=6, range 19077.1-20031.2 ns)
  19077.1 |####################
  19124.8 |
  19172.5 |
  19220.2 |
  19267.9 |
  19315.6 |####################
  19363.3 |
  19411.1 |
  19458.8 |
  19506.5 |
  19554.2 |
  19601.9 |
  19649.6 |
  19697.3 |
  19745.0 |####################
  19792.7 |########################################
  19840.4 |
  19888.1 |
  19935.8 |
  19983.5 |
  (0 below, 1 above range)

r_chain128_whole (n=6, range 46747.9-49176.4 ns)
  46747.9 |########################################
  46869.3 |########################################
  46990.8 |
  47112.2 |
  47233.6 |
  47355.0 |########################################
  47476.5 |
  47597.9 |
  47719.3 |
  47840.7 |
  47962.2 |
  48083.6 |
  48205.0 |
  48326.5 |########################################
  48447.9 |
  48569.3 |
  48690.7 |
  48812.2 |
  48933.6 |
  49055.0 |########################################
  (0 below, 1 above range)

```
