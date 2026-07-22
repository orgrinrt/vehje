# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), real profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_real_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_real_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_real_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_real_scalar has the worst median (322.01 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_real_vert8 at 122.23 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_real_vert8 dominates: 128% faster than the next best (carrier_vert_real_vert4)

carrier_vert_real_vert8 (122.23 us) leads carrier_vert_real_vert4 (278.44 us) by 128%, a clear separation rather than a photo finish. CV 9.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_real_vert8 beats baseline by 62% (significant)

carrier_vert_real_vert8 is -199.20 us (62%) faster than baseline carrier_vert_real_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_real_scalar is an outlier: 2.6x slower than the field

carrier_vert_real_scalar (322.01 us) is 2.6x the fastest (122.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_real_vert8 is fastest but the noisiest (CV 9.4%)

carrier_vert_real_vert8 wins on median (122.23 us) yet has the highest variance (CV 9.4%), while carrier_vert_real_scalar is the steadiest (CV 0.8%, 322.01 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_vert_real_vert8** at 122226.6 ns median (-62.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.63x (fastest 122226.6 ns, slowest 322007.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 325680ns | 325003ns | 322980ns | 324552ns | 328722ns | base |
| carrier_vert_real_vert4 | 280602ns | 280990ns | 243605ns | 280002ns | 300000ns | -13.84% |
| carrier_vert_real_vert8 | 128102ns | 124794ns | 117057ns | 123090ns | 141143ns | -60.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_real_scalar | 322608ns | 319888ns | 325661ns | base | 0.003 |
| carrier_vert_real_vert4 | 277858ns | 240915ns | 297050ns | -13.87% | 0.004 |
| carrier_vert_real_vert8 | 125505ns | 114335ns | 138457ns | -61.10% | 0.008 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_real_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_real_scalar | 0.003 | 35.5% |
| carrier_vert_real_vert4 | 0.004 | 41.1% |
| carrier_vert_real_vert8 | 0.008 | 93.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_real_scalar | 325680ns | 325680ns | base |
| carrier_vert_real_vert4 | 280602ns | 280602ns | -13.84% |
| carrier_vert_real_vert8 | 128102ns | 128102ns | -60.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 322008ns | base | --- | [320157, 325661] | --- | --- | --- | --- |
| carrier_vert_real_vert4 | 278442ns | -46598.8ns (-14.5%) | [-64545, -23107]ns | [258083, 297050] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_real_vert8 | 122227ns | -199203.1ns (-61.9%) | [-208302, -183804]ns | [115832, 138457] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_real_scalar | carrier_vert_real_vert4 | carrier_vert_real_vert8 |
|---|---|---|---|
| 1 | 320426ns | -5.8% | -53.3% |
| 2 | 319888ns | -8.6% | -61.5% |
| 3 | 324428ns | -15.0% | -60.7% |
| 4 | 322640ns | -14.7% | -62.4% |
| 5 | 321375ns | -25.0% | -64.4% |
| 6 | 326893ns | -14.0% | -64.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_real_scalar | -0.121 | ok |
| carrier_vert_real_vert4 | 0.138 | ok |
| carrier_vert_real_vert8 | 0.087 | ok |

**Consistency summary:**

- **carrier_vert_real_vert4**: won 6/6, lost 0/6
- **carrier_vert_real_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_real_scalar | 322275.1ns | 322608.5ns | 99.9% | HIGH |
| carrier_vert_real_vert4 | 256166.2ns | 277858.1ns | 92.2% | HIGH |
| carrier_vert_real_vert8 | 115624.3ns | 125505.3ns | 92.1% | HIGH |

## Distribution (algo ns)

```
carrier_vert_real_scalar (n=6, range 319887.5-325660.6 ns)
  319887.5 |########################################
  320176.2 |########################################
  320464.8 |
  320753.5 |
  321042.1 |
  321330.8 |########################################
  321619.4 |
  321908.1 |
  322196.7 |
  322485.4 |########################################
  322774.0 |
  323062.7 |
  323351.4 |
  323640.0 |
  323928.7 |
  324217.3 |########################################
  324506.0 |
  324794.6 |
  325083.3 |
  325371.9 |
  (0 below, 1 above range)

carrier_vert_real_vert4 (n=6, range 240915.0-297049.6 ns)
  240915.0 |####################
  243721.7 |
  246528.5 |
  249335.2 |
  252141.9 |
  254948.6 |
  257755.4 |
  260562.1 |
  263368.8 |
  266175.5 |
  268982.3 |
  271789.0 |
  274595.7 |########################################
  277402.5 |
  280209.2 |####################
  283015.9 |
  285822.6 |
  288629.4 |
  291436.1 |####################
  294242.8 |
  (0 below, 1 above range)

carrier_vert_real_vert8 (n=6, range 114335.0-138457.1 ns)
  114335.0 |########################################
  115541.1 |
  116747.2 |########################################
  117953.3 |
  119159.4 |
  120365.5 |########################################
  121571.6 |
  122777.7 |########################################
  123983.8 |
  125189.9 |
  126396.1 |########################################
  127602.2 |
  128808.3 |
  130014.4 |
  131220.5 |
  132426.6 |
  133632.7 |
  134838.8 |
  136044.9 |
  137251.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_real_scalar**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert4**: bridge=90.9% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert8**: bridge=94.7% of algo (FFI overhead may distort results)
