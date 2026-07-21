# tnum abstract arithmetic: add/and/or transfer functions

2 variants, 6 samples per variant.
Baseline: **hx_tnum__tadd**

## Highlights

Baseline for all deltas below: **hx_tnum__tadd**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_tnum__tadd) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_tnum__tadd has the worst median (115 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_tnum__tor at 52 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_tnum__tor dominates: 120% faster than the next best (hx_tnum__tadd)

hx_tnum__tor (52 ns) leads hx_tnum__tadd (115 ns) by 120%, a clear separation rather than a photo finish. CV 6.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_tnum__tor beats baseline by 56% (significant)

hx_tnum__tor is -64 ns (56%) faster than baseline hx_tnum__tadd, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hx_tnum__tor is fastest but the noisiest (CV 6.9%)

hx_tnum__tor wins on median (52 ns) yet has the highest variance (CV 6.9%), while hx_tnum__tadd is the steadiest (CV 6.3%, 115 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: hx_tnum__tor** at 52.3 ns median (-54.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.20x (fastest 52.3 ns, slowest 115.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_tnum__tadd | 2709ns | 2692ns | 2489ns | 2636ns | 2930ns | base |
| hx_tnum__tor | 2562ns | 2640ns | 2177ns | 2637ns | 2642ns | -5.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_tnum__tadd | 117ns | 108ns | 126ns | base | 0.546 |
| hx_tnum__tor | 51ns | 44ns | 54ns | -56.65% | 1.260 |

## Performance model

- Peak throughput: **1.448 Gops/s** (hx_tnum__tor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_tnum__tadd | 0.557 | 38.4% |
| hx_tnum__tor | 1.224 | 84.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_tnum__tadd | 2709ns | 2709ns | base |
| hx_tnum__tor | 2562ns | 2562ns | -5.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_tnum__tadd | 115ns | base | --- | [110, 126] | --- | --- | --- | --- |
| hx_tnum__tor | 52ns | -64.2ns (-55.8%) | [-73, -62]ns | [46, 54] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_tnum__tadd | hx_tnum__tor |
|---|---|---|
| 1 | 108ns | -59.0% |
| 2 | 112ns | -54.0% |
| 3 | 112ns | -57.4% |
| 4 | 123ns | -57.1% |
| 5 | 129ns | -58.0% |
| 6 | 118ns | -54.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_tnum__tadd | 0.367 | moderate+ |
| hx_tnum__tor | 0.035 | ok |

**Consistency summary:**

- **hx_tnum__tor**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_tnum__tadd | 3.2ns | 117.1ns | 2.8% |  |
| hx_tnum__tor | 2.8ns | 50.8ns | 5.6% | HIGH |

## Distribution (algo ns)

```
hx_tnum__tadd (n=6, range 107.9-126.2 ns)
    107.9 |####################
    108.8 |
    109.7 |
    110.7 |
    111.6 |
    112.5 |########################################
    113.4 |
    114.3 |
    115.2 |
    116.2 |
    117.1 |####################
    118.0 |
    118.9 |
    119.8 |
    120.7 |
    121.7 |
    122.6 |####################
    123.5 |
    124.4 |
    125.3 |
  (0 below, 1 above range)

hx_tnum__tor (n=6, range 44.2-54.0 ns)
     44.2 |########################################
     44.7 |
     45.2 |
     45.7 |
     46.2 |
     46.7 |
     47.1 |
     47.6 |########################################
     48.1 |
     48.6 |
     49.1 |
     49.6 |
     50.1 |
     50.6 |
     51.1 |
     51.5 |########################################
     52.0 |
     52.5 |########################################
     53.0 |
     53.5 |########################################
  (0 below, 1 above range)

```
