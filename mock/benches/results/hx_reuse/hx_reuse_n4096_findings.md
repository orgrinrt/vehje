# Record update: always-copy vs in-place-when-unique (exact-meet)

2 variants, 6 samples per variant.
Baseline: **hx_reuse__reuse**

## Highlights

Baseline for all deltas below: **hx_reuse__reuse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_reuse__reuse) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_reuse__reuse has the worst median (2.30 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_reuse__copy at 2.30 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (110 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: hx_reuse__copy** at 2297.9 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 2297.9 ns, slowest 2298.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reuse__copy | 4921ns | 4881ns | 4564ns | 4855ns | 5199ns | +1.80% |
| hx_reuse__reuse | 4834ns | 4883ns | 4020ns | 4881ns | 5171ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reuse__copy | 2315ns | 2150ns | 2445ns | +1.74% | 1.770 |
| hx_reuse__reuse | 2275ns | 1890ns | 2436ns | base | 1.800 |

## Performance model

- Peak throughput: **2.167 Gops/s** (hx_reuse__reuse; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reuse__copy | 1.782 | 82.2% |
| hx_reuse__reuse | 1.782 | 82.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reuse__copy | 4921ns | 4921ns | +1.80% |
| hx_reuse__reuse | 4834ns | 4834ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reuse__reuse | 2299ns | base | --- | [2090, 2436] | --- | --- | --- | --- |
| hx_reuse__copy | 2298ns | no significant difference | [-126, +199]ns | [2201, 2445] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reuse__reuse | hx_reuse__copy |
|---|---|---|
| 1 | 1890ns | +13.8% |
| 2 | 2298ns | -0.1% |
| 3 | 2366ns | +5.8% |
| 4 | 2506ns | -8.2% |
| 5 | 2299ns | -2.0% |
| 6 | 2290ns | +4.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reuse__copy | -0.092 | ok |
| hx_reuse__reuse | 0.095 | ok |

**Consistency summary:**

- **hx_reuse__copy**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reuse__copy | 2.8ns | 2314.5ns | 0.1% |  |
| hx_reuse__reuse | 3.7ns | 2275.0ns | 0.2% |  |

## Distribution (algo ns)

```
hx_reuse__copy (n=6, range 2150.0-2444.6 ns)
   2150.0 |########################################
   2164.7 |
   2179.5 |
   2194.2 |
   2208.9 |
   2223.7 |
   2238.4 |########################################
   2253.1 |
   2267.8 |
   2282.6 |########################################
   2297.3 |########################################
   2312.0 |
   2326.8 |
   2341.5 |
   2356.2 |
   2371.0 |########################################
   2385.7 |
   2400.4 |
   2415.1 |
   2429.9 |
  (0 below, 1 above range)

hx_reuse__reuse (n=6, range 1890.0-2436.0 ns)
   1890.0 |#############
   1917.3 |
   1944.6 |
   1971.9 |
   1999.2 |
   2026.5 |
   2053.8 |
   2081.1 |
   2108.4 |
   2135.7 |
   2163.0 |
   2190.3 |
   2217.6 |
   2244.9 |
   2272.2 |########################################
   2299.5 |
   2326.8 |
   2354.1 |#############
   2381.4 |
   2408.7 |
  (0 below, 1 above range)

```
