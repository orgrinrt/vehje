# Incremental compilation: cold rebuild vs warm content-addressed reload after one edit

2 variants, 6 samples per variant.
Baseline: **incr_cold**

## Highlights

Baseline for all deltas below: **incr_cold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (incr_cold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline incr_cold has the worst median (130.78 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest incr_warm at 39.89 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### incr_warm dominates: 228% faster than the next best (incr_cold)

incr_warm (39.89 us) leads incr_cold (130.78 us) by 228%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### incr_warm beats baseline by 70% (significant)

incr_warm is -91.01 us (70%) faster than baseline incr_cold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 3.3x the fastest

Fastest incr_warm (39.89 us) to slowest incr_cold (130.78 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: incr_warm** at 39887.9 ns median (-69.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.28x (fastest 39887.9 ns, slowest 130775.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| incr_cold | 132474ns | 133081ns | 127475ns | 131964ns | 135737ns | base |
| incr_warm | 41424ns | 42375ns | 38996ns | 41359ns | 42736ns | -68.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| incr_cold | 130175ns | 125259ns | 133374ns | base | 0.002 |
| incr_warm | 39006ns | 36700ns | 40256ns | -70.04% | 0.007 |

## Performance model

- Peak throughput: **0.007 Gops/s** (incr_warm; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| incr_cold | 0.002 | 28.1% |
| incr_warm | 0.006 | 92.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| incr_cold | 132474ns | 132474ns | base |
| incr_warm | 41424ns | 41424ns | -68.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| incr_cold | 130775ns | base | --- | [126377, 133374] | --- | --- | --- | --- |
| incr_warm | 39888ns | -91011.6ns (-69.6%) | [-94788, -87708]ns | [36874, 40256] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | incr_cold | incr_warm |
|---|---|---|
| 1 | 131836ns | -71.9% |
| 2 | 125259ns | -68.1% |
| 3 | 131618ns | -69.3% |
| 4 | 134911ns | -70.3% |
| 5 | 129932ns | -69.3% |
| 6 | 127495ns | -71.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| incr_cold | -0.152 | ok |
| incr_warm | 0.001 | ok |

**Consistency summary:**

- **incr_warm**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| incr_cold | 56.6ns | 130175.2ns | 0.0% |  |
| incr_warm | 81.3ns | 39006.1ns | 0.2% |  |

## Distribution (algo ns)

```
incr_cold (n=6, range 125258.8-133373.7 ns)
  125258.8 |########################################
  125664.5 |
  126070.3 |
  126476.0 |
  126881.8 |
  127287.5 |########################################
  127693.3 |
  128099.0 |
  128504.8 |
  128910.5 |
  129316.2 |
  129722.0 |########################################
  130127.7 |
  130533.5 |
  130939.2 |
  131345.0 |########################################
  131750.7 |########################################
  132156.5 |
  132562.2 |
  132968.0 |
  (0 below, 1 above range)

incr_warm (n=6, range 36700.0-40256.4 ns)
  36700.0 |########################################
  36877.8 |########################################
  37055.6 |
  37233.5 |
  37411.3 |
  37589.1 |
  37766.9 |
  37944.8 |
  38122.6 |
  38300.4 |
  38478.2 |
  38656.0 |
  38833.9 |
  39011.7 |
  39189.5 |
  39367.3 |
  39545.2 |
  39723.0 |########################################
  39900.8 |########################################
  40078.6 |########################################
  (0 below, 1 above range)

```
