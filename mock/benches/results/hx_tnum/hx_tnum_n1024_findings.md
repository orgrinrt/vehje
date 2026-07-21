# tnum abstract arithmetic: add/and/or transfer functions

2 variants, 6 samples per variant.
Baseline: **hx_tnum__tadd**

## Highlights

Baseline for all deltas below: **hx_tnum__tadd**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_tnum__tadd) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_tnum__tadd has the worst median (2.00 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_tnum__tor at 834 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_tnum__tor dominates: 140% faster than the next best (hx_tnum__tadd)

hx_tnum__tor (834 ns) leads hx_tnum__tadd (2.00 us) by 140%, a clear separation rather than a photo finish. CV 6.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_tnum__tor beats baseline by 58% (significant)

hx_tnum__tor is -1.17 us (58%) faster than baseline hx_tnum__tadd, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hx_tnum__tor is fastest but the noisiest (CV 6.5%)

hx_tnum__tor wins on median (834 ns) yet has the highest variance (CV 6.5%), while hx_tnum__tadd is the steadiest (CV 6.2%, 2.00 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: hx_tnum__tor** at 833.8 ns median (-58.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.40x (fastest 833.8 ns, slowest 1999.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_tnum__tadd | 4414ns | 4559ns | 3830ns | 4476ns | 4613ns | base |
| hx_tnum__tor | 3320ns | 3415ns | 2819ns | 3412ns | 3430ns | -24.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_tnum__tadd | 1936ns | 1680ns | 2025ns | base | 0.529 |
| hx_tnum__tor | 811ns | 691ns | 838ns | -58.11% | 1.263 |

## Performance model

- Peak throughput: **1.482 Gops/s** (hx_tnum__tor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_tnum__tadd | 0.512 | 34.6% |
| hx_tnum__tor | 1.228 | 82.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_tnum__tadd | 4414ns | 4414ns | base |
| hx_tnum__tor | 3320ns | 3320ns | -24.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_tnum__tadd | 1999ns | base | --- | [1785, 2025] | --- | --- | --- | --- |
| hx_tnum__tor | 834ns | -1166.0ns (-58.3%) | [-1187, -1023]ns | [761, 838] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_tnum__tadd | hx_tnum__tor |
|---|---|---|
| 1 | 1680ns | -58.9% |
| 2 | 1980ns | -58.0% |
| 3 | 2026ns | -58.7% |
| 4 | 2018ns | -58.7% |
| 5 | 1890ns | -55.8% |
| 6 | 2023ns | -58.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_tnum__tadd | -0.084 | ok |
| hx_tnum__tor | -0.012 | ok |

**Consistency summary:**

- **hx_tnum__tor**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_tnum__tadd | 3.3ns | 1936.4ns | 0.2% |  |
| hx_tnum__tor | 3.0ns | 811.1ns | 0.4% |  |

## Distribution (algo ns)

```
hx_tnum__tadd (n=6, range 1680.4-2024.8 ns)
   1680.4 |####################
   1697.6 |
   1714.8 |
   1732.1 |
   1749.3 |
   1766.5 |
   1783.7 |
   1800.9 |
   1818.1 |
   1835.4 |
   1852.6 |
   1869.8 |
   1887.0 |####################
   1904.2 |
   1921.4 |
   1938.7 |
   1955.9 |
   1973.1 |####################
   1990.3 |
   2007.5 |########################################
  (0 below, 1 above range)

hx_tnum__tor (n=6, range 690.8-838.2 ns)
    690.8 |##########
    698.2 |
    705.5 |
    712.9 |
    720.3 |
    727.6 |
    735.0 |
    742.4 |
    749.7 |
    757.1 |
    764.5 |
    771.8 |
    779.2 |
    786.6 |
    793.9 |
    801.3 |
    808.7 |
    816.0 |
    823.4 |
    830.8 |########################################
  (0 below, 1 above range)

```
