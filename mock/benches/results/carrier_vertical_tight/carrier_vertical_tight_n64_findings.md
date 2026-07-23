# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_tight_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_tight_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_tight_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_tight_scalar has the worst median (16.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_tight_vert8 at 7.41 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_tight_vert8 beats baseline by 57% (significant)

carrier_vert_tight_vert8 is -9.32 us (57%) faster than baseline carrier_vert_tight_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_tight_scalar is an outlier: 2.2x slower than the field

carrier_vert_tight_scalar (16.41 us) is 2.2x the fastest (7.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_tight_vert8 is fastest but the noisiest (CV 6.4%)

carrier_vert_tight_vert8 wins on median (7.41 us) yet has the highest variance (CV 6.4%), while carrier_vert_tight_scalar is the steadiest (CV 1.8%, 16.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_vert_tight_vert8** at 7409.8 ns median (-54.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.22x (fastest 7409.8 ns, slowest 16413.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 18731ns | 18622ns | 18385ns | 18554ns | 19170ns | base |
| carrier_vert_tight_vert4 | 10439ns | 10362ns | 10138ns | 10359ns | 10709ns | -44.27% |
| carrier_vert_tight_vert8 | 9895ns | 9794ns | 9300ns | 9638ns | 10577ns | -47.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 16511ns | 16194ns | 16904ns | base | 0.004 |
| carrier_vert_tight_vert4 | 8170ns | 7932ns | 8374ns | -50.52% | 0.008 |
| carrier_vert_tight_vert8 | 7514ns | 7049ns | 8038ns | -54.49% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 354511 | 1151748 | 0.308 | 1.00× |
| carrier_vert_tight_vert4 | 308094 | 787283 | 0.391 | 0.87× |
| carrier_vert_tight_vert8 | 297104 | 600397 | 0.495 | 0.84× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_tight_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_tight_scalar | 0.004 | 42.9% |
| carrier_vert_tight_vert4 | 0.008 | 86.7% |
| carrier_vert_tight_vert8 | 0.009 | 95.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_tight_scalar | 18731ns | 18731ns | base |
| carrier_vert_tight_vert4 | 10439ns | 10439ns | -44.27% |
| carrier_vert_tight_vert8 | 9895ns | 9895ns | -47.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 16414ns | base | --- | [16215, 16904] | --- | --- | --- | --- |
| carrier_vert_tight_vert4 | 8133ns | -8335.4ns (-50.8%) | [-8637, -8052]ns | [8002, 8374] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_tight_vert8 | 7410ns | -9319.8ns (-56.8%) | [-9459, -8213]ns | [7094, 8038] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_tight_scalar | carrier_vert_tight_vert4 | carrier_vert_tight_vert8 |
|---|---|---|---|
| 1 | 16194ns | -49.9% | -47.6% |
| 2 | 16237ns | -49.4% | -53.7% |
| 3 | 16471ns | -50.5% | -56.7% |
| 4 | 16887ns | -49.4% | -56.7% |
| 5 | 16921ns | -52.3% | -55.2% |
| 6 | 16357ns | -51.5% | -56.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_tight_scalar | 0.340 | moderate+ |
| carrier_vert_tight_vert4 | -0.115 | ok |
| carrier_vert_tight_vert8 | 0.023 | ok |

**Consistency summary:**

- **carrier_vert_tight_vert4**: won 6/6, lost 0/6
- **carrier_vert_tight_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 97961.8ns | 16511.1ns | 593.3% | HIGH |
| carrier_vert_tight_vert4 | 90793.2ns | 8169.8ns | 1111.3% | HIGH |
| carrier_vert_tight_vert8 | 89067.8ns | 7513.8ns | 1185.4% | HIGH |

## Distribution (algo ns)

```
carrier_vert_tight_scalar (n=6, range 16194.2-16904.2 ns)
  16194.2 |########################################
  16229.7 |########################################
  16265.2 |
  16300.7 |
  16336.2 |########################################
  16371.7 |
  16407.2 |
  16442.7 |########################################
  16478.2 |
  16513.7 |
  16549.2 |
  16584.7 |
  16620.2 |
  16655.7 |
  16691.2 |
  16726.7 |
  16762.2 |
  16797.7 |
  16833.2 |
  16868.7 |########################################
  (0 below, 1 above range)

carrier_vert_tight_vert4 (n=6, range 7932.1-8374.4 ns)
   7932.1 |########################################
   7954.2 |
   7976.3 |
   7998.4 |
   8020.6 |
   8042.7 |
   8064.8 |########################################
   8086.9 |
   8109.0 |########################################
   8131.1 |########################################
   8153.2 |
   8175.4 |
   8197.5 |########################################
   8219.6 |
   8241.7 |
   8263.8 |
   8285.9 |
   8308.1 |
   8330.2 |
   8352.3 |
  (0 below, 1 above range)

carrier_vert_tight_vert8 (n=6, range 7048.7-8037.8 ns)
   7048.7 |########################################
   7098.2 |########################################
   7147.6 |
   7197.1 |
   7246.5 |
   7296.0 |########################################
   7345.4 |
   7394.9 |
   7444.3 |
   7493.8 |########################################
   7543.2 |########################################
   7592.7 |
   7642.1 |
   7691.6 |
   7741.0 |
   7790.5 |
   7839.9 |
   7889.4 |
   7938.8 |
   7988.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_tight_scalar**: bridge=596.9% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert4**: bridge=1115.0% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert8**: bridge=1199.0% of algo (FFI overhead may distort results)
