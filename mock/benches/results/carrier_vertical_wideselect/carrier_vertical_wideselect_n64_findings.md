# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_wideselect_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_wideselect_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_wideselect_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_wideselect_scalar has the worst median (16.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_wideselect_vert8 at 7.33 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_wideselect_vert8 beats baseline by 54% (significant)

carrier_vert_wideselect_vert8 is -8.64 us (54%) faster than baseline carrier_vert_wideselect_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_wideselect_scalar is an outlier: 2.2x slower than the field

carrier_vert_wideselect_scalar (16.09 us) is 2.2x the fastest (7.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_wideselect_vert8** at 7332.9 ns median (-54.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.19x (fastest 7332.9 ns, slowest 16086.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 18163ns | 18393ns | 17516ns | 18107ns | 18569ns | base |
| carrier_vert_wideselect_vert4 | 9978ns | 10159ns | 9444ns | 9955ns | 10279ns | -45.06% |
| carrier_vert_wideselect_vert8 | 9617ns | 9654ns | 9342ns | 9560ns | 9841ns | -47.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 15892ns | 15328ns | 16258ns | base | 0.004 |
| carrier_vert_wideselect_vert4 | 7601ns | 7265ns | 7789ns | -52.17% | 0.008 |
| carrier_vert_wideselect_vert8 | 7300ns | 7037ns | 7480ns | -54.06% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 337714 | 1168366 | 0.289 | 1.00× |
| carrier_vert_wideselect_vert4 | 295683 | 865303 | 0.342 | 0.88× |
| carrier_vert_wideselect_vert8 | 292756 | 636105 | 0.460 | 0.87× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_wideselect_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.004 | 43.7% |
| carrier_vert_wideselect_vert4 | 0.008 | 91.3% |
| carrier_vert_wideselect_vert8 | 0.009 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_wideselect_scalar | 18163ns | 18163ns | base |
| carrier_vert_wideselect_vert4 | 9978ns | 9978ns | -45.06% |
| carrier_vert_wideselect_vert8 | 9617ns | 9617ns | -47.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 16086ns | base | --- | [15330, 16258] | --- | --- | --- | --- |
| carrier_vert_wideselect_vert4 | 7706ns | -8355.2ns (-51.9%) | [-8495, -8022]ns | [7309, 7789] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_wideselect_vert8 | 7333ns | -8639.0ns (-53.7%) | [-8986, -8150]ns | [7087, 7480] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_wideselect_scalar | carrier_vert_wideselect_vert4 | carrier_vert_wideselect_vert8 |
|---|---|---|---|
| 1 | 15328ns | -52.0% | -52.2% |
| 2 | 16244ns | -52.5% | -56.1% |
| 3 | 16158ns | -52.3% | -53.3% |
| 4 | 16272ns | -51.7% | -54.5% |
| 5 | 16015ns | -51.8% | -54.2% |
| 6 | 15333ns | -52.6% | -54.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_wideselect_scalar | -0.026 | ok |
| carrier_vert_wideselect_vert4 | 0.002 | ok |
| carrier_vert_wideselect_vert8 | -0.141 | ok |

**Consistency summary:**

- **carrier_vert_wideselect_vert4**: won 6/6, lost 0/6
- **carrier_vert_wideselect_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 95667.8ns | 15891.7ns | 602.0% | HIGH |
| carrier_vert_wideselect_vert4 | 89005.8ns | 7601.2ns | 1170.9% | HIGH |
| carrier_vert_wideselect_vert8 | 88588.5ns | 7299.9ns | 1213.6% | HIGH |

## Distribution (algo ns)

```
carrier_vert_wideselect_scalar (n=6, range 15327.5-16258.4 ns)
  15327.5 |########################################
  15374.0 |
  15420.6 |
  15467.1 |
  15513.7 |
  15560.2 |
  15606.8 |
  15653.3 |
  15699.8 |
  15746.4 |
  15792.9 |
  15839.5 |
  15886.0 |
  15932.6 |
  15979.1 |####################
  16025.6 |
  16072.2 |
  16118.7 |####################
  16165.3 |
  16211.8 |####################
  (0 below, 1 above range)

carrier_vert_wideselect_vert4 (n=6, range 7265.0-7788.5 ns)
   7265.0 |####################
   7291.2 |
   7317.4 |
   7343.5 |####################
   7369.7 |
   7395.9 |
   7422.1 |
   7448.2 |
   7474.4 |
   7500.6 |
   7526.8 |
   7553.0 |
   7579.1 |
   7605.3 |
   7631.5 |
   7657.7 |
   7683.8 |####################
   7710.0 |########################################
   7736.2 |
   7762.4 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert8 (n=6, range 7037.1-7480.0 ns)
   7037.1 |########################################
   7059.2 |
   7081.4 |
   7103.5 |
   7125.7 |########################################
   7147.8 |
   7170.0 |
   7192.1 |
   7214.3 |
   7236.4 |
   7258.6 |
   7280.7 |
   7302.8 |########################################
   7325.0 |########################################
   7347.1 |
   7369.3 |
   7391.4 |########################################
   7413.6 |
   7435.7 |
   7457.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_wideselect_scalar**: bridge=602.3% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert4**: bridge=1158.1% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert8**: bridge=1206.1% of algo (FFI overhead may distort results)
