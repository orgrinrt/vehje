# Cheap lowering: const-fold+CSE hash-cons vs recompute

2 variants, 6 samples per variant.
Baseline: **hx_cheaplower__fold**

## Highlights

Baseline for all deltas below: **hx_cheaplower__fold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_cheaplower__fold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_cheaplower__fold has the worst median (857 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_cheaplower__recompute at 346 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_cheaplower__recompute dominates: 148% faster than the next best (hx_cheaplower__fold)

hx_cheaplower__recompute (346 ns) leads hx_cheaplower__fold (857 ns) by 148%, a clear separation rather than a photo finish. CV 5.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_cheaplower__recompute beats baseline by 60% (significant)

hx_cheaplower__recompute is -514 ns (60%) faster than baseline hx_cheaplower__fold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: hx_cheaplower__recompute** at 346.1 ns median (-59.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.48x (fastest 346.1 ns, slowest 857.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_cheaplower__fold | 3149ns | 3085ns | 2958ns | 3067ns | 3368ns | base |
| hx_cheaplower__recompute | 2639ns | 2577ns | 2506ns | 2559ns | 2824ns | -16.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_cheaplower__fold | 875ns | 825ns | 934ns | base | 1.170 |
| hx_cheaplower__recompute | 354ns | 337ns | 378ns | -59.58% | 2.895 |

## Performance model

- Peak throughput: **3.041 Gops/s** (hx_cheaplower__recompute; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_cheaplower__fold | 1.195 | 39.3% |
| hx_cheaplower__recompute | 2.959 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_cheaplower__fold | 3149ns | 3149ns | base |
| hx_cheaplower__recompute | 2639ns | 2639ns | -16.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_cheaplower__fold | 857ns | base | --- | [834, 934] | --- | --- | --- | --- |
| hx_cheaplower__recompute | 346ns | -513.8ns (-59.9%) | [-573, -478]ns | [338, 378] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_cheaplower__fold | hx_cheaplower__recompute |
|---|---|---|
| 1 | 825ns | -54.7% |
| 2 | 870ns | -59.5% |
| 3 | 868ns | -61.0% |
| 4 | 844ns | -59.7% |
| 5 | 847ns | -60.2% |
| 6 | 999ns | -61.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_cheaplower__fold | -0.105 | ok |
| hx_cheaplower__recompute | -0.019 | ok |

**Consistency summary:**

- **hx_cheaplower__recompute**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_cheaplower__fold | 2.8ns | 875.2ns | 0.3% |  |
| hx_cheaplower__recompute | 3.3ns | 353.8ns | 0.9% |  |

## Distribution (algo ns)

```
hx_cheaplower__fold (n=6, range 824.6-934.2 ns)
    824.6 |########################################
    830.1 |
    835.6 |
    841.0 |########################################
    846.5 |########################################
    852.0 |
    857.5 |
    862.9 |########################################
    868.4 |########################################
    873.9 |
    879.4 |
    884.9 |
    890.3 |
    895.8 |
    901.3 |
    906.8 |
    912.2 |
    917.7 |
    923.2 |
    928.7 |
  (0 below, 1 above range)

hx_cheaplower__recompute (n=6, range 336.7-377.8 ns)
    336.7 |########################################
    338.8 |####################
    340.8 |
    342.9 |
    344.9 |
    347.0 |
    349.0 |
    351.1 |####################
    353.1 |
    355.2 |
    357.2 |
    359.3 |
    361.3 |
    363.4 |
    365.4 |
    367.5 |
    369.5 |
    371.6 |
    373.6 |####################
    375.7 |
  (0 below, 1 above range)

```
