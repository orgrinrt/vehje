# Incremental compilation: cold rebuild vs warm content-addressed reload after one edit

2 variants, 6 samples per variant.
Baseline: **incr_cold**

## Highlights

Baseline for all deltas below: **incr_cold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (incr_cold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline incr_cold has the worst median (498.65 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest incr_warm at 141.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### incr_warm dominates: 252% faster than the next best (incr_cold)

incr_warm (141.48 us) leads incr_cold (498.65 us) by 252%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### incr_warm beats baseline by 72% (significant)

incr_warm is -360.62 us (72%) faster than baseline incr_cold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 3.5x the fastest

Fastest incr_warm (141.48 us) to slowest incr_cold (498.65 us): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: incr_warm** at 141481.5 ns median (-71.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.52x (fastest 141481.5 ns, slowest 498650.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| incr_cold | 504796ns | 501166ns | 496408ns | 500878ns | 514869ns | base |
| incr_warm | 142634ns | 143745ns | 137518ns | 142164ns | 145898ns | -71.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| incr_cold | 502260ns | 494151ns | 512263ns | base | 0.002 |
| incr_warm | 140343ns | 135377ns | 143500ns | -72.06% | 0.007 |

## Performance model

- Peak throughput: **0.008 Gops/s** (incr_warm; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| incr_cold | 0.002 | 27.1% |
| incr_warm | 0.007 | 95.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| incr_cold | 504796ns | 504796ns | base |
| incr_warm | 142634ns | 142634ns | -71.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| incr_cold | 498650ns | base | --- | [495866, 512263] | --- | --- | --- | --- |
| incr_warm | 141482ns | -360622.0ns (-72.3%) | [-372645, -352482]ns | [136048, 143500] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | incr_cold | incr_warm |
|---|---|---|
| 1 | 499331ns | -72.9% |
| 2 | 494151ns | -71.2% |
| 3 | 512996ns | -72.2% |
| 4 | 511530ns | -73.3% |
| 5 | 497581ns | -71.0% |
| 6 | 497970ns | -71.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| incr_cold | 0.041 | ok |
| incr_warm | -0.423 | moderate- |

**Consistency summary:**

- **incr_warm**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| incr_cold | 113.7ns | 502259.6ns | 0.0% |  |
| incr_warm | 85.7ns | 140343.3ns | 0.1% |  |

## Distribution (algo ns)

```
incr_cold (n=6, range 494150.8-512262.9 ns)
  494150.8 |########################################
  495056.4 |
  495962.0 |
  496867.6 |########################################
  497773.2 |########################################
  498678.8 |########################################
  499584.4 |
  500490.0 |
  501395.6 |
  502301.2 |
  503206.8 |
  504112.5 |
  505018.1 |
  505923.7 |
  506829.3 |
  507734.9 |
  508640.5 |
  509546.1 |
  510451.7 |
  511357.3 |########################################
  (0 below, 1 above range)

incr_warm (n=6, range 135377.1-143500.0 ns)
  135377.1 |####################
  135783.2 |
  136189.4 |
  136595.5 |####################
  137001.7 |
  137407.8 |
  137814.0 |
  138220.1 |
  138626.3 |
  139032.4 |
  139438.5 |
  139844.7 |
  140250.8 |
  140657.0 |####################
  141063.1 |
  141469.3 |
  141875.4 |
  142281.6 |########################################
  142687.7 |
  143093.9 |
  (0 below, 1 above range)

```
