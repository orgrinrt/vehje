# Cheap lowering: const-fold+CSE hash-cons vs recompute

2 variants, 6 samples per variant.
Baseline: **hx_cheaplower__fold**

## Highlights

Baseline for all deltas below: **hx_cheaplower__fold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_cheaplower__fold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_cheaplower__fold has the worst median (107 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_cheaplower__recompute at 65 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_cheaplower__recompute dominates: 64% faster than the next best (hx_cheaplower__fold)

hx_cheaplower__recompute (65 ns) leads hx_cheaplower__fold (107 ns) by 64%, a clear separation rather than a photo finish. CV 14.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_cheaplower__recompute beats baseline by 42% (significant)

hx_cheaplower__recompute is -45 ns (42%) faster than baseline hx_cheaplower__fold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hx_cheaplower__recompute is fastest but the noisiest (CV 14.4%)

hx_cheaplower__recompute wins on median (65 ns) yet has the highest variance (CV 14.4%), while hx_cheaplower__fold is the steadiest (CV 14.2%, 107 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hx_cheaplower__fold is inconsistent: worst-20% is 1.5x its best-20%

hx_cheaplower__fold's best 20% of batches run at 80 ns but its worst 20% at 123 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hx_cheaplower__recompute** at 65.2 ns median (-39.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.64x (fastest 65.2 ns, slowest 107.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_cheaplower__fold | 3292ns | 3384ns | 2327ns | 3310ns | 3749ns | base |
| hx_cheaplower__recompute | 3562ns | 3310ns | 3122ns | 3250ns | 4249ns | +8.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_cheaplower__fold | 107ns | 80ns | 123ns | base | 0.598 |
| hx_cheaplower__recompute | 70ns | 61ns | 82ns | -34.85% | 0.917 |

## Performance model

- Peak throughput: **1.046 Gops/s** (hx_cheaplower__recompute; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_cheaplower__fold | 0.598 | 57.1% |
| hx_cheaplower__recompute | 0.982 | 93.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_cheaplower__fold | 3292ns | 3292ns | base |
| hx_cheaplower__recompute | 3562ns | 3562ns | +8.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_cheaplower__fold | 107ns | base | --- | [91, 123] | --- | --- | --- | --- |
| hx_cheaplower__recompute | 65ns | -44.9ns (-41.9%) | [-50, -17]ns | [62, 82] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_cheaplower__fold | hx_cheaplower__recompute |
|---|---|---|
| 1 | 80ns | +8.3% |
| 2 | 131ns | -39.8% |
| 3 | 115ns | -41.4% |
| 4 | 108ns | -43.1% |
| 5 | 103ns | -39.9% |
| 6 | 107ns | -40.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_cheaplower__fold | -0.339 | moderate- |
| hx_cheaplower__recompute | 0.492 | moderate+ |

**Consistency summary:**

- **hx_cheaplower__recompute**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_cheaplower__fold | 4.5ns | 107.1ns | 4.2% |  |
| hx_cheaplower__recompute | 5.0ns | 69.8ns | 7.1% | HIGH |

## Distribution (algo ns)

```
hx_cheaplower__fold (n=6, range 79.6-122.7 ns)
     79.6 |####################
     81.8 |
     83.9 |
     86.1 |
     88.2 |
     90.4 |
     92.5 |
     94.7 |
     96.8 |
     99.0 |
    101.2 |####################
    103.3 |
    105.5 |########################################
    107.6 |
    109.8 |
    111.9 |
    114.1 |####################
    116.2 |
    118.4 |
    120.5 |
  (0 below, 1 above range)

hx_cheaplower__recompute (n=6, range 61.2-82.5 ns)
     61.2 |########################################
     62.3 |####################
     63.3 |
     64.4 |
     65.5 |
     66.5 |####################
     67.6 |
     68.6 |
     69.7 |
     70.8 |
     71.8 |
     72.9 |
     74.0 |
     75.0 |
     76.1 |
     77.1 |
     78.2 |####################
     79.3 |
     80.3 |
     81.4 |
  (0 below, 1 above range)

```
