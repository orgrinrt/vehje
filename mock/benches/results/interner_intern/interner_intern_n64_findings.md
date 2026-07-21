# Interner intern hot path: FNV vs FxHash x load factor 25% vs 75%

4 variants, 6 samples per variant.
Baseline: **intern_fnv_lf25**

## Highlights

Baseline for all deltas below: **intern_fnv_lf25**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (30 ns) is smaller than the fastest variant's own run-to-run std-dev (70 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (intern_fnv_lf25)

The baseline intern_fnv_lf25 is the fastest (381 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### intern_fnv_lf75 is inconsistent: worst-20% is 1.6x its best-20%

intern_fnv_lf75's best 20% of batches run at 348 ns but its worst 20% at 550 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (intern_fnv_lf25) is the fastest** at 381.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.08x (fastest 381.0 ns, slowest 411.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| intern_fnv_lf25 | 3002ns | 2886ns | 2496ns | 2773ns | 3598ns | base |
| intern_fnv_lf75 | 3200ns | 2993ns | 2525ns | 2868ns | 4036ns | +6.60% |
| intern_fx_lf25 | 2925ns | 2912ns | 2542ns | 2808ns | 3292ns | -2.57% |
| intern_fx_lf75 | 3090ns | 2904ns | 2540ns | 2788ns | 3817ns | +2.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| intern_fnv_lf25 | 401ns | 328ns | 483ns | base | 0.159 |
| intern_fnv_lf75 | 437ns | 348ns | 550ns | +8.98% | 0.146 |
| intern_fx_lf25 | 437ns | 363ns | 545ns | +8.95% | 0.146 |
| intern_fx_lf75 | 450ns | 368ns | 564ns | +12.26% | 0.142 |

## Performance model

- Peak throughput: **0.195 Gops/s** (intern_fnv_lf25; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| intern_fnv_lf25 | 0.168 | 86.0% |
| intern_fnv_lf75 | 0.160 | 81.8% |
| intern_fx_lf25 | 0.160 | 81.9% |
| intern_fx_lf75 | 0.156 | 79.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| intern_fnv_lf25 | 3002ns | 3002ns | base |
| intern_fnv_lf75 | 3200ns | 3200ns | +6.60% |
| intern_fx_lf25 | 2925ns | 2925ns | -2.57% |
| intern_fx_lf75 | 3090ns | 3090ns | +2.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| intern_fnv_lf25 | 381ns | base | --- | [340, 483] | --- | --- | --- | --- |
| intern_fnv_lf75 | 401ns | +22.3ns (+5.9%) | [+4, +82]ns | [362, 550] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| intern_fx_lf25 | 400ns | no significant difference | [-32, +121]ns | [367, 545] | no | 0.6875 | 0.6875 | 0 |
| intern_fx_lf75 | 411ns | +41.7ns (+10.9%) | [+25, +81]ns | [376, 564] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | intern_fnv_lf25 | intern_fnv_lf75 | intern_fx_lf25 | intern_fx_lf75 |
|---|---|---|---|---|
| 1 | 543ns | +25.7% | -5.1% | +20.1% |
| 2 | 422ns | -2.8% | -8.8% | +12.4% |
| 3 | 328ns | +6.1% | +10.9% | +12.3% |
| 4 | 352ns | +6.7% | +5.1% | +9.6% |
| 5 | 368ns | +6.0% | +56.0% | +4.4% |
| 6 | 394ns | +5.7% | +5.2% | +10.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| intern_fnv_lf25 | 0.233 | moderate+ |
| intern_fnv_lf75 | 0.069 | ok |
| intern_fx_lf25 | -0.200 | ok |
| intern_fx_lf75 | 0.235 | moderate+ |

**Consistency summary:**

- **intern_fnv_lf75**: won 1/6, lost 5/6
- **intern_fx_lf25**: won 2/6, lost 4/6
- **intern_fx_lf75**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| intern_fnv_lf25 | 1956.5ns | 401.3ns | 487.5% | HIGH |
| intern_fnv_lf75 | 830.8ns | 437.3ns | 190.0% | HIGH |
| intern_fx_lf25 | 1847.4ns | 437.2ns | 422.5% | HIGH |
| intern_fx_lf75 | 808.0ns | 450.5ns | 179.4% | HIGH |

## Distribution (algo ns)

```
intern_fnv_lf25 (n=6, range 327.5-482.9 ns)
    327.5 |########################################
    335.3 |
    343.0 |
    350.8 |########################################
    358.6 |
    366.4 |########################################
    374.1 |
    381.9 |
    389.7 |########################################
    397.4 |
    405.2 |
    413.0 |
    420.7 |########################################
    428.5 |
    436.3 |
    444.0 |
    451.8 |
    459.6 |
    467.4 |
    475.1 |
  (0 below, 1 above range)

intern_fnv_lf75 (n=6, range 347.5-549.5 ns)
    347.5 |####################
    357.6 |
    367.7 |####################
    377.8 |
    387.9 |####################
    398.0 |
    408.1 |########################################
    418.2 |
    428.3 |
    438.4 |
    448.5 |
    458.6 |
    468.7 |
    478.8 |
    488.9 |
    499.0 |
    509.1 |
    519.2 |
    529.3 |
    539.4 |
  (0 below, 1 above range)

intern_fx_lf25 (n=6, range 363.3-545.0 ns)
    363.3 |########################################
    372.4 |
    381.5 |####################
    390.6 |
    399.6 |
    408.7 |####################
    417.8 |
    426.9 |
    436.0 |
    445.1 |
    454.1 |
    463.2 |
    472.3 |
    481.4 |
    490.5 |
    499.6 |
    508.7 |####################
    517.7 |
    526.8 |
    535.9 |
  (0 below, 1 above range)

intern_fx_lf75 (n=6, range 367.9-563.8 ns)
    367.9 |####################
    377.7 |########################################
    387.5 |
    397.3 |
    407.1 |
    416.9 |
    426.7 |
    436.4 |####################
    446.2 |
    456.0 |
    465.8 |####################
    475.6 |
    485.4 |
    495.2 |
    505.0 |
    514.8 |
    524.6 |
    534.4 |
    544.2 |
    554.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **intern_fnv_lf25**: bridge=484.3% of algo (FFI overhead may distort results)
- **intern_fnv_lf75**: CV=25.6% (high variance, measurements may be unstable)
- **intern_fnv_lf75**: bridge=196.8% of algo (FFI overhead may distort results)
- **intern_fx_lf25**: bridge=448.4% of algo (FFI overhead may distort results)
- **intern_fx_lf75**: CV=21.6% (high variance, measurements may be unstable)
- **intern_fx_lf75**: bridge=188.2% of algo (FFI overhead may distort results)
