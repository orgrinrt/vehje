# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_wideselect_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_wideselect_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_wideselect_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_wideselect_scalar has the worst median (80.40 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_wideselect_vert8 at 31.85 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_wideselect_vert8 beats baseline by 60% (significant)

carrier_vert_wideselect_vert8 is -48.47 us (60%) faster than baseline carrier_vert_wideselect_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_wideselect_scalar is an outlier: 2.5x slower than the field

carrier_vert_wideselect_scalar (80.40 us) is 2.5x the fastest (31.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_wideselect_vert8 is fastest but the noisiest (CV 5.2%)

carrier_vert_wideselect_vert8 wins on median (31.85 us) yet has the highest variance (CV 5.2%), while carrier_vert_wideselect_scalar is the steadiest (CV 2.2%, 80.40 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_vert_wideselect_vert8** at 31846.7 ns median (-60.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.52x (fastest 31846.7 ns, slowest 80397.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 82705ns | 82770ns | 79915ns | 82389ns | 84574ns | base |
| carrier_vert_wideselect_vert4 | 34594ns | 34650ns | 33155ns | 34481ns | 35483ns | -58.17% |
| carrier_vert_wideselect_vert8 | 34572ns | 34272ns | 32659ns | 33908ns | 36526ns | -58.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 80344ns | 77635ns | 82208ns | base | 0.003 |
| carrier_vert_wideselect_vert4 | 32311ns | 30959ns | 33122ns | -59.78% | 0.008 |
| carrier_vert_wideselect_vert8 | 32140ns | 30370ns | 33979ns | -60.00% | 0.008 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_wideselect_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.003 | 37.8% |
| carrier_vert_wideselect_vert4 | 0.008 | 93.8% |
| carrier_vert_wideselect_vert8 | 0.008 | 95.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_wideselect_scalar | 82705ns | 82705ns | base |
| carrier_vert_wideselect_vert4 | 34594ns | 34594ns | -58.17% |
| carrier_vert_wideselect_vert8 | 34572ns | 34572ns | -58.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 80398ns | base | --- | [78427, 82208] | --- | --- | --- | --- |
| carrier_vert_wideselect_vert4 | 32388ns | -47579.6ns (-59.2%) | [-50419, -46099]ns | [31425, 33122] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_wideselect_vert8 | 31847ns | -48465.4ns (-60.3%) | [-50361, -45786]ns | [30593, 33979] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_wideselect_scalar | carrier_vert_wideselect_vert4 | carrier_vert_wideselect_vert8 |
|---|---|---|---|
| 1 | 79854ns | -59.3% | -55.5% |
| 2 | 79218ns | -59.7% | -61.1% |
| 3 | 77635ns | -57.8% | -60.9% |
| 4 | 81246ns | -58.8% | -60.7% |
| 5 | 80941ns | -60.1% | -60.0% |
| 6 | 83170ns | -62.8% | -61.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.188 | ok |
| carrier_vert_wideselect_vert4 | 0.076 | ok |
| carrier_vert_wideselect_vert8 | -0.116 | ok |

**Consistency summary:**

- **carrier_vert_wideselect_vert4**: won 6/6, lost 0/6
- **carrier_vert_wideselect_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 140797.0ns | 80344.1ns | 175.2% | HIGH |
| carrier_vert_wideselect_vert4 | 97040.3ns | 32311.4ns | 300.3% | HIGH |
| carrier_vert_wideselect_vert8 | 95807.0ns | 32139.8ns | 298.1% | HIGH |

## Distribution (algo ns)

```
carrier_vert_wideselect_scalar (n=6, range 77635.4-82208.1 ns)
  77635.4 |########################################
  77864.0 |
  78092.7 |
  78321.3 |
  78549.9 |
  78778.6 |
  79007.2 |########################################
  79235.8 |
  79464.5 |
  79693.1 |########################################
  79921.8 |
  80150.4 |
  80379.0 |
  80607.7 |
  80836.3 |########################################
  81064.9 |########################################
  81293.6 |
  81522.2 |
  81750.8 |
  81979.5 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert4 (n=6, range 30959.2-33121.7 ns)
  30959.2 |########################################
  31067.3 |
  31175.5 |
  31283.6 |
  31391.7 |
  31499.8 |
  31608.0 |
  31716.1 |
  31824.2 |########################################
  31932.3 |
  32040.4 |
  32148.6 |
  32256.7 |########################################
  32364.8 |########################################
  32472.9 |
  32581.1 |
  32689.2 |########################################
  32797.3 |
  32905.4 |
  33013.6 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert8 (n=6, range 30370.4-33979.4 ns)
  30370.4 |########################################
  30550.9 |
  30731.3 |########################################
  30911.8 |
  31092.2 |
  31272.7 |
  31453.1 |
  31633.5 |########################################
  31814.0 |########################################
  31994.4 |
  32174.9 |
  32355.3 |########################################
  32535.8 |
  32716.2 |
  32896.7 |
  33077.1 |
  33257.6 |
  33438.0 |
  33618.5 |
  33798.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_wideselect_scalar**: bridge=177.8% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert4**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert8**: bridge=300.9% of algo (FFI overhead may distort results)
