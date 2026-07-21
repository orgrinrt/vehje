# tnum abstract arithmetic: add/and/or transfer functions

2 variants, 6 samples per variant.
Baseline: **hx_tnum__tadd**

## Highlights

Baseline for all deltas below: **hx_tnum__tadd**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_tnum__tadd) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_tnum__tadd has the worst median (425 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_tnum__tor at 174 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_tnum__tor dominates: 144% faster than the next best (hx_tnum__tadd)

hx_tnum__tor (174 ns) leads hx_tnum__tadd (425 ns) by 144%, a clear separation rather than a photo finish. CV 7.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_tnum__tor beats baseline by 59% (significant)

hx_tnum__tor is -252 ns (59%) faster than baseline hx_tnum__tadd, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hx_tnum__tor is fastest but the noisiest (CV 7.1%)

hx_tnum__tor wins on median (174 ns) yet has the highest variance (CV 7.1%), while hx_tnum__tadd is the steadiest (CV 4.1%, 425 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: hx_tnum__tor** at 173.8 ns median (-59.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.44x (fastest 173.8 ns, slowest 424.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_tnum__tadd | 2664ns | 2635ns | 2571ns | 2617ns | 2780ns | base |
| hx_tnum__tor | 2445ns | 2392ns | 2328ns | 2377ns | 2606ns | -8.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_tnum__tadd | 430ns | 411ns | 449ns | base | 0.596 |
| hx_tnum__tor | 177ns | 168ns | 190ns | -58.72% | 1.443 |

## Performance model

- Peak throughput: **1.525 Gops/s** (hx_tnum__tor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_tnum__tadd | 0.603 | 39.5% |
| hx_tnum__tor | 1.473 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_tnum__tadd | 2664ns | 2664ns | base |
| hx_tnum__tor | 2445ns | 2445ns | -8.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_tnum__tadd | 425ns | base | --- | [415, 449] | --- | --- | --- | --- |
| hx_tnum__tor | 174ns | -252.5ns (-59.4%) | [-259, -245]ns | [169, 190] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_tnum__tadd | hx_tnum__tor |
|---|---|---|
| 1 | 411ns | -58.7% |
| 2 | 428ns | -59.1% |
| 3 | 422ns | -59.1% |
| 4 | 432ns | -59.4% |
| 5 | 420ns | -60.0% |
| 6 | 466ns | -56.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_tnum__tadd | -0.187 | ok |
| hx_tnum__tor | -0.216 | moderate- |

**Consistency summary:**

- **hx_tnum__tor**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_tnum__tadd | 4.5ns | 429.8ns | 1.1% |  |
| hx_tnum__tor | 3.5ns | 177.4ns | 2.0% |  |

## Distribution (algo ns)

```
hx_tnum__tadd (n=6, range 410.8-449.1 ns)
    410.8 |########################################
    412.7 |
    414.6 |
    416.6 |
    418.5 |########################################
    420.4 |########################################
    422.3 |
    424.2 |
    426.1 |########################################
    428.1 |
    430.0 |
    431.9 |########################################
    433.8 |
    435.7 |
    437.6 |
    439.6 |
    441.5 |
    443.4 |
    445.3 |
    447.2 |
  (0 below, 1 above range)

hx_tnum__tor (n=6, range 167.9-189.8 ns)
    167.9 |####################
    169.0 |####################
    170.1 |
    171.2 |
    172.3 |####################
    173.4 |
    174.5 |########################################
    175.6 |
    176.7 |
    177.8 |
    178.9 |
    179.9 |
    181.0 |
    182.1 |
    183.2 |
    184.3 |
    185.4 |
    186.5 |
    187.6 |
    188.7 |
  (0 below, 1 above range)

```
