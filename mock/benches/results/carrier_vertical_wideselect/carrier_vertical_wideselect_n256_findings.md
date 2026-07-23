# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_wideselect_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_wideselect_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_wideselect_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_wideselect_scalar has the worst median (65.38 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_wideselect_vert8 at 29.91 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_wideselect_vert8 beats baseline by 55% (significant)

carrier_vert_wideselect_vert8 is -35.65 us (55%) faster than baseline carrier_vert_wideselect_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_wideselect_scalar is an outlier: 2.2x slower than the field

carrier_vert_wideselect_scalar (65.38 us) is 2.2x the fastest (29.91 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_wideselect_scalar shows warm-up / thermal drift (autocorr +0.53)

carrier_vert_wideselect_scalar's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_wideselect_vert8** at 29913.5 ns median (-54.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.19x (fastest 29913.5 ns, slowest 65380.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 67872ns | 67612ns | 66959ns | 67557ns | 68800ns | base |
| carrier_vert_wideselect_vert4 | 34110ns | 34145ns | 33281ns | 33882ns | 34867ns | -49.74% |
| carrier_vert_wideselect_vert8 | 32227ns | 32211ns | 31378ns | 32056ns | 32908ns | -52.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 65530ns | 64762ns | 66231ns | base | 0.004 |
| carrier_vert_wideselect_vert4 | 31842ns | 31051ns | 32549ns | -51.41% | 0.008 |
| carrier_vert_wideselect_vert8 | 29912ns | 29204ns | 30559ns | -54.35% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 615527 | 1996460 | 0.308 | 1.00× |
| carrier_vert_wideselect_vert4 | 397492 | 1088163 | 0.365 | 0.65× |
| carrier_vert_wideselect_vert8 | 372581 | 777381 | 0.479 | 0.61× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_wideselect_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.004 | 44.7% |
| carrier_vert_wideselect_vert4 | 0.008 | 91.6% |
| carrier_vert_wideselect_vert8 | 0.009 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_wideselect_scalar | 67872ns | 67872ns | base |
| carrier_vert_wideselect_vert4 | 34110ns | 34110ns | -49.74% |
| carrier_vert_wideselect_vert8 | 32227ns | 32227ns | -52.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 65381ns | base | --- | [64978, 66231] | --- | --- | --- | --- |
| carrier_vert_wideselect_vert4 | 31888ns | -33587.5ns (-51.4%) | [-34563, -32914]ns | [31088, 32549] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_wideselect_vert8 | 29914ns | -35650.6ns (-54.5%) | [-36326, -34876]ns | [29264, 30559] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_wideselect_scalar | carrier_vert_wideselect_vert4 | carrier_vert_wideselect_vert8 |
|---|---|---|---|
| 1 | 65194ns | -52.4% | -55.0% |
| 2 | 64762ns | -50.2% | -53.1% |
| 3 | 65234ns | -51.1% | -54.3% |
| 4 | 65527ns | -51.3% | -55.4% |
| 5 | 66108ns | -52.9% | -53.5% |
| 6 | 66354ns | -50.5% | -54.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.533 | HIGH+ (drift/warm-up) |
| carrier_vert_wideselect_vert4 | -0.458 | moderate- |
| carrier_vert_wideselect_vert8 | -0.419 | moderate- |

**Consistency summary:**

- **carrier_vert_wideselect_vert4**: won 6/6, lost 0/6
- **carrier_vert_wideselect_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 131387.9ns | 65529.9ns | 200.5% | HIGH |
| carrier_vert_wideselect_vert4 | 95443.5ns | 31841.8ns | 299.7% | HIGH |
| carrier_vert_wideselect_vert8 | 90002.1ns | 29912.2ns | 300.9% | HIGH |

## Distribution (algo ns)

```
carrier_vert_wideselect_scalar (n=6, range 64761.7-66231.0 ns)
  64761.7 |########################################
  64835.2 |
  64908.6 |
  64982.1 |
  65055.6 |
  65129.0 |########################################
  65202.5 |########################################
  65276.0 |
  65349.4 |
  65422.9 |
  65496.4 |########################################
  65569.8 |
  65643.3 |
  65716.8 |
  65790.2 |
  65863.7 |
  65937.2 |
  66010.6 |
  66084.1 |########################################
  66157.6 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert4 (n=6, range 31050.8-32549.2 ns)
  31050.8 |####################
  31125.7 |####################
  31200.6 |
  31275.6 |
  31350.5 |
  31425.4 |
  31500.3 |
  31575.2 |
  31650.1 |
  31725.1 |
  31800.0 |
  31874.9 |########################################
  31949.8 |
  32024.7 |
  32099.6 |
  32174.6 |
  32249.5 |####################
  32324.4 |
  32399.3 |
  32474.2 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert8 (n=6, range 29204.2-30558.8 ns)
  29204.2 |########################################
  29271.9 |########################################
  29339.7 |
  29407.4 |
  29475.1 |
  29542.8 |
  29610.6 |
  29678.3 |
  29746.0 |########################################
  29813.7 |
  29881.5 |
  29949.2 |
  30016.9 |########################################
  30084.7 |
  30152.4 |
  30220.1 |
  30287.8 |
  30355.6 |########################################
  30423.3 |
  30491.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_wideselect_scalar**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **carrier_vert_wideselect_scalar**: bridge=200.9% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert4**: bridge=299.2% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert8**: bridge=300.8% of algo (FFI overhead may distort results)
