# Cheap lowering: const-fold+CSE hash-cons vs recompute

2 variants, 6 samples per variant.
Baseline: **hx_cheaplower__fold**

## Highlights

Baseline for all deltas below: **hx_cheaplower__fold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_cheaplower__fold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_cheaplower__fold has the worst median (13.94 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_cheaplower__recompute at 5.87 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_cheaplower__recompute dominates: 138% faster than the next best (hx_cheaplower__fold)

hx_cheaplower__recompute (5.87 us) leads hx_cheaplower__fold (13.94 us) by 138%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_cheaplower__recompute beats baseline by 59% (significant)

hx_cheaplower__recompute is -8.21 us (59%) faster than baseline hx_cheaplower__fold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: hx_cheaplower__recompute** at 5867.2 ns median (-57.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.38x (fastest 5867.2 ns, slowest 13940.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_cheaplower__fold | 16465ns | 16279ns | 14811ns | 16063ns | 17893ns | base |
| hx_cheaplower__recompute | 8225ns | 8373ns | 7501ns | 8137ns | 8719ns | -50.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_cheaplower__fold | 14097ns | 12681ns | 15319ns | base | 1.162 |
| hx_cheaplower__recompute | 5754ns | 5255ns | 6070ns | -59.18% | 2.847 |

## Performance model

- Peak throughput: **3.118 Gops/s** (hx_cheaplower__recompute; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_cheaplower__fold | 1.175 | 37.7% |
| hx_cheaplower__recompute | 2.792 | 89.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_cheaplower__fold | 16465ns | 16465ns | base |
| hx_cheaplower__recompute | 8225ns | 8225ns | -50.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_cheaplower__fold | 13940ns | base | --- | [13031, 15319] | --- | --- | --- | --- |
| hx_cheaplower__recompute | 5867ns | -8212.5ns (-58.9%) | [-9249, -7566]ns | [5325, 6070] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_cheaplower__fold | hx_cheaplower__recompute |
|---|---|---|
| 1 | 12681ns | -55.2% |
| 2 | 15320ns | -60.3% |
| 3 | 14324ns | -57.7% |
| 4 | 15318ns | -60.4% |
| 5 | 13381ns | -60.7% |
| 6 | 13556ns | -60.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_cheaplower__fold | -0.284 | moderate- |
| hx_cheaplower__recompute | 0.287 | moderate+ |

**Consistency summary:**

- **hx_cheaplower__recompute**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_cheaplower__fold | 3.6ns | 14096.7ns | 0.0% |  |
| hx_cheaplower__recompute | 4.6ns | 5754.2ns | 0.1% |  |

## Distribution (algo ns)

```
hx_cheaplower__fold (n=6, range 12681.2-15318.8 ns)
  12681.2 |########################################
  12813.1 |
  12945.0 |
  13076.8 |
  13208.7 |
  13340.6 |########################################
  13472.5 |########################################
  13604.3 |
  13736.2 |
  13868.1 |
  14000.0 |
  14131.9 |
  14263.7 |########################################
  14395.6 |
  14527.5 |
  14659.4 |
  14791.2 |
  14923.1 |
  15055.0 |
  15186.9 |########################################
  (0 below, 1 above range)

hx_cheaplower__recompute (n=6, range 5255.0-6070.0 ns)
   5255.0 |####################
   5295.8 |
   5336.5 |
   5377.2 |####################
   5418.0 |
   5458.8 |
   5499.5 |
   5540.2 |
   5581.0 |
   5621.8 |
   5662.5 |####################
   5703.2 |
   5744.0 |
   5784.8 |
   5825.5 |
   5866.2 |
   5907.0 |
   5947.8 |
   5988.5 |
   6029.2 |########################################
  (0 below, 1 above range)

```
