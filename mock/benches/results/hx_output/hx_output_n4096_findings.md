# Interpolation output: format-to-temp+copy vs format-in-place

2 variants, 6 samples per variant.
Baseline: **hx_output__inplace**

## Highlights

Baseline for all deltas below: **hx_output__inplace**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_output__inplace) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_output__inplace has the worst median (7.07 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_output__temp at 6.89 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_output__temp is fastest but the noisiest (CV 8.3%)

hx_output__temp wins on median (6.89 us) yet has the highest variance (CV 8.3%), while hx_output__inplace is the steadiest (CV 7.2%, 7.07 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (181 ns) is smaller than the fastest variant's own run-to-run std-dev (571 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader hx_output__temp vs stability leader hx_output__inplace (+3% speed for 1.1x steadier)

hx_output__temp is fastest (6.89 us, CV 8.3%); hx_output__inplace gives up 2.6% median for 1.1x lower variance (CV 7.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### hx_output__temp's edge over baseline is significant but tiny (-8 ns, 0.12%)

hx_output__temp differs from baseline hx_output__inplace by -8 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: hx_output__temp** at 6890.6 ns median (-2.6% vs baseline)
- Spread: 1.03x (fastest 6890.6 ns, slowest 7071.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_output__inplace | 9477ns | 9357ns | 8665ns | 9146ns | 10378ns | base |
| hx_output__temp | 9495ns | 9243ns | 8748ns | 9112ns | 10442ns | +0.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_output__inplace | 7108ns | 6482ns | 7736ns | base | 0.576 |
| hx_output__temp | 7106ns | 6540ns | 7854ns | -0.03% | 0.576 |

## Performance model

- Peak throughput: **0.632 Gops/s** (hx_output__inplace; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_output__inplace | 0.579 | 91.7% |
| hx_output__temp | 0.594 | 94.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_output__inplace | 9477ns | 9477ns | base |
| hx_output__temp | 9495ns | 9495ns | +0.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_output__inplace | 7072ns | base | --- | [6517, 7736] | --- | --- | --- | --- |
| hx_output__temp | 6891ns | no significant difference | [-225, +227]ns | [6574, 7854] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_output__inplace | hx_output__temp |
|---|---|---|
| 1 | 6552ns | +0.9% |
| 2 | 6924ns | -4.2% |
| 3 | 6482ns | +0.9% |
| 4 | 7612ns | +5.2% |
| 5 | 7860ns | -2.0% |
| 6 | 7220ns | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_output__inplace | 0.233 | moderate+ |
| hx_output__temp | 0.282 | moderate+ |

**Consistency summary:**

- **hx_output__temp**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_output__inplace | 3.0ns | 7108.4ns | 0.0% |  |
| hx_output__temp | 2.8ns | 7106.2ns | 0.0% |  |

## Distribution (algo ns)

```
hx_output__inplace (n=6, range 6482.1-7736.1 ns)
   6482.1 |########################################
   6544.8 |########################################
   6607.5 |
   6670.2 |
   6732.9 |
   6795.6 |
   6858.3 |
   6921.0 |########################################
   6983.7 |
   7046.4 |
   7109.1 |
   7171.8 |########################################
   7234.5 |
   7297.2 |
   7359.9 |
   7422.6 |
   7485.3 |
   7548.0 |
   7610.7 |########################################
   7673.4 |
  (0 below, 1 above range)

hx_output__temp (n=6, range 6540.4-7853.8 ns)
   6540.4 |####################
   6606.1 |########################################
   6671.7 |
   6737.4 |
   6803.1 |
   6868.7 |
   6934.4 |
   7000.1 |
   7065.7 |
   7131.4 |####################
   7197.1 |
   7262.7 |
   7328.4 |
   7394.1 |
   7459.7 |
   7525.4 |
   7591.1 |
   7656.7 |####################
   7722.4 |
   7788.1 |
  (0 below, 1 above range)

```
