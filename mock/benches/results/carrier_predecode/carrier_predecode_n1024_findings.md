# Predecode: zero-copy wire decode vs flat predecoded form (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_predec_wire**

## Highlights

Baseline for all deltas below: **carrier_predec_wire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_predec_wire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_predec_wire has the worst median (39.02 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_predec_flatthread at 26.31 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_predec_flatthread dominates: 31% faster than the next best (carrier_predec_flat)

carrier_predec_flatthread (26.31 us) leads carrier_predec_flat (34.56 us) by 31%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_predec_flatthread beats baseline by 31% (significant)

carrier_predec_flatthread is -12.02 us (31%) faster than baseline carrier_predec_wire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_predec_flat is inconsistent: worst-20% is 1.9x its best-20%

carrier_predec_flat's best 20% of batches run at 31.02 us but its worst 20% at 57.58 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_predec_flatthread** at 26311.9 ns median (-32.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.48x (fastest 26311.9 ns, slowest 39019.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_predec_flat | 43733ns | 36973ns | 33201ns | 36471ns | 59893ns | +4.38% |
| carrier_predec_flatthread | 28666ns | 28576ns | 27364ns | 28492ns | 29578ns | -31.58% |
| carrier_predec_wire | 41898ns | 41429ns | 36996ns | 41149ns | 45472ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_predec_flat | 41419ns | 31018ns | 57576ns | +4.79% | 0.025 |
| carrier_predec_flatthread | 26392ns | 25183ns | 27231ns | -33.23% | 0.039 |
| carrier_predec_wire | 39526ns | 34794ns | 43065ns | base | 0.026 |

## Performance model

- Peak throughput: **0.041 Gops/s** (carrier_predec_flatthread; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_predec_flat | 0.030 | 72.9% |
| carrier_predec_flatthread | 0.039 | 95.7% |
| carrier_predec_wire | 0.026 | 64.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_predec_flat | 43733ns | 43733ns | +4.38% |
| carrier_predec_flatthread | 28666ns | 28666ns | -31.58% |
| carrier_predec_wire | 41898ns | 41898ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_predec_wire | 39020ns | base | --- | [36493, 43065] | --- | --- | --- | --- |
| carrier_predec_flat | 34560ns | no significant difference | [-5152, +14512]ns | [32119, 57576] | no | 0.6875 | 0.6875 | 0 |
| carrier_predec_flatthread | 26312ns | -12023.1ns (-30.8%) | [-16874, -10504]ns | [25634, 27231] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_predec_wire | carrier_predec_flat | carrier_predec_flatthread |
|---|---|---|---|
| 1 | 34794ns | -10.9% | -27.6% |
| 2 | 38723ns | -14.2% | -31.5% |
| 3 | 39333ns | +42.5% | -29.0% |
| 4 | 46796ns | +26.3% | -43.8% |
| 5 | 38192ns | -9.4% | -31.0% |
| 6 | 39317ns | -12.2% | -33.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_predec_flat | 0.189 | ok |
| carrier_predec_flatthread | -0.028 | ok |
| carrier_predec_wire | -0.088 | ok |

**Consistency summary:**

- **carrier_predec_flat**: won 4/6, lost 2/6
- **carrier_predec_flatthread**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_predec_flat | 2449.7ns | 41418.7ns | 5.9% | HIGH |
| carrier_predec_flatthread | 2381.8ns | 26392.2ns | 9.0% | HIGH |
| carrier_predec_wire | 159.3ns | 39525.8ns | 0.4% |  |

## Distribution (algo ns)

```
carrier_predec_flat (n=6, range 31018.3-57576.4 ns)
  31018.3 |####################
  32346.2 |####################
  33674.1 |########################################
  35002.0 |
  36329.9 |
  37657.8 |
  38985.7 |
  40313.7 |
  41641.6 |
  42969.5 |
  44297.4 |
  45625.3 |
  46953.2 |
  48281.1 |
  49609.0 |
  50936.9 |
  52264.8 |
  53592.7 |
  54920.6 |####################
  56248.5 |
  (0 below, 1 above range)

carrier_predec_flatthread (n=6, range 25183.3-27230.7 ns)
  25183.3 |########################################
  25285.7 |
  25388.0 |
  25490.4 |
  25592.8 |
  25695.1 |
  25797.5 |
  25899.9 |
  26002.2 |########################################
  26104.6 |
  26207.0 |########################################
  26309.3 |########################################
  26411.7 |
  26514.1 |########################################
  26616.4 |
  26718.8 |
  26821.2 |
  26923.5 |
  27025.9 |
  27128.3 |
  (0 below, 1 above range)

carrier_predec_wire (n=6, range 34794.2-43064.6 ns)
  34794.2 |####################
  35207.7 |
  35621.2 |
  36034.8 |
  36448.3 |
  36861.8 |
  37275.3 |
  37688.8 |
  38102.3 |####################
  38515.9 |####################
  38929.4 |########################################
  39342.9 |
  39756.4 |
  40169.9 |
  40583.4 |
  40997.0 |
  41410.5 |
  41824.0 |
  42237.5 |
  42651.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_predec_flat**: CV=27.8% (high variance, measurements may be unstable)
