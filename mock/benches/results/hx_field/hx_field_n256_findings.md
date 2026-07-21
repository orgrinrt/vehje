# Field access: direct-offset vs linear-scan vs hash-lookup vs inline-cache

3 variants, 6 samples per variant.
Baseline: **hx_field__direct**

## Highlights

Baseline for all deltas below: **hx_field__direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_field__direct dominates: 98% faster than the next best (hx_field__linear)

hx_field__direct (108 ns) leads hx_field__linear (213 ns) by 98%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_field__hash is an outlier: 2.0x slower than the field

hx_field__hash (217 ns) is 2.0x the fastest (108 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (hx_field__direct)

The baseline hx_field__direct is the fastest (108 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_field__direct) is the fastest** at 107.7 ns median
- 2 variants significantly slower than baseline
- Spread: 2.02x (fastest 107.7 ns, slowest 217.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_field__direct | 2770ns | 2699ns | 2672ns | 2696ns | 2929ns | base |
| hx_field__hash | 2783ns | 2789ns | 2306ns | 2784ns | 3020ns | +0.50% |
| hx_field__linear | 2798ns | 2804ns | 2596ns | 2737ns | 2991ns | +1.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_field__direct | 107ns | 100ns | 114ns | base | 2.382 |
| hx_field__hash | 215ns | 181ns | 232ns | +100.03% | 1.191 |
| hx_field__linear | 213ns | 198ns | 228ns | +98.28% | 1.201 |

## Performance model

- Peak throughput: **2.550 Gops/s** (hx_field__direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_field__direct | 2.377 | 93.2% |
| hx_field__hash | 1.178 | 46.2% |
| hx_field__linear | 1.202 | 47.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_field__direct | 2770ns | 2770ns | base |
| hx_field__hash | 2783ns | 2783ns | +0.50% |
| hx_field__linear | 2798ns | 2798ns | +1.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_field__direct | 108ns | base | --- | [101, 114] | --- | --- | --- | --- |
| hx_field__hash | 217ns | +108.3ns (+100.6%) | [+89, +125]ns | [195, 232] | YES | 0.0313 | 0.0313 | 0 |
| hx_field__linear | 213ns | +111.6ns (+103.7%) | [+91, +114]ns | [199, 228] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_field__direct | hx_field__hash | hx_field__linear |
|---|---|---|---|
| 1 | 106ns | +70.6% | +86.3% |
| 2 | 113ns | +107.8% | +99.3% |
| 3 | 100ns | +108.4% | +110.4% |
| 4 | 102ns | +124.9% | +110.2% |
| 5 | 114ns | +91.3% | +101.9% |
| 6 | 109ns | +98.8% | +82.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_field__direct | -0.213 | moderate- |
| hx_field__hash | -0.468 | moderate- |
| hx_field__linear | -0.491 | moderate- |

**Consistency summary:**

- **hx_field__hash**: won 0/6, lost 6/6
- **hx_field__linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_field__direct | 3.8ns | 107.5ns | 3.6% |  |
| hx_field__hash | 3.7ns | 215.0ns | 1.7% |  |
| hx_field__linear | 3.7ns | 213.1ns | 1.7% |  |

## Distribution (algo ns)

```
hx_field__direct (n=6, range 100.4-113.5 ns)
    100.4 |########################################
    101.1 |
    101.7 |########################################
    102.4 |
    103.0 |
    103.7 |
    104.3 |
    105.0 |
    105.6 |########################################
    106.3 |
    107.0 |
    107.6 |
    108.3 |
    108.9 |########################################
    109.6 |
    110.2 |
    110.9 |
    111.5 |
    112.2 |
    112.8 |########################################
  (0 below, 1 above range)

hx_field__hash (n=6, range 181.2-232.5 ns)
    181.2 |########################################
    183.8 |
    186.3 |
    188.9 |
    191.5 |
    194.0 |
    196.6 |
    199.2 |
    201.7 |
    204.3 |
    206.8 |########################################
    209.4 |
    212.0 |
    214.5 |########################################
    217.1 |########################################
    219.7 |
    222.2 |
    224.8 |
    227.4 |########################################
    229.9 |
  (0 below, 1 above range)

hx_field__linear (n=6, range 197.9-227.7 ns)
    197.9 |########################################
    199.4 |########################################
    200.9 |
    202.4 |
    203.9 |
    205.3 |
    206.8 |
    208.3 |
    209.8 |########################################
    211.3 |
    212.8 |
    214.3 |########################################
    215.8 |
    217.3 |
    218.8 |
    220.2 |
    221.7 |
    223.2 |
    224.7 |########################################
    226.2 |
  (0 below, 1 above range)

```
