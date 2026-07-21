# Cheap lowering: const-fold+CSE hash-cons vs recompute

2 variants, 6 samples per variant.
Baseline: **hx_cheaplower__fold**

## Highlights

Baseline for all deltas below: **hx_cheaplower__fold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_cheaplower__fold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_cheaplower__fold has the worst median (3.86 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_cheaplower__recompute at 1.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_cheaplower__recompute dominates: 151% faster than the next best (hx_cheaplower__fold)

hx_cheaplower__recompute (1.53 us) leads hx_cheaplower__fold (3.86 us) by 151%, a clear separation rather than a photo finish. CV 7.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_cheaplower__recompute beats baseline by 60% (significant)

hx_cheaplower__recompute is -2.32 us (60%) faster than baseline hx_cheaplower__fold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: hx_cheaplower__recompute** at 1533.1 ns median (-60.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.51x (fastest 1533.1 ns, slowest 3855.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_cheaplower__fold | 6322ns | 6440ns | 5305ns | 6305ns | 6857ns | base |
| hx_cheaplower__recompute | 4019ns | 4119ns | 3445ns | 4027ns | 4294ns | -36.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_cheaplower__fold | 3782ns | 3177ns | 4097ns | base | 1.083 |
| hx_cheaplower__recompute | 1499ns | 1286ns | 1602ns | -60.37% | 2.733 |

## Performance model

- Peak throughput: **3.186 Gops/s** (hx_cheaplower__recompute; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_cheaplower__fold | 1.062 | 33.4% |
| hx_cheaplower__recompute | 2.672 | 83.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_cheaplower__fold | 6322ns | 6322ns | base |
| hx_cheaplower__recompute | 4019ns | 4019ns | -36.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_cheaplower__fold | 3855ns | base | --- | [3394, 4097] | --- | --- | --- | --- |
| hx_cheaplower__recompute | 1533ns | -2321.1ns (-60.2%) | [-2496, -2033]ns | [1361, 1602] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_cheaplower__fold | hx_cheaplower__recompute |
|---|---|---|
| 1 | 3177ns | -59.5% |
| 2 | 4002ns | -61.7% |
| 3 | 3853ns | -60.2% |
| 4 | 3611ns | -60.2% |
| 5 | 3858ns | -60.2% |
| 6 | 4192ns | -60.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_cheaplower__fold | -0.179 | ok |
| hx_cheaplower__recompute | -0.053 | ok |

**Consistency summary:**

- **hx_cheaplower__recompute**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_cheaplower__fold | 3.0ns | 3782.0ns | 0.1% |  |
| hx_cheaplower__recompute | 3.6ns | 1498.8ns | 0.2% |  |

## Distribution (algo ns)

```
hx_cheaplower__fold (n=6, range 3176.7-4097.1 ns)
   3176.7 |####################
   3222.7 |
   3268.7 |
   3314.8 |
   3360.8 |
   3406.8 |
   3452.8 |
   3498.8 |
   3544.9 |
   3590.9 |####################
   3636.9 |
   3682.9 |
   3728.9 |
   3775.0 |
   3821.0 |########################################
   3867.0 |
   3913.0 |
   3959.0 |####################
   4005.1 |
   4051.1 |
  (0 below, 1 above range)

hx_cheaplower__recompute (n=6, range 1285.8-1602.3 ns)
   1285.8 |#############
   1301.6 |
   1317.5 |
   1333.3 |
   1349.1 |
   1364.9 |
   1380.8 |
   1396.6 |
   1412.4 |
   1428.2 |#############
   1444.1 |
   1459.9 |
   1475.7 |
   1491.5 |
   1507.4 |
   1523.2 |########################################
   1539.0 |
   1554.8 |
   1570.7 |
   1586.5 |
  (0 below, 1 above range)

```
