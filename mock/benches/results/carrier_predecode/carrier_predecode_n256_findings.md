# Predecode: zero-copy wire decode vs flat predecoded form (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_predec_wire**

## Highlights

Baseline for all deltas below: **carrier_predec_wire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_predec_wire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_predec_wire has the worst median (10.82 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_predec_flatthread at 7.46 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_predec_flatthread dominates: 23% faster than the next best (carrier_predec_flat)

carrier_predec_flatthread (7.46 us) leads carrier_predec_flat (9.19 us) by 23%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_predec_flatthread beats baseline by 32% (significant)

carrier_predec_flatthread is -3.42 us (32%) faster than baseline carrier_predec_wire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_predec_flatthread** at 7460.4 ns median (-31.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.45x (fastest 7460.4 ns, slowest 10823.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_predec_flat | 11210ns | 11812ns | 9740ns | 11189ns | 11975ns | -14.40% |
| carrier_predec_flatthread | 9725ns | 10039ns | 8678ns | 9838ns | 10080ns | -25.74% |
| carrier_predec_wire | 13096ns | 13420ns | 11242ns | 13328ns | 13674ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_predec_flat | 8702ns | 7592ns | 9241ns | -17.36% | 0.029 |
| carrier_predec_flatthread | 7226ns | 6441ns | 7493ns | -31.37% | 0.035 |
| carrier_predec_wire | 10530ns | 9058ns | 10969ns | base | 0.024 |

## Performance model

- Peak throughput: **0.040 Gops/s** (carrier_predec_flatthread; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_predec_flat | 0.028 | 70.1% |
| carrier_predec_flatthread | 0.034 | 86.3% |
| carrier_predec_wire | 0.024 | 59.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_predec_flat | 11210ns | 11210ns | -14.40% |
| carrier_predec_flatthread | 9725ns | 9725ns | -25.74% |
| carrier_predec_wire | 13096ns | 13096ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_predec_wire | 10823ns | base | --- | [9798, 10969] | --- | --- | --- | --- |
| carrier_predec_flat | 9190ns | -1682.0ns (-15.5%) | [-2369, -1432]ns | [7675, 9241] | YES | 0.0313 | 0.0313 | 0 |
| carrier_predec_flatthread | 7460ns | -3417.2ns (-31.6%) | [-3515, -2978]ns | [6725, 7493] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_predec_wire | carrier_predec_flat | carrier_predec_flatthread |
|---|---|---|---|
| 1 | 10538ns | -27.9% | -33.5% |
| 2 | 10856ns | -14.4% | -31.2% |
| 3 | 10953ns | -16.1% | -31.5% |
| 4 | 10985ns | -16.3% | -31.9% |
| 5 | 10790ns | -14.8% | -31.0% |
| 6 | 9058ns | -14.4% | -28.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_predec_flat | -0.110 | ok |
| carrier_predec_flatthread | -0.036 | ok |
| carrier_predec_wire | 0.025 | ok |

**Consistency summary:**

- **carrier_predec_flat**: won 6/6, lost 0/6
- **carrier_predec_flatthread**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_predec_flat | 731.7ns | 8702.0ns | 8.4% | HIGH |
| carrier_predec_flatthread | 764.8ns | 7226.2ns | 10.6% | HIGH |
| carrier_predec_wire | 89.0ns | 10529.9ns | 0.8% |  |

## Distribution (algo ns)

```
carrier_predec_flat (n=6, range 7592.5-9241.2 ns)
   7592.5 |#############
   7674.9 |
   7757.4 |#############
   7839.8 |
   7922.2 |
   8004.7 |
   8087.1 |
   8169.6 |
   8252.0 |
   8334.4 |
   8416.9 |
   8499.3 |
   8581.8 |
   8664.2 |
   8746.6 |
   8829.1 |
   8911.5 |
   8993.9 |
   9076.4 |
   9158.8 |########################################
  (0 below, 1 above range)

carrier_predec_flatthread (n=6, range 6441.2-7493.0 ns)
   6441.2 |#############
   6493.8 |
   6546.4 |
   6599.0 |
   6651.6 |
   6704.1 |
   6756.7 |
   6809.3 |
   6861.9 |
   6914.5 |
   6967.1 |#############
   7019.7 |
   7072.2 |
   7124.8 |
   7177.4 |
   7230.0 |
   7282.6 |
   7335.2 |
   7387.8 |
   7440.4 |########################################
  (0 below, 1 above range)

carrier_predec_wire (n=6, range 9057.5-10969.0 ns)
   9057.5 |####################
   9153.1 |
   9248.6 |
   9344.2 |
   9439.8 |
   9535.4 |
   9630.9 |
   9726.5 |
   9822.1 |
   9917.7 |
  10013.2 |
  10108.8 |
  10204.4 |
  10299.9 |
  10395.5 |
  10491.1 |####################
  10586.7 |
  10682.2 |
  10777.8 |########################################
  10873.4 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_predec_flatthread**: bridge=10.5% of algo (FFI overhead may distort results)
