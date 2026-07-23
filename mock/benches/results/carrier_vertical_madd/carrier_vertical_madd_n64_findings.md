# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_madd_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_madd_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_madd_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_madd_scalar has the worst median (17.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_madd_vert8 at 7.87 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_madd_vert8 dominates: 22% faster than the next best (carrier_vert_madd_vert4)

carrier_vert_madd_vert8 (7.87 us) leads carrier_vert_madd_vert4 (9.64 us) by 22%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_madd_vert8 beats baseline by 55% (significant)

carrier_vert_madd_vert8 is -9.70 us (55%) faster than baseline carrier_vert_madd_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_madd_scalar is an outlier: 2.2x slower than the field

carrier_vert_madd_scalar (17.57 us) is 2.2x the fastest (7.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_madd_vert8** at 7874.6 ns median (-55.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.23x (fastest 7874.6 ns, slowest 17565.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 19785ns | 19828ns | 19383ns | 19719ns | 20084ns | base |
| carrier_vert_madd_vert4 | 11996ns | 11939ns | 11530ns | 11817ns | 12498ns | -39.37% |
| carrier_vert_madd_vert8 | 10192ns | 10141ns | 9952ns | 10130ns | 10405ns | -48.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 17548ns | 17199ns | 17823ns | base | 0.004 |
| carrier_vert_madd_vert4 | 9600ns | 9210ns | 9907ns | -45.29% | 0.007 |
| carrier_vert_madd_vert8 | 7889ns | 7641ns | 8040ns | -55.04% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 335051 | 1030110 | 0.325 | 1.00× |
| carrier_vert_madd_vert4 | 309859 | 705628 | 0.439 | 0.92× |
| carrier_vert_madd_vert8 | 296305 | 588543 | 0.503 | 0.88× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_madd_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_madd_scalar | 0.004 | 43.5% |
| carrier_vert_madd_vert4 | 0.007 | 79.2% |
| carrier_vert_madd_vert8 | 0.008 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_madd_scalar | 19785ns | 19785ns | base |
| carrier_vert_madd_vert4 | 11996ns | 11996ns | -39.37% |
| carrier_vert_madd_vert8 | 10192ns | 10192ns | -48.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 17566ns | base | --- | [17255, 17823] | --- | --- | --- | --- |
| carrier_vert_madd_vert4 | 9642ns | -7946.7ns (-45.2%) | [-8178, -7718]ns | [9251, 9907] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_madd_vert8 | 7875ns | -9699.0ns (-55.2%) | [-9782, -9496]ns | [7751, 8040] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_madd_scalar | carrier_vert_madd_vert4 | carrier_vert_madd_vert8 |
|---|---|---|---|
| 1 | 17559ns | -45.5% | -55.2% |
| 2 | 17902ns | -43.8% | -54.8% |
| 3 | 17312ns | -43.9% | -55.9% |
| 4 | 17572ns | -47.6% | -55.3% |
| 5 | 17199ns | -46.0% | -54.2% |
| 6 | 17743ns | -45.1% | -55.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_madd_scalar | -0.475 | moderate- |
| carrier_vert_madd_vert4 | 0.139 | ok |
| carrier_vert_madd_vert8 | -0.427 | moderate- |

**Consistency summary:**

- **carrier_vert_madd_vert4**: won 6/6, lost 0/6
- **carrier_vert_madd_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 90868.3ns | 17548.0ns | 517.8% | HIGH |
| carrier_vert_madd_vert4 | 90636.6ns | 9600.2ns | 944.1% | HIGH |
| carrier_vert_madd_vert8 | 87709.7ns | 7888.8ns | 1111.8% | HIGH |

## Distribution (algo ns)

```
carrier_vert_madd_scalar (n=6, range 17199.2-17822.7 ns)
  17199.2 |####################
  17230.4 |
  17261.5 |
  17292.7 |####################
  17323.9 |
  17355.1 |
  17386.2 |
  17417.4 |
  17448.6 |
  17479.8 |
  17510.9 |
  17542.1 |########################################
  17573.3 |
  17604.5 |
  17635.6 |
  17666.8 |
  17698.0 |
  17729.2 |####################
  17760.3 |
  17791.5 |
  (0 below, 1 above range)

carrier_vert_madd_vert4 (n=6, range 9210.0-9907.3 ns)
   9210.0 |########################################
   9244.9 |
   9279.7 |########################################
   9314.6 |
   9349.5 |
   9384.3 |
   9419.2 |
   9454.1 |
   9488.9 |
   9523.8 |
   9558.6 |########################################
   9593.5 |
   9628.4 |
   9663.2 |
   9698.1 |########################################
   9733.0 |########################################
   9767.8 |
   9802.7 |
   9837.6 |
   9872.4 |
  (0 below, 1 above range)

carrier_vert_madd_vert8 (n=6, range 7640.8-8040.4 ns)
   7640.8 |#############
   7660.8 |
   7680.8 |
   7700.7 |
   7720.7 |
   7740.7 |
   7760.7 |
   7780.7 |
   7800.6 |
   7820.6 |
   7840.6 |
   7860.6 |########################################
   7880.6 |
   7900.5 |
   7920.5 |
   7940.5 |
   7960.5 |
   7980.5 |#############
   8000.4 |
   8020.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_madd_scalar**: bridge=514.0% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert4**: bridge=949.0% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert8**: bridge=1105.6% of algo (FFI overhead may distort results)
