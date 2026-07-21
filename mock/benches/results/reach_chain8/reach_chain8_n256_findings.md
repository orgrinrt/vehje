# Reachability fixpoint: whole-column vs real semi-naive (chain8)

2 variants, 6 samples per variant.
Baseline: **r_chain8_whole**

## Highlights

Baseline for all deltas below: **r_chain8_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain8_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain8_whole has the worst median (99.19 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain8_semi at 34.04 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain8_semi dominates: 191% faster than the next best (r_chain8_whole)

r_chain8_semi (34.04 us) leads r_chain8_whole (99.19 us) by 191%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain8_semi beats baseline by 65% (significant)

r_chain8_semi is -64.51 us (65%) faster than baseline r_chain8_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: r_chain8_semi** at 34036.1 ns median (-65.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.91x (fastest 34036.1 ns, slowest 99188.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain8_semi | 36244ns | 36589ns | 33887ns | 36189ns | 37504ns | -64.04% |
| r_chain8_whole | 100781ns | 101689ns | 94005ns | 99532ns | 106042ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain8_semi | 33738ns | 31601ns | 34896ns | -65.69% | 0.008 |
| r_chain8_whole | 98325ns | 91729ns | 103465ns | base | 0.003 |

## Performance model

- Peak throughput: **0.008 Gops/s** (r_chain8_semi; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain8_semi | 0.008 | 92.8% |
| r_chain8_whole | 0.003 | 31.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain8_semi | 36244ns | 36244ns | -64.04% |
| r_chain8_whole | 100781ns | 100781ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain8_whole | 99189ns | base | --- | [92320, 103465] | --- | --- | --- | --- |
| r_chain8_semi | 34036ns | -64511.7ns (-65.0%) | [-69210, -60038]ns | [32282, 34896] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain8_whole | r_chain8_semi |
|---|---|---|
| 1 | 92910ns | -66.0% |
| 2 | 91729ns | -64.1% |
| 3 | 99913ns | -65.3% |
| 4 | 98465ns | -64.8% |
| 5 | 106098ns | -66.9% |
| 6 | 100832ns | -66.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain8_semi | 0.292 | moderate+ |
| r_chain8_whole | 0.324 | moderate+ |

**Consistency summary:**

- **r_chain8_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain8_semi | 254.4ns | 33738.1ns | 0.8% |  |
| r_chain8_whole | 81.1ns | 98324.6ns | 0.1% |  |

## Distribution (algo ns)

```
r_chain8_semi (n=6, range 31601.2-34896.2 ns)
  31601.2 |####################
  31766.0 |
  31930.7 |
  32095.5 |
  32260.2 |
  32425.0 |
  32589.7 |
  32754.5 |
  32919.2 |####################
  33083.9 |
  33248.7 |
  33413.4 |####################
  33578.2 |
  33742.9 |
  33907.7 |
  34072.4 |
  34237.2 |
  34401.9 |
  34566.7 |########################################
  34731.4 |
  (0 below, 1 above range)

r_chain8_whole (n=6, range 91728.7-103465.4 ns)
  91728.7 |########################################
  92315.5 |
  92902.4 |########################################
  93489.2 |
  94076.0 |
  94662.9 |
  95249.7 |
  95836.5 |
  96423.4 |
  97010.2 |
  97597.0 |
  98183.9 |########################################
  98770.7 |
  99357.6 |########################################
  99944.4 |
  100531.2 |########################################
  101118.1 |
  101704.9 |
  102291.7 |
  102878.6 |
  (0 below, 1 above range)

```
