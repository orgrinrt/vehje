# Effect lattice: thermometer-OR join vs per-lane branch max

2 variants, 6 samples per variant.
Baseline: **hx_effect__thermo**

## Highlights

Baseline for all deltas below: **hx_effect__thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_effect__thermo) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_effect__thermo has the worst median (17.76 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_effect__branchmax at 14.93 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_effect__branchmax dominates: 19% faster than the next best (hx_effect__thermo)

hx_effect__branchmax (14.93 us) leads hx_effect__thermo (17.76 us) by 19%, a clear separation rather than a photo finish. CV 7.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_effect__branchmax is fastest but the noisiest (CV 7.1%)

hx_effect__branchmax wins on median (14.93 us) yet has the highest variance (CV 7.1%), while hx_effect__thermo is the steadiest (CV 6.4%, 17.76 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hx_effect__branchmax shows warm-up / thermal drift (autocorr +0.55)

hx_effect__branchmax's per-pass series has lag-1 autocorrelation +0.55, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: hx_effect__branchmax** at 14929.8 ns median (-15.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.19x (fastest 14929.8 ns, slowest 17755.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_effect__branchmax | 17022ns | 17389ns | 15352ns | 16719ns | 18311ns | -13.07% |
| hx_effect__thermo | 19582ns | 20291ns | 17437ns | 19554ns | 20695ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_effect__branchmax | 14616ns | 13193ns | 15723ns | -14.71% | 1.121 |
| hx_effect__thermo | 17137ns | 15263ns | 18106ns | base | 0.956 |

## Performance model

- Peak throughput: **1.242 Gops/s** (hx_effect__branchmax; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_effect__branchmax | 1.097 | 88.4% |
| hx_effect__thermo | 0.923 | 74.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_effect__branchmax | 17022ns | 17022ns | -13.07% |
| hx_effect__thermo | 19582ns | 19582ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_effect__thermo | 17756ns | base | --- | [15548, 18106] | --- | --- | --- | --- |
| hx_effect__branchmax | 14930ns | -2365.1ns (-13.3%) | [-3176, -2020]ns | [13196, 15723] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_effect__thermo | hx_effect__branchmax |
|---|---|---|
| 1 | 15834ns | -16.6% |
| 2 | 15263ns | -13.6% |
| 3 | 18138ns | -19.0% |
| 4 | 18074ns | -16.1% |
| 5 | 17677ns | -11.1% |
| 6 | 17835ns | -11.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_effect__branchmax | 0.554 | HIGH+ (drift/warm-up) |
| hx_effect__thermo | 0.304 | moderate+ |

**Consistency summary:**

- **hx_effect__branchmax**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_effect__branchmax | 3.0ns | 14616.2ns | 0.0% |  |
| hx_effect__thermo | 2.6ns | 17136.7ns | 0.0% |  |

## Distribution (algo ns)

```
hx_effect__branchmax (n=6, range 13192.9-15722.8 ns)
  13192.9 |########################################
  13319.4 |
  13445.9 |
  13572.4 |
  13698.9 |
  13825.4 |
  13951.9 |
  14078.3 |
  14204.8 |
  14331.3 |
  14457.8 |
  14584.3 |####################
  14710.8 |
  14837.3 |
  14963.8 |
  15090.3 |####################
  15216.8 |
  15343.3 |
  15469.8 |
  15596.3 |####################
  (0 below, 1 above range)

hx_effect__thermo (n=6, range 15263.3-18105.8 ns)
  15263.3 |########################################
  15405.4 |
  15547.6 |
  15689.7 |
  15831.8 |########################################
  15973.9 |
  16116.1 |
  16258.2 |
  16400.3 |
  16542.4 |
  16684.6 |
  16826.7 |
  16968.8 |
  17111.0 |
  17253.1 |
  17395.2 |
  17537.3 |########################################
  17679.5 |
  17821.6 |########################################
  17963.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **hx_effect__branchmax**: autocorrelation=0.55 (measurement drift or warm-up artifact)
