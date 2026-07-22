# Predecode: zero-copy wire decode vs flat predecoded form (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_predec_wire**

## Highlights

Baseline for all deltas below: **carrier_predec_wire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_predec_wire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_predec_wire has the worst median (9.84 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_predec_flat at 9.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: carrier_predec_flat** at 9188.8 ns median (-6.6% vs baseline)
- Spread: 1.07x (fastest 9188.8 ns, slowest 9841.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_predec_flat | 11604ns | 11767ns | 10503ns | 11767ns | 11910ns | -5.33% |
| carrier_predec_wire | 12257ns | 12243ns | 10635ns | 11974ns | 13493ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_predec_flat | 9078ns | 8265ns | 9319ns | -7.63% | 0.028 |
| carrier_predec_wire | 9828ns | 8522ns | 10810ns | base | 0.026 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_predec_flat; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_predec_flat | 0.028 | 89.9% |
| carrier_predec_wire | 0.026 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_predec_flat | 11604ns | 11604ns | -5.33% |
| carrier_predec_wire | 12257ns | 12257ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_predec_wire | 9841ns | base | --- | [8832, 10810] | --- | --- | --- | --- |
| carrier_predec_flat | 9189ns | no significant difference | [-1599, +357]ns | [8726, 9319] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_predec_wire | carrier_predec_flat |
|---|---|---|
| 1 | 8522ns | +7.8% |
| 2 | 11083ns | -17.1% |
| 3 | 10537ns | -10.3% |
| 4 | 9570ns | -13.6% |
| 5 | 10112ns | -9.1% |
| 6 | 9142ns | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_predec_flat | -0.382 | moderate- |
| carrier_predec_wire | -0.273 | moderate- |

**Consistency summary:**

- **carrier_predec_flat**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_predec_flat | 759.8ns | 9077.9ns | 8.4% | HIGH |
| carrier_predec_wire | 80.1ns | 9827.7ns | 0.8% |  |

## Distribution (algo ns)

```
carrier_predec_flat (n=6, range 8264.6-9318.5 ns)
   8264.6 |##########
   8317.3 |
   8370.0 |
   8422.7 |
   8475.4 |
   8528.1 |
   8580.8 |
   8633.5 |
   8686.2 |
   8738.9 |
   8791.6 |
   8844.3 |
   8897.0 |
   8949.7 |
   9002.4 |
   9055.1 |
   9107.8 |
   9160.5 |########################################
   9213.2 |
   9265.9 |
  (0 below, 1 above range)

carrier_predec_wire (n=6, range 8522.1-10810.0 ns)
   8522.1 |########################################
   8636.5 |
   8750.9 |
   8865.3 |
   8979.7 |
   9094.1 |########################################
   9208.5 |
   9322.9 |
   9437.3 |
   9551.7 |########################################
   9666.0 |
   9780.4 |
   9894.8 |
  10009.2 |########################################
  10123.6 |
  10238.0 |
  10352.4 |
  10466.8 |########################################
  10581.2 |
  10695.6 |
  (0 below, 1 above range)

```
