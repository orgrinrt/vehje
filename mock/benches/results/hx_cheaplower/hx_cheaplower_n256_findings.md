# Cheap lowering: const-fold+CSE hash-cons vs recompute

2 variants, 6 samples per variant.
Baseline: **hx_cheaplower__fold**

## Highlights

Baseline for all deltas below: **hx_cheaplower__fold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_cheaplower__fold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_cheaplower__fold has the worst median (271 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_cheaplower__recompute at 116 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_cheaplower__recompute dominates: 133% faster than the next best (hx_cheaplower__fold)

hx_cheaplower__recompute (116 ns) leads hx_cheaplower__fold (271 ns) by 133%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_cheaplower__recompute beats baseline by 57% (significant)

hx_cheaplower__recompute is -155 ns (57%) faster than baseline hx_cheaplower__fold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: hx_cheaplower__recompute** at 116.5 ns median (-57.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.33x (fastest 116.5 ns, slowest 270.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_cheaplower__fold | 2812ns | 2860ns | 2340ns | 2856ns | 2981ns | base |
| hx_cheaplower__recompute | 2699ns | 2690ns | 2510ns | 2683ns | 2817ns | -4.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_cheaplower__fold | 269ns | 225ns | 290ns | base | 0.953 |
| hx_cheaplower__recompute | 117ns | 110ns | 123ns | -56.33% | 2.181 |

## Performance model

- Peak throughput: **2.319 Gops/s** (hx_cheaplower__recompute; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_cheaplower__fold | 0.945 | 40.8% |
| hx_cheaplower__recompute | 2.198 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_cheaplower__fold | 2812ns | 2812ns | base |
| hx_cheaplower__recompute | 2699ns | 2699ns | -4.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_cheaplower__fold | 271ns | base | --- | [245, 290] | --- | --- | --- | --- |
| hx_cheaplower__recompute | 116ns | -155.4ns (-57.4%) | [-167, -132]ns | [112, 123] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_cheaplower__fold | hx_cheaplower__recompute |
|---|---|---|
| 1 | 225ns | -51.0% |
| 2 | 272ns | -57.2% |
| 3 | 269ns | -57.6% |
| 4 | 280ns | -58.4% |
| 5 | 265ns | -56.2% |
| 6 | 300ns | -56.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_cheaplower__fold | -0.100 | ok |
| hx_cheaplower__recompute | -0.023 | ok |

**Consistency summary:**

- **hx_cheaplower__recompute**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_cheaplower__fold | 2.9ns | 268.8ns | 1.1% |  |
| hx_cheaplower__recompute | 3.1ns | 117.4ns | 2.7% |  |

## Distribution (algo ns)

```
hx_cheaplower__fold (n=6, range 225.4-290.0 ns)
    225.4 |########################################
    228.6 |
    231.9 |
    235.1 |
    238.3 |
    241.6 |
    244.8 |
    248.0 |
    251.2 |
    254.5 |
    257.7 |
    260.9 |
    264.2 |########################################
    267.4 |########################################
    270.6 |########################################
    273.9 |
    277.1 |
    280.3 |########################################
    283.5 |
    286.8 |
  (0 below, 1 above range)

hx_cheaplower__recompute (n=6, range 110.4-123.3 ns)
    110.4 |####################
    111.0 |
    111.7 |
    112.3 |
    113.0 |
    113.6 |####################
    114.3 |
    114.9 |
    115.6 |####################
    116.2 |########################################
    116.9 |
    117.5 |
    118.2 |
    118.8 |
    119.5 |
    120.1 |
    120.8 |
    121.4 |
    122.1 |
    122.7 |
  (0 below, 1 above range)

```
