# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), real profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_real_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_real_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_real_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_real_scalar has the worst median (67.87 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_real_vert8 at 28.22 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_real_vert8 dominates: 17% faster than the next best (carrier_vert_real_vert4)

carrier_vert_real_vert8 (28.22 us) leads carrier_vert_real_vert4 (33.00 us) by 17%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_real_vert8 beats baseline by 59% (significant)

carrier_vert_real_vert8 is -39.79 us (59%) faster than baseline carrier_vert_real_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_real_scalar is an outlier: 2.4x slower than the field

carrier_vert_real_scalar (67.87 us) is 2.4x the fastest (28.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_real_vert8** at 28223.9 ns median (-58.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.40x (fastest 28223.9 ns, slowest 67873.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 70494ns | 70337ns | 69700ns | 70185ns | 71355ns | base |
| carrier_vert_real_vert4 | 35236ns | 35350ns | 34462ns | 35281ns | 35556ns | -50.02% |
| carrier_vert_real_vert8 | 30555ns | 30471ns | 30090ns | 30411ns | 31004ns | -56.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_real_scalar | 68069ns | 67179ns | 68957ns | base | 0.004 |
| carrier_vert_real_vert4 | 32951ns | 32312ns | 33263ns | -51.59% | 0.008 |
| carrier_vert_real_vert8 | 28240ns | 27896ns | 28586ns | -58.51% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_real_scalar | 640739 | 1988721 | 0.322 | 1.00× |
| carrier_vert_real_vert4 | 416073 | 1014761 | 0.410 | 0.65× |
| carrier_vert_real_vert8 | 408028 | 822660 | 0.496 | 0.64× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_real_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_real_scalar | 0.004 | 41.1% |
| carrier_vert_real_vert4 | 0.008 | 84.5% |
| carrier_vert_real_vert8 | 0.009 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_real_scalar | 70494ns | 70494ns | base |
| carrier_vert_real_vert4 | 35236ns | 35236ns | -50.02% |
| carrier_vert_real_vert8 | 30555ns | 30555ns | -56.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 67874ns | base | --- | [67377, 68957] | --- | --- | --- | --- |
| carrier_vert_real_vert4 | 33005ns | -34818.5ns (-51.3%) | [-35842, -34693]ns | [32586, 33263] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_real_vert8 | 28224ns | -39791.9ns (-58.6%) | [-40905, -38791]ns | [27910, 28586] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_real_scalar | carrier_vert_real_vert4 | carrier_vert_real_vert8 |
|---|---|---|---|
| 1 | 67179ns | -51.9% | -57.1% |
| 2 | 67575ns | -51.3% | -58.0% |
| 3 | 68715ns | -51.8% | -59.4% |
| 4 | 67630ns | -51.4% | -58.2% |
| 5 | 69200ns | -52.2% | -59.2% |
| 6 | 68118ns | -51.0% | -59.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_real_scalar | -0.206 | moderate- |
| carrier_vert_real_vert4 | 0.050 | ok |
| carrier_vert_real_vert8 | 0.073 | ok |

**Consistency summary:**

- **carrier_vert_real_vert4**: won 6/6, lost 0/6
- **carrier_vert_real_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_real_scalar | 136501.3ns | 68069.4ns | 200.5% | HIGH |
| carrier_vert_real_vert4 | 99023.3ns | 32951.4ns | 300.5% | HIGH |
| carrier_vert_real_vert8 | 101670.2ns | 28240.1ns | 360.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_real_scalar (n=6, range 67179.2-68957.1 ns)
  67179.2 |########################################
  67268.1 |
  67357.0 |
  67445.9 |
  67534.8 |########################################
  67623.7 |########################################
  67712.6 |
  67801.5 |
  67890.4 |
  67979.3 |
  68068.1 |########################################
  68157.0 |
  68245.9 |
  68334.8 |
  68423.7 |
  68512.6 |
  68601.5 |
  68690.4 |########################################
  68779.3 |
  68868.2 |
  (0 below, 1 above range)

carrier_vert_real_vert4 (n=6, range 32311.7-33263.1 ns)
  32311.7 |########################################
  32359.3 |
  32406.8 |
  32454.4 |
  32502.0 |
  32549.6 |
  32597.1 |
  32644.7 |
  32692.3 |
  32739.9 |
  32787.4 |
  32835.0 |########################################
  32882.6 |
  32930.1 |########################################
  32977.7 |
  33025.3 |
  33072.9 |########################################
  33120.4 |########################################
  33168.0 |
  33215.6 |
  (0 below, 1 above range)

carrier_vert_real_vert8 (n=6, range 27896.2-28586.5 ns)
  27896.2 |########################################
  27930.7 |
  27965.2 |
  27999.7 |
  28034.2 |
  28068.8 |
  28103.3 |
  28137.8 |
  28172.3 |
  28206.8 |########################################
  28241.3 |
  28275.8 |
  28310.4 |
  28344.9 |
  28379.4 |####################
  28413.9 |
  28448.4 |
  28482.9 |
  28517.4 |
  28551.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_real_scalar**: bridge=200.5% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert4**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert8**: bridge=368.7% of algo (FFI overhead may distort results)
