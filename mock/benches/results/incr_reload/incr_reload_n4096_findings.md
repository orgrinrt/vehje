# Incremental compilation: cold rebuild vs warm content-addressed reload after one edit

2 variants, 6 samples per variant.
Baseline: **incr_cold**

## Highlights

Baseline for all deltas below: **incr_cold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (incr_cold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline incr_cold has the worst median (2.03 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest incr_warm at 553.12 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### incr_warm dominates: 266% faster than the next best (incr_cold)

incr_warm (553.12 us) leads incr_cold (2.03 ms) by 266%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### incr_warm beats baseline by 73% (significant)

incr_warm is -1.47 ms (73%) faster than baseline incr_cold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 3.7x the fastest

Fastest incr_warm (553.12 us) to slowest incr_cold (2.03 ms): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: incr_warm** at 553116.4 ns median (-72.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.66x (fastest 553116.4 ns, slowest 2026495.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| incr_cold | 2044012ns | 2030032ns | 2025716ns | 2028614ns | 2076259ns | base |
| incr_warm | 555668ns | 555864ns | 549978ns | 553974ns | 561054ns | -72.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| incr_cold | 2040742ns | 2021774ns | 2073480ns | base | 0.002 |
| incr_warm | 552951ns | 546910ns | 558648ns | -72.90% | 0.007 |

## Performance model

- Peak throughput: **0.007 Gops/s** (incr_warm; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| incr_cold | 0.002 | 27.0% |
| incr_warm | 0.007 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| incr_cold | 2044012ns | 2044012ns | base |
| incr_warm | 555668ns | 555668ns | -72.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| incr_cold | 2026496ns | base | --- | [2022249, 2073480] | --- | --- | --- | --- |
| incr_warm | 553116ns | -1473013.8ns (-72.7%) | [-1520577, -1469782]ns | [547088, 558648] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | incr_cold | incr_warm |
|---|---|---|
| 1 | 2029918ns | -72.5% |
| 2 | 2022723ns | -72.6% |
| 3 | 2073773ns | -73.0% |
| 4 | 2023073ns | -72.7% |
| 5 | 2021774ns | -72.9% |
| 6 | 2073187ns | -73.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| incr_cold | -0.388 | moderate- |
| incr_warm | 0.344 | moderate+ |

**Consistency summary:**

- **incr_warm**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| incr_cold | 247.5ns | 2040741.6ns | 0.0% |  |
| incr_warm | 163.3ns | 552950.8ns | 0.0% |  |

## Distribution (algo ns)

```
incr_cold (n=6, range 2021774.2-2073480.2 ns)
  2021774.2 |########################################
  2024359.5 |
  2026944.8 |
  2029530.1 |#############
  2032115.4 |
  2034700.7 |
  2037286.0 |
  2039871.3 |
  2042456.6 |
  2045041.9 |
  2047627.2 |
  2050212.5 |
  2052797.8 |
  2055383.1 |
  2057968.4 |
  2060553.7 |
  2063139.0 |
  2065724.3 |
  2068309.6 |
  2070894.9 |#############
  (0 below, 1 above range)

incr_warm (n=6, range 546910.4-558648.1 ns)
  546910.4 |########################################
  547497.3 |
  548084.2 |
  548671.1 |
  549257.9 |
  549844.8 |
  550431.7 |
  551018.6 |
  551605.5 |####################
  552192.4 |
  552779.3 |
  553366.2 |
  553953.0 |####################
  554539.9 |
  555126.8 |
  555713.7 |
  556300.6 |
  556887.5 |
  557474.4 |
  558061.3 |####################
  (0 below, 1 above range)

```
