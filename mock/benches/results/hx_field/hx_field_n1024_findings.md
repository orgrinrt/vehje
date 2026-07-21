# Field access: direct-offset vs linear-scan vs hash-lookup vs inline-cache

3 variants, 6 samples per variant.
Baseline: **hx_field__direct**

## Highlights

Baseline for all deltas below: **hx_field__direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_field__direct dominates: 132% faster than the next best (hx_field__linear)

hx_field__direct (289 ns) leads hx_field__linear (671 ns) by 132%, a clear separation rather than a photo finish. CV 9.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_field__hash is an outlier: 2.3x slower than the field

hx_field__hash (673 ns) is 2.3x the fastest (289 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (hx_field__direct)

The baseline hx_field__direct is the fastest (289 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_field__direct) is the fastest** at 288.9 ns median
- 2 variants significantly slower than baseline
- Spread: 2.33x (fastest 288.9 ns, slowest 673.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_field__direct | 2702ns | 2686ns | 2406ns | 2595ns | 3009ns | base |
| hx_field__hash | 3081ns | 3036ns | 2760ns | 2948ns | 3442ns | +14.05% |
| hx_field__linear | 3067ns | 3031ns | 2764ns | 2943ns | 3405ns | +13.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_field__direct | 292ns | 263ns | 324ns | base | 3.507 |
| hx_field__hash | 684ns | 614ns | 765ns | +134.37% | 1.497 |
| hx_field__linear | 679ns | 612ns | 753ns | +132.42% | 1.509 |

## Performance model

- Peak throughput: **3.895 Gops/s** (hx_field__direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_field__direct | 3.544 | 91.0% |
| hx_field__hash | 1.521 | 39.1% |
| hx_field__linear | 1.526 | 39.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_field__direct | 2702ns | 2702ns | base |
| hx_field__hash | 3081ns | 3081ns | +14.05% |
| hx_field__linear | 3067ns | 3067ns | +13.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_field__direct | 289ns | base | --- | [263, 324] | --- | --- | --- | --- |
| hx_field__hash | 673ns | +385.1ns (+133.3%) | [+350, +441]ns | [615, 765] | YES | 0.0313 | 0.0313 | 0 |
| hx_field__linear | 671ns | +371.5ns (+128.6%) | [+347, +441]ns | [612, 753] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_field__direct | hx_field__hash | hx_field__linear |
|---|---|---|---|
| 1 | 266ns | +131.3% | +130.1% |
| 2 | 263ns | +133.5% | +132.7% |
| 3 | 264ns | +133.3% | +131.9% |
| 4 | 312ns | +134.7% | +148.6% |
| 5 | 312ns | +134.3% | +134.3% |
| 6 | 335ns | +138.1% | +117.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_field__direct | 0.454 | moderate+ |
| hx_field__hash | 0.436 | moderate+ |
| hx_field__linear | 0.359 | moderate+ |

**Consistency summary:**

- **hx_field__hash**: won 0/6, lost 6/6
- **hx_field__linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_field__direct | 2.8ns | 292.0ns | 0.9% |  |
| hx_field__hash | 3.1ns | 684.2ns | 0.5% |  |
| hx_field__linear | 2.4ns | 678.6ns | 0.3% |  |

## Distribution (algo ns)

```
hx_field__direct (n=6, range 262.9-323.6 ns)
    262.9 |########################################
    265.9 |####################
    269.0 |
    272.0 |
    275.0 |
    278.1 |
    281.1 |
    284.1 |
    287.2 |
    290.2 |
    293.2 |
    296.3 |
    299.3 |
    302.3 |
    305.4 |
    308.4 |
    311.4 |########################################
    314.5 |
    317.5 |
    320.5 |
  (0 below, 1 above range)

hx_field__hash (n=6, range 613.8-765.0 ns)
    613.8 |########################################
    621.4 |
    628.9 |
    636.5 |
    644.0 |
    651.6 |
    659.2 |
    666.7 |
    674.3 |
    681.8 |
    689.4 |
    697.0 |
    704.5 |
    712.1 |
    719.6 |
    727.2 |##########################
    734.8 |
    742.3 |
    749.9 |
    757.4 |
  (0 below, 1 above range)

hx_field__linear (n=6, range 611.7-753.1 ns)
    611.7 |########################################
    618.8 |
    625.8 |
    632.9 |
    640.0 |
    647.0 |
    654.1 |
    661.2 |
    668.3 |
    675.3 |
    682.4 |
    689.5 |
    696.5 |
    703.6 |
    710.7 |
    717.8 |
    724.8 |##########################
    731.9 |
    739.0 |
    746.0 |
  (0 below, 1 above range)

```
