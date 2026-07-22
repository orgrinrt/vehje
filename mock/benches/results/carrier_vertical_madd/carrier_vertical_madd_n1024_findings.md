# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_madd_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_madd_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_madd_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_madd_scalar has the worst median (372.85 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_madd_vert8 at 141.93 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_madd_vert8 dominates: 46% faster than the next best (carrier_vert_madd_vert4)

carrier_vert_madd_vert8 (141.93 us) leads carrier_vert_madd_vert4 (207.49 us) by 46%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_madd_vert8 beats baseline by 62% (significant)

carrier_vert_madd_vert8 is -232.93 us (62%) faster than baseline carrier_vert_madd_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_madd_scalar is an outlier: 2.6x slower than the field

carrier_vert_madd_scalar (372.85 us) is 2.6x the fastest (141.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_madd_vert8** at 141934.0 ns median (-61.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.63x (fastest 141934.0 ns, slowest 372850.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 375812ns | 375877ns | 371296ns | 374741ns | 379677ns | base |
| carrier_vert_madd_vert4 | 210196ns | 210495ns | 208542ns | 210031ns | 211270ns | -44.07% |
| carrier_vert_madd_vert8 | 144658ns | 144709ns | 140859ns | 143879ns | 147727ns | -61.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 372860ns | 368403ns | 376613ns | base | 0.003 |
| carrier_vert_madd_vert4 | 207381ns | 205673ns | 208544ns | -44.38% | 0.005 |
| carrier_vert_madd_vert8 | 141945ns | 138382ns | 144946ns | -61.93% | 0.007 |

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_vert_madd_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_madd_scalar | 0.003 | 37.1% |
| carrier_vert_madd_vert4 | 0.005 | 66.7% |
| carrier_vert_madd_vert8 | 0.007 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_madd_scalar | 375812ns | 375812ns | base |
| carrier_vert_madd_vert4 | 210196ns | 210196ns | -44.07% |
| carrier_vert_madd_vert8 | 144658ns | 144658ns | -61.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 372851ns | base | --- | [369115, 376613] | --- | --- | --- | --- |
| carrier_vert_madd_vert4 | 207486ns | -164307.4ns (-44.1%) | [-169915, -162212]ns | [206114, 208544] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_madd_vert8 | 141934ns | -232928.8ns (-62.5%) | [-235645, -224169]ns | [138956, 144946] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_madd_scalar | carrier_vert_madd_vert4 | carrier_vert_madd_vert8 |
|---|---|---|---|
| 1 | 378847ns | -45.7% | -62.6% |
| 2 | 369827ns | -44.0% | -61.1% |
| 3 | 372109ns | -44.0% | -62.8% |
| 4 | 374379ns | -44.5% | -62.0% |
| 5 | 368403ns | -43.9% | -60.4% |
| 6 | 373592ns | -44.1% | -62.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_madd_scalar | -0.396 | moderate- |
| carrier_vert_madd_vert4 | -0.150 | ok |
| carrier_vert_madd_vert8 | -0.445 | moderate- |

**Consistency summary:**

- **carrier_vert_madd_vert4**: won 6/6, lost 0/6
- **carrier_vert_madd_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 372188.2ns | 372859.5ns | 99.8% | HIGH |
| carrier_vert_madd_vert4 | 208154.3ns | 207381.2ns | 100.4% | HIGH |
| carrier_vert_madd_vert8 | 143140.8ns | 141945.1ns | 100.8% | HIGH |

## Distribution (algo ns)

```
carrier_vert_madd_scalar (n=6, range 368402.9-376613.0 ns)
  368402.9 |########################################
  368813.4 |
  369223.9 |
  369634.4 |########################################
  370044.9 |
  370455.4 |
  370865.9 |
  371276.4 |
  371686.9 |
  372097.4 |########################################
  372507.9 |
  372918.4 |
  373328.9 |########################################
  373739.4 |
  374149.9 |########################################
  374560.4 |
  374970.9 |
  375381.4 |
  375791.9 |
  376202.4 |
  (0 below, 1 above range)

carrier_vert_madd_vert4 (n=6, range 205673.3-208543.5 ns)
  205673.3 |########################################
  205816.8 |
  205960.3 |
  206103.8 |
  206247.3 |
  206390.8 |
  206534.4 |########################################
  206677.9 |
  206821.4 |
  206964.9 |
  207108.4 |########################################
  207251.9 |
  207395.4 |
  207538.9 |
  207682.4 |########################################
  207826.0 |
  207969.5 |
  208113.0 |
  208256.5 |########################################
  208400.0 |
  (0 below, 1 above range)

carrier_vert_madd_vert8 (n=6, range 138382.5-144945.9 ns)
  138382.5 |########################################
  138710.7 |
  139038.8 |
  139367.0 |########################################
  139695.2 |
  140023.3 |
  140351.5 |
  140679.7 |
  141007.8 |
  141336.0 |########################################
  141664.2 |
  141992.3 |########################################
  142320.5 |
  142648.7 |
  142976.8 |
  143305.0 |
  143633.2 |########################################
  143961.3 |
  144289.5 |
  144617.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_madd_scalar**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert4**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert8**: bridge=100.8% of algo (FFI overhead may distort results)
