# Interpreter dispatch shapes: switch vs fnptr-table vs computed

2 variants, 6 samples per variant.
Baseline: **hx_dispatch__switch**

## Highlights

Baseline for all deltas below: **hx_dispatch__switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_dispatch__switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_dispatch__switch has the worst median (191 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_dispatch__fnptr at 171 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_dispatch__fnptr dominates: 12% faster than the next best (hx_dispatch__switch)

hx_dispatch__fnptr (171 ns) leads hx_dispatch__switch (191 ns) by 12%, a clear separation rather than a photo finish. CV 13.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (20 ns) is smaller than the fastest variant's own run-to-run std-dev (22 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: hx_dispatch__fnptr** at 170.6 ns median (-10.5% vs baseline)
- Spread: 1.12x (fastest 170.6 ns, slowest 190.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_dispatch__fnptr | 2448ns | 2405ns | 2318ns | 2378ns | 2620ns | -9.67% |
| hx_dispatch__switch | 2711ns | 2437ns | 2330ns | 2407ns | 3356ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_dispatch__fnptr | 179ns | 163ns | 201ns | -15.57% | 0.358 |
| hx_dispatch__switch | 211ns | 187ns | 257ns | base | 0.303 |

## Performance model

- Peak throughput: **0.393 Gops/s** (hx_dispatch__fnptr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_dispatch__fnptr | 0.375 | 95.5% |
| hx_dispatch__switch | 0.336 | 85.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_dispatch__fnptr | 2448ns | 2448ns | -9.67% |
| hx_dispatch__switch | 2711ns | 2711ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_dispatch__switch | 191ns | base | --- | [187, 257] | --- | --- | --- | --- |
| hx_dispatch__fnptr | 171ns | no significant difference | [-87, +10]ns | [164, 201] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_dispatch__switch | hx_dispatch__fnptr |
|---|---|---|
| 1 | 192ns | +18.2% |
| 2 | 192ns | -11.1% |
| 3 | 321ns | -46.8% |
| 4 | 187ns | -12.7% |
| 5 | 187ns | -11.8% |
| 6 | 190ns | -7.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_dispatch__fnptr | 0.017 | ok |
| hx_dispatch__switch | -0.233 | moderate- |

**Consistency summary:**

- **hx_dispatch__fnptr**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_dispatch__fnptr | 3.4ns | 178.5ns | 1.9% |  |
| hx_dispatch__switch | 3.1ns | 211.5ns | 1.5% |  |

## Distribution (algo ns)

```
hx_dispatch__fnptr (n=6, range 162.9-201.1 ns)
    162.9 |########################################
    164.8 |########################################
    166.7 |
    168.6 |########################################
    170.5 |########################################
    172.4 |
    174.3 |########################################
    176.3 |
    178.2 |
    180.1 |
    182.0 |
    183.9 |
    185.8 |
    187.7 |
    189.6 |
    191.5 |
    193.4 |
    195.3 |
    197.2 |
    199.1 |
  (0 below, 1 above range)

hx_dispatch__switch (n=6, range 186.7-256.9 ns)
    186.7 |########################################
    190.2 |##########################
    193.7 |
    197.2 |
    200.7 |
    204.2 |
    207.7 |
    211.3 |
    214.8 |
    218.3 |
    221.8 |
    225.3 |
    228.8 |
    232.3 |
    235.8 |
    239.3 |
    242.8 |
    246.3 |
    249.8 |
    253.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **hx_dispatch__switch**: CV=23.2% (high variance, measurements may be unstable)
