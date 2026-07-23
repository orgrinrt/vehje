# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_scatter_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_scatter_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_scatter_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_scatter_scalar has the worst median (67.03 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_scatter_vert8 at 28.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_scatter_vert8 dominates: 18% faster than the next best (carrier_vert_scatter_vert4)

carrier_vert_scatter_vert8 (28.08 us) leads carrier_vert_scatter_vert4 (33.13 us) by 18%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_scatter_vert8 beats baseline by 58% (significant)

carrier_vert_scatter_vert8 is -39.21 us (58%) faster than baseline carrier_vert_scatter_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_scatter_scalar is an outlier: 2.4x slower than the field

carrier_vert_scatter_scalar (67.03 us) is 2.4x the fastest (28.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_scatter_vert8 shows alternating (throttle bounce) (autocorr -0.62)

carrier_vert_scatter_vert8's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_scatter_vert8** at 28081.4 ns median (-58.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.39x (fastest 28081.4 ns, slowest 67025.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 69769ns | 69407ns | 68855ns | 69232ns | 71033ns | base |
| carrier_vert_scatter_vert4 | 35499ns | 35526ns | 34819ns | 35431ns | 35939ns | -49.12% |
| carrier_vert_scatter_vert8 | 30532ns | 30359ns | 30095ns | 30317ns | 31075ns | -56.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 67355ns | 66314ns | 68670ns | base | 0.004 |
| carrier_vert_scatter_vert4 | 33097ns | 32475ns | 33531ns | -50.86% | 0.008 |
| carrier_vert_scatter_vert8 | 28178ns | 27675ns | 28658ns | -58.16% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 637541 | 2000579 | 0.319 | 1.00× |
| carrier_vert_scatter_vert4 | 416328 | 1030046 | 0.404 | 0.65× |
| carrier_vert_scatter_vert8 | 425121 | 868142 | 0.490 | 0.67× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_scatter_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_scatter_scalar | 0.004 | 41.3% |
| carrier_vert_scatter_vert4 | 0.008 | 83.5% |
| carrier_vert_scatter_vert8 | 0.009 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_scatter_scalar | 69769ns | 69769ns | base |
| carrier_vert_scatter_vert4 | 35499ns | 35499ns | -49.12% |
| carrier_vert_scatter_vert8 | 30532ns | 30532ns | -56.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 67025ns | base | --- | [66369, 68670] | --- | --- | --- | --- |
| carrier_vert_scatter_vert4 | 33133ns | -33918.5ns (-50.6%) | [-35705, -33150]ns | [32627, 33531] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_scatter_vert8 | 28081ns | -39206.0ns (-58.5%) | [-40429, -37895]ns | [27794, 28658] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_scatter_scalar | carrier_vert_scatter_vert4 | carrier_vert_scatter_vert8 |
|---|---|---|---|
| 1 | 66585ns | -51.2% | -58.0% |
| 2 | 69181ns | -52.1% | -59.2% |
| 3 | 68159ns | -51.9% | -58.5% |
| 4 | 66424ns | -49.8% | -58.0% |
| 5 | 66314ns | -50.1% | -56.2% |
| 6 | 67465ns | -50.0% | -59.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_scatter_scalar | 0.026 | ok |
| carrier_vert_scatter_vert4 | -0.112 | ok |
| carrier_vert_scatter_vert8 | -0.617 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_vert_scatter_vert4**: won 6/6, lost 0/6
- **carrier_vert_scatter_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 135256.0ns | 67354.7ns | 200.8% | HIGH |
| carrier_vert_scatter_vert4 | 99293.4ns | 33097.0ns | 300.0% | HIGH |
| carrier_vert_scatter_vert8 | 106204.4ns | 28177.9ns | 376.9% | HIGH |

## Distribution (algo ns)

```
carrier_vert_scatter_scalar (n=6, range 66314.2-68669.8 ns)
  66314.2 |########################################
  66432.0 |
  66549.8 |####################
  66667.5 |
  66785.3 |
  66903.1 |
  67020.9 |
  67138.7 |
  67256.4 |
  67374.2 |####################
  67492.0 |
  67609.8 |
  67727.6 |
  67845.3 |
  67963.1 |
  68080.9 |####################
  68198.7 |
  68316.5 |
  68434.2 |
  68552.0 |
  (0 below, 1 above range)

carrier_vert_scatter_vert4 (n=6, range 32474.6-33531.3 ns)
  32474.6 |####################
  32527.4 |
  32580.3 |
  32633.1 |
  32685.9 |
  32738.8 |####################
  32791.6 |
  32844.4 |
  32897.3 |
  32950.1 |
  33002.9 |
  33055.8 |
  33108.6 |########################################
  33161.5 |
  33214.3 |
  33267.1 |
  33320.0 |####################
  33372.8 |
  33425.6 |
  33478.5 |
  (0 below, 1 above range)

carrier_vert_scatter_vert8 (n=6, range 27675.0-28658.0 ns)
  27675.0 |########################################
  27724.1 |
  27773.3 |
  27822.4 |
  27871.6 |########################################
  27920.7 |########################################
  27969.9 |
  28019.0 |
  28068.2 |
  28117.3 |
  28166.5 |########################################
  28215.6 |
  28264.8 |########################################
  28313.9 |
  28363.1 |
  28412.2 |
  28461.4 |
  28510.5 |
  28559.7 |
  28608.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_scatter_scalar**: bridge=200.8% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert4**: bridge=300.0% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert8**: bridge=387.5% of algo (FFI overhead may distort results)
