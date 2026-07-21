# tnum abstract arithmetic: add/and/or transfer functions

2 variants, 6 samples per variant.
Baseline: **hx_tnum__tadd**

## Highlights

Baseline for all deltas below: **hx_tnum__tadd**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_tnum__tadd) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_tnum__tadd has the worst median (30.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_tnum__tor at 12.60 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_tnum__tor dominates: 143% faster than the next best (hx_tnum__tadd)

hx_tnum__tor (12.60 us) leads hx_tnum__tadd (30.57 us) by 143%, a clear separation rather than a photo finish. CV 4.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_tnum__tor beats baseline by 61% (significant)

hx_tnum__tor is -18.64 us (61%) faster than baseline hx_tnum__tadd, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: hx_tnum__tor** at 12596.2 ns median (-58.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.43x (fastest 12596.2 ns, slowest 30566.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_tnum__tadd | 33509ns | 33022ns | 32802ns | 32961ns | 34683ns | base |
| hx_tnum__tor | 14780ns | 15026ns | 13319ns | 15021ns | 15149ns | -55.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_tnum__tadd | 30976ns | 30365ns | 31996ns | base | 0.529 |
| hx_tnum__tor | 12392ns | 11153ns | 12707ns | -59.99% | 1.322 |

## Performance model

- Peak throughput: **1.469 Gops/s** (hx_tnum__tor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_tnum__tadd | 0.536 | 36.5% |
| hx_tnum__tor | 1.301 | 88.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_tnum__tadd | 33509ns | 33509ns | base |
| hx_tnum__tor | 14780ns | 14780ns | -55.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_tnum__tadd | 30566ns | base | --- | [30365, 31996] | --- | --- | --- | --- |
| hx_tnum__tor | 12596ns | -18640.5ns (-61.0%) | [-19383, -17728]ns | [11873, 12707] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_tnum__tadd | hx_tnum__tor |
|---|---|---|
| 1 | 30365ns | -63.3% |
| 2 | 30365ns | -57.9% |
| 3 | 31840ns | -60.4% |
| 4 | 32152ns | -60.7% |
| 5 | 30465ns | -58.7% |
| 6 | 30668ns | -58.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_tnum__tadd | 0.129 | ok |
| hx_tnum__tor | -0.141 | ok |

**Consistency summary:**

- **hx_tnum__tor**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_tnum__tadd | 3.5ns | 30975.7ns | 0.0% |  |
| hx_tnum__tor | 3.4ns | 12392.1ns | 0.0% |  |

## Distribution (algo ns)

```
hx_tnum__tadd (n=6, range 30364.6-31996.1 ns)
  30364.6 |########################################
  30446.2 |####################
  30527.7 |
  30609.3 |####################
  30690.9 |
  30772.5 |
  30854.0 |
  30935.6 |
  31017.2 |
  31098.8 |
  31180.3 |
  31261.9 |
  31343.5 |
  31425.0 |
  31506.6 |
  31588.2 |
  31669.8 |
  31751.3 |
  31832.9 |####################
  31914.5 |
  (0 below, 1 above range)

hx_tnum__tor (n=6, range 11153.3-12706.8 ns)
  11153.3 |#############
  11231.0 |
  11308.7 |
  11386.3 |
  11464.0 |
  11541.7 |
  11619.4 |
  11697.0 |
  11774.7 |
  11852.4 |
  11930.1 |
  12007.8 |
  12085.4 |
  12163.1 |
  12240.8 |
  12318.5 |
  12396.1 |
  12473.8 |
  12551.5 |########################################
  12629.2 |#############
  (0 below, 1 above range)

```
