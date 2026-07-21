# Effect lattice: thermometer-OR join vs per-lane branch max

2 variants, 6 samples per variant.
Baseline: **hx_effect__thermo**

## Highlights

Baseline for all deltas below: **hx_effect__thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_effect__thermo) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_effect__thermo has the worst median (1.16 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_effect__branchmax at 1.03 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_effect__branchmax dominates: 13% faster than the next best (hx_effect__thermo)

hx_effect__branchmax (1.03 us) leads hx_effect__thermo (1.16 us) by 13%, a clear separation rather than a photo finish. CV 6.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_effect__branchmax is fastest but the noisiest (CV 6.6%)

hx_effect__branchmax wins on median (1.03 us) yet has the highest variance (CV 6.6%), while hx_effect__thermo is the steadiest (CV 4.6%, 1.16 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: hx_effect__branchmax** at 1029.2 ns median (-11.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.13x (fastest 1029.2 ns, slowest 1159.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_effect__branchmax | 3469ns | 3608ns | 2983ns | 3527ns | 3624ns | -7.46% |
| hx_effect__thermo | 3748ns | 3735ns | 3483ns | 3731ns | 3906ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_effect__branchmax | 998ns | 860ns | 1050ns | -14.30% | 1.026 |
| hx_effect__thermo | 1164ns | 1079ns | 1217ns | base | 0.880 |

## Performance model

- Peak throughput: **1.191 Gops/s** (hx_effect__branchmax; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_effect__branchmax | 0.995 | 83.6% |
| hx_effect__thermo | 0.883 | 74.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_effect__branchmax | 3469ns | 3469ns | -7.46% |
| hx_effect__thermo | 3748ns | 3748ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_effect__thermo | 1159ns | base | --- | [1116, 1217] | --- | --- | --- | --- |
| hx_effect__branchmax | 1029ns | -156.2ns (-13.5%) | [-229, -114]ns | [914, 1050] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_effect__thermo | hx_effect__branchmax |
|---|---|---|
| 1 | 1079ns | -20.3% |
| 2 | 1162ns | -9.6% |
| 3 | 1156ns | -16.4% |
| 4 | 1154ns | -10.1% |
| 5 | 1261ns | -19.0% |
| 6 | 1172ns | -10.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_effect__branchmax | -0.293 | moderate- |
| hx_effect__thermo | 0.003 | ok |

**Consistency summary:**

- **hx_effect__branchmax**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_effect__branchmax | 3.3ns | 997.6ns | 0.3% |  |
| hx_effect__thermo | 3.3ns | 1164.1ns | 0.3% |  |

## Distribution (algo ns)

```
hx_effect__branchmax (n=6, range 860.0-1050.2 ns)
    860.0 |########################################
    869.5 |
    879.0 |
    888.5 |
    898.0 |
    907.5 |
    917.1 |
    926.6 |
    936.1 |
    945.6 |
    955.1 |
    964.6 |########################################
    974.1 |
    983.6 |
    993.1 |
   1002.7 |
   1012.2 |########################################
   1021.7 |
   1031.2 |########################################
   1040.7 |########################################
  (0 below, 1 above range)

hx_effect__thermo (n=6, range 1078.8-1216.7 ns)
   1078.8 |########################################
   1085.7 |
   1092.6 |
   1099.5 |
   1106.4 |
   1113.3 |
   1120.2 |
   1127.0 |
   1133.9 |
   1140.8 |
   1147.7 |########################################
   1154.6 |########################################
   1161.5 |########################################
   1168.4 |########################################
   1175.3 |
   1182.2 |
   1189.1 |
   1196.0 |
   1202.9 |
   1209.8 |
  (0 below, 1 above range)

```
