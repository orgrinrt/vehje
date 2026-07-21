# Reachability fixpoint: whole-column vs real semi-naive (chain8)

2 variants, 6 samples per variant.
Baseline: **r_chain8_whole**

## Highlights

Baseline for all deltas below: **r_chain8_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain8_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain8_whole has the worst median (1.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain8_semi at 410.66 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain8_semi dominates: 255% faster than the next best (r_chain8_whole)

r_chain8_semi (410.66 us) leads r_chain8_whole (1.46 ms) by 255%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain8_semi beats baseline by 72% (significant)

r_chain8_semi is -1.05 ms (72%) faster than baseline r_chain8_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 3.6x the fastest

Fastest r_chain8_semi (410.66 us) to slowest r_chain8_whole (1.46 ms): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_chain8_semi** at 410662.7 ns median (-71.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.55x (fastest 410662.7 ns, slowest 1459761.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain8_semi | 413640ns | 412936ns | 396532ns | 410127ns | 427465ns | -71.80% |
| r_chain8_whole | 1466771ns | 1463147ns | 1458158ns | 1462281ns | 1477813ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain8_semi | 411347ns | 394198ns | 425160ns | -71.89% | 0.010 |
| r_chain8_whole | 1463515ns | 1454856ns | 1474945ns | base | 0.003 |

## Performance model

- Peak throughput: **0.010 Gops/s** (r_chain8_semi; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain8_semi | 0.010 | 96.0% |
| r_chain8_whole | 0.003 | 27.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain8_semi | 413640ns | 413640ns | -71.80% |
| r_chain8_whole | 1466771ns | 1466771ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain8_whole | 1459761ns | base | --- | [1455838, 1474945] | --- | --- | --- | --- |
| r_chain8_semi | 410663ns | -1051092.5ns (-72.0%) | [-1070956, -1034455]ns | [398218, 425160] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain8_whole | r_chain8_semi |
|---|---|---|
| 1 | 1457583ns | -70.6% |
| 2 | 1485996ns | -72.8% |
| 3 | 1461939ns | -72.5% |
| 4 | 1463893ns | -71.2% |
| 5 | 1454856ns | -72.9% |
| 6 | 1456820ns | -71.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain8_semi | -0.478 | moderate- |
| r_chain8_whole | -0.173 | ok |

**Consistency summary:**

- **r_chain8_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain8_semi | 1008.8ns | 411346.8ns | 0.2% |  |
| r_chain8_whole | 562.0ns | 1463514.6ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain8_semi (n=6, range 394197.9-425159.6 ns)
  394197.9 |########################################
  395746.0 |
  397294.1 |
  398842.1 |
  400390.2 |
  401938.3 |########################################
  403486.4 |########################################
  405034.5 |
  406582.6 |
  408130.6 |
  409678.7 |
  411226.8 |
  412774.9 |
  414323.0 |
  415871.1 |########################################
  417419.1 |
  418967.2 |
  420515.3 |########################################
  422063.4 |
  423611.5 |
  (0 below, 1 above range)

r_chain8_whole (n=6, range 1454855.8-1474944.6 ns)
  1454855.8 |########################################
  1455860.2 |########################################
  1456864.7 |########################################
  1457869.1 |
  1458873.6 |
  1459878.0 |
  1460882.4 |
  1461886.9 |########################################
  1462891.3 |########################################
  1463895.7 |
  1464900.2 |
  1465904.6 |
  1466909.1 |
  1467913.5 |
  1468917.9 |
  1469922.4 |
  1470926.8 |
  1471931.2 |
  1472935.7 |
  1473940.1 |
  (0 below, 1 above range)

```
