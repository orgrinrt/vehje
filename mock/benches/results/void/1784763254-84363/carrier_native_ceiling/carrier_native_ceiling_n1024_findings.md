# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (43.19 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 28.95 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 49% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (28.95 ms) leads carrier_ceil_interp (43.19 ms) by 49%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 33% (significant)

carrier_ceil_native is -14.21 ms (33%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 28953142.5 ns median (-33.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.49x (fastest 28953142.5 ns, slowest 43193523.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 43286667ns | 43198072ns | 43149529ns | 43191134ns | 43498535ns | base |
| carrier_ceil_native | 29215870ns | 28957640ns | 28897416ns | 28937736ns | 29792298ns | -32.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 43281959ns | 43144741ns | 43493840ns | base | 0.000 |
| carrier_ceil_native | 29211187ns | 28892583ns | 29787492ns | -32.51% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 66.9% |
| carrier_ceil_native | 0.000 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 43286667ns | 43286667ns | base |
| carrier_ceil_native | 29215870ns | 29215870ns | -32.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 43193524ns | base | --- | [43158513, 43493840] | --- | --- | --- | --- |
| carrier_ceil_native | 28953142ns | -14207054.3ns (-32.9%) | [-14362988, -13642273]ns | [28892928, 29787492] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 43185761ns | -33.1% |
| 2 | 43329437ns | -33.3% |
| 3 | 43144741ns | -32.8% |
| 4 | 43658242ns | -30.9% |
| 5 | 43172285ns | -33.1% |
| 6 | 43201287ns | -32.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | -0.499 | moderate- |
| carrier_ceil_native | -0.303 | moderate- |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 43267447.4ns | 43281958.9ns | 100.0% | HIGH |
| carrier_ceil_native | 28964312.5ns | 29211187.3ns | 99.2% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 43144741.2-43493839.6 ns)
  43144741.2 |########################################
  43162196.1 |########################################
  43179651.0 |########################################
  43197106.0 |########################################
  43214560.9 |
  43232015.8 |
  43249470.7 |
  43266925.6 |
  43284380.6 |
  43301835.5 |
  43319290.4 |########################################
  43336745.3 |
  43354200.2 |
  43371655.2 |
  43389110.1 |
  43406565.0 |
  43424019.9 |
  43441474.8 |
  43458929.8 |
  43476384.7 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 28892583.3-29787491.9 ns)
  28892583.3 |########################################
  28937328.7 |
  28982074.2 |#############
  29026819.6 |
  29071565.0 |
  29116310.4 |
  29161055.9 |
  29205801.3 |
  29250546.7 |
  29295292.1 |
  29340037.6 |
  29384783.0 |#############
  29429528.4 |
  29474273.9 |
  29519019.3 |
  29563764.7 |
  29608510.1 |
  29653255.6 |
  29698001.0 |
  29742746.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=100.0% of algo (FFI overhead may distort results)
