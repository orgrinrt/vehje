# tnum abstract arithmetic: add/and/or transfer functions

2 variants, 6 samples per variant.
Baseline: **hx_tnum__tadd**

## Highlights

Baseline for all deltas below: **hx_tnum__tadd**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_tnum__tadd) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_tnum__tadd has the worst median (6.94 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_tnum__tor at 3.14 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_tnum__tor dominates: 121% faster than the next best (hx_tnum__tadd)

hx_tnum__tor (3.14 us) leads hx_tnum__tadd (6.94 us) by 121%, a clear separation rather than a photo finish. CV 9.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_tnum__tor beats baseline by 59% (significant)

hx_tnum__tor is -4.07 us (59%) faster than baseline hx_tnum__tadd, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hx_tnum__tor is fastest but the noisiest (CV 9.9%)

hx_tnum__tor wins on median (3.14 us) yet has the highest variance (CV 9.9%), while hx_tnum__tadd is the steadiest (CV 7.3%, 6.94 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hx_tnum__tadd shows alternating (throttle bounce) (autocorr -0.52)

hx_tnum__tadd's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: hx_tnum__tor** at 3138.9 ns median (-54.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.21x (fastest 3138.9 ns, slowest 6939.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_tnum__tadd | 9466ns | 9145ns | 8830ns | 9088ns | 10352ns | base |
| hx_tnum__tor | 5596ns | 5562ns | 4990ns | 5393ns | 6204ns | -40.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_tnum__tadd | 7177ns | 6684ns | 7851ns | base | 0.571 |
| hx_tnum__tor | 3162ns | 2822ns | 3506ns | -55.94% | 1.295 |

## Performance model

- Peak throughput: **1.451 Gops/s** (hx_tnum__tor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_tnum__tadd | 0.590 | 40.7% |
| hx_tnum__tor | 1.305 | 89.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_tnum__tadd | 9466ns | 9466ns | base |
| hx_tnum__tor | 5596ns | 5596ns | -40.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_tnum__tadd | 6940ns | base | --- | [6740, 7851] | --- | --- | --- | --- |
| hx_tnum__tor | 3139ns | -4073.6ns (-58.7%) | [-4345, -3626]ns | [2841, 3506] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_tnum__tadd | hx_tnum__tor |
|---|---|---|
| 1 | 7033ns | -59.3% |
| 2 | 8109ns | -55.0% |
| 3 | 6684ns | -49.7% |
| 4 | 7594ns | -55.7% |
| 5 | 6846ns | -57.4% |
| 6 | 6797ns | -58.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_tnum__tadd | -0.520 | HIGH- (thermal bounce) |
| hx_tnum__tor | 0.040 | ok |

**Consistency summary:**

- **hx_tnum__tor**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_tnum__tadd | 2.5ns | 7177.1ns | 0.0% |  |
| hx_tnum__tor | 3.8ns | 3162.3ns | 0.1% |  |

## Distribution (algo ns)

```
hx_tnum__tadd (n=6, range 6683.7-7851.3 ns)
   6683.7 |########################################
   6742.1 |########################################
   6800.5 |########################################
   6858.8 |
   6917.2 |
   6975.6 |########################################
   7034.0 |
   7092.4 |
   7150.7 |
   7209.1 |
   7267.5 |
   7325.9 |
   7384.3 |
   7442.6 |
   7501.0 |
   7559.4 |########################################
   7617.8 |
   7676.2 |
   7734.5 |
   7792.9 |
  (0 below, 1 above range)

hx_tnum__tor (n=6, range 2822.1-3506.4 ns)
   2822.1 |####################
   2856.3 |####################
   2890.5 |####################
   2924.8 |
   2959.0 |
   2993.2 |
   3027.4 |
   3061.6 |
   3095.8 |
   3130.1 |
   3164.3 |
   3198.5 |
   3232.7 |
   3266.9 |
   3301.1 |
   3335.4 |########################################
   3369.6 |
   3403.8 |
   3438.0 |
   3472.2 |
  (0 below, 1 above range)

```
