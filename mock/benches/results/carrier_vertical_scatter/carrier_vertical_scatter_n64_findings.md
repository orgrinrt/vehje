# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_scatter_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_scatter_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_scatter_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_scatter_scalar has the worst median (19.54 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_scatter_vert8 at 6.90 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_scatter_vert8 beats baseline by 64% (significant)

carrier_vert_scatter_vert8 is -12.50 us (64%) faster than baseline carrier_vert_scatter_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_scatter_scalar is an outlier: 2.8x slower than the field

carrier_vert_scatter_scalar (19.54 us) is 2.8x the fastest (6.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_scatter_vert8** at 6897.9 ns median (-64.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.83x (fastest 6897.9 ns, slowest 19544.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 22497ns | 22020ns | 20609ns | 21732ns | 24588ns | base |
| carrier_vert_scatter_vert4 | 9727ns | 9788ns | 9266ns | 9682ns | 10027ns | -56.76% |
| carrier_vert_scatter_vert8 | 9385ns | 9365ns | 9022ns | 9299ns | 9695ns | -58.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 20003ns | 18223ns | 21973ns | base | 0.003 |
| carrier_vert_scatter_vert4 | 7394ns | 7100ns | 7629ns | -63.04% | 0.009 |
| carrier_vert_scatter_vert8 | 6927ns | 6640ns | 7154ns | -65.37% | 0.009 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_vert_scatter_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_scatter_scalar | 0.003 | 34.0% |
| carrier_vert_scatter_vert4 | 0.009 | 89.3% |
| carrier_vert_scatter_vert8 | 0.009 | 96.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_scatter_scalar | 22497ns | 22497ns | base |
| carrier_vert_scatter_vert4 | 9727ns | 9727ns | -56.76% |
| carrier_vert_scatter_vert8 | 9385ns | 9385ns | -58.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 19544ns | base | --- | [18491, 21973] | --- | --- | --- | --- |
| carrier_vert_scatter_vert4 | 7436ns | -12257.1ns (-62.7%) | [-14707, -10863]ns | [7116, 7629] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_scatter_vert8 | 6898ns | -12503.8ns (-64.0%) | [-15031, -11693]ns | [6729, 7154] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_scatter_scalar | carrier_vert_scatter_vert4 | carrier_vert_scatter_vert8 |
|---|---|---|---|
| 1 | 19869ns | -64.3% | -63.7% |
| 2 | 19447ns | -63.3% | -63.5% |
| 3 | 24077ns | -69.1% | -71.6% |
| 4 | 19641ns | -62.1% | -65.3% |
| 5 | 18760ns | -59.9% | -62.9% |
| 6 | 18223ns | -57.6% | -63.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_scatter_scalar | -0.046 | ok |
| carrier_vert_scatter_vert4 | 0.412 | moderate+ |
| carrier_vert_scatter_vert8 | 0.145 | ok |

**Consistency summary:**

- **carrier_vert_scatter_vert4**: won 6/6, lost 0/6
- **carrier_vert_scatter_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 96625.3ns | 20002.7ns | 483.1% | HIGH |
| carrier_vert_scatter_vert4 | 90760.2ns | 7393.7ns | 1227.5% | HIGH |
| carrier_vert_scatter_vert8 | 89561.1ns | 6926.9ns | 1293.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_scatter_scalar (n=6, range 18222.9-21972.8 ns)
  18222.9 |########################################
  18410.4 |
  18597.9 |########################################
  18785.4 |
  18972.9 |
  19160.4 |
  19347.9 |########################################
  19535.3 |########################################
  19722.8 |########################################
  19910.3 |
  20097.8 |
  20285.3 |
  20472.8 |
  20660.3 |
  20847.8 |
  21035.3 |
  21222.8 |
  21410.3 |
  21597.8 |
  21785.3 |
  (0 below, 1 above range)

carrier_vert_scatter_vert4 (n=6, range 7100.0-7628.5 ns)
   7100.0 |####################
   7126.4 |####################
   7152.9 |
   7179.3 |
   7205.7 |
   7232.1 |
   7258.6 |
   7285.0 |
   7311.4 |
   7337.8 |
   7364.3 |
   7390.7 |
   7417.1 |########################################
   7443.6 |
   7470.0 |
   7496.4 |
   7522.8 |####################
   7549.3 |
   7575.7 |
   7602.1 |
  (0 below, 1 above range)

carrier_vert_scatter_vert8 (n=6, range 6639.6-7153.9 ns)
   6639.6 |########################################
   6665.3 |
   6691.0 |
   6716.8 |
   6742.5 |
   6768.2 |
   6793.9 |########################################
   6819.6 |########################################
   6845.3 |
   6871.1 |
   6896.8 |
   6922.5 |
   6948.2 |########################################
   6973.9 |
   6999.6 |
   7025.4 |
   7051.1 |
   7076.8 |########################################
   7102.5 |
   7128.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_scatter_scalar**: bridge=501.8% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert4**: bridge=1223.1% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert8**: bridge=1297.6% of algo (FFI overhead may distort results)
