# Predecode: zero-copy wire decode vs flat predecoded form (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_predec_wire**

## Highlights

Baseline for all deltas below: **carrier_predec_wire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_predec_wire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_predec_wire has the worst median (2.75 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_predec_flat at 2.20 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_predec_flat dominates: 25% faster than the next best (carrier_predec_wire)

carrier_predec_flat (2.20 us) leads carrier_predec_wire (2.75 us) by 25%, a clear separation rather than a photo finish. CV 11.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: carrier_predec_flat** at 2197.7 ns median (-20.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.25x (fastest 2197.7 ns, slowest 2746.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_predec_flat | 4632ns | 4772ns | 3880ns | 4576ns | 5091ns | -9.85% |
| carrier_predec_wire | 5138ns | 5542ns | 4146ns | 5122ns | 5657ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_predec_flat | 2135ns | 1778ns | 2383ns | -17.43% | 0.030 |
| carrier_predec_wire | 2585ns | 2047ns | 2923ns | base | 0.025 |

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_predec_flat; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_predec_flat | 0.029 | 80.9% |
| carrier_predec_wire | 0.023 | 64.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_predec_flat | 4632ns | 4632ns | -9.85% |
| carrier_predec_wire | 5138ns | 5138ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_predec_wire | 2747ns | base | --- | [2086, 2923] | --- | --- | --- | --- |
| carrier_predec_flat | 2198ns | -407.9ns (-14.9%) | [-725, -219]ns | [1823, 2383] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_predec_wire | carrier_predec_flat |
|---|---|---|
| 1 | 2125ns | -12.1% |
| 2 | 2047ns | -13.1% |
| 3 | 3092ns | -28.8% |
| 4 | 2747ns | -19.9% |
| 5 | 2747ns | -6.6% |
| 6 | 2754ns | -20.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_predec_flat | 0.329 | moderate+ |
| carrier_predec_wire | 0.131 | ok |

**Consistency summary:**

- **carrier_predec_flat**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_predec_flat | 321.8ns | 2134.6ns | 15.1% | HIGH |
| carrier_predec_wire | 59.9ns | 2585.2ns | 2.3% |  |

## Distribution (algo ns)

```
carrier_predec_flat (n=6, range 1778.3-2382.7 ns)
   1778.3 |#############
   1808.5 |
   1838.7 |#############
   1869.0 |
   1899.2 |
   1929.4 |
   1959.6 |
   1989.8 |
   2020.1 |
   2050.3 |
   2080.5 |
   2110.7 |
   2140.9 |
   2171.2 |########################################
   2201.4 |
   2231.6 |
   2261.8 |
   2292.0 |
   2322.3 |
   2352.5 |
  (0 below, 1 above range)

carrier_predec_wire (n=6, range 2047.1-2923.1 ns)
   2047.1 |####################
   2090.9 |####################
   2134.7 |
   2178.5 |
   2222.3 |
   2266.1 |
   2309.9 |
   2353.7 |
   2397.5 |
   2441.3 |
   2485.1 |
   2528.9 |
   2572.7 |
   2616.5 |
   2660.3 |
   2704.1 |########################################
   2747.9 |####################
   2791.7 |
   2835.5 |
   2879.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_predec_flat**: bridge=15.2% of algo (FFI overhead may distort results)
