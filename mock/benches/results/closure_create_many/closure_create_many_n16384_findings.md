# Closure representation: create-many-call-once, flat vs linked (creation cost dominates)

2 variants, 6 samples per variant.
Baseline: **closure_create_many_flat**

## Highlights

Baseline for all deltas below: **closure_create_many_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (closure_create_many_flat) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline closure_create_many_flat has the worst median (2.47 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest closure_create_many_linked at 1.16 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### closure_create_many_linked dominates: 112% faster than the next best (closure_create_many_flat)

closure_create_many_linked (1.16 ms) leads closure_create_many_flat (2.47 ms) by 112%, a clear separation rather than a photo finish. CV 27.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### closure_create_many_linked beats baseline by 52% (significant)

closure_create_many_linked is -1.28 ms (52%) faster than baseline closure_create_many_flat, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### closure_create_many_linked is fastest but the noisiest (CV 27.8%)

closure_create_many_linked wins on median (1.16 ms) yet has the highest variance (CV 27.8%), while closure_create_many_flat is the steadiest (CV 5.4%, 2.47 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: closure_create_many_linked** at 1162862.1 ns median (-52.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.12x (fastest 1162862.1 ns, slowest 2467911.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| closure_create_many_flat | 2517657ns | 2470901ns | 2403138ns | 2458218ns | 2664076ns | base |
| closure_create_many_linked | 1310803ns | 1165966ns | 1143317ns | 1162404ns | 1617144ns | -47.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| closure_create_many_flat | 2514599ns | 2400442ns | 2660619ns | base | 0.007 |
| closure_create_many_linked | 1307503ns | 1140728ns | 1612978ns | -48.00% | 0.013 |

## Performance model

- Peak throughput: **0.014 Gops/s** (closure_create_many_linked; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| closure_create_many_flat | 0.007 | 46.2% |
| closure_create_many_linked | 0.014 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| closure_create_many_flat | 2517657ns | 2517657ns | base |
| closure_create_many_linked | 1310803ns | 1310803ns | -47.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| closure_create_many_flat | 2467911ns | base | --- | [2415266, 2660619] | --- | --- | --- | --- |
| closure_create_many_linked | 1162862ns | -1281522.9ns (-51.9%) | [-1469779, -869987]ns | [1146667, 1612978] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | closure_create_many_flat | closure_create_many_linked |
|---|---|---|
| 1 | 2400442ns | -52.0% |
| 2 | 2521280ns | -19.5% |
| 3 | 2490480ns | -53.7% |
| 4 | 2799958ns | -57.3% |
| 5 | 2445342ns | -52.1% |
| 6 | 2430091ns | -53.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| closure_create_many_flat | -0.203 | moderate- |
| closure_create_many_linked | -0.268 | moderate- |

**Consistency summary:**

- **closure_create_many_linked**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| closure_create_many_flat | 59.4ns | 2514598.9ns | 0.0% |  |
| closure_create_many_linked | 58.8ns | 1307502.6ns | 0.0% |  |

## Distribution (algo ns)

```
closure_create_many_flat (n=6, range 2400441.7-2660619.1 ns)
  2400441.7 |########################################
  2413450.6 |
  2426459.4 |########################################
  2439468.3 |########################################
  2452477.2 |
  2465486.1 |
  2478494.9 |########################################
  2491503.8 |
  2504512.7 |
  2517521.6 |########################################
  2530530.4 |
  2543539.3 |
  2556548.2 |
  2569557.0 |
  2582565.9 |
  2595574.8 |
  2608583.7 |
  2621592.5 |
  2634601.4 |
  2647610.3 |
  (0 below, 1 above range)

closure_create_many_linked (n=6, range 1140728.3-1612978.4 ns)
  1140728.3 |########################################
  1164340.8 |#############
  1187953.3 |#############
  1211565.8 |
  1235178.3 |
  1258790.8 |
  1282403.3 |
  1306015.8 |
  1329628.3 |
  1353240.8 |
  1376853.3 |
  1400465.8 |
  1424078.3 |
  1447690.8 |
  1471303.3 |
  1494915.8 |
  1518528.3 |
  1542140.8 |
  1565753.3 |
  1589365.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **closure_create_many_linked**: CV=24.7% (high variance, measurements may be unstable)
