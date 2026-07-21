# Field access: direct-offset vs linear-scan vs hash-lookup vs inline-cache

3 variants, 6 samples per variant.
Baseline: **hx_field__direct**

## Highlights

Baseline for all deltas below: **hx_field__direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_field__direct dominates: 153% faster than the next best (hx_field__hash)

hx_field__direct (4.39 us) leads hx_field__hash (11.11 us) by 153%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_field__linear is an outlier: 2.5x slower than the field

hx_field__linear (11.18 us) is 2.5x the fastest (4.39 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (hx_field__direct)

The baseline hx_field__direct is the fastest (4.39 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_field__direct) is the fastest** at 4394.4 ns median
- 2 variants significantly slower than baseline
- Spread: 2.54x (fastest 4394.4 ns, slowest 11178.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_field__direct | 6963ns | 6978ns | 6510ns | 6968ns | 7181ns | base |
| hx_field__hash | 13410ns | 13690ns | 11291ns | 13688ns | 14052ns | +92.60% |
| hx_field__linear | 13683ns | 13766ns | 12928ns | 13760ns | 13945ns | +96.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_field__direct | 4384ns | 4102ns | 4519ns | base | 3.737 |
| hx_field__hash | 10880ns | 9153ns | 11406ns | +148.15% | 1.506 |
| hx_field__linear | 11106ns | 10465ns | 11328ns | +153.31% | 1.475 |

## Performance model

- Peak throughput: **3.994 Gops/s** (hx_field__direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_field__direct | 3.728 | 93.3% |
| hx_field__hash | 1.475 | 36.9% |
| hx_field__linear | 1.466 | 36.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_field__direct | 6963ns | 6963ns | base |
| hx_field__hash | 13410ns | 13410ns | +92.60% |
| hx_field__linear | 13683ns | 13683ns | +96.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_field__direct | 4394ns | base | --- | [4240, 4519] | --- | --- | --- | --- |
| hx_field__hash | 11106ns | +6712.1ns (+152.7%) | [+5630, +7145]ns | [10128, 11406] | YES | 0.0313 | 0.0313 | 0 |
| hx_field__linear | 11179ns | +6763.4ns (+153.9%) | [+6327, +7075]ns | [10812, 11328] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_field__direct | hx_field__hash | hx_field__linear |
|---|---|---|---|
| 1 | 4378ns | +109.1% | +139.0% |
| 2 | 4421ns | +163.6% | +152.8% |
| 3 | 4617ns | +140.5% | +142.2% |
| 4 | 4403ns | +152.2% | +160.5% |
| 5 | 4102ns | +172.1% | +172.7% |
| 6 | 4385ns | +153.2% | +154.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_field__direct | 0.052 | ok |
| hx_field__hash | -0.259 | moderate- |
| hx_field__linear | 0.041 | ok |

**Consistency summary:**

- **hx_field__hash**: won 0/6, lost 6/6
- **hx_field__linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_field__direct | 3.3ns | 4384.4ns | 0.1% |  |
| hx_field__hash | 2.1ns | 10880.1ns | 0.0% |  |
| hx_field__linear | 3.8ns | 11106.2ns | 0.0% |  |

## Distribution (algo ns)

```
hx_field__direct (n=6, range 4101.7-4519.0 ns)
   4101.7 |####################
   4122.6 |
   4143.4 |
   4164.3 |
   4185.1 |
   4206.0 |
   4226.9 |
   4247.7 |
   4268.6 |
   4289.5 |
   4310.3 |
   4331.2 |
   4352.1 |
   4372.9 |########################################
   4393.8 |####################
   4414.6 |####################
   4435.5 |
   4456.4 |
   4477.2 |
   4498.1 |
  (0 below, 1 above range)

hx_field__hash (n=6, range 9153.3-11405.8 ns)
   9153.3 |##########
   9265.9 |
   9378.6 |
   9491.2 |
   9603.8 |
   9716.4 |
   9829.1 |
   9941.7 |
  10054.3 |
  10166.9 |
  10279.6 |
  10392.2 |
  10504.8 |
  10617.5 |
  10730.1 |
  10842.7 |
  10955.3 |
  11068.0 |########################################
  11180.6 |
  11293.2 |
  (0 below, 1 above range)

hx_field__linear (n=6, range 10464.6-11328.0 ns)
  10464.6 |##########
  10507.8 |
  10550.9 |
  10594.1 |
  10637.3 |
  10680.4 |
  10723.6 |
  10766.8 |
  10809.9 |
  10853.1 |
  10896.3 |
  10939.4 |
  10982.6 |
  11025.8 |
  11068.9 |
  11112.1 |
  11155.3 |########################################
  11198.4 |
  11241.6 |
  11284.8 |
  (0 below, 1 above range)

```
