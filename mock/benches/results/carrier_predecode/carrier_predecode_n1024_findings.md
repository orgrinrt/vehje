# Predecode: zero-copy wire decode vs flat predecoded form (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_predec_wire**

## Highlights

Baseline for all deltas below: **carrier_predec_wire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_predec_wire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_predec_wire has the worst median (38.22 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_predec_flat at 35.28 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.94 us) is smaller than the fastest variant's own run-to-run std-dev (10.40 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### carrier_predec_flat is inconsistent: worst-20% is 1.7x its best-20%

carrier_predec_flat's best 20% of batches run at 31.92 us but its worst 20% at 53.99 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_predec_flat** at 35276.7 ns median (-7.7% vs baseline)
- Spread: 1.08x (fastest 35276.7 ns, slowest 38216.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_predec_flat | 42745ns | 37599ns | 34104ns | 36498ns | 56436ns | -5.21% |
| carrier_predec_wire | 45097ns | 40575ns | 37463ns | 39865ns | 56761ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_predec_flat | 40418ns | 31921ns | 53987ns | -5.40% | 0.025 |
| carrier_predec_wire | 42725ns | 35271ns | 54234ns | base | 0.024 |

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_predec_flat; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_predec_flat | 0.029 | 90.5% |
| carrier_predec_wire | 0.027 | 83.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_predec_flat | 42745ns | 42745ns | -5.21% |
| carrier_predec_wire | 45097ns | 45097ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_predec_wire | 38216ns | base | --- | [35726, 54234] | --- | --- | --- | --- |
| carrier_predec_flat | 35277ns | no significant difference | [-7957, +5599]ns | [31991, 53987] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_predec_wire | carrier_predec_flat |
|---|---|---|
| 1 | 36180ns | -11.8% |
| 2 | 35271ns | +2.0% |
| 3 | 40389ns | -20.6% |
| 4 | 39446ns | -12.3% |
| 5 | 68078ns | -11.1% |
| 6 | 36986ns | +28.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_predec_flat | 0.229 | moderate+ |
| carrier_predec_wire | -0.196 | ok |

**Consistency summary:**

- **carrier_predec_flat**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_predec_flat | 2469.9ns | 40418.2ns | 6.1% | HIGH |
| carrier_predec_wire | 154.4ns | 42725.1ns | 0.4% |  |

## Distribution (algo ns)

```
carrier_predec_flat (n=6, range 31921.2-53986.9 ns)
  31921.2 |########################################
  33024.5 |
  34127.8 |####################
  35231.1 |####################
  36334.3 |
  37437.6 |
  38540.9 |
  39644.2 |
  40747.5 |
  41850.8 |
  42954.0 |
  44057.3 |
  45160.6 |
  46263.9 |
  47367.2 |####################
  48470.5 |
  49573.8 |
  50677.0 |
  51780.3 |
  52883.6 |
  (0 below, 1 above range)

carrier_predec_wire (n=6, range 35271.2-54233.5 ns)
  35271.2 |########################################
  36219.3 |####################
  37167.4 |
  38115.6 |
  39063.7 |####################
  40011.8 |####################
  40959.9 |
  41908.0 |
  42856.1 |
  43804.3 |
  44752.4 |
  45700.5 |
  46648.6 |
  47596.7 |
  48544.8 |
  49493.0 |
  50441.1 |
  51389.2 |
  52337.3 |
  53285.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_predec_flat**: CV=25.7% (high variance, measurements may be unstable)
- **carrier_predec_wire**: CV=26.9% (high variance, measurements may be unstable)
