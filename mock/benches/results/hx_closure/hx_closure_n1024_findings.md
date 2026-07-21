# Closure representation: flat-capture vs linked-env

2 variants, 6 samples per variant.
Baseline: **hx_closure__linked**

## Highlights

Baseline for all deltas below: **hx_closure__linked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_closure__linked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_closure__linked has the worst median (333 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_closure__flat at 326 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (7 ns) is smaller than the fastest variant's own run-to-run std-dev (21 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### hx_closure__flat's edge over baseline is significant but tiny (4 ns, 1.32%)

hx_closure__flat differs from baseline hx_closure__linked by 4 ns (1.32%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: hx_closure__flat** at 326.5 ns median (-2.1% vs baseline)
- Spread: 1.02x (fastest 326.5 ns, slowest 333.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_closure__flat | 2911ns | 2860ns | 2715ns | 2816ns | 3152ns | +0.42% |
| hx_closure__linked | 2899ns | 2906ns | 2388ns | 2903ns | 3148ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_closure__flat | 333ns | 310ns | 361ns | +0.28% | 3.078 |
| hx_closure__linked | 332ns | 273ns | 360ns | base | 3.087 |

## Performance model

- Peak throughput: **3.747 Gops/s** (hx_closure__linked; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_closure__flat | 3.137 | 83.7% |
| hx_closure__linked | 3.072 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_closure__flat | 2911ns | 2911ns | +0.42% |
| hx_closure__linked | 2899ns | 2899ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_closure__linked | 333ns | base | --- | [302, 360] | --- | --- | --- | --- |
| hx_closure__flat | 326ns | no significant difference | [-34, +32]ns | [311, 361] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_closure__linked | hx_closure__flat |
|---|---|---|
| 1 | 273ns | +13.3% |
| 2 | 336ns | +8.0% |
| 3 | 331ns | +8.4% |
| 4 | 359ns | -11.3% |
| 5 | 330ns | -5.4% |
| 6 | 362ns | -7.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_closure__flat | -0.011 | ok |
| hx_closure__linked | -0.072 | ok |

**Consistency summary:**

- **hx_closure__flat**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_closure__flat | 1.8ns | 332.6ns | 0.5% |  |
| hx_closure__linked | 2.7ns | 331.7ns | 0.8% |  |

## Distribution (algo ns)

```
hx_closure__flat (n=6, range 309.6-360.6 ns)
    309.6 |########################################
    312.2 |
    314.7 |
    317.2 |####################
    319.8 |
    322.4 |
    324.9 |
    327.5 |
    330.0 |
    332.6 |####################
    335.1 |
    337.7 |
    340.2 |
    342.8 |
    345.3 |
    347.9 |
    350.4 |
    353.0 |
    355.5 |
    358.1 |####################
  (0 below, 1 above range)

hx_closure__linked (n=6, range 273.3-360.2 ns)
    273.3 |####################
    277.6 |
    282.0 |
    286.3 |
    290.7 |
    295.0 |
    299.4 |
    303.7 |
    308.1 |
    312.4 |
    316.8 |
    321.1 |
    325.4 |
    329.8 |########################################
    334.1 |####################
    338.5 |
    342.8 |
    347.2 |
    351.5 |
    355.9 |####################
  (0 below, 1 above range)

```
