# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), real profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_real_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_real_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_real_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_real_scalar has the worst median (83.83 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_real_vert8 at 30.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_real_vert8 dominates: 16% faster than the next best (carrier_vert_real_vert4)

carrier_vert_real_vert8 (30.54 us) leads carrier_vert_real_vert4 (35.47 us) by 16%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_real_vert8 beats baseline by 64% (significant)

carrier_vert_real_vert8 is -53.36 us (64%) faster than baseline carrier_vert_real_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_real_scalar is an outlier: 2.7x slower than the field

carrier_vert_real_scalar (83.83 us) is 2.7x the fastest (30.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_real_vert8** at 30540.8 ns median (-63.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.74x (fastest 30540.8 ns, slowest 83828.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 86397ns | 86401ns | 85696ns | 86341ns | 86831ns | base |
| carrier_vert_real_vert4 | 37820ns | 38119ns | 34444ns | 38107ns | 39077ns | -56.23% |
| carrier_vert_real_vert8 | 33320ns | 33116ns | 32746ns | 33075ns | 33974ns | -61.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_real_scalar | 83841ns | 83063ns | 84395ns | base | 0.003 |
| carrier_vert_real_vert4 | 35225ns | 32091ns | 36479ns | -57.99% | 0.007 |
| carrier_vert_real_vert8 | 30706ns | 30175ns | 31310ns | -63.38% | 0.008 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_real_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_real_scalar | 0.003 | 36.0% |
| carrier_vert_real_vert4 | 0.007 | 85.1% |
| carrier_vert_real_vert8 | 0.008 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_real_scalar | 86397ns | 86397ns | base |
| carrier_vert_real_vert4 | 37820ns | 37820ns | -56.23% |
| carrier_vert_real_vert8 | 33320ns | 33320ns | -61.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 83829ns | base | --- | [83299, 84395] | --- | --- | --- | --- |
| carrier_vert_real_vert4 | 35466ns | -48143.9ns (-57.4%) | [-50044, -47661]ns | [33729, 36479] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_real_vert8 | 30541ns | -53363.3ns (-63.7%) | [-54053, -51989]ns | [30266, 31310] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_real_scalar | carrier_vert_real_vert4 | carrier_vert_real_vert8 |
|---|---|---|---|
| 1 | 83668ns | -61.6% | -63.5% |
| 2 | 83063ns | -57.3% | -63.0% |
| 3 | 84580ns | -56.9% | -64.3% |
| 4 | 83990ns | -57.8% | -63.9% |
| 5 | 83535ns | -57.7% | -61.8% |
| 6 | 84209ns | -56.7% | -63.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_real_scalar | -0.341 | moderate- |
| carrier_vert_real_vert4 | 0.008 | ok |
| carrier_vert_real_vert8 | -0.244 | moderate- |

**Consistency summary:**

- **carrier_vert_real_vert4**: won 6/6, lost 0/6
- **carrier_vert_real_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_real_scalar | 134876.7ns | 83840.8ns | 160.9% | HIGH |
| carrier_vert_real_vert4 | 105601.1ns | 35224.7ns | 299.8% | HIGH |
| carrier_vert_real_vert8 | 92263.9ns | 30705.7ns | 300.5% | HIGH |

## Distribution (algo ns)

```
carrier_vert_real_scalar (n=6, range 83062.9-84394.6 ns)
  83062.9 |########################################
  83129.5 |
  83196.1 |
  83262.7 |
  83329.2 |
  83395.8 |
  83462.4 |
  83529.0 |########################################
  83595.6 |
  83662.2 |########################################
  83728.8 |
  83795.3 |
  83861.9 |
  83928.5 |########################################
  83995.1 |
  84061.7 |
  84128.3 |
  84194.8 |########################################
  84261.4 |
  84328.0 |
  (0 below, 1 above range)

carrier_vert_real_vert4 (n=6, range 32091.2-36478.8 ns)
  32091.2 |####################
  32310.6 |
  32530.0 |
  32749.3 |
  32968.7 |
  33188.1 |
  33407.5 |
  33626.8 |
  33846.2 |
  34065.6 |
  34285.0 |
  34504.4 |
  34723.7 |
  34943.1 |
  35162.5 |####################
  35381.9 |########################################
  35601.2 |
  35820.6 |
  36040.0 |
  36259.4 |####################
  (0 below, 1 above range)

carrier_vert_real_vert8 (n=6, range 30175.4-31310.2 ns)
  30175.4 |########################################
  30232.1 |
  30288.9 |
  30345.6 |########################################
  30402.4 |
  30459.1 |########################################
  30515.8 |
  30572.6 |########################################
  30629.3 |
  30686.1 |########################################
  30742.8 |
  30799.5 |
  30856.3 |
  30913.0 |
  30969.8 |
  31026.5 |
  31083.2 |
  31140.0 |
  31196.7 |
  31253.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_real_scalar**: bridge=161.8% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert4**: bridge=299.6% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert8**: bridge=300.9% of algo (FFI overhead may distort results)
