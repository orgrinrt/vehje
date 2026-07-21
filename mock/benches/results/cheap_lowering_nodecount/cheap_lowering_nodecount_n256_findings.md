# Cheap lowering node-count reduction: fold+CSE vs fold-only (metric = node count, not time)

2 variants, 6 samples per variant.
Baseline: **cl_foldonly**

## Highlights

Baseline for all deltas below: **cl_foldonly**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cl_foldonly dominates: 61% faster than the next best (cl_foldcse)

cl_foldonly (481 ns) leads cl_foldcse (772 ns) by 61%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cl_foldonly is fastest but the noisiest (CV 6.4%)

cl_foldonly wins on median (481 ns) yet has the highest variance (CV 6.4%), while cl_foldcse is the steadiest (CV 3.0%, 772 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (cl_foldonly)

The baseline cl_foldonly is the fastest (481 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (cl_foldonly) is the fastest** at 480.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.61x (fastest 480.8 ns, slowest 771.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cl_foldcse | 3292ns | 3352ns | 3148ns | 3288ns | 3370ns | +9.78% |
| cl_foldonly | 2998ns | 3062ns | 2546ns | 3058ns | 3136ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cl_foldcse | 761ns | 726ns | 782ns | +62.51% | 0.337 |
| cl_foldonly | 468ns | 399ns | 484ns | base | 0.547 |

## Performance model

- Peak throughput: **0.641 Gops/s** (cl_foldonly; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cl_foldcse | 0.332 | 51.7% |
| cl_foldonly | 0.532 | 83.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cl_foldcse | 3292ns | 3292ns | +9.78% |
| cl_foldonly | 2998ns | 2998ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cl_foldonly | 481ns | base | --- | [440, 484] | --- | --- | --- | --- |
| cl_foldcse | 772ns | +293.4ns (+61.0%) | [+269, +315]ns | [728, 782] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | cl_foldonly | cl_foldcse |
|---|---|---|
| 1 | 399ns | +81.8% |
| 2 | 480ns | +61.1% |
| 3 | 482ns | +59.9% |
| 4 | 486ns | +60.3% |
| 5 | 480ns | +63.3% |
| 6 | 481ns | +51.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cl_foldcse | -0.122 | ok |
| cl_foldonly | -0.005 | ok |

**Consistency summary:**

- **cl_foldcse**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cl_foldcse | 766.1ns | 760.8ns | 100.7% | HIGH |
| cl_foldonly | 416.7ns | 468.1ns | 89.0% | HIGH |

## Distribution (algo ns)

```
cl_foldcse (n=6, range 725.8-782.1 ns)
    725.8 |########################################
    728.6 |########################################
    731.4 |
    734.2 |
    737.1 |
    739.9 |
    742.7 |
    745.5 |
    748.3 |
    751.1 |
    754.0 |
    756.8 |
    759.6 |
    762.4 |
    765.2 |
    768.0 |########################################
    770.8 |########################################
    773.7 |
    776.5 |
    779.3 |########################################
  (0 below, 1 above range)

cl_foldonly (n=6, range 399.2-483.9 ns)
    399.2 |##########
    403.4 |
    407.7 |
    411.9 |
    416.1 |
    420.4 |
    424.6 |
    428.9 |
    433.1 |
    437.3 |
    441.6 |
    445.8 |
    450.0 |
    454.3 |
    458.5 |
    462.8 |
    467.0 |
    471.2 |
    475.5 |
    479.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **cl_foldcse**: bridge=100.9% of algo (FFI overhead may distort results)
- **cl_foldonly**: bridge=89.4% of algo (FFI overhead may distort results)
