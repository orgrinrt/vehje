# Interpolation output: format-to-temp+copy vs format-in-place

2 variants, 6 samples per variant.
Baseline: **hx_output__inplace**

## Highlights

Baseline for all deltas below: **hx_output__inplace**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_output__inplace) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_output__inplace has the worst median (50.20 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_output__temp at 44.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_output__temp dominates: 14% faster than the next best (hx_output__inplace)

hx_output__temp (44.08 us) leads hx_output__inplace (50.20 us) by 14%, a clear separation rather than a photo finish. CV 7.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_output__temp is fastest but the noisiest (CV 7.1%)

hx_output__temp wins on median (44.08 us) yet has the highest variance (CV 7.1%), while hx_output__inplace is the steadiest (CV 6.9%, 50.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hx_output__temp shows alternating (throttle bounce) (autocorr -0.52)

hx_output__temp's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: hx_output__temp** at 44077.7 ns median (-12.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.14x (fastest 44077.7 ns, slowest 50196.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_output__inplace | 52885ns | 52524ns | 48072ns | 51678ns | 57102ns | base |
| hx_output__temp | 46831ns | 46364ns | 43349ns | 45452ns | 50639ns | -11.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_output__inplace | 50562ns | 45914ns | 54652ns | base | 0.324 |
| hx_output__temp | 44545ns | 41145ns | 48317ns | -11.90% | 0.368 |

## Performance model

- Peak throughput: **0.398 Gops/s** (hx_output__temp; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_output__inplace | 0.326 | 82.0% |
| hx_output__temp | 0.372 | 93.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_output__inplace | 52885ns | 52885ns | base |
| hx_output__temp | 46831ns | 46831ns | -11.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_output__inplace | 50196ns | base | --- | [46837, 54652] | --- | --- | --- | --- |
| hx_output__temp | 44078ns | -5553.3ns (-11.1%) | [-9809, -2688]ns | [41240, 48317] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_output__inplace | hx_output__temp |
|---|---|---|
| 1 | 47760ns | -3.6% |
| 2 | 48554ns | -15.3% |
| 3 | 45914ns | -8.3% |
| 4 | 55760ns | -13.1% |
| 5 | 53545ns | -22.8% |
| 6 | 51839ns | -7.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_output__inplace | 0.142 | ok |
| hx_output__temp | -0.522 | HIGH- (thermal bounce) |

**Consistency summary:**

- **hx_output__temp**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_output__inplace | 4.1ns | 50562.0ns | 0.0% |  |
| hx_output__temp | 3.2ns | 44545.1ns | 0.0% |  |

## Distribution (algo ns)

```
hx_output__inplace (n=6, range 45914.2-54652.5 ns)
  45914.2 |########################################
  46351.1 |
  46788.0 |
  47224.9 |
  47661.9 |########################################
  48098.8 |
  48535.7 |########################################
  48972.6 |
  49409.5 |
  49846.4 |
  50283.3 |
  50720.3 |
  51157.2 |
  51594.1 |########################################
  52031.0 |
  52467.9 |
  52904.8 |
  53341.8 |########################################
  53778.7 |
  54215.6 |
  (0 below, 1 above range)

hx_output__temp (n=6, range 41144.6-48317.1 ns)
  41144.6 |########################################
  41503.2 |
  41861.8 |####################
  42220.5 |
  42579.1 |
  42937.7 |
  43296.3 |
  43655.0 |
  44013.6 |
  44372.2 |
  44730.8 |
  45089.5 |
  45448.1 |
  45806.7 |####################
  46165.3 |
  46524.0 |
  46882.6 |
  47241.2 |
  47599.8 |
  47958.5 |####################
  (0 below, 1 above range)

```
