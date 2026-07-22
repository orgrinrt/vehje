# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_madd_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_madd_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_madd_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_madd_scalar has the worst median (21.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_madd_vert8 at 8.46 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_madd_vert8 dominates: 22% faster than the next best (carrier_vert_madd_vert4)

carrier_vert_madd_vert8 (8.46 us) leads carrier_vert_madd_vert4 (10.33 us) by 22%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_madd_vert8 beats baseline by 60% (significant)

carrier_vert_madd_vert8 is -12.82 us (60%) faster than baseline carrier_vert_madd_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_madd_scalar is an outlier: 2.5x slower than the field

carrier_vert_madd_scalar (21.28 us) is 2.5x the fastest (8.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_madd_vert8** at 8457.9 ns median (-60.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.52x (fastest 8457.9 ns, slowest 21279.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 24013ns | 23858ns | 23454ns | 23832ns | 24564ns | base |
| carrier_vert_madd_vert4 | 12727ns | 12935ns | 11515ns | 12859ns | 13136ns | -47.00% |
| carrier_vert_madd_vert8 | 11076ns | 11072ns | 10908ns | 11041ns | 11211ns | -53.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 21380ns | 21021ns | 21727ns | base | 0.003 |
| carrier_vert_madd_vert4 | 10158ns | 9156ns | 10457ns | -52.49% | 0.006 |
| carrier_vert_madd_vert8 | 8485ns | 8358ns | 8634ns | -60.31% | 0.008 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_madd_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_madd_scalar | 0.003 | 39.3% |
| carrier_vert_madd_vert4 | 0.006 | 80.9% |
| carrier_vert_madd_vert8 | 0.008 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_madd_scalar | 24013ns | 24013ns | base |
| carrier_vert_madd_vert4 | 12727ns | 12727ns | -47.00% |
| carrier_vert_madd_vert8 | 11076ns | 11076ns | -53.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 21279ns | base | --- | [21132, 21727] | --- | --- | --- | --- |
| carrier_vert_madd_vert4 | 10325ns | -11192.1ns (-52.6%) | [-11625, -10847]ns | [9692, 10457] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_madd_vert8 | 8458ns | -12824.4ns (-60.3%) | [-13332, -12528]ns | [8362, 8634] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_madd_scalar | carrier_vert_madd_vert4 | carrier_vert_madd_vert8 |
|---|---|---|---|
| 1 | 21021ns | -56.4% | -59.6% |
| 2 | 21244ns | -51.0% | -60.0% |
| 3 | 21575ns | -52.6% | -60.9% |
| 4 | 21879ns | -52.0% | -61.8% |
| 5 | 21271ns | -51.1% | -60.7% |
| 6 | 21288ns | -51.8% | -58.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_madd_scalar | 0.166 | ok |
| carrier_vert_madd_vert4 | -0.093 | ok |
| carrier_vert_madd_vert8 | -0.110 | ok |

**Consistency summary:**

- **carrier_vert_madd_vert4**: won 6/6, lost 0/6
- **carrier_vert_madd_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 103110.3ns | 21379.6ns | 482.3% | HIGH |
| carrier_vert_madd_vert4 | 91836.4ns | 10158.3ns | 904.1% | HIGH |
| carrier_vert_madd_vert8 | 92236.9ns | 8484.7ns | 1087.1% | HIGH |

## Distribution (algo ns)

```
carrier_vert_madd_scalar (n=6, range 21020.8-21727.1 ns)
  21020.8 |####################
  21056.1 |
  21091.4 |
  21126.7 |
  21162.1 |
  21197.4 |
  21232.7 |####################
  21268.0 |########################################
  21303.3 |
  21338.6 |
  21373.9 |
  21409.3 |
  21444.6 |
  21479.9 |
  21515.2 |
  21550.5 |####################
  21585.8 |
  21621.2 |
  21656.5 |
  21691.8 |
  (0 below, 1 above range)

carrier_vert_madd_vert4 (n=6, range 9155.8-10457.1 ns)
   9155.8 |####################
   9220.9 |
   9285.9 |
   9351.0 |
   9416.1 |
   9481.1 |
   9546.2 |
   9611.3 |
   9676.3 |
   9741.4 |
   9806.5 |
   9871.5 |
   9936.6 |
  10001.6 |
  10066.7 |
  10131.8 |
  10196.8 |########################################
  10261.9 |
  10327.0 |
  10392.0 |########################################
  (0 below, 1 above range)

carrier_vert_madd_vert8 (n=6, range 8357.5-8633.8 ns)
   8357.5 |########################################
   8371.3 |
   8385.1 |
   8398.9 |
   8412.8 |
   8426.6 |####################
   8440.4 |
   8454.2 |
   8468.0 |
   8481.8 |####################
   8495.6 |####################
   8509.4 |
   8523.2 |
   8537.1 |
   8550.9 |
   8564.7 |
   8578.5 |
   8592.3 |
   8606.1 |
   8619.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_madd_scalar**: bridge=486.1% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert4**: bridge=888.5% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert8**: bridge=1095.6% of algo (FFI overhead may distort results)
