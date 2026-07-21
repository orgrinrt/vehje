# Record update: always-copy vs in-place-when-unique (exact-meet)

2 variants, 6 samples per variant.
Baseline: **hx_reuse__reuse**

## Highlights

Baseline for all deltas below: **hx_reuse__reuse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_reuse__reuse) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_reuse__reuse has the worst median (188 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_reuse__copy at 169 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_reuse__copy dominates: 11% faster than the next best (hx_reuse__reuse)

hx_reuse__copy (169 ns) leads hx_reuse__reuse (188 ns) by 11%, a clear separation rather than a photo finish. CV 7.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: hx_reuse__copy** at 168.8 ns median (-10.0% vs baseline)
- Spread: 1.11x (fastest 168.8 ns, slowest 187.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reuse__copy | 2662ns | 2746ns | 2378ns | 2623ns | 2862ns | -3.42% |
| hx_reuse__reuse | 2756ns | 2991ns | 2271ns | 2758ns | 2995ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reuse__copy | 166ns | 147ns | 180ns | -4.55% | 1.540 |
| hx_reuse__reuse | 174ns | 140ns | 193ns | base | 1.470 |

## Performance model

- Peak throughput: **1.829 Gops/s** (hx_reuse__reuse; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reuse__copy | 1.517 | 83.0% |
| hx_reuse__reuse | 1.365 | 74.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reuse__copy | 2662ns | 2662ns | -3.42% |
| hx_reuse__reuse | 2756ns | 2756ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reuse__reuse | 188ns | base | --- | [142, 193] | --- | --- | --- | --- |
| hx_reuse__copy | 169ns | no significant difference | [-20, +8]ns | [150, 180] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reuse__reuse | hx_reuse__copy |
|---|---|---|
| 1 | 144ns | +6.3% |
| 2 | 195ns | -9.7% |
| 3 | 191ns | -3.7% |
| 4 | 188ns | -8.6% |
| 5 | 187ns | -11.4% |
| 6 | 140ns | +4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reuse__copy | 0.165 | ok |
| hx_reuse__reuse | -0.088 | ok |

**Consistency summary:**

- **hx_reuse__copy**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reuse__copy | 3.3ns | 166.2ns | 2.0% |  |
| hx_reuse__reuse | 3.2ns | 174.2ns | 1.9% |  |

## Distribution (algo ns)

```
hx_reuse__copy (n=6, range 146.7-180.0 ns)
    146.7 |########################################
    148.4 |
    150.0 |
    151.7 |########################################
    153.4 |
    155.0 |
    156.7 |
    158.4 |
    160.0 |
    161.7 |
    163.3 |
    165.0 |########################################
    166.7 |
    168.3 |
    170.0 |
    171.7 |########################################
    173.3 |
    175.0 |########################################
    176.7 |
    178.3 |
  (0 below, 1 above range)

hx_reuse__reuse (n=6, range 140.0-192.9 ns)
    140.0 |########################################
    142.6 |########################################
    145.3 |
    147.9 |
    150.6 |
    153.2 |
    155.9 |
    158.5 |
    161.2 |
    163.8 |
    166.4 |
    169.1 |
    171.7 |
    174.4 |
    177.0 |
    179.7 |
    182.3 |
    185.0 |########################################
    187.6 |########################################
    190.3 |########################################
  (0 below, 1 above range)

```
