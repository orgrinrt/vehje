# Incremental compilation: cold rebuild vs warm content-addressed reload after one edit

2 variants, 6 samples per variant.
Baseline: **incr_cold**

## Highlights

Baseline for all deltas below: **incr_cold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (incr_cold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline incr_cold has the worst median (24.44 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest incr_warm at 9.75 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### incr_warm dominates: 151% faster than the next best (incr_cold)

incr_warm (9.75 us) leads incr_cold (24.44 us) by 151%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### incr_warm beats baseline by 60% (significant)

incr_warm is -14.67 us (60%) faster than baseline incr_cold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: incr_warm** at 9752.5 ns median (-60.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.51x (fastest 9752.5 ns, slowest 24441.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| incr_cold | 26575ns | 26702ns | 25816ns | 26685ns | 26789ns | base |
| incr_warm | 12059ns | 12039ns | 12017ns | 12033ns | 12118ns | -54.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| incr_cold | 24322ns | 23637ns | 24506ns | base | 0.003 |
| incr_warm | 9775ns | 9747ns | 9826ns | -59.81% | 0.007 |

## Performance model

- Peak throughput: **0.007 Gops/s** (incr_warm; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| incr_cold | 0.003 | 39.9% |
| incr_warm | 0.007 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| incr_cold | 26575ns | 26575ns | base |
| incr_warm | 12059ns | 12059ns | -54.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| incr_cold | 24441ns | base | --- | [24018, 24506] | --- | --- | --- | --- |
| incr_warm | 9752ns | -14670.5ns (-60.0%) | [-14759, -14210]ns | [9747, 9826] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | incr_cold | incr_warm |
|---|---|---|
| 1 | 24478ns | -60.2% |
| 2 | 24534ns | -60.3% |
| 3 | 24399ns | -60.0% |
| 4 | 24464ns | -60.1% |
| 5 | 24419ns | -59.5% |
| 6 | 23637ns | -58.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| incr_cold | 0.014 | ok |
| incr_warm | -0.251 | moderate- |

**Consistency summary:**

- **incr_warm**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| incr_cold | 48.2ns | 24321.7ns | 0.2% |  |
| incr_warm | 64.0ns | 9775.3ns | 0.7% |  |

## Distribution (algo ns)

```
incr_cold (n=6, range 23637.1-24506.0 ns)
  23637.1 |####################
  23680.5 |
  23724.0 |
  23767.4 |
  23810.9 |
  23854.3 |
  23897.8 |
  23941.2 |
  23984.7 |
  24028.1 |
  24071.5 |
  24115.0 |
  24158.4 |
  24201.9 |
  24245.3 |
  24288.8 |
  24332.2 |
  24375.7 |########################################
  24419.1 |
  24462.6 |########################################
  (0 below, 1 above range)

incr_warm (n=6, range 9746.7-9826.5 ns)
   9746.7 |########################################
   9750.7 |
   9754.7 |#############
   9758.7 |
   9762.7 |#############
   9766.6 |
   9770.6 |
   9774.6 |
   9778.6 |
   9782.6 |
   9786.6 |
   9790.6 |
   9794.6 |
   9798.5 |
   9802.5 |
   9806.5 |
   9810.5 |
   9814.5 |
   9818.5 |
   9822.5 |
  (0 below, 1 above range)

```
